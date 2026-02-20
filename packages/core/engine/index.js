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

function normalizeFromRust(payload) {
  return {
    solar: {
      day: payload.solar.day,
      month: payload.solar.month,
      year: payload.solar.year,
      dayOfWeek: null,
      dayOfWeekName: payload.solar.day_of_week,
      dateString: payload.solar.date_string,
    },
    lunar: {
      day: payload.lunar.day,
      month: payload.lunar.month,
      year: payload.lunar.year,
      isLeapMonth: payload.lunar.is_leap_month,
      dateString: payload.lunar.date_string,
    },
    jd: null,
    canChi: {
      day: {
        can: payload.canchi.day_can,
        chi: payload.canchi.day_chi,
        full: payload.canchi.day,
        conGiap: '',
        nguHanh: { can: '', chi: '' },
      },
      month: {
        can: payload.canchi.month_can,
        chi: payload.canchi.month_chi,
        full: payload.canchi.month,
        conGiap: '',
        nguHanh: { can: '', chi: '' },
      },
      year: {
        can: payload.canchi.year_can,
        chi: payload.canchi.year_chi,
        full: payload.canchi.year,
        conGiap: '',
        nguHanh: { can: '', chi: '' },
      },
      full: `${payload.canchi.day}, tháng ${payload.canchi.month}, năm ${payload.canchi.year}`,
    },
    tietKhi: {
      name: payload.tiet_khi.name,
      description: payload.tiet_khi.description,
      season: payload.tiet_khi.season,
      index: null,
      longitude: null,
      currentLongitude: null,
    },
    gioHoangDao: {
      goodHours: payload.gio_hoang_dao.hours
        .filter((h) => h.is_good)
        .map((h) => ({
          hourChi: h.name,
          timeRange: h.time_range,
          star: h.star,
          hourIndex: h.hour,
          isGood: true,
        })),
      goodHourCount: payload.gio_hoang_dao.good_hour_count,
      allHours: payload.gio_hoang_dao.hours.map((h) => ({
        hourChi: h.name,
        timeRange: h.time_range,
        star: h.star,
        hourIndex: h.hour,
        isGood: h.is_good,
      })),
      summary: `Giờ tốt: ${payload.gio_hoang_dao.good_hour_count}`,
    },
    _meta: {
      version: 'rust-cli-adapter',
      timezone: 7,
      backend: 'rust-cli',
      methods: {
        dayCanChi: 'rust-core',
        monthCanChi: 'rust-core',
        yearCanChi: 'rust-core',
        tietKhi: 'rust-core',
        gioHoangDao: 'rust-core',
      },
      conventions: {
        timezone: 'UTC+7 (Vietnam)',
        dayBoundary: 'local midnight',
      },
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
