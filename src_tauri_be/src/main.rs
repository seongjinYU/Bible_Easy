// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// 모듈 선언
mod bible_verse;
mod book_mappings;
mod db_query;
mod pattern_parser;
mod settings;
mod accessibility;
mod keyboard_listener;
mod hangul;

use std::path::PathBuf;

/// DB 파일 경로를 반환합니다.
fn get_db_path() -> String {
    // TODO: 빌드 후에는 앱 리소스 경로로 변경 필요
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop(); // src_tauri_be -> bible_easy
    path.push("bible.db");
    path.to_str().unwrap().to_string()
}

/// 성경 구절을 조회하는 Tauri Command
///
/// # Arguments
/// * `pattern` - 사용자 입력 패턴 (예: "#창 1:1")
///
/// # Returns
/// * `Ok(String)` - 조회된 구절 텍스트
/// * `Err(String)` - 에러 메시지
#[tauri::command]
fn get_bible_verse(pattern: String) -> Result<String, String> {
    let db_path = get_db_path();

    bible_verse::get_verse(&db_path, &pattern).map_err(|e| format!("{:?}", e))
}

/// 앱 설정을 불러오는 Tauri Command
///
/// # Returns
/// * `AppSettings` - 현재 설정
#[tauri::command]
fn get_settings() -> settings::AppSettings {
    settings::load_settings()
}

/// 앱 설정을 저장하는 Tauri Command
///
/// # Arguments
/// * `settings` - 저장할 설정
///
/// # Returns
/// * `Ok(())` - 저장 성공
/// * `Err(String)` - 저장 실패
#[tauri::command]
fn update_settings(settings: settings::AppSettings) -> Result<(), String> {
    settings::save_settings(&settings)
}

/// 접근성 권한 확인 Tauri Command
///
/// # Returns
/// * `true` - 권한 있음
/// * `false` - 권한 없음
#[tauri::command]
fn check_accessibility() -> bool {
    accessibility::check_accessibility_permission()
}

/// 접근성 권한 요청 Tauri Command
///
/// 시스템 설정 창을 엽니다.
#[tauri::command]
fn request_accessibility() {
    accessibility::request_accessibility_permission();
}

fn main() {
    // 키보드 리스너 시작
    keyboard_listener::start_listener();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_bible_verse,
            get_settings,
            update_settings,
            check_accessibility,
            request_accessibility
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
