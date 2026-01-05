#!/usr/bin/env node

/**
 * bible.json을 SQLite 데이터베이스로 변환하는 스크립트
 *
 * 사용법:
 *   node scripts/convert_to_sqlite.js
 *
 * 출력:
 *   bible.db (SQLite 데이터베이스 파일)
 */

const fs = require('fs');
const path = require('path');
const sqlite3 = require('better-sqlite3');

// 경로 설정
const BIBLE_JSON_PATH = path.join(__dirname, '../bible.json');
const OUTPUT_DB_PATH = path.join(__dirname, '../bible.db');

console.log('🔄 Bible.json → SQLite 변환 시작...\n');

// 1. bible.json 읽기
console.log('📖 bible.json 읽는 중...');
const bibleData = JSON.parse(fs.readFileSync(BIBLE_JSON_PATH, 'utf8'));
console.log(`✅ 총 ${bibleData.length}권 로드 완료\n`);

// 2. SQLite DB 생성
console.log('🗄️  SQLite 데이터베이스 생성 중...');
if (fs.existsSync(OUTPUT_DB_PATH)) {
    fs.unlinkSync(OUTPUT_DB_PATH);
    console.log('  ↳ 기존 DB 삭제');
}

const db = sqlite3(OUTPUT_DB_PATH);

// 3. 테이블 생성
console.log('📋 테이블 생성 중...');
db.exec(`
    CREATE TABLE verses (
        book_abbrev TEXT NOT NULL,
        chapter INTEGER NOT NULL,
        verse INTEGER NOT NULL,
        text TEXT NOT NULL,
        PRIMARY KEY (book_abbrev, chapter, verse)
    );

    CREATE INDEX idx_lookup ON verses(book_abbrev, chapter, verse);
`);
console.log('✅ 테이블 생성 완료\n');

// 4. 데이터 삽입
console.log('💾 데이터 삽입 중...');

const insert = db.prepare(`
    INSERT INTO verses (book_abbrev, chapter, verse, text)
    VALUES (?, ?, ?, ?)
`);

let totalVerses = 0;

// 트랜잭션으로 묶어서 빠르게 삽입
const insertMany = db.transaction((books) => {
    for (const book of books) {
        const bookAbbrev = book.abbrev;

        book.chapters.forEach((chapter, chapterIndex) => {
            const chapterNumber = chapterIndex + 1; // 1부터 시작

            chapter.forEach((verseText, verseIndex) => {
                const verseNumber = verseIndex + 1; // 1부터 시작

                insert.run(bookAbbrev, chapterNumber, verseNumber, verseText);
                totalVerses++;
            });
        });

        // 진행 상황 출력
        process.stdout.write(`  ↳ ${book.abbrev} 완료 (총 ${totalVerses}절)\r`);
    }
});

insertMany(bibleData);

console.log(`\n✅ 총 ${totalVerses}개 구절 삽입 완료\n`);

// 5. 통계 출력
const stats = db.prepare(`
    SELECT
        COUNT(DISTINCT book_abbrev) as book_count,
        COUNT(DISTINCT book_abbrev || '-' || chapter) as chapter_count,
        COUNT(*) as verse_count
    FROM verses
`).get();

console.log('📊 변환 결과:');
console.log(`  • 총 권수: ${stats.book_count}권`);
console.log(`  • 총 장수: ${stats.chapter_count}장`);
console.log(`  • 총 절수: ${stats.verse_count}절`);

// 6. 샘플 조회 테스트
console.log('\n🔍 샘플 조회 테스트:');
const sample = db.prepare(`
    SELECT * FROM verses
    WHERE book_abbrev = 'gn' AND chapter = 1 AND verse = 1
`).get();

console.log(`  창세기 1:1 = "${sample.text}"`);

// 7. DB 파일 크기 확인
const dbSize = fs.statSync(OUTPUT_DB_PATH).size;
const dbSizeMB = (dbSize / 1024 / 1024).toFixed(2);
console.log(`\n💾 DB 파일 크기: ${dbSizeMB} MB`);

db.close();

console.log('\n✅ 변환 완료!');
console.log(`📁 출력 파일: ${OUTPUT_DB_PATH}`);
