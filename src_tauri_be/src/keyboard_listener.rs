/// macOS 키보드 이벤트 리스너
///
/// CGEvent API를 사용하여 시스템 전역 키보드 이벤트를 감지합니다.

#[cfg(target_os = "macos")]
use core_foundation::runloop::{kCFRunLoopCommonModes, CFRunLoop};
#[cfg(target_os = "macos")]
use core_graphics::event::{
    CGEventTap, CGEventTapLocation, CGEventTapOptions, CGEventTapPlacement,
    CGEventType, EventField, CGEventFlags,
};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use crate::hangul::HangulComposer;
use enigo::Keyboard;

// 텍스트 교체 중 플래그 (enigo 이벤트 무시용)
static REPLACING: AtomicBool = AtomicBool::new(false);

/// 입력 버퍼 상태
#[derive(Debug, Clone)]
struct BufferState {
    /// 현재 버퍼 활성화 여부 (# 입력 후)
    active: bool,
    /// 누적된 문자열
    buffer: String,
    /// 한글 조합 상태
    hangul: HangulComposer,
}

impl BufferState {
    fn new() -> Self {
        Self {
            active: false,
            buffer: String::new(),
            hangul: HangulComposer::new(),
        }
    }

    fn start(&mut self) {
        self.active = true;
        self.buffer.clear();
        self.buffer.push('#');
        self.hangul.clear();
    }

    fn push(&mut self, c: char) {
        if self.active {
            self.buffer.push(c);
        }
    }

    fn pop(&mut self) {
        if !self.active {
            return;
        }

        // 조합 중인 글자가 있으면 조합만 지우고, 없으면 버퍼의 마지막 글자 제거
        if !self.hangul.is_empty() {
            // 조합 중인 글자 전체 지우기 (글자 단위)
            self.hangul.clear();
        } else if self.buffer.len() > 1 {
            // # 까지는 제거하지 않음
            self.buffer.pop();
        }
    }

    fn clear(&mut self) {
        self.active = false;
        self.buffer.clear();
        self.hangul.clear();
    }

    fn get_pattern(&self) -> String {
        let mut result = self.buffer.clone();
        // 현재 조합 중인 글자가 있으면 추가
        if let Some(composed) = self.hangul.to_char() {
            result.push(composed);
        }
        result
    }

    /// 문자 입력 처리 (한글 조합 포함)
    fn input_char(&mut self, c: char) {
        if !self.active {
            return;
        }

        // 한글 자모인지 확인
        if crate::hangul::char_to_jamo(c).is_some() {
            // 한글 자모인 경우: 조합 시도
            let (completed, _composing) = self.hangul.input(c);
            if let Some(ch) = completed {
                self.buffer.push(ch);
            }
        } else {
            // 한글 자모가 아닌 경우 (숫자, 콜론, 공백 등)
            // 먼저 조합 중인 한글 완성
            if let Some(ch) = self.hangul.to_char() {
                self.buffer.push(ch);
                self.hangul.clear();
            }
            // 그 다음 문자 추가
            self.buffer.push(c);
        }
    }
}

/// 키보드 레이아웃을 미리 가져오기 (메인 스레드에서 안전하게 호출)
#[cfg(target_os = "macos")]
unsafe fn get_keyboard_layout() -> *const u8 {
    use core_foundation::base::TCFType;
    use core_foundation::string::CFString;

    type TISInputSourceRef = *mut std::os::raw::c_void;
    type CFDataRef = *const std::os::raw::c_void;

    #[link(name = "Carbon", kind = "framework")]
    extern "C" {
        fn TISCopyCurrentKeyboardInputSource() -> TISInputSourceRef;
        fn TISCopyCurrentASCIICapableKeyboardLayoutInputSource() -> TISInputSourceRef;
        fn TISGetInputSourceProperty(
            source: TISInputSourceRef,
            property_key: core_foundation::string::CFStringRef,
        ) -> CFDataRef;
        fn CFDataGetBytePtr(data: CFDataRef) -> *const u8;
    }

    let property_key = CFString::from_static_string("TISPropertyUnicodeKeyLayoutData");

    // 먼저 현재 입력 소스 시도
    let mut input_source = TISCopyCurrentKeyboardInputSource();

    if input_source.is_null() {
        println!("⚠️  현재 입력 소스를 가져올 수 없습니다");
        return std::ptr::null();
    }

    let mut layout_data = TISGetInputSourceProperty(input_source, property_key.as_concrete_TypeRef());

    // 현재 입력 소스가 레이아웃 데이터를 제공하지 않으면 ASCII capable 소스 시도
    if layout_data.is_null() {
        println!("⚠️  현재 입력 소스에서 레이아웃 데이터 없음, ASCII capable 소스 시도");
        input_source = TISCopyCurrentASCIICapableKeyboardLayoutInputSource();

        if input_source.is_null() {
            println!("⚠️  ASCII capable 입력 소스를 가져올 수 없습니다");
            return std::ptr::null();
        }

        layout_data = TISGetInputSourceProperty(input_source, property_key.as_concrete_TypeRef());

        if layout_data.is_null() {
            println!("⚠️  ASCII capable 소스에서도 레이아웃 데이터 없음");
            return std::ptr::null();
        }
    }

    let layout_ptr = CFDataGetBytePtr(layout_data);

    if layout_ptr.is_null() {
        println!("⚠️  키보드 레이아웃 포인터가 null입니다");
        return std::ptr::null();
    }

    println!("✅ 키보드 레이아웃 로딩 완료");
    layout_ptr
}

/// UCKeyTranslate를 사용한 keycode -> unicode 변환 (사전 로딩된 레이아웃 사용)
#[cfg(target_os = "macos")]
unsafe fn extract_unicode_from_keycode(
    layout_ptr: *const u8,
    keycode: i64,
    flags: core_graphics::event::CGEventFlags,
) -> Option<String> {
    #[link(name = "Carbon", kind = "framework")]
    extern "C" {
        fn LMGetKbdType() -> u8;
        fn UCKeyTranslate(
            layout_ptr: *const u8,
            keycode: u16,
            key_action: u16,
            modifier_state: u32,
            keyboard_type: u32,
            key_translate_options: u32,
            dead_key_state: *mut u32,
            max_string_length: usize,
            actual_string_length: *mut usize,
            unicode_string: *mut u16,
        ) -> i32;
    }

    if layout_ptr.is_null() {
        return None;
    }

    // Modifier 플래그 변환 (Carbon Event Manager 값)
    let mut modifier_state: u32 = 0;
    if flags.contains(core_graphics::event::CGEventFlags::CGEventFlagShift) {
        modifier_state |= 0x0200; // shiftKey (bit 9)
    }
    if flags.contains(core_graphics::event::CGEventFlags::CGEventFlagAlternate) {
        modifier_state |= 0x0800; // optionKey (bit 11)
    }
    if flags.contains(core_graphics::event::CGEventFlags::CGEventFlagControl) {
        modifier_state |= 0x1000; // controlKey (bit 12)
    }
    if flags.contains(core_graphics::event::CGEventFlags::CGEventFlagCommand) {
        modifier_state |= 0x0100; // cmdKey (bit 8)
    }

    let mut dead_key_state: u32 = 0;
    let mut unicode_string: [u16; 4] = [0; 4];
    let mut actual_length: usize = 0;

    let result = UCKeyTranslate(
        layout_ptr,
        keycode as u16,
        2, // kUCKeyActionDown
        modifier_state >> 8, // modifier state를 upper byte로 이동
        LMGetKbdType() as u32,
        0,
        &mut dead_key_state,
        4,
        &mut actual_length,
        unicode_string.as_mut_ptr(),
    );

    if result != 0 || actual_length == 0 {
        return None;
    }

    // UTF-16 -> String 변환
    let text = String::from_utf16(&unicode_string[..actual_length]).ok()?;

    if !text.is_empty() {
        println!("🔍 추출: {:?} (keycode: {})", text, keycode);
    }

    Some(text)
}

/// 영문을 한글로 변환 (한영 전환 실수 대응)
///
/// 예: "ckd" -> "창"
fn convert_english_to_hangul(input: &str) -> String {
    let mut result = String::new();
    let mut composer = HangulComposer::new();

    for ch in input.chars() {
        // # 문자는 그대로 유지
        if ch == '#' {
            // 조합 중인 글자가 있으면 완성
            if let Some(completed) = composer.to_char() {
                result.push(completed);
                composer.clear();
            }
            result.push('#');
            continue;
        }

        // 한글 자모로 변환 가능한지 확인
        if crate::hangul::char_to_jamo(ch).is_some() {
            let (completed, _composing) = composer.input(ch);
            if let Some(completed_char) = completed {
                result.push(completed_char);
            }
        } else {
            // 한글 자모가 아니면 (숫자, 콜론, 공백 등) 먼저 조합 중인 글자 완성
            if let Some(completed) = composer.to_char() {
                result.push(completed);
                composer.clear();
            }
            result.push(ch);
        }
    }

    // 마지막 조합 중인 글자 완성
    if let Some(completed) = composer.to_char() {
        result.push(completed);
    }

    result
}

/// 키 이벤트 처리
///
/// UCKeyTranslate를 사용하여 keycode를 unicode로 변환하고 버퍼를 업데이트합니다.
#[cfg(target_os = "macos")]
fn handle_key_event(
    layout_ptr: *const u8,
    keycode: i64,
    flags: core_graphics::event::CGEventFlags,
    buffer: &mut BufferState,
) {
    // macOS 키코드 매핑
    const KEY_BACKSPACE: i64 = 51;
    const KEY_ENTER: i64 = 36;
    const KEY_SPACE: i64 = 49;
    const KEY_ESCAPE: i64 = 53;

    match keycode {
        KEY_BACKSPACE => {
            buffer.pop();
            println!("🔙 Backspace (buffer: {})", buffer.get_pattern());
        }
        KEY_ENTER => {
            if buffer.active {
                let pattern = buffer.get_pattern();
                println!("✨ 패턴 감지: {}", pattern);

                // 버퍼 비활성화 (enigo 백스페이스 이벤트 무시용)
                buffer.active = false;

                // DB 경로
                let mut db_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
                db_path.pop(); // src_tauri_be -> bible_easy
                db_path.push("bible.db");
                let db_path_str = db_path.to_str().unwrap();

                // 1. 원본 패턴으로 시도
                let mut verse_result = crate::bible_verse::get_verse(db_path_str, &pattern);

                // 2. 실패하면 영문→한글 변환 후 시도
                if verse_result.is_err() {
                    let converted = convert_english_to_hangul(&pattern);
                    if converted != pattern {
                        println!("🔄 영문→한글 변환: {} -> {}", pattern, converted);
                        verse_result = crate::bible_verse::get_verse(db_path_str, &converted);
                    }
                }

                // 3. 성공하면 텍스트 교체
                match verse_result {
                    Ok(verse_text) => {
                        println!("✅ 구절 찾음: {}", verse_text);

                        // enigo로 텍스트 교체 (패턴 삭제 + 구절 삽입)
                        let pattern_len = pattern.chars().count();
                        println!("🔢 패턴: '{}' (길이: {} 글자)", pattern, pattern_len);
                        let mut enigo = enigo::Enigo::new(&enigo::Settings::default()).unwrap();

                        // 패턴 삭제: Core Graphics API로 직접 백스페이스 전송
                        println!("⌫ CGEvent로 백스페이스 {}번 전송", pattern_len);

                        use core_graphics::event::{CGEvent, CGEventTapLocation, CGKeyCode};
                        use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

                        let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState).unwrap();
                        const BACKSPACE_KEYCODE: CGKeyCode = 51;

                        for i in 0..pattern_len {
                            // KeyDown 이벤트
                            if let Ok(key_down) = CGEvent::new_keyboard_event(source.clone(), BACKSPACE_KEYCODE, true) {
                                key_down.post(CGEventTapLocation::HID);
                            }

                            std::thread::sleep(std::time::Duration::from_millis(20));

                            // KeyUp 이벤트
                            if let Ok(key_up) = CGEvent::new_keyboard_event(source.clone(), BACKSPACE_KEYCODE, false) {
                                key_up.post(CGEventTapLocation::HID);
                            }

                            std::thread::sleep(std::time::Duration::from_millis(80));

                            if (i + 1) % 2 == 0 || i == pattern_len - 1 {
                                println!("  ⌫ {} / {} 완료", i + 1, pattern_len);
                            }
                        }
                        println!("  ✅ 삭제 완료");

                        // 백스페이스 완료 후 텍스트 삽입 시작 - 이제 플래그 설정
                        REPLACING.store(true, Ordering::SeqCst);

                        // 구절 삽입
                        let _ = enigo.text(&verse_text);

                        println!("🎉 텍스트 교체 완료!");

                        // 200ms 후 플래그 해제 (별도 스레드)
                        std::thread::spawn(|| {
                            std::thread::sleep(std::time::Duration::from_millis(200));
                            REPLACING.store(false, Ordering::SeqCst);
                            println!("🔓 텍스트 교체 종료, 리스너 재활성화");
                        });
                    }
                    Err(e) => {
                        println!("⚠️  패턴 매칭 실패: {:?}", e);
                    }
                }

                buffer.clear();
            }
        }
        KEY_ESCAPE => {
            if buffer.active {
                println!("❌ 입력 취소");
                buffer.clear();
            }
        }
        _ => {
            // Unicode 문자열 추출 (사전 로딩된 레이아웃 사용)
            let unicode_result = unsafe { extract_unicode_from_keycode(layout_ptr, keycode, flags) };

            // 디버깅: 추출 결과 로깅
            match &unicode_result {
                Some(text) if !text.is_empty() => {
                    for ch in text.chars() {
                        // # 트리거 감지
                        if ch == '#' && !buffer.active {
                            buffer.start();
                            println!("🎯 트리거 시작: #");
                        } else if buffer.active {
                            // 버퍼가 활성화된 상태에서만 문자 추가 (한글 조합 포함)
                            buffer.input_char(ch);
                            println!("⌨️  입력: {} (buffer: {})", ch, buffer.get_pattern());
                        }
                    }
                }
                Some(_text) => {
                    // 빈 문자열 (무시)
                }
                None => {
                    // 추출 실패 (modifier 키 등)
                }
            }
        }
    }
}

/// 키보드 리스너 시작
///
/// 백그라운드 스레드에서 실행되며, 모든 키보드 이벤트를 감지합니다.
#[cfg(target_os = "macos")]
pub fn start_listener() {
    println!("🎧 키보드 리스너 시작...");

    // 키보드 레이아웃을 미리 로딩 (메인 스레드에서 안전하게 호출)
    let layout_ptr = unsafe { get_keyboard_layout() };

    if layout_ptr.is_null() {
        eprintln!("❌ 키보드 레이아웃을 로딩할 수 없습니다");
        return;
    }

    // 포인터를 usize로 변환 (Send를 위해)
    let layout_addr = layout_ptr as usize;

    // 버퍼 상태
    let buffer = Arc::new(Mutex::new(BufferState::new()));

    std::thread::spawn(move || {
        let buffer_clone = buffer.clone();

        // usize를 다시 포인터로 변환
        let layout_ptr = layout_addr as *const u8;

        // 감지할 이벤트 타입 (키 다운만)
        let event_types = vec![CGEventType::KeyDown];

        // 이벤트 탭 생성 (콜백은 클로저로)
        match CGEventTap::new(
                CGEventTapLocation::HID,
                CGEventTapPlacement::HeadInsertEventTap,
                CGEventTapOptions::Default,
                event_types,
                move |_proxy, event_type, event| {
                    // 텍스트 교체 중이면 모든 이벤트 무시 (enigo 이벤트 필터링)
                    if REPLACING.load(Ordering::SeqCst) {
                        return None;
                    }

                    // 키 다운 이벤트만 처리
                    match event_type {
                        CGEventType::KeyDown => {
                            // 키 코드 추출
                            let keycode = event.get_integer_value_field(EventField::KEYBOARD_EVENT_KEYCODE);

                            // Flags 확인 (Shift, Command 등)
                            let flags = event.get_flags();

                            // 버퍼 접근
                            if let Ok(mut buf) = buffer_clone.lock() {
                                handle_key_event(layout_ptr, keycode, flags, &mut buf);
                            }
                        }
                        _ => {}
                    }

                    // 이벤트를 다음으로 전달 (None = 차단하지 않음)
                    None
                },
            ) {
                Ok(tap) => {
                    println!("✅ 이벤트 탭 생성 성공");

                    // Run loop에 추가
                    let loop_source = tap
                        .mach_port
                        .create_runloop_source(0)
                        .expect("Failed to create runloop source");

                    let current_loop = CFRunLoop::get_current();
                    unsafe {
                        current_loop.add_source(&loop_source, kCFRunLoopCommonModes);
                    }

                    // 탭 활성화
                    tap.enable();

                    println!("🚀 키보드 리스너 실행 중... (모든 키 입력 로깅)");

                    // Run loop 시작 (무한 루프)
                    CFRunLoop::run_current();
                }
                Err(()) => {
                    eprintln!("❌ 이벤트 탭 생성 실패!");
                    eprintln!("💡 '손쉬운 사용' 권한이 필요합니다.");
                    eprintln!("   시스템 설정 > 개인정보 보호 > 손쉬운 사용 > Bible Easy 허용");
                }
            }
    });
}

// macOS가 아닌 플랫폼에서는 빈 구현
#[cfg(not(target_os = "macos"))]
pub fn start_listener() {
    println!("⚠️  키보드 리스너는 macOS에서만 지원됩니다.");
}
