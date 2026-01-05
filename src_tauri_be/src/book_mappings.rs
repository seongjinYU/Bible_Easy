/// 성경 66권 한글 약어 → 영어 약어 매핑
///
/// 사용자가 입력하는 한글 약어(1자 또는 풀네임)를 bible.json의 영어 약어로 변환합니다.
///
/// # 규칙
/// - 1자 약어: 창, 출, 레, 민, 신 등
/// - 풀네임: 창세기, 출애굽기, 레위기 등
/// - 특수 케이스: 사무엘상/하, 열왕기상/하, 역대상/하 등은 "삼상", "왕상", "대상" 형식
///
/// # 예시
/// ```
/// "창" → "gn"
/// "창세기" → "gn"
/// "삼상" → "1sm"
/// "사무엘상" → "1sm"
/// ```
use std::collections::HashMap;

/// 한글 약어 → 영어 약어 매핑을 생성합니다.
#[allow(dead_code)] // TODO: 패턴 분석 로직에서 사용할 예정
pub fn create_korean_to_english_mapping() -> HashMap<String, String> {
    let mut map = HashMap::new();

    // ========== 구약 39권 ==========

    // 창세기
    map.insert("창".to_string(), "gn".to_string());
    map.insert("창세기".to_string(), "gn".to_string());

    // 출애굽기
    map.insert("출".to_string(), "ex".to_string());
    map.insert("출애굽기".to_string(), "ex".to_string());

    // 레위기
    map.insert("레".to_string(), "lv".to_string());
    map.insert("레위기".to_string(), "lv".to_string());

    // 민수기
    map.insert("민".to_string(), "nm".to_string());
    map.insert("민수기".to_string(), "nm".to_string());

    // 신명기
    map.insert("신".to_string(), "dt".to_string());
    map.insert("신명기".to_string(), "dt".to_string());

    // 여호수아
    map.insert("수".to_string(), "js".to_string());
    map.insert("여호수아".to_string(), "js".to_string());

    // 사사기
    map.insert("삿".to_string(), "jud".to_string());
    map.insert("사사기".to_string(), "jud".to_string());

    // 룻기
    map.insert("룻".to_string(), "rt".to_string());
    map.insert("룻기".to_string(), "rt".to_string());

    // 사무엘상
    map.insert("삼상".to_string(), "1sm".to_string());
    map.insert("사무엘상".to_string(), "1sm".to_string());

    // 사무엘하
    map.insert("삼하".to_string(), "2sm".to_string());
    map.insert("사무엘하".to_string(), "2sm".to_string());

    // 열왕기상
    map.insert("왕상".to_string(), "1kgs".to_string());
    map.insert("열왕기상".to_string(), "1kgs".to_string());

    // 열왕기하
    map.insert("왕하".to_string(), "2kgs".to_string());
    map.insert("열왕기하".to_string(), "2kgs".to_string());

    // 역대상
    map.insert("대상".to_string(), "1ch".to_string());
    map.insert("역대상".to_string(), "1ch".to_string());

    // 역대하
    map.insert("대하".to_string(), "2ch".to_string());
    map.insert("역대하".to_string(), "2ch".to_string());

    // 에스라
    map.insert("스".to_string(), "ezr".to_string());
    map.insert("에스라".to_string(), "ezr".to_string());

    // 느헤미야
    map.insert("느".to_string(), "ne".to_string());
    map.insert("느헤미야".to_string(), "ne".to_string());

    // 에스더
    map.insert("에".to_string(), "et".to_string());
    map.insert("에스더".to_string(), "et".to_string());

    // 욥기
    map.insert("욥".to_string(), "job".to_string());
    map.insert("욥기".to_string(), "job".to_string());

    // 시편
    map.insert("시".to_string(), "ps".to_string());
    map.insert("시편".to_string(), "ps".to_string());

    // 잠언
    map.insert("잠".to_string(), "prv".to_string());
    map.insert("잠언".to_string(), "prv".to_string());

    // 전도서
    map.insert("전".to_string(), "ec".to_string());
    map.insert("전도서".to_string(), "ec".to_string());

    // 아가
    map.insert("아".to_string(), "so".to_string());
    map.insert("아가".to_string(), "so".to_string());

    // 이사야
    map.insert("사".to_string(), "is".to_string());
    map.insert("이사야".to_string(), "is".to_string());

    // 예레미야
    map.insert("렘".to_string(), "jr".to_string());
    map.insert("예레미야".to_string(), "jr".to_string());

    // 예레미야애가
    map.insert("애".to_string(), "lm".to_string());
    map.insert("예레미야애가".to_string(), "lm".to_string());
    map.insert("애가".to_string(), "lm".to_string());

    // 에스겔
    map.insert("겔".to_string(), "ez".to_string());
    map.insert("에스겔".to_string(), "ez".to_string());

    // 다니엘
    map.insert("단".to_string(), "dn".to_string());
    map.insert("다니엘".to_string(), "dn".to_string());

    // 호세아
    map.insert("호".to_string(), "ho".to_string());
    map.insert("호세아".to_string(), "ho".to_string());

    // 요엘
    map.insert("욜".to_string(), "jl".to_string());
    map.insert("요엘".to_string(), "jl".to_string());

    // 아모스
    map.insert("암".to_string(), "am".to_string());
    map.insert("아모스".to_string(), "am".to_string());

    // 오바댜
    map.insert("옵".to_string(), "ob".to_string());
    map.insert("오바댜".to_string(), "ob".to_string());

    // 요나
    map.insert("욘".to_string(), "jn".to_string());
    map.insert("요나".to_string(), "jn".to_string());

    // 미가
    map.insert("미".to_string(), "mi".to_string());
    map.insert("미가".to_string(), "mi".to_string());

    // 나훔
    map.insert("나".to_string(), "na".to_string());
    map.insert("나훔".to_string(), "na".to_string());

    // 하박국
    map.insert("합".to_string(), "hk".to_string());
    map.insert("하박국".to_string(), "hk".to_string());

    // 스바냐
    map.insert("습".to_string(), "zp".to_string());
    map.insert("스바냐".to_string(), "zp".to_string());

    // 학개
    map.insert("학".to_string(), "hg".to_string());
    map.insert("학개".to_string(), "hg".to_string());

    // 스가랴
    map.insert("슥".to_string(), "zc".to_string());
    map.insert("스가랴".to_string(), "zc".to_string());

    // 말라기
    map.insert("말".to_string(), "ml".to_string());
    map.insert("말라기".to_string(), "ml".to_string());

    // ========== 신약 27권 ==========

    // 마태복음
    map.insert("마".to_string(), "mt".to_string());
    map.insert("마태복음".to_string(), "mt".to_string());

    // 마가복음
    map.insert("막".to_string(), "mk".to_string());
    map.insert("마가복음".to_string(), "mk".to_string());

    // 누가복음
    map.insert("눅".to_string(), "lk".to_string());
    map.insert("누가복음".to_string(), "lk".to_string());

    // 요한복음
    map.insert("요".to_string(), "jo".to_string());
    map.insert("요한복음".to_string(), "jo".to_string());

    // 사도행전
    map.insert("행".to_string(), "act".to_string());
    map.insert("사도행전".to_string(), "act".to_string());

    // 로마서
    map.insert("롬".to_string(), "rm".to_string());
    map.insert("로마서".to_string(), "rm".to_string());

    // 고린도전서
    map.insert("고전".to_string(), "1co".to_string());
    map.insert("고린도전서".to_string(), "1co".to_string());

    // 고린도후서
    map.insert("고후".to_string(), "2co".to_string());
    map.insert("고린도후서".to_string(), "2co".to_string());

    // 갈라디아서
    map.insert("갈".to_string(), "gl".to_string());
    map.insert("갈라디아서".to_string(), "gl".to_string());

    // 에베소서
    map.insert("엡".to_string(), "eph".to_string());
    map.insert("에베소서".to_string(), "eph".to_string());

    // 빌립보서
    map.insert("빌".to_string(), "ph".to_string());
    map.insert("빌립보서".to_string(), "ph".to_string());

    // 골로새서
    map.insert("골".to_string(), "cl".to_string());
    map.insert("골로새서".to_string(), "cl".to_string());

    // 데살로니가전서
    map.insert("살전".to_string(), "1ts".to_string());
    map.insert("데살로니가전서".to_string(), "1ts".to_string());

    // 데살로니가후서
    map.insert("살후".to_string(), "2ts".to_string());
    map.insert("데살로니가후서".to_string(), "2ts".to_string());

    // 디모데전서
    map.insert("딤전".to_string(), "1tm".to_string());
    map.insert("디모데전서".to_string(), "1tm".to_string());

    // 디모데후서
    map.insert("딤후".to_string(), "2tm".to_string());
    map.insert("디모데후서".to_string(), "2tm".to_string());

    // 디도서
    map.insert("딛".to_string(), "tt".to_string());
    map.insert("디도서".to_string(), "tt".to_string());

    // 빌레몬서
    map.insert("몬".to_string(), "phm".to_string());
    map.insert("빌레몬서".to_string(), "phm".to_string());

    // 히브리서
    map.insert("히".to_string(), "hb".to_string());
    map.insert("히브리서".to_string(), "hb".to_string());

    // 야고보서
    map.insert("약".to_string(), "jm".to_string());
    map.insert("야고보서".to_string(), "jm".to_string());

    // 베드로전서
    map.insert("벧전".to_string(), "1pe".to_string());
    map.insert("베드로전서".to_string(), "1pe".to_string());

    // 베드로후서
    map.insert("벧후".to_string(), "2pe".to_string());
    map.insert("베드로후서".to_string(), "2pe".to_string());

    // 요한일서
    map.insert("요일".to_string(), "1jo".to_string());
    map.insert("요한일서".to_string(), "1jo".to_string());
    map.insert("요한1서".to_string(), "1jo".to_string());

    // 요한이서
    map.insert("요이".to_string(), "2jo".to_string());
    map.insert("요한이서".to_string(), "2jo".to_string());
    map.insert("요한2서".to_string(), "2jo".to_string());

    // 요한삼서
    map.insert("요삼".to_string(), "3jo".to_string());
    map.insert("요한삼서".to_string(), "3jo".to_string());
    map.insert("요한3서".to_string(), "3jo".to_string());

    // 유다서
    map.insert("유".to_string(), "jd".to_string());
    map.insert("유다서".to_string(), "jd".to_string());

    // 요한계시록
    map.insert("계".to_string(), "re".to_string());
    map.insert("요한계시록".to_string(), "re".to_string());
    map.insert("계시록".to_string(), "re".to_string());

    map
}

/// 영어 약어 → 한글 풀네임 매핑을 생성합니다.
pub fn create_english_to_korean_mapping() -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();

    // 구약
    map.insert("gn".to_string(), "창세기".to_string());
    map.insert("ex".to_string(), "출애굽기".to_string());
    map.insert("lv".to_string(), "레위기".to_string());
    map.insert("nm".to_string(), "민수기".to_string());
    map.insert("dt".to_string(), "신명기".to_string());
    map.insert("js".to_string(), "여호수아".to_string());
    map.insert("jud".to_string(), "사사기".to_string());
    map.insert("rt".to_string(), "룻기".to_string());
    map.insert("1sm".to_string(), "사무엘상".to_string());
    map.insert("2sm".to_string(), "사무엘하".to_string());
    map.insert("1kgs".to_string(), "열왕기상".to_string());
    map.insert("2kgs".to_string(), "열왕기하".to_string());
    map.insert("1ch".to_string(), "역대상".to_string());
    map.insert("2ch".to_string(), "역대하".to_string());
    map.insert("ezr".to_string(), "에스라".to_string());
    map.insert("ne".to_string(), "느헤미야".to_string());
    map.insert("et".to_string(), "에스더".to_string());
    map.insert("job".to_string(), "욥기".to_string());
    map.insert("ps".to_string(), "시편".to_string());
    map.insert("prv".to_string(), "잠언".to_string());
    map.insert("ec".to_string(), "전도서".to_string());
    map.insert("so".to_string(), "아가".to_string());
    map.insert("is".to_string(), "이사야".to_string());
    map.insert("jr".to_string(), "예레미야".to_string());
    map.insert("lm".to_string(), "예레미야애가".to_string());
    map.insert("ez".to_string(), "에스겔".to_string());
    map.insert("dn".to_string(), "다니엘".to_string());
    map.insert("ho".to_string(), "호세아".to_string());
    map.insert("jl".to_string(), "요엘".to_string());
    map.insert("am".to_string(), "아모스".to_string());
    map.insert("ob".to_string(), "오바댜".to_string());
    map.insert("jn".to_string(), "요나".to_string());
    map.insert("mi".to_string(), "미가".to_string());
    map.insert("na".to_string(), "나훔".to_string());
    map.insert("hk".to_string(), "하박국".to_string());
    map.insert("zp".to_string(), "스바냐".to_string());
    map.insert("hg".to_string(), "학개".to_string());
    map.insert("zc".to_string(), "스가랴".to_string());
    map.insert("ml".to_string(), "말라기".to_string());

    // 신약
    map.insert("mt".to_string(), "마태복음".to_string());
    map.insert("mk".to_string(), "마가복음".to_string());
    map.insert("lk".to_string(), "누가복음".to_string());
    map.insert("jo".to_string(), "요한복음".to_string());
    map.insert("act".to_string(), "사도행전".to_string());
    map.insert("rm".to_string(), "로마서".to_string());
    map.insert("1co".to_string(), "고린도전서".to_string());
    map.insert("2co".to_string(), "고린도후서".to_string());
    map.insert("gl".to_string(), "갈라디아서".to_string());
    map.insert("eph".to_string(), "에베소서".to_string());
    map.insert("ph".to_string(), "빌립보서".to_string());
    map.insert("cl".to_string(), "골로새서".to_string());
    map.insert("1ts".to_string(), "데살로니가전서".to_string());
    map.insert("2ts".to_string(), "데살로니가후서".to_string());
    map.insert("1tm".to_string(), "디모데전서".to_string());
    map.insert("2tm".to_string(), "디모데후서".to_string());
    map.insert("tt".to_string(), "디도서".to_string());
    map.insert("phm".to_string(), "빌레몬서".to_string());
    map.insert("hb".to_string(), "히브리서".to_string());
    map.insert("jm".to_string(), "야고보서".to_string());
    map.insert("1pe".to_string(), "베드로전서".to_string());
    map.insert("2pe".to_string(), "베드로후서".to_string());
    map.insert("1jo".to_string(), "요한일서".to_string());
    map.insert("2jo".to_string(), "요한이서".to_string());
    map.insert("3jo".to_string(), "요한삼서".to_string());
    map.insert("jd".to_string(), "유다서".to_string());
    map.insert("re".to_string(), "요한계시록".to_string());

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_korean_mapping_basic() {
        let map = create_korean_to_english_mapping();

        // 창세기 테스트
        assert_eq!(map.get("창"), Some(&"gn".to_string()));
        assert_eq!(map.get("창세기"), Some(&"gn".to_string()));

        // 출애굽기 테스트
        assert_eq!(map.get("출"), Some(&"ex".to_string()));
        assert_eq!(map.get("출애굽기"), Some(&"ex".to_string()));
    }

    #[test]
    fn test_korean_mapping_special_cases() {
        let map = create_korean_to_english_mapping();

        // 사무엘상/하
        assert_eq!(map.get("삼상"), Some(&"1sm".to_string()));
        assert_eq!(map.get("사무엘상"), Some(&"1sm".to_string()));
        assert_eq!(map.get("삼하"), Some(&"2sm".to_string()));

        // 고린도전서/후서
        assert_eq!(map.get("고전"), Some(&"1co".to_string()));
        assert_eq!(map.get("고린도전서"), Some(&"1co".to_string()));
        assert_eq!(map.get("고후"), Some(&"2co".to_string()));

        // 요한일서/이서/삼서
        assert_eq!(map.get("요일"), Some(&"1jo".to_string()));
        assert_eq!(map.get("요이"), Some(&"2jo".to_string()));
        assert_eq!(map.get("요삼"), Some(&"3jo".to_string()));
    }

    #[test]
    fn test_all_66_books_have_short_and_full_names() {
        let map = create_korean_to_english_mapping();

        // 66권 모두 최소 2개 이상의 매핑이 있어야 함 (1자 + 풀네임)
        // 총 매핑 개수는 66 * 2 = 132개 이상이어야 함
        // (일부 책은 추가 별칭이 있을 수 있음: 애가, 계시록 등)
        assert!(map.len() >= 132, "총 매핑 개수: {}", map.len());
    }
}
