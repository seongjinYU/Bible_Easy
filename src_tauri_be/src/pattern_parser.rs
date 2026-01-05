/// 성경 구절 패턴 분석 모듈
///
/// 사용자가 입력한 패턴(예: "#창 1:1")을 파싱하여 책 약어, 장, 절을 추출합니다.
use crate::book_mappings::create_korean_to_english_mapping;
use regex::Regex;
use std::sync::OnceLock;

/// 파싱 에러 타입
#[derive(Debug, PartialEq)]
pub enum ParseError {
    /// 패턴 형식이 잘못됨
    InvalidFormat,
    /// 책 이름을 찾을 수 없음
    BookNotFound(String),
    /// 장/절 번호가 잘못됨
    InvalidNumber,
}

/// 파싱 결과 타입 (book_abbrev, chapter, verse_start, verse_end)
/// verse_end가 None이면 단일 절, Some이면 범위 조회
pub type ParseResult = Result<(String, u32, u32, Option<u32>), ParseError>;

/// 패턴 매칭용 정규표현식 (싱글톤)
fn get_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| {
        // 패턴: #[한글책이름][공백?][장][공백?]:[공백?][절][공백?](-[공백?][절])?
        // # 바로 뒤에 한글이 와야 함 (공백 불허)
        // 예: #창 1:1, #창세기 20 : 1, #창1:1-3, #삼상 17:45-47
        Regex::new(r"^#([가-힣]+)\s*(\d+)\s*:\s*(\d+)(?:\s*-\s*(\d+))?$").unwrap()
    })
}

/// 성경 구절 패턴을 파싱합니다.
///
/// # Arguments
/// * `input` - 입력 문자열 (예: "#창 1:1", "#창세기 1:1-3")
///
/// # Returns
/// * `Ok((book_abbrev, chapter, verse_start, verse_end))` - 파싱 성공
/// * `Err(ParseError)` - 파싱 실패
///
/// # Example
/// ```
/// let result = parse_pattern("#창 1:1")?;
/// assert_eq!(result, ("gn".to_string(), 1, 1, None));
///
/// let result = parse_pattern("#창 1:1-3")?;
/// assert_eq!(result, ("gn".to_string(), 1, 1, Some(3)));
/// ```
pub fn parse_pattern(input: &str) -> ParseResult {
    let re = get_regex();

    // 정규표현식 매칭
    let captures = re.captures(input).ok_or(ParseError::InvalidFormat)?;

    // 그룹 추출
    let korean_book = captures.get(1).unwrap().as_str();
    let chapter_str = captures.get(2).unwrap().as_str();
    let verse_start_str = captures.get(3).unwrap().as_str();
    let verse_end_str = captures.get(4).map(|m| m.as_str());

    // 한글 책 이름 → 영어 약어 변환
    let mapping = create_korean_to_english_mapping();
    let english_abbrev = mapping
        .get(korean_book)
        .ok_or_else(|| ParseError::BookNotFound(korean_book.to_string()))?;

    // 장/절 번호 파싱
    let chapter: u32 = chapter_str.parse().map_err(|_| ParseError::InvalidNumber)?;
    let verse_start: u32 = verse_start_str.parse().map_err(|_| ParseError::InvalidNumber)?;
    let verse_end: Option<u32> = verse_end_str
        .map(|s| s.parse().map_err(|_| ParseError::InvalidNumber))
        .transpose()?;

    Ok((english_abbrev.clone(), chapter, verse_start, verse_end))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pattern_basic() {
        // 창세기 1:1 (단일 절)
        let result = parse_pattern("#창 1:1");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ("gn".to_string(), 1, 1, None));
    }

    #[test]
    fn test_parse_pattern_range() {
        // 창세기 1:1-3 (범위)
        let result = parse_pattern("#창 1:1-3");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ("gn".to_string(), 1, 1, Some(3)));

        // 시편 119:1-8
        let result = parse_pattern("#시 119:1-8");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ("ps".to_string(), 119, 1, Some(8)));
    }

    #[test]
    fn test_parse_pattern_full_name() {
        // 창세기 풀네임
        let result = parse_pattern("#창세기 1:1");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ("gn".to_string(), 1, 1, None));

        // 범위도 지원
        let result = parse_pattern("#창세기 1:1-5");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ("gn".to_string(), 1, 1, Some(5)));
    }

    #[test]
    fn test_parse_pattern_no_space() {
        // 공백 없이
        let result = parse_pattern("#창1:1");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ("gn".to_string(), 1, 1, None));

        // 범위도 공백 없이
        let result = parse_pattern("#창1:1-3");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ("gn".to_string(), 1, 1, Some(3)));
    }

    #[test]
    fn test_parse_pattern_special_books() {
        // 사무엘상
        let result = parse_pattern("#삼상 17:47");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ("1sm".to_string(), 17, 47, None));

        // 요한일서
        let result = parse_pattern("#요일 3:16");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ("1jo".to_string(), 3, 16, None));
    }

    #[test]
    fn test_parse_pattern_various_books() {
        // 출애굽기
        let result = parse_pattern("#출 20:3");
        assert_eq!(result, Ok(("ex".to_string(), 20, 3, None)));

        // 요한복음
        let result = parse_pattern("#요 3:16");
        assert_eq!(result, Ok(("jo".to_string(), 3, 16, None)));

        // 로마서
        let result = parse_pattern("#롬 8:28");
        assert_eq!(result, Ok(("rm".to_string(), 8, 28, None)));

        // 계시록
        let result = parse_pattern("#계 21:1");
        assert_eq!(result, Ok(("re".to_string(), 21, 1, None)));
    }

    #[test]
    fn test_parse_pattern_invalid_format() {
        // # 없음
        let result = parse_pattern("창 1:1");
        assert_eq!(result, Err(ParseError::InvalidFormat));

        // : 없음
        let result = parse_pattern("#창 1 1");
        assert_eq!(result, Err(ParseError::InvalidFormat));

        // 완전히 잘못된 형식
        let result = parse_pattern("hello world");
        assert_eq!(result, Err(ParseError::InvalidFormat));
    }

    #[test]
    fn test_parse_pattern_book_not_found() {
        // 존재하지 않는 책
        let result = parse_pattern("#없는책 1:1");
        assert!(matches!(result, Err(ParseError::BookNotFound(_))));
    }

    #[test]
    fn test_parse_pattern_large_numbers() {
        // 큰 장/절 번호
        let result = parse_pattern("#시 119:176");
        assert_eq!(result, Ok(("ps".to_string(), 119, 176, None)));

        // 큰 번호 범위
        let result = parse_pattern("#시 119:170-176");
        assert_eq!(result, Ok(("ps".to_string(), 119, 170, Some(176))));
    }

    #[test]
    fn test_parse_pattern_with_various_spaces() {
        // # 뒤 공백은 불허 (실패해야 함)
        let result = parse_pattern("# 창세기 20:1");
        assert_eq!(result, Err(ParseError::InvalidFormat));

        // # 바로 뒤 한글 + 장 앞 공백 (허용)
        let result = parse_pattern("#창세기 20 :1");
        assert_eq!(result, Ok(("gn".to_string(), 20, 1, None)));

        // : 뒤 공백 (허용)
        let result = parse_pattern("#창세기 20: 1");
        assert_eq!(result, Ok(("gn".to_string(), 20, 1, None)));

        // 장/:절 사이 공백 (허용)
        let result = parse_pattern("#창세기 20 : 1");
        assert_eq!(result, Ok(("gn".to_string(), 20, 1, None)));

        // 범위에 공백 (허용)
        let result = parse_pattern("#창 1 : 1 - 3");
        assert_eq!(result, Ok(("gn".to_string(), 1, 1, Some(3))));
    }
}
