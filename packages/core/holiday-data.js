const solarHolidaysData = require('../../data/holidays/solar-holidays.json');
const lunarFestivalsData = require('../../data/holidays/lunar-festivals.json');

function getPrimaryName(names) {
  return names && Array.isArray(names.vi) && names.vi[0] ? names.vi[0] : '';
}

function getPrimaryDescription(names) {
  return names && Array.isArray(names.en) && names.en[0] ? names.en[0] : '';
}

function getLunarFestivalRows() {
  return lunarFestivalsData.festivals
    .filter((festival) => !festival.isSolar)
    .map((festival) => ({
      name: getPrimaryName(festival.names),
      description: getPrimaryDescription(festival.names),
      lunarDay: festival.lunarDay,
      lunarMonth: festival.lunarMonth,
      yearOffset: festival.yearOffset || 0,
    }));
}

function getSolarHolidayRows() {
  return solarHolidaysData.holidays.map((holiday) => ({
    name: getPrimaryName(holiday.names),
    description: getPrimaryDescription(holiday.names),
    solarDay: holiday.solarDay,
    solarMonth: holiday.solarMonth,
    category: holiday.category,
    isMajor: holiday.isMajor,
  }));
}

module.exports = {
  getLunarFestivalRows,
  getSolarHolidayRows,
};
