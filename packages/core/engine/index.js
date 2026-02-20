/**
 * Rust-backed engine adapter.
 *
 * Preferred path:
 * - call the Rust CLI (`amlich json <YYYY-MM-DD>`) and normalize payload shape.
 *
 * Fallback path:
 * - use the previous JS implementation from `legacy-engine.js`.
 */

const { spawnSync } = require('node:child_process');
const legacy = require('./legacy-engine.js');

const warned = {
  rustUnavailable: false,
};

function pad2(n) {
  return String(n).padStart(2, '0');
}

function normalizeCanChi(value) {
  return {
    canIndex: value.can_index,
    chiIndex: value.chi_index,
    can: value.can,
    chi: value.chi,
    full: value.full,
    conGiap: value.con_giap,
    nguHanh: {
      can: value.ngu_hanh.can,
      chi: value.ngu_hanh.chi,
    },
  };
}

function normalizeHour(value) {
  return {
    hourIndex: value.hour_index,
    hourChi: value.hour_chi,
    timeRange: value.time_range,
    star: value.star,
    isGood: value.is_good,
  };
}

function normalizeFromRust(payload) {
  return {
    solar: {
      day: payload.solar.day,
      month: payload.solar.month,
      year: payload.solar.year,
      dayOfWeek: payload.solar.day_of_week,
      dayOfWeekName: payload.solar.day_of_week_name,
      dateString: payload.solar.date_string,
    },
    lunar: {
      day: payload.lunar.day,
      month: payload.lunar.month,
      year: payload.lunar.year,
      isLeapMonth: payload.lunar.is_leap_month,
      dateString: payload.lunar.date_string,
    },
    jd: payload.jd,
    canChi: {
      day: normalizeCanChi(payload.canchi.day),
      month: normalizeCanChi(payload.canchi.month),
      year: normalizeCanChi(payload.canchi.year),
      full: payload.canchi.full,
    },
    tietKhi: {
      index: payload.tiet_khi.index,
      name: payload.tiet_khi.name,
      description: payload.tiet_khi.description,
      longitude: payload.tiet_khi.longitude,
      currentLongitude: payload.tiet_khi.current_longitude,
      season: payload.tiet_khi.season,
    },
    gioHoangDao: {
      dayChi: payload.gio_hoang_dao.day_chi,
      goodHourCount: payload.gio_hoang_dao.good_hour_count,
      goodHours: payload.gio_hoang_dao.good_hours.map(normalizeHour),
      allHours: payload.gio_hoang_dao.all_hours.map(normalizeHour),
      summary: payload.gio_hoang_dao.summary,
    },
  };
}

function getDayInfoFromRust(dd, mm, yyyy, options = {}) {
  const cliPath = process.env.AMLICH_CLI_PATH || 'amlich';
  const date = `${yyyy}-${pad2(mm)}-${pad2(dd)}`;

  const result = spawnSync(cliPath, ['json', date], {
    encoding: 'utf8',
    env: {
      ...process.env,
      AMLICH_TIMEZONE: String(options.timezone || 7),
    },
  });

  if (result.error || result.status !== 0) {
    return null;
  }

  try {
    const parsed = JSON.parse(result.stdout);
    return normalizeFromRust(parsed);
  } catch {
    return null;
  }
}

function warnRustUnavailableOnce() {
  if (process.env.AMLICH_SUPPRESS_FALLBACK_WARN === '1') {
    return;
  }
  if (warned.rustUnavailable) {
    return;
  }
  warned.rustUnavailable = true;
  console.warn('[amlich] Rust CLI backend unavailable, falling back to legacy JS engine.');
}

function getDayInfo(dd, mm, yyyy, options = {}) {
  const rustInfo = getDayInfoFromRust(dd, mm, yyyy, options);
  if (rustInfo) {
    return rustInfo;
  }

  warnRustUnavailableOnce();
  return legacy.getDayInfo(dd, mm, yyyy, options);
}

function getCanChi(dd, mm, yyyy, options = {}) {
  const info = getDayInfo(dd, mm, yyyy, options);
  return info.canChi;
}

function formatDayInfo(dayInfo) {
  return legacy.formatDayInfo(dayInfo);
}

module.exports = {
  getDayInfo,
  getCanChi,
  formatDayInfo,
};
