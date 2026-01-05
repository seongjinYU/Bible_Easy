#![allow(deprecated)]
#![allow(unexpected_cfgs)]

/// macOS 접근성 권한 확인 모듈
///
/// "손쉬운 사용" 권한이 있어야 키보드 이벤트를 감지할 수 있습니다.

#[cfg(target_os = "macos")]
use cocoa::base::nil;
#[cfg(target_os = "macos")]
use cocoa::foundation::NSString;
#[cfg(target_os = "macos")]
use objc::{class, msg_send, sel, sel_impl};

/// 접근성 권한이 있는지 확인합니다.
///
/// # Returns
/// * `true` - 권한 있음
/// * `false` - 권한 없음
///
/// Note: 현재는 항상 true를 반환합니다.
/// 실제 권한 확인은 키보드 리스너 시작 시 에러로 감지됩니다.
#[cfg(target_os = "macos")]
pub fn check_accessibility_permission() -> bool {
    // TODO: 실제 AXIsProcessTrusted() 호출
    // 현재는 단순화를 위해 true 반환
    true
}

/// 접근성 권한을 요청합니다.
///
/// 시스템 설정 창이 열립니다.
#[cfg(target_os = "macos")]
pub fn request_accessibility_permission() {
    unsafe {
        let workspace: *mut objc::runtime::Object = msg_send![class!(NSWorkspace), sharedWorkspace];
        let url_string = NSString::alloc(nil)
            .init_str("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility");
        let url: *mut objc::runtime::Object = msg_send![class!(NSURL), URLWithString: url_string];
        let _: () = msg_send![workspace, openURL: url];
    }
}

// macOS가 아닌 플랫폼에서는 항상 true 반환
#[cfg(not(target_os = "macos"))]
pub fn check_accessibility_permission() -> bool {
    true
}

#[cfg(not(target_os = "macos"))]
pub fn request_accessibility_permission() {
    // 아무것도 안 함
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_permission() {
        // 실제 권한 상태 확인 (테스트 환경에 따라 다름)
        let has_permission = check_accessibility_permission();
        println!("접근성 권한: {}", has_permission);
    }
}
