/// 성경 구절 데이터베이스 조회 모듈
///
/// SQLite 데이터베이스에서 성경 구절을 조회하는 함수들을 제공합니다.
use rusqlite::{Connection, Result};

/// 성경 구절을 조회합니다 (단일 절).
///
/// # Arguments
/// * `db_path` - SQLite 데이터베이스 파일 경로
/// * `book_abbrev` - 성경 책 약어 (예: "gn", "ex")
/// * `chapter` - 장 번호 (1부터 시작)
/// * `verse` - 절 번호 (1부터 시작)
///
/// # Returns
/// * `Ok(String)` - 조회된 구절 텍스트
/// * `Err(rusqlite::Error)` - 조회 실패 시 에러
///
/// # Example
/// ```
/// let verse = query_verse("bible.db", "gn", 1, 1)?;
/// assert_eq!(verse, "태초에 하나님이 천지를 창조하시니라");
/// ```
pub fn query_verse(db_path: &str, book_abbrev: &str, chapter: u32, verse: u32) -> Result<String> {
    let conn = Connection::open(db_path)?;

    let text: String = conn.query_row(
        "SELECT text FROM verses WHERE book_abbrev = ?1 AND chapter = ?2 AND verse = ?3",
        [book_abbrev, &chapter.to_string(), &verse.to_string()],
        |row| row.get(0),
    )?;

    Ok(text)
}

/// 성경 구절을 범위 조회합니다 (여러 절).
///
/// # Arguments
/// * `db_path` - SQLite 데이터베이스 파일 경로
/// * `book_abbrev` - 성경 책 약어 (예: "gn", "ex")
/// * `chapter` - 장 번호 (1부터 시작)
/// * `verse_start` - 시작 절 번호
/// * `verse_end` - 끝 절 번호
///
/// # Returns
/// * `Ok((String, u32, u32))` - (조회된 구절 텍스트, 실제 시작 절, 실제 끝 절)
/// * `Err(rusqlite::Error)` - 조회 실패 시 에러
///
/// # Example
/// ```
/// let (verses, start, end) = query_verse_range("bible.db", "gn", 1, 1, 3)?;
/// // verses: "1 태초에 하나님이 천지를 창조하시니라 2 땅이 혼돈하고..."
/// // start: 1, end: 3
/// ```
pub fn query_verse_range(
    db_path: &str,
    book_abbrev: &str,
    chapter: u32,
    verse_start: u32,
    verse_end: u32,
) -> Result<(String, u32, u32)> {
    let conn = Connection::open(db_path)?;

    let mut stmt = conn.prepare(
        "SELECT verse, text FROM verses
         WHERE book_abbrev = ?1 AND chapter = ?2 AND verse >= ?3 AND verse <= ?4
         ORDER BY verse",
    )?;

    let verses = stmt.query_map(
        [
            book_abbrev,
            &chapter.to_string(),
            &verse_start.to_string(),
            &verse_end.to_string(),
        ],
        |row| {
            let verse_num: u32 = row.get(0)?;
            let text: String = row.get(1)?;
            Ok((verse_num, format!("{} {}", verse_num, text)))
        },
    )?;

    let mut result = Vec::new();
    let mut verse_numbers = Vec::new();
    for verse in verses {
        let (num, text) = verse?;
        verse_numbers.push(num);
        result.push(text);
    }

    if result.is_empty() {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    let actual_start = *verse_numbers.first().unwrap();
    let actual_end = *verse_numbers.last().unwrap();

    Ok((result.join(" "), actual_start, actual_end))
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
    fn test_query_verse_genesis_1_1() {
        let db_path = get_test_db_path();

        let result = query_verse(&db_path, "gn", 1, 1);

        assert!(result.is_ok(), "DB 조회 실패: {:?}", result.err());

        let verse = result.unwrap();
        assert_eq!(verse, "태초에 하나님이 천지를 창조하시니라");
    }

    #[test]
    fn test_query_verse_john_3_16() {
        let db_path = get_test_db_path();

        let result = query_verse(&db_path, "jo", 3, 16);

        assert!(result.is_ok(), "DB 조회 실패: {:?}", result.err());

        let verse = result.unwrap();
        assert!(verse.contains("하나님이"));
        assert!(verse.contains("세상을"));
    }

    #[test]
    fn test_query_verse_not_found() {
        let db_path = get_test_db_path();

        // 존재하지 않는 구절 조회
        let result = query_verse(&db_path, "gn", 999, 999);

        assert!(result.is_err(), "존재하지 않는 구절이 조회됨");
    }

    #[test]
    fn test_query_verse_invalid_book() {
        let db_path = get_test_db_path();

        // 존재하지 않는 책 약어
        let result = query_verse(&db_path, "invalid", 1, 1);

        assert!(result.is_err(), "존재하지 않는 책이 조회됨");
    }

    #[test]
    fn test_query_verse_range_genesis() {
        let db_path = get_test_db_path();

        // 창세기 1:1-3
        let result = query_verse_range(&db_path, "gn", 1, 1, 3);

        assert!(result.is_ok(), "범위 조회 실패: {:?}", result.err());

        let (text, start, end) = result.unwrap();
        assert_eq!(start, 1);
        assert_eq!(end, 3);
        assert!(text.contains("1 태초에"));
        assert!(text.contains("2 땅이"));
        assert!(text.contains("3 하나님이 가라사대 빛이"));
    }

    #[test]
    fn test_query_verse_range_single() {
        let db_path = get_test_db_path();

        // 단일 절 범위 (1:1-1)
        let result = query_verse_range(&db_path, "gn", 1, 1, 1);

        assert!(result.is_ok());
        let (text, start, end) = result.unwrap();
        assert_eq!(start, 1);
        assert_eq!(end, 1);
        assert!(text.contains("1 태초에"));
    }

    #[test]
    fn test_query_verse_range_not_found() {
        let db_path = get_test_db_path();

        // 존재하지 않는 범위
        let result = query_verse_range(&db_path, "gn", 999, 1, 10);

        assert!(result.is_err(), "존재하지 않는 범위가 조회됨");
    }
}
