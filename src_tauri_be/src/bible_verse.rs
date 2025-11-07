/// 성경 구절 조회 통합 모듈
///
/// 패턴 파싱 + DB 조회를 통합하여 사용자 입력부터 구절 텍스트까지 전체 플로우를 처리합니다.
use crate::book_mappings::create_english_to_korean_mapping;
use crate::db_query;
use crate::pattern_parser::{parse_pattern, ParseError};

/// 통합 조회 에러 타입
#[derive(Debug)]
#[allow(dead_code)] // 에러 필드는 Debug 포맷으로 사용됨
pub enum VerseError {
    /// 패턴 파싱 실패
    ParseError(ParseError),
    /// DB 조회 실패
    DatabaseError(rusqlite::Error),
}

impl From<ParseError> for VerseError {
    fn from(err: ParseError) -> Self {
        VerseError::ParseError(err)
    }
}

impl From<rusqlite::Error> for VerseError {
    fn from(err: rusqlite::Error) -> Self {
        VerseError::DatabaseError(err)
    }
}

/// 사용자 입력 패턴으로부터 성경 구절을 조회합니다.
///
/// # Arguments
/// * `db_path` - SQLite 데이터베이스 파일 경로
/// * `pattern` - 사용자 입력 패턴 (예: "#창 1:1", "#창 1:1-3")
///
/// # Returns
/// * `Ok(String)` - 조회된 구절 텍스트
/// * `Err(VerseError)` - 파싱 또는 조회 실패
///
/// # Example
/// ```
/// let verse = get_verse("bible.db", "#창 1:1")?;
/// assert_eq!(verse, "태초에 하나님이 천지를 창조하시니라 [창세기 1:1]");
///
/// let verses = get_verse("bible.db", "#창 1:1-3")?;
/// // "1 태초에... 2 땅이... 3 하나님이... [창세기 1:1-3]"
/// ```
pub fn get_verse(db_path: &str, pattern: &str) -> Result<String, VerseError> {
    // 1. 패턴 파싱
    let (book_abbrev, chapter, verse_start, verse_end) = parse_pattern(pattern)?;

    // 2. DB 조회
    let (text, actual_verse_start, actual_verse_end) = match verse_end {
        // 범위 조회
        Some(end) if end != verse_start => {
            let (text, start, end) = db_query::query_verse_range(db_path, &book_abbrev, chapter, verse_start, end)?;
            (text, start, Some(end))
        }
        // 같은 절 범위 (1:31-31) 또는 단일 절 조회
        _ => {
            let text = db_query::query_verse(db_path, &book_abbrev, chapter, verse_start)?;
            (text, verse_start, None)
        }
    };

    // 3. 참조 표시 생성
    let book_name_map = create_english_to_korean_mapping();
    let korean_name = book_name_map
        .get(&book_abbrev)
        .unwrap_or(&book_abbrev)
        .clone();

    let reference = match actual_verse_end {
        Some(end) => format!("[{} {}:{}-{}]", korean_name, chapter, actual_verse_start, end),
        None => format!("[{} {}:{}]", korean_name, chapter, actual_verse_start),
    };

    // 4. 구절 + 참조 반환
    Ok(format!("{} {}", text, reference))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// 테스트용 DB 경로를 반환합니다.
    fn get_test_db_path() -> String {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.pop(); // src_tauri_be -> bible_easy
        path.push("bible.db");
        path.to_str().unwrap().to_string()
    }

    #[test]
    fn test_get_verse_genesis_1_1() {
        let db_path = get_test_db_path();

        let result = get_verse(&db_path, "#창 1:1");

        assert!(result.is_ok(), "전체 플로우 실패: {:?}", result.err());
        let text = result.unwrap();
        assert!(text.contains("태초에 하나님이 천지를 창조하시니라"));
        assert!(text.contains("[창세기 1:1]"));
    }

    #[test]
    fn test_get_verse_genesis_full_name() {
        let db_path = get_test_db_path();

        let result = get_verse(&db_path, "#창세기 1:1");

        assert!(result.is_ok());
        let text = result.unwrap();
        assert!(text.contains("태초에 하나님이 천지를 창조하시니라"));
        assert!(text.contains("[창세기 1:1]"));
    }

    #[test]
    fn test_get_verse_john_3_16() {
        let db_path = get_test_db_path();

        let result = get_verse(&db_path, "#요 3:16");

        assert!(result.is_ok());
        let verse = result.unwrap();
        assert!(verse.contains("하나님이"));
        assert!(verse.contains("세상을"));
        assert!(verse.contains("독생자"));
        assert!(verse.contains("[요한복음 3:16]"));
    }

    #[test]
    fn test_get_verse_various_books() {
        let db_path = get_test_db_path();

        // 출애굽기 20:3
        let result = get_verse(&db_path, "#출 20:3");
        assert!(result.is_ok());
        let verse = result.unwrap();
        assert!(verse.contains("나 외에는"));
        assert!(verse.contains("[출애굽기 20:3]"));

        // 시편 23:1
        let result = get_verse(&db_path, "#시 23:1");
        assert!(result.is_ok());
        let verse = result.unwrap();
        assert!(verse.contains("여호와"));
        assert!(verse.contains("목자"));
        assert!(verse.contains("[시편 23:1]"));

        // 로마서 8:28
        let result = get_verse(&db_path, "#롬 8:28");
        assert!(result.is_ok());
        let verse = result.unwrap();
        assert!(verse.contains("하나님"));
        assert!(verse.contains("선"));
        assert!(verse.contains("[로마서 8:28]"));

        // 계시록 21:1
        let result = get_verse(&db_path, "#계 21:1");
        assert!(result.is_ok());
        let verse = result.unwrap();
        assert!(verse.contains("새 하늘"));
        assert!(verse.contains("새 땅"));
        assert!(verse.contains("[요한계시록 21:1]"));
    }

    #[test]
    fn test_get_verse_special_books() {
        let db_path = get_test_db_path();

        // 사무엘상 17:47
        let result = get_verse(&db_path, "#삼상 17:47");
        assert!(result.is_ok());
        let verse = result.unwrap();
        assert!(verse.contains("[사무엘상 17:47]"));

        // 고린도전서 13:13
        let result = get_verse(&db_path, "#고전 13:13");
        assert!(result.is_ok());
        let verse = result.unwrap();
        assert!(verse.contains("믿음"));
        assert!(verse.contains("소망"));
        assert!(verse.contains("사랑"));
        assert!(verse.contains("[고린도전서 13:13]"));
    }

    #[test]
    fn test_get_verse_no_space() {
        let db_path = get_test_db_path();

        // 공백 없이
        let result = get_verse(&db_path, "#창1:1");
        assert!(result.is_ok());
        let verse = result.unwrap();
        assert!(verse.contains("태초에 하나님이 천지를 창조하시니라"));
        assert!(verse.contains("[창세기 1:1]"));
    }

    #[test]
    fn test_get_verse_invalid_pattern() {
        let db_path = get_test_db_path();

        // 잘못된 패턴
        let result = get_verse(&db_path, "창 1:1"); // # 없음
        assert!(result.is_err());
        assert!(matches!(result, Err(VerseError::ParseError(_))));
    }

    #[test]
    fn test_get_verse_book_not_found() {
        let db_path = get_test_db_path();

        // 존재하지 않는 책
        let result = get_verse(&db_path, "#없는책 1:1");
        assert!(result.is_err());
        assert!(matches!(result, Err(VerseError::ParseError(_))));
    }

    #[test]
    fn test_get_verse_verse_not_found() {
        let db_path = get_test_db_path();

        // 존재하지 않는 구절
        let result = get_verse(&db_path, "#창 999:999");
        assert!(result.is_err());
        assert!(matches!(result, Err(VerseError::DatabaseError(_))));
    }

    #[test]
    fn test_get_verse_range_basic() {
        let db_path = get_test_db_path();

        // 창세기 1:1-3 범위 조회
        let result = get_verse(&db_path, "#창 1:1-3");

        assert!(result.is_ok(), "범위 조회 실패: {:?}", result.err());

        let text = result.unwrap();
        assert!(text.contains("1 태초에"));
        assert!(text.contains("2 땅이"));
        assert!(text.contains("3 하나님이"));
        assert!(text.contains("[창세기 1:1-3]"));
    }

    #[test]
    fn test_get_verse_range_full_name() {
        let db_path = get_test_db_path();

        // 풀네임으로 범위 조회
        let result = get_verse(&db_path, "#창세기 1:1-5");

        assert!(result.is_ok());
        let text = result.unwrap();
        assert!(text.contains("1 태초에"));
        assert!(text.contains("5 빛을 낮이라"));
        assert!(text.contains("[창세기 1:1-5]"));
    }

    #[test]
    fn test_get_verse_range_no_space() {
        let db_path = get_test_db_path();

        // 공백 없이 범위 조회
        let result = get_verse(&db_path, "#요3:16-17");

        assert!(result.is_ok());
        let text = result.unwrap();
        assert!(text.contains("16"));
        assert!(text.contains("17"));
        assert!(text.contains("[요한복음 3:16-17]"));
    }

    #[test]
    fn test_get_verse_range_long() {
        let db_path = get_test_db_path();

        // 긴 범위 (시편 23편 전체: 1-6)
        let result = get_verse(&db_path, "#시 23:1-6");

        assert!(result.is_ok());
        let text = result.unwrap();
        assert!(text.contains("1"));
        assert!(text.contains("목자"));
        assert!(text.contains("6"));
        assert!(text.contains("[시편 23:1-6]"));
    }

    #[test]
    fn test_get_verse_same_range() {
        let db_path = get_test_db_path();

        // 같은 절 범위 (1:31-31) → 단일 절처럼 표시되어야 함
        let result = get_verse(&db_path, "#창 1:31-31");

        assert!(result.is_ok());
        let text = result.unwrap();
        assert!(text.contains("하나님이"));
        // [창세기 1:31-31]이 아닌 [창세기 1:31]로 표시되어야 함
        assert!(text.contains("[창세기 1:31]"));
        assert!(!text.contains("[창세기 1:31-31]"));
    }

    #[test]
    fn test_get_verse_range_exceeding() {
        let db_path = get_test_db_path();

        // 창세기 1장은 31절까지만 존재, 32절 요청
        let result = get_verse(&db_path, "#창 1:30-32");

        assert!(result.is_ok());
        let text = result.unwrap();
        // 30절과 31절만 조회되어야 함
        assert!(text.contains("30"));
        assert!(text.contains("31"));
        // [창세기 1:30-32]가 아닌 [창세기 1:30-31]로 표시되어야 함
        assert!(text.contains("[창세기 1:30-31]"));
        assert!(!text.contains("[창세기 1:30-32]"));
    }

    #[test]
    fn test_get_verse_with_spaces() {
        let db_path = get_test_db_path();

        // # 뒤 공백은 불허 (실패해야 함)
        let result = get_verse(&db_path, "# 창세기 20:1");
        assert!(result.is_err());
        assert!(matches!(result, Err(VerseError::ParseError(_))));

        // 나머지 공백은 허용됨
        let result1 = get_verse(&db_path, "#창세기 20 :1").unwrap();
        let result2 = get_verse(&db_path, "#창세기 20: 1").unwrap();
        let result3 = get_verse(&db_path, "#창세기 20 : 1").unwrap();

        // 모든 결과가 동일해야 함
        assert_eq!(result1, result2);
        assert_eq!(result2, result3);
    }
}
