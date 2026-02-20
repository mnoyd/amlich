/**
 * Test suite for Vietnamese Lunar Calendar Expert Engine.
 *
 * Verifies Can Chi calculations against known dates and checks adapter behavior
 * for Rust-backed and fallback legacy paths.
 */

const { spawnSync } = require('node:child_process');
const fs = require('node:fs');
const path = require('node:path');
const { getDayInfo, formatDayInfo } = require('./index.js');

const workspaceCliPath = path.resolve(__dirname, '../../../target/debug/amlich');
if (!process.env.AMLICH_CLI_PATH && fs.existsSync(workspaceCliPath)) {
  process.env.AMLICH_CLI_PATH = workspaceCliPath;
}

// Keep test output clean when intentionally forcing fallback behavior.
if (!process.env.AMLICH_SUPPRESS_FALLBACK_WARN) {
  process.env.AMLICH_SUPPRESS_FALLBACK_WARN = '1';
}

const modeArgIndex = process.argv.indexOf('--mode');
const mode = modeArgIndex >= 0 ? process.argv[modeArgIndex + 1] : 'all';
if (!['all', 'rust', 'fallback'].includes(mode)) {
  console.error(`Invalid mode '${mode}'. Use: all | rust | fallback`);
  process.exit(1);
}

const REFERENCE_DATES = [
  {
    solar: { day: 10, month: 2, year: 2024 },
    expected: {
      dayCanChi: 'Giáp Thìn',
      monthCanChi: 'Bính Dần',
      yearCanChi: 'Giáp Thìn',
      lunar: { day: 1, month: 1, year: 2024 },
      description: 'Tết Nguyên Đán 2024 (First day of Lunar New Year)',
    },
  },
  {
    solar: { day: 29, month: 1, year: 2025 },
    expected: {
      dayCanChi: 'Mậu Tuất',
      monthCanChi: 'Mậu Dần',
      yearCanChi: 'Ất Tỵ',
      lunar: { day: 1, month: 1, year: 2025 },
      description: 'Tết Nguyên Đán 2025',
    },
  },
  {
    solar: { day: 22, month: 1, year: 2023 },
    expected: {
      dayCanChi: 'Canh Thìn',
      monthCanChi: 'Giáp Dần',
      yearCanChi: 'Quý Mão',
      lunar: { day: 1, month: 1, year: 2023 },
      description: 'Tết Nguyên Đán 2023',
    },
  },
  {
    solar: { day: 1, month: 1, year: 2024 },
    expected: {
      dayCanChi: 'Giáp Tý',
      monthCanChi: 'Giáp Tý',
      yearCanChi: 'Quý Mão',
      lunar: { day: 20, month: 11, year: 2023 },
      description: 'New Year 2024 (solar)',
    },
  },
  {
    solar: { day: 1, month: 1, year: 2000 },
    expected: {
      dayCanChi: 'Mậu Ngọ',
      monthCanChi: 'Bính Tý',
      yearCanChi: 'Kỷ Mão',
      lunar: { day: 25, month: 11, year: 1999 },
      description: 'Y2K - Millennium reference date',
    },
  },
];

function hasRustCli() {
  const bin = process.env.AMLICH_CLI_PATH || 'amlich';
  const probe = spawnSync(bin, ['json', '2024-02-10'], { encoding: 'utf8' });
  return !probe.error && probe.status === 0;
}

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}

function withCliPath(tempPath, fn) {
  const prev = process.env.AMLICH_CLI_PATH;
  process.env.AMLICH_CLI_PATH = tempPath;
  try {
    return fn();
  } finally {
    if (prev === undefined) {
      delete process.env.AMLICH_CLI_PATH;
    } else {
      process.env.AMLICH_CLI_PATH = prev;
    }
  }
}

function runReferenceTests() {
  let passCount = 0;
  let failCount = 0;

  for (const [index, testCase] of REFERENCE_DATES.entries()) {
    console.log(`\nTest ${index + 1}: ${testCase.expected.description}`);
    console.log('-'.repeat(80));

    const { day, month, year } = testCase.solar;
    const info = getDayInfo(day, month, year);

    console.log(formatDayInfo(info));

    let testPassed = true;

    if (info.canChi.day.full !== testCase.expected.dayCanChi) {
      console.log(`❌ Day Can Chi: Expected ${testCase.expected.dayCanChi}, got ${info.canChi.day.full} (FAIL)`);
      testPassed = false;
    } else {
      console.log(`✅ Day Can Chi: ${info.canChi.day.full} (PASS)`);
    }

    if (!info.canChi.month.full.startsWith(testCase.expected.monthCanChi)) {
      console.log(`❌ Month Can Chi: Expected ${testCase.expected.monthCanChi}, got ${info.canChi.month.full} (FAIL)`);
      testPassed = false;
    } else {
      console.log(`✅ Month Can Chi: ${info.canChi.month.full} (PASS)`);
    }

    if (info.canChi.year.full !== testCase.expected.yearCanChi) {
      console.log(`❌ Year Can Chi: Expected ${testCase.expected.yearCanChi}, got ${info.canChi.year.full} (FAIL)`);
      testPassed = false;
    } else {
      console.log(`✅ Year Can Chi: ${info.canChi.year.full} (PASS)`);
    }

    const lunarMatch =
      info.lunar.day === testCase.expected.lunar.day &&
      info.lunar.month === testCase.expected.lunar.month &&
      info.lunar.year === testCase.expected.lunar.year;

    if (!lunarMatch) {
      console.log(
        `❌ Lunar date: Expected ${testCase.expected.lunar.day}/${testCase.expected.lunar.month}/${testCase.expected.lunar.year}, got ${info.lunar.dateString} (FAIL)`
      );
      testPassed = false;
    } else {
      console.log(`✅ Lunar date: ${info.lunar.dateString} (PASS)`);
    }

    if (testPassed) {
      passCount += 1;
    } else {
      failCount += 1;
    }
  }

  return { passCount, failCount };
}

function testFallbackPath() {
  const info = withCliPath('__amlich_missing_binary__', () => getDayInfo(10, 2, 2024));
  assert(Boolean(info._meta?.methods), 'fallback path should expose legacy _meta.methods');
}

function testRustPathShape() {
  if (!hasRustCli()) {
    throw new Error('Rust CLI is not available for rust mode assertions');
  }

  const rustInfo = getDayInfo(10, 2, 2024);
  assert(rustInfo._meta === undefined, 'rust path should not inject legacy _meta');
  assert(Number.isInteger(rustInfo.solar.dayOfWeek), 'rust path dayOfWeek must be populated');
  assert(Number.isFinite(rustInfo.tietKhi.currentLongitude), 'rust path currentLongitude must be populated');
  assert(
    typeof rustInfo.canChi.day.conGiap === 'string' && rustInfo.canChi.day.conGiap.length > 0,
    'rust path conGiap must be populated'
  );
}

console.log(`🧪 Vietnamese Lunar Calendar Expert Engine - Test Suite (${mode} mode)\n`);
console.log('='.repeat(80));

let passCount = 0;
let failCount = 0;

if (mode === 'fallback') {
  const result = withCliPath('__amlich_missing_binary__', runReferenceTests);
  passCount += result.passCount;
  failCount += result.failCount;

  try {
    testFallbackPath();
    console.log('\n✅ Fallback path: PASS');
  } catch (err) {
    failCount += 1;
    console.log(`\n❌ Fallback path: ${err.message}`);
  }
}

if (mode === 'rust') {
  if (!hasRustCli()) {
    console.log('❌ Rust path mode requested but CLI is unavailable');
    process.exit(1);
  }

  const result = runReferenceTests();
  passCount += result.passCount;
  failCount += result.failCount;

  try {
    testRustPathShape();
    console.log('\n✅ Rust path shape: PASS');
  } catch (err) {
    failCount += 1;
    console.log(`\n❌ Rust path shape: ${err.message}`);
  }
}

if (mode === 'all') {
  const result = runReferenceTests();
  passCount += result.passCount;
  failCount += result.failCount;

  try {
    testFallbackPath();
    console.log('\n✅ Fallback path: PASS');
  } catch (err) {
    failCount += 1;
    console.log(`\n❌ Fallback path: ${err.message}`);
  }

  if (hasRustCli()) {
    try {
      testRustPathShape();
      console.log('✅ Rust path shape: PASS');
    } catch (err) {
      failCount += 1;
      console.log(`❌ Rust path shape: ${err.message}`);
    }
  } else {
    console.log('ℹ️ Rust path shape: SKIPPED (amlich CLI not available)');
  }
}

console.log('\n' + '='.repeat(80));
console.log(`\n📊 Test Results: ${passCount} passed, ${failCount} failed`);

if (failCount === 0) {
  console.log('✅ All tests passed!\n');
  process.exit(0);
}

console.log('❌ Some tests failed.\n');
process.exit(1);
