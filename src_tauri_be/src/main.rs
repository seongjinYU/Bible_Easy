// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// TODO: 여기에 성경 구절 자동완성 기능을 구현할 예정
// - 키보드 후킹
// - 패턴 분석 (#창 1:1)
// - DB 조회
// - 텍스트 치환

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
