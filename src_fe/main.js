console.log('Bible Easy is running!');

// Tauri API 확인
console.log('window.__TAURI__:', window.__TAURI__);

// Tauri API 가져오기
const invoke = window.__TAURI__?.core?.invoke;

if (!invoke) {
    console.error('❌ Tauri API를 찾을 수 없습니다!');
    alert('Tauri API 로드 실패! 앱을 다시 시작해주세요.');
} else {
    console.log('✅ Tauri API 로드 성공');
}

// DOM 요소
const patternInput = document.getElementById('patternInput');
const searchBtn = document.getElementById('searchBtn');
const resultBox = document.getElementById('result');
const enabledToggle = document.getElementById('enabledToggle');
const saveSettingsBtn = document.getElementById('saveSettingsBtn');

// 결과 표시 함수
function showResult(text, isError = false) {
    resultBox.className = 'result-box ' + (isError ? 'error' : 'success');

    if (isError) {
        resultBox.innerHTML = `<p class="error-text">❌ ${text}</p>`;
    } else {
        resultBox.innerHTML = `<p class="verse-text">"${text}"</p>`;
    }
}

// 로딩 표시 함수
function showLoading() {
    resultBox.className = 'result-box';
    resultBox.innerHTML = '<p class="placeholder">🔍 조회 중...</p>';
}

// 성경 구절 조회 함수
async function searchVerse() {
    console.log('searchVerse 함수 호출됨');

    const pattern = patternInput.value.trim();
    console.log('입력된 패턴:', pattern);

    if (!pattern) {
        showResult('패턴을 입력해주세요', true);
        return;
    }

    try {
        showLoading();
        console.log('Rust 함수 호출 시작...');

        // Rust 함수 호출
        const verse = await invoke('get_bible_verse', { pattern });

        console.log('조회 성공:', verse);
        showResult(verse, false);
    } catch (error) {
        console.error('조회 실패:', error);
        showResult(`조회 실패: ${error}`, true);
    }
}

// 이벤트 리스너
searchBtn.addEventListener('click', searchVerse);

patternInput.addEventListener('keypress', (e) => {
    if (e.key === 'Enter') {
        searchVerse();
    }
});

// 샘플 패턴 (디버깅용)
console.log('샘플 패턴:');
console.log('  #창 1:1');
console.log('  #요 3:16');
console.log('  #시 23:1');

// 설정 불러오기
async function loadSettings() {
    try {
        console.log('설정 불러오기...');
        const settings = await invoke('get_settings');
        console.log('불러온 설정:', settings);

        // UI에 반영
        enabledToggle.checked = settings.enabled;
    } catch (error) {
        console.error('설정 불러오기 실패:', error);
    }
}

// 설정 저장
async function saveSettings() {
    try {
        console.log('설정 저장 중...');

        const settings = {
            enabled: enabledToggle.checked,
            auto_start: false // 추후 구현
        };

        console.log('저장할 설정:', settings);
        await invoke('update_settings', { settings });

        console.log('✅ 설정 저장 성공');
        alert('✅ 설정이 저장되었습니다!\n\n트리거 문자: # (고정)');
    } catch (error) {
        console.error('설정 저장 실패:', error);
        alert('❌ 설정 저장에 실패했습니다: ' + error);
    }
}

// 설정 버튼 이벤트
saveSettingsBtn.addEventListener('click', saveSettings);

// 앱 시작 시 설정 불러오기
loadSettings();
