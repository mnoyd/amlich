import fs from 'node:fs';
import path from 'node:path';

const rootDir = path.resolve(path.dirname(new URL(import.meta.url).pathname), '..');

function readJson(relPath) {
  const abs = path.join(rootDir, relPath);
  return JSON.parse(fs.readFileSync(abs, 'utf8'));
}

function fail(message) {
  console.error(`ERROR: ${message}`);
  process.exitCode = 1;
}

function assert(condition, message) {
  if (!condition) fail(message);
}

function isNonEmptyString(value) {
  return typeof value === 'string' && value.trim().length > 0;
}

function validateSolar(solar) {
  const allowedCategories = new Set([
    'public-holiday',
    'commemorative',
    'professional',
    'social',
    'international',
  ]);

  assert(Array.isArray(solar.holidays), 'solar-holidays.json: holidays must be an array');

  const ids = new Set();
  const dateKeys = new Set();
  const categoryCounts = new Map();

  for (const [i, h] of solar.holidays.entries()) {
    const p = `solar-holidays.json holidays[${i}]`;
    assert(isNonEmptyString(h.id), `${p}: id must be non-empty string`);
    assert(/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(h.id), `${p}: id must be kebab-case`);
    assert(!ids.has(h.id), `${p}: duplicate id '${h.id}'`);
    ids.add(h.id);

    assert(h.isSolar === true, `${p}: isSolar must be true`);
    assert(Number.isInteger(h.solarDay) && h.solarDay >= 1 && h.solarDay <= 31, `${p}: invalid solarDay`);
    assert(Number.isInteger(h.solarMonth) && h.solarMonth >= 1 && h.solarMonth <= 12, `${p}: invalid solarMonth`);
    assert(allowedCategories.has(h.category), `${p}: invalid category '${h.category}'`);
    assert(typeof h.isMajor === 'boolean', `${p}: isMajor must be boolean`);

    assert(h.names && Array.isArray(h.names.vi) && h.names.vi.length > 0, `${p}: names.vi required`);
    assert(h.names && Array.isArray(h.names.en) && h.names.en.length > 0, `${p}: names.en required`);
    assert(isNonEmptyString(h.names.vi[0]), `${p}: names.vi[0] required`);
    assert(isNonEmptyString(h.names.en[0]), `${p}: names.en[0] required`);

    const key = `${h.solarMonth}-${h.solarDay}-${h.names.vi[0]}`;
    assert(!dateKeys.has(key), `${p}: duplicate date+name '${key}'`);
    dateKeys.add(key);

    categoryCounts.set(h.category, (categoryCounts.get(h.category) || 0) + 1);
  }

  return categoryCounts;
}

function validateLunar(lunar) {
  assert(Array.isArray(lunar.festivals), 'lunar-festivals.json: festivals must be an array');

  const ids = new Set();
  const keys = new Set();

  for (const [i, f] of lunar.festivals.entries()) {
    const p = `lunar-festivals.json festivals[${i}]`;
    assert(isNonEmptyString(f.id), `${p}: id must be non-empty string`);
    assert(/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(f.id), `${p}: id must be kebab-case`);
    assert(!ids.has(f.id), `${p}: duplicate id '${f.id}'`);
    ids.add(f.id);

    assert(Number.isInteger(f.lunarDay) && f.lunarDay >= 0 && f.lunarDay <= 30, `${p}: invalid lunarDay`);
    assert(Number.isInteger(f.lunarMonth) && f.lunarMonth >= 0 && f.lunarMonth <= 12, `${p}: invalid lunarMonth`);
    assert(f.category === 'festival', `${p}: category must be 'festival'`);
    assert(typeof f.isMajor === 'boolean', `${p}: isMajor must be boolean`);
    assert(f.yearOffset === 0 || f.yearOffset === -1, `${p}: yearOffset must be 0 or -1`);

    if (f.isSolar === true) {
      assert(Number.isInteger(f.solarDay) && f.solarDay >= 1 && f.solarDay <= 31, `${p}: invalid solarDay for solar festival`);
      assert(Number.isInteger(f.solarMonth) && f.solarMonth >= 1 && f.solarMonth <= 12, `${p}: invalid solarMonth for solar festival`);
    }

    assert(f.names && Array.isArray(f.names.vi) && f.names.vi.length > 0, `${p}: names.vi required`);
    assert(f.names && Array.isArray(f.names.en) && f.names.en.length > 0, `${p}: names.en required`);
    assert(isNonEmptyString(f.names.vi[0]), `${p}: names.vi[0] required`);
    assert(isNonEmptyString(f.names.en[0]), `${p}: names.en[0] required`);

    const key = `${f.lunarMonth}-${f.lunarDay}-${f.yearOffset}-${f.names.vi[0]}`;
    assert(!keys.has(key), `${p}: duplicate lunar key '${key}'`);
    keys.add(key);
  }

  const ongTao = lunar.festivals.find((f) => f.id === 'ong-tao');
  const giaoThua = lunar.festivals.find((f) => f.id === 'giao-thua');
  assert(ongTao && ongTao.yearOffset === -1, "lunar-festivals.json: 'ong-tao' must have yearOffset -1");
  assert(giaoThua && giaoThua.yearOffset === -1, "lunar-festivals.json: 'giao-thua' must have yearOffset -1");
}

function validateCrossFile(solar, lunar) {
  const solarIds = new Set(solar.holidays.map((h) => h.id));
  for (const f of lunar.festivals) {
    assert(!solarIds.has(f.id), `cross-file: duplicate id '${f.id}' exists in both solar and lunar data`);
  }
}

const solar = readJson('data/holidays/solar-holidays.json');
const lunar = readJson('data/holidays/lunar-festivals.json');

const categoryCounts = validateSolar(solar);
validateLunar(lunar);
validateCrossFile(solar, lunar);

if (process.exitCode && process.exitCode !== 0) {
  process.exit(process.exitCode);
}

console.log('Holiday data validation passed.');
console.log(`Solar holidays: ${solar.holidays.length}`);
console.log(`Lunar festivals: ${lunar.festivals.length}`);
console.log('Solar category counts:');
for (const [k, v] of [...categoryCounts.entries()].sort(([a], [b]) => a.localeCompare(b))) {
  console.log(`- ${k}: ${v}`);
}
