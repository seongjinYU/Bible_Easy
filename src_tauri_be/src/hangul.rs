/// 한글 2벌식 조합 모듈
///
/// keycode로부터 추출한 영문 문자를 한글 자소로 변환하고,
/// 초성+중성+종성을 조합하여 완성형 한글을 생성합니다.

/// 한글 자소 타입
#[derive(Debug, Clone, Copy)]
pub enum HangulJamo {
    /// 초성
    Cho(usize),
    /// 중성
    Jung(usize),
}

/// 영문 문자를 한글 자소로 변환 (2벌식 기준)
pub fn char_to_jamo(ch: char) -> Option<HangulJamo> {
    match ch {
        // 초성 (자음)
        'r' => Some(HangulJamo::Cho(0)),  // ㄱ
        'R' => Some(HangulJamo::Cho(1)),  // ㄲ
        's' => Some(HangulJamo::Cho(2)),  // ㄴ
        'e' => Some(HangulJamo::Cho(3)),  // ㄷ
        'E' => Some(HangulJamo::Cho(4)),  // ㄸ
        'f' => Some(HangulJamo::Cho(5)),  // ㄹ
        'a' => Some(HangulJamo::Cho(6)),  // ㅁ
        'q' => Some(HangulJamo::Cho(7)),  // ㅂ
        'Q' => Some(HangulJamo::Cho(8)),  // ㅃ
        't' => Some(HangulJamo::Cho(9)),  // ㅅ
        'T' => Some(HangulJamo::Cho(10)), // ㅆ
        'd' => Some(HangulJamo::Cho(11)), // ㅇ
        'w' => Some(HangulJamo::Cho(12)), // ㅈ
        'W' => Some(HangulJamo::Cho(13)), // ㅉ
        'c' => Some(HangulJamo::Cho(14)), // ㅊ
        'z' => Some(HangulJamo::Cho(15)), // ㅋ
        'x' => Some(HangulJamo::Cho(16)), // ㅌ
        'v' => Some(HangulJamo::Cho(17)), // ㅍ
        'g' => Some(HangulJamo::Cho(18)), // ㅎ

        // 중성 (모음)
        'k' => Some(HangulJamo::Jung(0)),  // ㅏ
        'o' => Some(HangulJamo::Jung(1)),  // ㅐ
        'i' => Some(HangulJamo::Jung(2)),  // ㅑ
        'O' => Some(HangulJamo::Jung(3)),  // ㅒ
        'j' => Some(HangulJamo::Jung(4)),  // ㅓ
        'p' => Some(HangulJamo::Jung(5)),  // ㅔ
        'u' => Some(HangulJamo::Jung(6)),  // ㅕ
        'P' => Some(HangulJamo::Jung(7)),  // ㅖ
        'h' => Some(HangulJamo::Jung(8)),  // ㅗ
        'y' => Some(HangulJamo::Jung(12)), // ㅛ
        'n' => Some(HangulJamo::Jung(13)), // ㅜ
        'b' => Some(HangulJamo::Jung(17)), // ㅠ
        'm' => Some(HangulJamo::Jung(18)), // ㅡ
        'l' => Some(HangulJamo::Jung(20)), // ㅣ

        _ => None,
    }
}

/// 초성을 종성으로 변환 (가능한 경우)
pub fn cho_to_jong(cho_idx: usize) -> Option<usize> {
    match cho_idx {
        0 => Some(1),  // ㄱ
        2 => Some(4),  // ㄴ
        3 => Some(7),  // ㄷ
        5 => Some(8),  // ㄹ
        6 => Some(16), // ㅁ
        7 => Some(17), // ㅂ
        9 => Some(19), // ㅅ
        11 => Some(21), // ㅇ
        12 => Some(22), // ㅈ
        14 => Some(23), // ㅊ
        15 => Some(24), // ㅋ
        16 => Some(25), // ㅌ
        17 => Some(26), // ㅍ
        18 => Some(27), // ㅎ
        _ => None,
    }
}

/// 복합 모음 조합 (2벌식): 두 단모음을 합쳐 복합모음 인덱스 반환
fn compose_vowel(base: usize, second: usize) -> Option<usize> {
    match (base, second) {
        (8, 0)   => Some(9),  // ㅗ + ㅏ = ㅘ
        (8, 1)   => Some(10), // ㅗ + ㅐ = ㅙ
        (8, 20)  => Some(11), // ㅗ + ㅣ = ㅚ
        (13, 4)  => Some(14), // ㅜ + ㅓ = ㅝ
        (13, 5)  => Some(15), // ㅜ + ㅔ = ㅞ
        (13, 20) => Some(16), // ㅜ + ㅣ = ㅟ
        (18, 20) => Some(19), // ㅡ + ㅣ = ㅢ
        _ => None,
    }
}

/// 복합 모음 분해 (backspace): 복합모음 → 기저 단모음 인덱스 반환
fn decompose_vowel(jung: usize) -> Option<usize> {
    match jung {
        9 | 10 | 11 => Some(8),   // ㅘ/ㅙ/ㅚ → ㅗ
        14 | 15 | 16 => Some(13), // ㅝ/ㅞ/ㅟ → ㅜ
        19 => Some(18),            // ㅢ → ㅡ
        _ => None,
    }
}

/// 종성을 초성으로 변환 (받침 분리용)
pub fn jong_to_cho(jong_idx: usize) -> Option<usize> {
    match jong_idx {
        1 => Some(0),  // ㄱ
        4 => Some(2),  // ㄴ
        7 => Some(3),  // ㄷ
        8 => Some(5),  // ㄹ
        16 => Some(6), // ㅁ
        17 => Some(7), // ㅂ
        19 => Some(9), // ㅅ
        21 => Some(11), // ㅇ
        22 => Some(12), // ㅈ
        23 => Some(14), // ㅊ
        24 => Some(15), // ㅋ
        25 => Some(16), // ㅌ
        26 => Some(17), // ㅍ
        27 => Some(18), // ㅎ
        _ => None,
    }
}

/// 한글 조합기
#[derive(Debug, Clone)]
pub struct HangulComposer {
    /// 초성 (0-18, None = 없음)
    cho: Option<usize>,
    /// 중성 (0-20, None = 없음)
    jung: Option<usize>,
    /// 종성 (0-27, None = 없음)
    jong: Option<usize>,
}

impl HangulComposer {
    pub fn new() -> Self {
        Self {
            cho: None,
            jung: None,
            jong: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.cho.is_none()
    }

    pub fn clear(&mut self) {
        self.cho = None;
        self.jung = None;
        self.jong = None;
    }

    /// Backspace 처리: 한 단계씩 분해 (복합 모음은 기저 모음으로 분해)
    /// Returns: true if something was removed, false if already empty
    pub fn backspace(&mut self) -> bool {
        if self.jong.is_some() {
            self.jong = None;
            true
        } else if let Some(jung) = self.jung {
            if let Some(base) = decompose_vowel(jung) {
                // 복합 모음 → 기저 모음으로 분해 (ㅘ → ㅗ)
                self.jung = Some(base);
            } else {
                self.jung = None;
            }
            true
        } else if self.cho.is_some() {
            self.cho = None;
            true
        } else {
            false
        }
    }

    /// 현재 조합 상태를 완성형 한글로 변환
    pub fn to_char(&self) -> Option<char> {
        if let (Some(cho), Some(jung)) = (self.cho, self.jung) {
            let jong = self.jong.unwrap_or(0);
            let code = 0xAC00 + (cho * 588) + (jung * 28) + jong;
            char::from_u32(code as u32)
        } else {
            None
        }
    }

    /// 문자 입력 처리
    ///
    /// Returns: (완성된 문자, 현재 조합 중인 문자)
    pub fn input(&mut self, ch: char) -> (Option<char>, Option<char>) {
        let jamo = char_to_jamo(ch);

        match jamo {
            Some(HangulJamo::Cho(cho_idx)) => {
                if self.cho.is_none() {
                    // 새로운 글자 시작
                    self.cho = Some(cho_idx);
                    (None, self.to_char())
                } else if self.jung.is_some() {
                    // 종성으로 시도
                    if let Some(jong_idx) = cho_to_jong(cho_idx) {
                        if self.jong.is_none() {
                            self.jong = Some(jong_idx);
                            (None, self.to_char())
                        } else {
                            // 이미 종성이 있으면 새로운 글자 시작
                            let completed = self.to_char();
                            self.clear();
                            self.cho = Some(cho_idx);
                            (completed, self.to_char())
                        }
                    } else {
                        // 종성 불가능한 자음이면 새로운 글자
                        let completed = self.to_char();
                        self.clear();
                        self.cho = Some(cho_idx);
                        (completed, self.to_char())
                    }
                } else {
                    // 초성만 있는 상태에서 다른 초성 입력 = 새로운 글자
                    self.clear();
                    self.cho = Some(cho_idx);
                    (None, self.to_char())
                }
            }
            Some(HangulJamo::Jung(jung_idx)) => {
                if self.cho.is_some() && self.jung.is_none() && self.jong.is_none() {
                    // 중성 추가 (초성만 있는 상태)
                    self.jung = Some(jung_idx);
                    (None, self.to_char())
                } else if self.cho.is_some() && self.jung.is_some() && self.jong.is_none() {
                    // 중성이 이미 있는 상태 → 복합 모음 조합 시도 (ㅗ+ㅏ=ㅘ 등)
                    if let Some(compound) = compose_vowel(self.jung.unwrap(), jung_idx) {
                        self.jung = Some(compound);
                        (None, self.to_char())
                    } else {
                        // 복합 불가 → 현재 글자 완성 후 새 모음 대기
                        let completed = self.to_char();
                        self.clear();
                        (completed, None)
                    }
                } else if self.jong.is_some() {
                    // 종성이 있는 상태에서 중성 입력 → 종성을 빼서 새 글자 시작
                    let jong_idx = self.jong.unwrap();

                    // 종성을 초성으로 변환
                    if let Some(new_cho) = jong_to_cho(jong_idx) {
                        // 이전 글자 완성 (종성 없이)
                        self.jong = None;
                        let completed = self.to_char();

                        // 새 글자 시작 (종성을 초성으로)
                        self.clear();
                        self.cho = Some(new_cho);
                        self.jung = Some(jung_idx);

                        (completed, self.to_char())
                    } else {
                        // 종성을 초성으로 변환 불가능 (이론상 발생 안 함)
                        let completed = self.to_char();
                        self.clear();
                        (completed, None)
                    }
                } else {
                    // 조합 불가 = 완성 후 새 글자
                    let completed = self.to_char();
                    self.clear();
                    (completed, None)
                }
            }
            None => {
                // 한글이 아닌 문자 = 완성 후 그대로 전달
                let completed = self.to_char();
                self.clear();
                (completed, None)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hangul_composition_with_jong() {
        let mut composer = HangulComposer::new();

        // "강" = ㄱ(r) + ㅏ(k) + ㅇ(d)
        assert_eq!(composer.input('r'), (None, None));
        assert_eq!(composer.input('k'), (None, Some('가')));
        assert_eq!(composer.input('d'), (None, Some('강')));
    }

    #[test]
    fn test_jong_separation() {
        let mut composer = HangulComposer::new();

        // "고전" = ㄱ(r) + ㅗ(h) + ㅈ(w) + ㅓ(j) + ㄴ(s)
        // r → ㄱ
        assert_eq!(composer.input('r'), (None, None));
        // h → 가 + ㅗ = 고
        assert_eq!(composer.input('h'), (None, Some('고')));
        // w → 고 + 종성ㅈ = 곶 (임시)
        assert_eq!(composer.input('w'), (None, Some('곶')));
        // j → 종성ㅈ을 빼서 "고" 완성, 새 글자 "저" 시작
        assert_eq!(composer.input('j'), (Some('고'), Some('저')));
        // s → 저 + 종성ㄴ = 전
        assert_eq!(composer.input('s'), (None, Some('전')));
    }

    #[test]
    fn test_multiple_words() {
        let mut composer = HangulComposer::new();

        // "창" = ㅊ(c) + ㅏ(k) + ㅇ(d)
        assert_eq!(composer.input('c'), (None, None));
        assert_eq!(composer.input('k'), (None, Some('차')));
        assert_eq!(composer.input('d'), (None, Some('창')));
    }

    #[test]
    fn test_compound_vowel_wang() {
        let mut composer = HangulComposer::new();

        // "왕" = ㅇ(d) + ㅘ(h+k) + ㅇ(d)
        assert_eq!(composer.input('d'), (None, None));          // ㅇ
        assert_eq!(composer.input('h'), (None, Some('오')));    // ㅗ → 오
        assert_eq!(composer.input('k'), (None, Some('와')));    // ㅗ+ㅏ=ㅘ → 와
        assert_eq!(composer.input('d'), (None, Some('왕')));    // 종성 ㅇ → 왕
    }

    #[test]
    fn test_compound_vowel_weo() {
        let mut composer = HangulComposer::new();

        // "원" = ㅇ(d) + ㅝ(n+j) + ㄴ(s)
        assert_eq!(composer.input('d'), (None, None));
        assert_eq!(composer.input('n'), (None, Some('우')));    // ㅜ → 우
        assert_eq!(composer.input('j'), (None, Some('워')));    // ㅜ+ㅓ=ㅝ → 워
        assert_eq!(composer.input('s'), (None, Some('원')));    // 종성 ㄴ → 원
    }

    #[test]
    fn test_compound_vowel_backspace() {
        let mut composer = HangulComposer::new();

        // "와" 입력 후 backspace → "오" 로 돌아가야 함
        composer.input('d'); // ㅇ
        composer.input('h'); // ㅗ
        composer.input('k'); // ㅘ (복합)
        assert_eq!(composer.to_char(), Some('와'));

        composer.backspace(); // ㅘ → ㅗ 분해
        assert_eq!(composer.to_char(), Some('오'));

        composer.backspace(); // ㅗ 제거
        assert_eq!(composer.to_char(), None);
    }

    #[test]
    fn test_wang_sang_pattern() {
        // "왕상" 전체 조합 테스트 (열왕기상 약어)
        let mut composer = HangulComposer::new();
        let mut result = String::new();

        // 왕: d h k d
        for ch in "dhkd".chars() {
            let (completed, _) = composer.input(ch);
            if let Some(c) = completed { result.push(c); }
        }
        // 상: t k d (ㅅ+ㅏ+ㅇ)
        for ch in "tkd".chars() {
            let (completed, _) = composer.input(ch);
            if let Some(c) = completed { result.push(c); }
        }
        // 마지막 조합 중인 글자 flush
        if let Some(c) = composer.to_char() { result.push(c); }

        assert_eq!(result, "왕상");
    }
}
