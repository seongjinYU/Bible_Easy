/// 앱 설정 관리 모듈
///
/// 사용자 설정을 저장하고 불러오는 기능을 제공합니다.
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// 앱 설정 구조체
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    /// 기능 활성화 여부
    pub enabled: bool,
    /// 자동 시작 (추후 구현)
    pub auto_start: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            auto_start: false,
        }
    }
}

/// 트리거 문자 (고정)
pub const TRIGGER_CHAR: &str = "#";

/// 설정 파일 경로를 반환합니다.
fn get_settings_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("BibleEasy");
    fs::create_dir_all(&path).ok();
    path.push("settings.json");
    path
}

/// 설정을 불러옵니다.
///
/// # Returns
/// * `AppSettings` - 저장된 설정 또는 기본 설정
pub fn load_settings() -> AppSettings {
    let path = get_settings_path();

    if let Ok(content) = fs::read_to_string(&path) {
        if let Ok(settings) = serde_json::from_str(&content) {
            return settings;
        }
    }

    // 파일이 없거나 파싱 실패 시 기본값 반환
    AppSettings::default()
}

/// 설정을 저장합니다.
///
/// # Arguments
/// * `settings` - 저장할 설정
///
/// # Returns
/// * `Ok(())` - 저장 성공
/// * `Err(String)` - 저장 실패
pub fn save_settings(settings: &AppSettings) -> Result<(), String> {
    let path = get_settings_path();

    let json = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;

    fs::write(&path, json).map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = AppSettings::default();
        assert!(settings.enabled);
        assert!(!settings.auto_start);
    }

    #[test]
    fn test_trigger_char_constant() {
        assert_eq!(TRIGGER_CHAR, "#");
    }

    #[test]
    fn test_settings_serialization() {
        let settings = AppSettings::default();
        let json = serde_json::to_string(&settings).unwrap();
        let deserialized: AppSettings = serde_json::from_str(&json).unwrap();

        assert_eq!(settings.enabled, deserialized.enabled);
        assert_eq!(settings.auto_start, deserialized.auto_start);
    }

    #[test]
    fn test_save_and_load_settings() {
        let mut settings = AppSettings::default();
        settings.enabled = false;

        // 저장
        save_settings(&settings).unwrap();

        // 불러오기
        let loaded = load_settings();
        assert_eq!(loaded.enabled, false);
    }
}
