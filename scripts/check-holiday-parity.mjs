import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const { getVietnameseHolidays } = require("../packages/core/vietnamese-holidays.js");
const lunarFestivals = require("../data/holidays/lunar-festivals.json");
const solarHolidays = require("../data/holidays/solar-holidays.json");

// Rust fixture snapshot for get_major_holidays(year), captured for 2024-2026.
const RUST_MAJOR_FIXTURE = {
  2024: [
    "2024-01-01|Tết Dương Lịch",
    "2024-01-09|Ngày Truyền Thống Học Sinh - Sinh Viên Việt Nam",
    "2024-02-02|Ông Táo chầu trời",
    "2024-02-03|Ngày Thành Lập Đảng Cộng Sản Việt Nam",
    "2024-02-09|Giao Thừa",
    "2024-02-10|Tết Nguyên Đán",
    "2024-02-11|Mùng 2 Tết",
    "2024-02-12|Mùng 3 Tết",
    "2024-02-14|Lễ Tình Nhân",
    "2024-02-24|Tết Nguyên Tiêu",
    "2024-02-27|Ngày Thầy Thuốc Việt Nam",
    "2024-03-08|Ngày Quốc Tế Phụ Nữ",
    "2024-03-15|Ngày Quyền Của Người Tiêu Dùng Việt Nam",
    "2024-03-20|Ngày Quốc Tế Hạnh Phúc",
    "2024-03-22|Ngày Nước Thế Giới",
    "2024-03-26|Ngày Thành Lập Đoàn Thanh Niên Cộng Sản Hồ Chí Minh",
    "2024-04-01|Ngày Cá Tháng Tư",
    "2024-04-05|Tết Thanh Minh",
    "2024-04-07|Ngày Sức Khỏe Thế Giới",
    "2024-04-11|Tết Hàn Thực",
    "2024-04-21|Ngày Sách và Văn Hóa Đọc Việt Nam",
    "2024-04-22|Ngày Trái Đất",
    "2024-04-27|Ngày Kiến Trúc Sư Việt Nam",
    "2024-04-30|Ngày Giải Phóng Miền Nam",
    "2024-05-01|Ngày Quốc Tế Lao Động",
    "2024-05-07|Ngày Chiến Thắng Điện Biên Phủ",
    "2024-05-09|Ngày Chiến Thắng Phát Xít",
    "2024-05-12|Ngày của Mẹ",
    "2024-05-15|Ngày Quốc Tế Gia Đình",
    "2024-05-18|Ngày Khoa Học và Công Nghệ Việt Nam",
    "2024-05-19|Ngày Sinh Chủ Tịch Hồ Chí Minh",
    "2024-05-22|Lễ Phật Đản",
    "2024-06-01|Ngày Quốc Tế Thiếu Nhi",
    "2024-06-05|Ngày Môi Trường Thế Giới",
    "2024-06-10|Tết Đoan Ngọ",
    "2024-06-16|Ngày của Cha",
    "2024-06-28|Ngày Gia Đình Việt Nam",
    "2024-07-11|Ngày Dân Số Thế Giới",
    "2024-07-27|Ngày Thương Binh - Liệt Sĩ",
    "2024-08-12|Ngày Quốc Tế Thanh Niên",
    "2024-08-18|Lễ Vu Lan",
    "2024-08-19|Ngày Cách Mạng Tháng Tám",
    "2024-08-19|Ngày Truyền Thống Công An Nhân Dân",
    "2024-09-02|Ngày Quốc Khánh",
    "2024-09-08|Ngày Xóa Mù Chữ Quốc Tế",
    "2024-09-17|Tết Trung Thu",
    "2024-09-21|Ngày Quốc Tế Hòa Bình",
    "2024-10-01|Ngày Quốc Tế Người Cao Tuổi",
    "2024-10-10|Ngày Chuyển Đổi Số Quốc Gia",
    "2024-10-10|Ngày Giải Phóng Thủ Đô",
    "2024-10-11|Tết Trùng Cửu",
    "2024-10-13|Ngày Doanh Nhân Việt Nam",
    "2024-10-20|Ngày Phụ Nữ Việt Nam",
    "2024-10-31|Halloween",
    "2024-11-09|Ngày Pháp Luật Việt Nam",
    "2024-11-15|Tết Hạ Nguyên",
    "2024-11-19|Ngày Quốc Tế Nam Giới",
    "2024-11-20|Ngày Nhà Giáo Việt Nam",
    "2024-11-23|Ngày Di Sản Văn Hóa Việt Nam",
    "2024-11-23|Ngày Thành Lập Hội Chữ Thập Đỏ Việt Nam",
    "2024-12-01|Ngày Thế Giới Phòng Chống AIDS",
    "2024-12-03|Ngày Quốc Tế Người Khuyết Tật",
    "2024-12-19|Ngày Toàn Quốc Kháng Chiến",
    "2024-12-22|Ngày Thành Lập Quân Đội Nhân Dân Việt Nam",
    "2024-12-25|Lễ Giáng Sinh",
  ],
  2025: [
    "2025-01-01|Tết Dương Lịch",
    "2025-01-09|Ngày Truyền Thống Học Sinh - Sinh Viên Việt Nam",
    "2025-01-22|Ông Táo chầu trời",
    "2025-01-29|Giao Thừa",
    "2025-01-29|Tết Nguyên Đán",
    "2025-01-30|Mùng 2 Tết",
    "2025-01-31|Mùng 3 Tết",
    "2025-02-03|Ngày Thành Lập Đảng Cộng Sản Việt Nam",
    "2025-02-12|Tết Nguyên Tiêu",
    "2025-02-14|Lễ Tình Nhân",
    "2025-02-27|Ngày Thầy Thuốc Việt Nam",
    "2025-03-08|Ngày Quốc Tế Phụ Nữ",
    "2025-03-15|Ngày Quyền Của Người Tiêu Dùng Việt Nam",
    "2025-03-20|Ngày Quốc Tế Hạnh Phúc",
    "2025-03-22|Ngày Nước Thế Giới",
    "2025-03-26|Ngày Thành Lập Đoàn Thanh Niên Cộng Sản Hồ Chí Minh",
    "2025-03-31|Tết Hàn Thực",
    "2025-04-01|Ngày Cá Tháng Tư",
    "2025-04-05|Tết Thanh Minh",
    "2025-04-07|Ngày Sức Khỏe Thế Giới",
    "2025-04-21|Ngày Sách và Văn Hóa Đọc Việt Nam",
    "2025-04-22|Ngày Trái Đất",
    "2025-04-27|Ngày Kiến Trúc Sư Việt Nam",
    "2025-04-30|Ngày Giải Phóng Miền Nam",
    "2025-05-01|Ngày Quốc Tế Lao Động",
    "2025-05-07|Ngày Chiến Thắng Điện Biên Phủ",
    "2025-05-09|Ngày Chiến Thắng Phát Xít",
    "2025-05-11|Ngày của Mẹ",
    "2025-05-12|Lễ Phật Đản",
    "2025-05-15|Ngày Quốc Tế Gia Đình",
    "2025-05-18|Ngày Khoa Học và Công Nghệ Việt Nam",
    "2025-05-19|Ngày Sinh Chủ Tịch Hồ Chí Minh",
    "2025-05-31|Tết Đoan Ngọ",
    "2025-06-01|Ngày Quốc Tế Thiếu Nhi",
    "2025-06-05|Ngày Môi Trường Thế Giới",
    "2025-06-15|Ngày của Cha",
    "2025-06-28|Ngày Gia Đình Việt Nam",
    "2025-07-11|Ngày Dân Số Thế Giới",
    "2025-07-27|Ngày Thương Binh - Liệt Sĩ",
    "2025-08-12|Ngày Quốc Tế Thanh Niên",
    "2025-08-19|Ngày Cách Mạng Tháng Tám",
    "2025-08-19|Ngày Truyền Thống Công An Nhân Dân",
    "2025-09-02|Ngày Quốc Khánh",
    "2025-09-06|Lễ Vu Lan",
    "2025-09-08|Ngày Xóa Mù Chữ Quốc Tế",
    "2025-09-21|Ngày Quốc Tế Hòa Bình",
    "2025-10-01|Ngày Quốc Tế Người Cao Tuổi",
    "2025-10-06|Tết Trung Thu",
    "2025-10-10|Ngày Chuyển Đổi Số Quốc Gia",
    "2025-10-10|Ngày Giải Phóng Thủ Đô",
    "2025-10-13|Ngày Doanh Nhân Việt Nam",
    "2025-10-20|Ngày Phụ Nữ Việt Nam",
    "2025-10-29|Tết Trùng Cửu",
    "2025-10-31|Halloween",
    "2025-11-09|Ngày Pháp Luật Việt Nam",
    "2025-11-19|Ngày Quốc Tế Nam Giới",
    "2025-11-20|Ngày Nhà Giáo Việt Nam",
    "2025-11-23|Ngày Di Sản Văn Hóa Việt Nam",
    "2025-11-23|Ngày Thành Lập Hội Chữ Thập Đỏ Việt Nam",
    "2025-12-01|Ngày Thế Giới Phòng Chống AIDS",
    "2025-12-03|Ngày Quốc Tế Người Khuyết Tật",
    "2025-12-04|Tết Hạ Nguyên",
    "2025-12-19|Ngày Toàn Quốc Kháng Chiến",
    "2025-12-22|Ngày Thành Lập Quân Đội Nhân Dân Việt Nam",
    "2025-12-25|Lễ Giáng Sinh",
  ],
  2026: [
    "2026-01-01|Tết Dương Lịch",
    "2026-01-09|Ngày Truyền Thống Học Sinh - Sinh Viên Việt Nam",
    "2026-02-03|Ngày Thành Lập Đảng Cộng Sản Việt Nam",
    "2026-02-10|Ông Táo chầu trời",
    "2026-02-14|Lễ Tình Nhân",
    "2026-02-17|Giao Thừa",
    "2026-02-17|Tết Nguyên Đán",
    "2026-02-18|Mùng 2 Tết",
    "2026-02-19|Mùng 3 Tết",
    "2026-02-27|Ngày Thầy Thuốc Việt Nam",
    "2026-03-03|Tết Nguyên Tiêu",
    "2026-03-08|Ngày Quốc Tế Phụ Nữ",
    "2026-03-15|Ngày Quyền Của Người Tiêu Dùng Việt Nam",
    "2026-03-20|Ngày Quốc Tế Hạnh Phúc",
    "2026-03-22|Ngày Nước Thế Giới",
    "2026-03-26|Ngày Thành Lập Đoàn Thanh Niên Cộng Sản Hồ Chí Minh",
    "2026-04-01|Ngày Cá Tháng Tư",
    "2026-04-05|Tết Thanh Minh",
    "2026-04-07|Ngày Sức Khỏe Thế Giới",
    "2026-04-19|Tết Hàn Thực",
    "2026-04-21|Ngày Sách và Văn Hóa Đọc Việt Nam",
    "2026-04-22|Ngày Trái Đất",
    "2026-04-27|Ngày Kiến Trúc Sư Việt Nam",
    "2026-04-30|Ngày Giải Phóng Miền Nam",
    "2026-05-01|Ngày Quốc Tế Lao Động",
    "2026-05-07|Ngày Chiến Thắng Điện Biên Phủ",
    "2026-05-09|Ngày Chiến Thắng Phát Xít",
    "2026-05-10|Ngày của Mẹ",
    "2026-05-15|Ngày Quốc Tế Gia Đình",
    "2026-05-18|Ngày Khoa Học và Công Nghệ Việt Nam",
    "2026-05-19|Ngày Sinh Chủ Tịch Hồ Chí Minh",
    "2026-05-31|Lễ Phật Đản",
    "2026-06-01|Ngày Quốc Tế Thiếu Nhi",
    "2026-06-05|Ngày Môi Trường Thế Giới",
    "2026-06-19|Tết Đoan Ngọ",
    "2026-06-21|Ngày của Cha",
    "2026-06-28|Ngày Gia Đình Việt Nam",
    "2026-07-11|Ngày Dân Số Thế Giới",
    "2026-07-27|Ngày Thương Binh - Liệt Sĩ",
    "2026-08-12|Ngày Quốc Tế Thanh Niên",
    "2026-08-19|Ngày Cách Mạng Tháng Tám",
    "2026-08-19|Ngày Truyền Thống Công An Nhân Dân",
    "2026-08-27|Lễ Vu Lan",
    "2026-09-02|Ngày Quốc Khánh",
    "2026-09-08|Ngày Xóa Mù Chữ Quốc Tế",
    "2026-09-21|Ngày Quốc Tế Hòa Bình",
    "2026-09-25|Tết Trung Thu",
    "2026-10-01|Ngày Quốc Tế Người Cao Tuổi",
    "2026-10-10|Ngày Chuyển Đổi Số Quốc Gia",
    "2026-10-10|Ngày Giải Phóng Thủ Đô",
    "2026-10-13|Ngày Doanh Nhân Việt Nam",
    "2026-10-18|Tết Trùng Cửu",
    "2026-10-20|Ngày Phụ Nữ Việt Nam",
    "2026-10-31|Halloween",
    "2026-11-09|Ngày Pháp Luật Việt Nam",
    "2026-11-19|Ngày Quốc Tế Nam Giới",
    "2026-11-20|Ngày Nhà Giáo Việt Nam",
    "2026-11-23|Ngày Di Sản Văn Hóa Việt Nam",
    "2026-11-23|Ngày Thành Lập Hội Chữ Thập Đỏ Việt Nam",
    "2026-11-23|Tết Hạ Nguyên",
    "2026-12-01|Ngày Thế Giới Phòng Chống AIDS",
    "2026-12-03|Ngày Quốc Tế Người Khuyết Tật",
    "2026-12-19|Ngày Toàn Quốc Kháng Chiến",
    "2026-12-22|Ngày Thành Lập Quân Đội Nhân Dân Việt Nam",
    "2026-12-25|Lễ Giáng Sinh",
  ],
};

const MAJOR_NAMES = new Set([
  ...lunarFestivals.festivals.map((festival) => festival.names.vi[0]),
  ...solarHolidays.holidays.filter((holiday) => holiday.isMajor).map((holiday) => holiday.names.vi[0]),
  "Ngày của Mẹ",
  "Ngày của Cha",
]);

function getJsMajorSnapshot(year) {
  return getVietnameseHolidays(year)
    .filter((holiday) => MAJOR_NAMES.has(holiday.name))
    .map((holiday) => `${holiday.dateString}|${holiday.name}`)
    .sort((a, b) => a.localeCompare(b, "vi"));
}

function diffSets(expected, actual) {
  const expectedSet = new Set(expected);
  const actualSet = new Set(actual);

  const missing = expected.filter((item) => !actualSet.has(item));
  const extra = actual.filter((item) => !expectedSet.has(item));

  return { missing, extra };
}

let hasError = false;

for (const year of [2024, 2025, 2026]) {
  const expected = RUST_MAJOR_FIXTURE[year];
  const actual = getJsMajorSnapshot(year);
  const { missing, extra } = diffSets(expected, actual);

  if (missing.length === 0 && extra.length === 0) {
    console.log(`OK ${year}: ${actual.length} major holidays match Rust fixture`);
    continue;
  }

  hasError = true;
  console.error(`ERROR ${year}: parity mismatch`);

  if (missing.length > 0) {
    console.error("  Missing in JS:");
    for (const item of missing) {
      console.error(`  - ${item}`);
    }
  }

  if (extra.length > 0) {
    console.error("  Extra in JS:");
    for (const item of extra) {
      console.error(`  - ${item}`);
    }
  }
}

if (hasError) {
  process.exitCode = 1;
} else {
  console.log("Holiday parity check passed.");
}
