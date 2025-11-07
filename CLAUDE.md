# 📖 [프로젝트명] 코딩 규칙서 (Coding Conventions)

이 문서는 [프로젝트명]의 코드 일관성 유지를 위한 핵심 코딩 규칙을 정의합니다. 모든 기여자는 이 규칙을 따르는 것을 원칙으로 합니다.

### 프로젝트 구조

* **규칙:** Tauri의 기본 구조를 엄격히 준수합니다.
* **설명:**
    * `src-tauri/` (Rust): 모든 백엔드 로직, 네이티브 API 호출, DB 처리.
    * `src/` (Web UI): 모든 프론트엔드 로직 (설정 창, 권한 안내 창).
    * **원칙:** Rust 코드에 UI 로직을, JS/TS 코드에 핵심 엔진 로직(DB 조회 등)을 구현하지 않습니다. 모든 통신은 Tauri Command/Event를 통합니다.

---

## 2. 💻 개발 방법론: TDD (테스트 주도 개발)

* **규칙:** 모든 핵심 로직(엔진, 파서, DB 조회)은 **테스트 주도 개발(TDD)**을 따릅니다.
* **프로세스:** **Red-Green-Refactor** 사이클을 준수합니다.
    1.  **Red:** 실패하는 테스트 케이스를 먼저 작성합니다. (예: `#창 1:1`을 파싱하는 테스트)
    2.  **Green:** 이 테스트를 통과하는 **최소한의** 코드를 작성합니다.
    3.  **Refactor:** 코드를 정리하고 중복을 제거합니다. (이때 테스트는 계속 Green 상태를 유지해야 함)
* **적용 대상:**
    * **Rust (엔진):** 패턴 분석 로직(Regex), 약어 매핑(HashMap), DB 조회 함수 등 순수 로직은 **유닛 테스트(Unit Test)가 필수**입니다. (`#[cfg(test)]` 사용)
    * **Web (UI):** UI 로직은 TDD 대신 E2E(End-to-End) 테스트나 수동 테스트로 대체할 수 있습니다.
* **검증:** 새로운 기능 브랜치(`feat/...`)는 `develop` 브랜치에 병합(merge)되기 전, `cargo test`를 통과해야 합니다.

---

## 3. 🦀 Rust (백엔드 / 핵심 엔진)

### 포맷팅 (Formatting)

* **규칙:** **`rustfmt`**를 사용합니다.
* **실천:** IDE(VS Code)에서 저장 시 자동 실행되도록 설정하며, 커밋 전에는 항상 `cargo fmt`를 실행합니다.

### 코드 품질 (Linting)

* **규칙:** **`clippy`**를 사용합니다.
* **실천:** `cargo clippy`를 실행하여 나오는 모든 **경고(Warning)를 수정**하는 것을 원칙으로 합니다.

### 네이밍 (Naming)

* **규칙:** Rust 표준 네이밍 규칙을 따릅니다.
    * `snake_case` (뱀_케이스): 변수, 함수명 (예: `fn get_verse(...)`)
    * `PascalCase` (파스칼케이스): Struct, Enum, Trait (예: `struct PatternMatcher`)

### 에러 핸들링 (Error Handling)

* **규칙:** **`unwrap()` / `expect()` 사용을 금지합니다.** (테스트 코드 제외)
* **설명:** 시스템 백그라운드 유틸리티는 작은 오류로 인해 프로그램 전체가 죽는(`panic!`) 것을 반드시 막아야 합니다.
* **실천:**
    * 모든 오류는 `Result<T, E>`를 통해 반환하고 `?` 연산자로 전파합니다.
    * `Option<T>`은 `if let Some(value) = ...` 또는 `match` 구문으로 안전하게 처리합니다.

### 주석 (Comments)

* **규칙:** `pub`(공개) 함수나 구조체(Struct)에는 `///` (Doc Comments)를 사용합니다.
* **설명:** 다른 개발자(또는 미래의 나)가 코드만 보고도 기능을 이해할 수 있도록 명확히 설명합니다.

---

## 4. 🕸️ 웹 (프론트엔드 / 설정 UI)

### 언어

* **규칙:** **TypeScript (TS)** 사용을 강력히 권장합니다. (일반 JavaScript(JS) 대신)
* **설명:** Rust의 엄격한 타입 시스템과 데이터 연동 시 실수를 방지합니다.

### 포맷팅 (Formatting)

* **규칙:** **Prettier**를 사용합니다.
* **실천:** IDE(VS Code)에서 저장 시 자동 실행되도록 설정합니다.

### 네이밍 (Naming)

* **규칙:** 웹 표준 네이밍 규칙을 따릅니다.
    * `camelCase` (카멜케이스): 변수, 함수명 (예: `function openSettings()`)
    * `PascalCase` (파스칼케이스): (만약 컴포넌트나 클래스를 쓴다면)

### 스타일링 (CSS)

* **규칙:** 인라인 스타일 (`style="..."`)을 금지합니다.
* **설명:** 별도의 `.css` 파일을 사용하고, 명확한 클래스 이름을 사용합니다. (예: BEM, CSS Modules)

---

## 5. 🔗 Tauri (Rust ↔ Web 연동)

### 데이터 통신

* **규칙:** Rust와 Web 간의 데이터 구조(타입)는 항상 일치해야 합니다.
* **실천 (Rust):**
    * `serde` 라이브러리를 사용합니다.
    * `#[derive(Serialize, Deserialize)]`를 구조체에 추가하여 JSON으로 자동 변환되게 합니다.
* **실천 (TypeScript):**
    * Rust의 Struct와 1:1로 매칭되는 `interface`를 TS에 정의합니다.

* **예시:**

    **Rust (`src-tauri/`):**
    ```rust
    #[derive(Serialize, Deserialize)]
    struct AppSettings {
        trigger_char: String,
        auto_start: bool,
    }
    ```

    **TypeScript (`src/`):**
    ```typescript
    interface AppSettings {
      trigger_char: string;
      auto_start: boolean;
    }
    ```

### 명령어 (Commands)

* **규칙 (Web → Rust):** UI(JS)에서 Rust로의 호출은 **`#[tauri::command]`**만 사용합니다.
* **규칙 (Rust → Web):** Rust에서 UI(JS)로의 푸시(Push) 알림은 **`Events`**를 사용합니다.
* **예시:** '손쉬운 사용' 권한이 없는 것을 Rust가 감지 → JS로 `ask_permission` 이벤트를 전송 → JS가 이 이벤트를 받아 권한 안내 창을 띄웁니다.

---

## 6. ⌨️ 키보드 후킹 구현 가이드 (Phase 5)

### 패턴 인식 규칙

* **트리거:** `#` (해시) 문자
* **허용 패턴:**
  * ✅ `#창세기 1:1` (# 바로 뒤에 한글)
  * ✅ `#창 1:1-3` (범위 조회)
  * ✅ `#창세기 20 : 1` (장/절 사이 공백 허용)
  * ❌ `# 창세기 1:1` (# 뒤 공백 불허)

* **설계 이유:**
  * `#창`, `#요`, `#롬` 같은 패턴은 일반 텍스트에서 거의 나타나지 않음
  * `# 1. 할 일` 같은 마크다운 리스트와 충돌 방지
  * 키보드 후킹 로직이 단순하고 명확해짐

### 버퍼 관리 로직 (핵심)

키보드 입력을 추적하는 버퍼는 다음 원칙을 따라야 합니다:

#### 시나리오: 사용자가 입력 중 수정하는 경우
```
사용자 입력: #창세기 1:1
버퍼 상태: "#창세기 1:1"

→ Backspace 8번 (1:1 지움)
버퍼 상태: "#창세기"

→ "2:1" 입력
버퍼 상태: "#창세기 2:1"

→ Enter/Space 누름
→ ✅ 패턴 매칭 성공: "#창세기 2:1"
```

#### 구현 원칙

**1. `#` 입력 감지 시 버퍼 초기화**
```rust
'#' => {
    buffer.clear();  // ✅ 이전 내용 모두 삭제
    buffer.push('#');
    in_pattern = true;
}
```
- 새로운 패턴이 시작되므로 기존 버퍼 내용 무시
- `"#창세기 1:1#창세기 2:1"` 같은 오염 방지

**2. Backspace 처리**
```rust
Backspace if in_pattern => {
    buffer.pop();
    if buffer.is_empty() {
        in_pattern = false;
    }
}
```
- 사용자가 입력 중 수정할 수 있도록 지원
- 버퍼가 비면 패턴 모드 종료

**3. 트리거 키 (Space/Enter) 처리**
```rust
Space | Enter if in_pattern => {
    match get_verse(&buffer) {
        Ok(verse) => {
            // ✅ 성공 → 자동 치환
            replace_text(&buffer, &verse);
        }
        Err(_) => {
            // ✅ 실패 → 조용히 무시 (에러 표시 안함)
            // 사용자가 일반 텍스트를 입력한 것
        }
    }
    buffer.clear();
    in_pattern = false;
}
```

**4. 일반 문자 추가**
```rust
c if in_pattern => {
    buffer.push(c);
}
```

**5. 버퍼 크기 제한 (보안)**
```rust
c if in_pattern => {
    if buffer.len() < 50 {  // 최대 50자
        buffer.push(c);
    } else {
        // 너무 긴 입력은 패턴이 아님
        buffer.clear();
        in_pattern = false;
    }
}
```
- 비정상적으로 긴 입력 방지
- 메모리 누수 방지

### 에러 처리 원칙

| 상황 | 에러 처리 | 이유 |
|------|-----------|------|
| **UI 입력창 (현재)** | ❌ 에러 메시지 표시 | 사용자가 명시적으로 조회 시도 |
| **키보드 후킹 (Phase 5)** | 🔇 조용히 무시 | 일반 텍스트 입력 (트리거 아님) |

**중요:** 키보드 후킹 모드에서는 패턴 매칭 실패 시 **아무런 동작도 하지 않습니다**. 사용자에게 에러를 표시하거나 알림을 보내지 않습니다. 이는 사용자가 트리거를 사용하지 않고 일반 텍스트를 입력한 것으로 간주합니다.