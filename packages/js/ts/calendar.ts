const MIN_YEAR = 1900;
const MAX_YEAR = 2100;
const VN_TZ = 7;
const SYNODIC_MONTH = 29.530588853;
const NEW_MOON_EPOCH = 2415021.076998695;

const ZODIAC = ["Tý", "Sửu", "Dần", "Mão", "Thìn", "Tỵ", "Ngọ", "Mùi", "Thân", "Dậu", "Tuất", "Hợi"] as const;
const STEMS = ["Giáp", "Ất", "Bính", "Đinh", "Mậu", "Kỷ", "Canh", "Tân", "Nhâm", "Quý"] as const;
const BRANCHES = ZODIAC;
const JIEQI = [
  "Xuân Phân", "Thanh Minh", "Cốc Vũ", "Lập Hạ", "Tiểu Mãn", "Mang Chủng",
  "Hạ Chí", "Tiểu Thử", "Đại Thử", "Lập Thu", "Xử Thử", "Bạch Lộ",
  "Thu Phân", "Hàn Lộ", "Sương Giáng", "Lập Đông", "Tiểu Tuyết", "Đại Tuyết",
  "Đông Chí", "Tiểu Hàn", "Đại Hàn", "Lập Xuân", "Vũ Thủy", "Kinh Trập",
] as const;

export type SolarDate = { year: number; month: number; day: number };

export type LunarDate = {
  year: number;
  month: number;
  day: number;
  leap: boolean;
  zodiac: string;
  stemBranch: string;
  vietnameseName: string;
  isTet: boolean;
};

export type JieQi = {
  name: string;
  solarDate: SolarDate;
  exactTime?: { hour: number; minute: number; second: number };
};

export type Holiday = {
  name: string;
  solarDate: SolarDate;
  lunarDate: LunarDate;
  isPublicHoliday: boolean;
};

export type GoodDay = {
  lunarDate: LunarDate;
  solarDate: SolarDate;
  category: string;
  description: string;
};

function floor(x: number): number {
  return Math.floor(x);
}

function jdFromDate(day: number, month: number, year: number): number {
  const a = floor((14 - month) / 12);
  const y = year + 4800 - a;
  const m = month + 12 * a - 3;
  let jd =
    day +
    floor((153 * m + 2) / 5) +
    365 * y +
    floor(y / 4) -
    floor(y / 100) +
    floor(y / 400) -
    32045;
  if (jd < 2299161) {
    jd = day + floor((153 * m + 2) / 5) + 365 * y + floor(y / 4) - 32083;
  }
  return jd;
}

function jdToDate(jd: number): SolarDate {
  let b: number;
  let c: number;
  if (jd > 2299160) {
    const a = jd + 32044;
    b = floor((4 * a + 3) / 146097);
    c = a - floor((b * 146097) / 4);
  } else {
    b = 0;
    c = jd + 32082;
  }
  const d = floor((4 * c + 3) / 1461);
  const e = c - floor((1461 * d) / 4);
  const m = floor((5 * e + 2) / 153);
  return {
    day: e - floor((153 * m + 2) / 5) + 1,
    month: m + 3 - 12 * floor(m / 10),
    year: b * 100 + d - 4800 + floor(m / 10),
  };
}

function newMoon(k: number): number {
  const t = k / 1236.85;
  const t2 = t * t;
  const t3 = t2 * t;
  const dr = Math.PI / 180;
  let jd1 = 2415020.75933 + 29.53058868 * k + 0.0001178 * t2 - 0.000000155 * t3;
  jd1 += 0.00033 * Math.sin((166.56 + 132.87 * t - 0.009173 * t2) * dr);
  const m = 359.2242 + 29.10535608 * k - 0.0000333 * t2 - 0.00000347 * t3;
  const mpr = 306.0253 + 385.81691806 * k + 0.0107306 * t2 + 0.00001236 * t3;
  const f = 21.2964 + 390.67050646 * k - 0.0016528 * t2 - 0.00000239 * t3;
  let c1 = (0.1734 - 0.000393 * t) * Math.sin(m * dr) + 0.0021 * Math.sin(2 * dr * m);
  c1 = c1 - 0.4068 * Math.sin(mpr * dr) + 0.0161 * Math.sin(dr * 2 * mpr);
  c1 -= 0.0004 * Math.sin(dr * 3 * mpr);
  c1 = c1 + 0.0104 * Math.sin(dr * 2 * f) - 0.0051 * Math.sin(dr * (m + mpr));
  c1 = c1 - 0.0074 * Math.sin(dr * (m - mpr)) + 0.0004 * Math.sin(dr * (2 * f + m));
  c1 = c1 - 0.0004 * Math.sin(dr * (2 * f - m)) - 0.0006 * Math.sin(dr * (2 * f + mpr));
  c1 = c1 + 0.001 * Math.sin(dr * (2 * f - mpr)) + 0.0005 * Math.sin(dr * (2 * mpr + m));
  const deltat =
    t < -11
      ? 0.001 + 0.000839 * t + 0.0002261 * t2 - 0.00000845 * t3 - 0.000000081 * t * t3
      : -0.000278 + 0.000265 * t + 0.000262 * t2;
  return jd1 + c1 - deltat;
}

function newMoonDay(k: number, timeZone: number): number {
  return floor(newMoon(k) + 0.5 + timeZone / 24);
}

function sunLongitudeRad(jdn: number): number {
  const t = (jdn - 2451545.0) / 36525;
  const t2 = t * t;
  const dr = Math.PI / 180;
  const m = 357.5291 + 35999.0503 * t - 0.0001559 * t2 - 0.00000048 * t * t2;
  const l0 = 280.46645 + 36000.76983 * t + 0.0003032 * t2;
  let dl = (1.9146 - 0.004817 * t - 0.000014 * t2) * Math.sin(dr * m);
  dl += (0.019993 - 0.000101 * t) * Math.sin(dr * 2 * m) + 0.00029 * Math.sin(dr * 3 * m);
  let l = (l0 + dl) * dr;
  l -= Math.PI * 2 * Math.floor(l / (Math.PI * 2));
  return l;
}

function sunLongitudeDeg(jdn: number): number {
  return (sunLongitudeRad(jdn) * 180) / Math.PI;
}

function majorTermIndex(dayNumber: number, timeZone: number): number {
  return floor((sunLongitudeRad(dayNumber - 0.5 - timeZone / 24) / Math.PI) * 6);
}

function lunarMonth11(year: number, timeZone: number): number {
  const off = jdFromDate(31, 12, year) - 2415021;
  const k = floor(off / SYNODIC_MONTH);
  let nm = newMoonDay(k, timeZone);
  if (majorTermIndex(nm, timeZone) >= 9) {
    nm = newMoonDay(k - 1, timeZone);
  }
  return nm;
}

function leapMonthOffset(a11: number, timeZone: number): number {
  const k = floor(0.5 + (a11 - NEW_MOON_EPOCH) / SYNODIC_MONTH);
  let i = 1;
  let arc = majorTermIndex(newMoonDay(k + i, timeZone), timeZone);
  for (;;) {
    const last = arc;
    i += 1;
    arc = majorTermIndex(newMoonDay(k + i, timeZone), timeZone);
    if (arc === last || i >= 14) break;
  }
  return i - 1;
}

function daysInMonth(year: number, month: number): number {
  if ([1, 3, 5, 7, 8, 10, 12].includes(month)) return 31;
  if ([4, 6, 9, 11].includes(month)) return 30;
  const leap = (year % 4 === 0 && year % 100 !== 0) || year % 400 === 0;
  return leap ? 29 : 28;
}

function validateSolar(year: number, month: number, day: number): void {
  if (year < MIN_YEAR || year > MAX_YEAR) {
    throw new Error(`year out of supported range (1900-2100): ${year}`);
  }
  if (month < 1 || month > 12 || day < 1 || day > daysInMonth(year, month)) {
    throw new Error(`invalid date: ${year}-${month}-${day}`);
  }
}

function remEuclid(n: number, m: number): number {
  return ((n % m) + m) % m;
}

export function getZodiac(lunarYear: number): string {
  return ZODIAC[remEuclid(lunarYear - 1984, 12)];
}

export function getStemBranch(lunarYear: number): string {
  const idx = remEuclid(lunarYear - 1984, 60);
  return `${STEMS[idx % 10]} ${BRANCHES[idx % 12]}`;
}

function vietnameseName(year: number, month: number, day: number, leap: boolean): string {
  const dayName = day === 15 ? "Rằm" : day <= 10 ? `Mùng ${day}` : `Ngày ${day}`;
  const leapStr = leap ? " (nhuận)" : "";
  const monthName =
    month === 1 && day <= 3 && !leap ? "Tết" : `Tháng ${month}${leapStr}`;
  return `${dayName} ${monthName} ${getStemBranch(year)}`;
}

function toLunarDate(year: number, month: number, day: number, leap: boolean): LunarDate {
  return {
    year,
    month,
    day,
    leap,
    zodiac: getZodiac(year),
    stemBranch: getStemBranch(year),
    vietnameseName: vietnameseName(year, month, day, leap),
    isTet: month === 1 && day <= 3 && !leap,
  };
}

export function solarToLunar(year: number, month: number, day: number): LunarDate {
  validateSolar(year, month, day);
  const dayNumber = jdFromDate(day, month, year);
  const k = floor((dayNumber - NEW_MOON_EPOCH) / SYNODIC_MONTH);
  let monthStart = newMoonDay(k + 1, VN_TZ);
  if (monthStart > dayNumber) monthStart = newMoonDay(k, VN_TZ);
  let a11 = lunarMonth11(year, VN_TZ);
  let b11 = a11;
  let lunarYear: number;
  if (a11 >= monthStart) {
    lunarYear = year;
    a11 = lunarMonth11(year - 1, VN_TZ);
  } else {
    lunarYear = year + 1;
    b11 = lunarMonth11(year + 1, VN_TZ);
  }
  const lunarDay = dayNumber - monthStart + 1;
  const diff = floor((monthStart - a11) / 29);
  let lunarLeap = false;
  let lunarMonth = diff + 11;
  if (b11 - a11 > 365) {
    const leapDiff = leapMonthOffset(a11, VN_TZ);
    if (diff >= leapDiff) {
      lunarMonth = diff + 10;
      if (diff === leapDiff) lunarLeap = true;
    }
  }
  if (lunarMonth > 12) lunarMonth -= 12;
  if (lunarMonth >= 11 && diff < 4) lunarYear -= 1;
  return toLunarDate(lunarYear, lunarMonth, lunarDay, lunarLeap);
}

export function lunarToSolar(
  year: number,
  month: number,
  day: number,
  leap = false,
): SolarDate {
  if (year < MIN_YEAR || year > MAX_YEAR) {
    throw new Error(`year out of supported range (1900-2100): ${year}`);
  }
  if (month < 1 || month > 12) throw new Error(`invalid lunar month: ${month}`);
  if (day < 1 || day > 30) throw new Error(`invalid lunar day: ${day}`);

  const a11 = month < 11 ? lunarMonth11(year - 1, VN_TZ) : lunarMonth11(year, VN_TZ);
  const b11 = month < 11 ? lunarMonth11(year, VN_TZ) : lunarMonth11(year + 1, VN_TZ);
  let off = month - 11;
  if (off < 0) off += 12;
  if (b11 - a11 > 365) {
    const leapOff = leapMonthOffset(a11, VN_TZ);
    let leapMonthN = leapOff - 2;
    if (leapMonthN < 0) leapMonthN += 12;
    if (leap && month !== leapMonthN) {
      throw new Error("requested leap month does not exist in that year");
    }
    if (leap || off >= leapOff) off += 1;
  } else if (leap) {
    throw new Error("requested leap month does not exist in that year");
  }
  const k = floor(0.5 + (a11 - NEW_MOON_EPOCH) / SYNODIC_MONTH);
  const monthStart = newMoonDay(k + off, VN_TZ);
  return jdToDate(monthStart + day - 1);
}

export function leapMonth(lunarYear: number): number | null {
  const a11 = lunarMonth11(lunarYear - 1, VN_TZ);
  const b11 = lunarMonth11(lunarYear, VN_TZ);
  if (b11 - a11 <= 365) return null;
  let month = leapMonthOffset(a11, VN_TZ) - 2;
  if (month < 0) month += 12;
  return month;
}

function solarLongitudeToJd(targetDeg: number, year: number): number {
  const monthGuess = ((targetDeg + 90) % 360) / 30;
  const month = (floor(monthGuess) % 12) + 1;
  let jd = jdFromDate(1, month, year);
  for (let i = 0; i < 20; i++) {
    const lon = sunLongitudeDeg(jd);
    let diff = targetDeg - lon;
    while (diff > 180) diff -= 360;
    while (diff < -180) diff += 360;
    jd += diff / 0.985647;
  }
  return jd;
}

export function getJieqi(year: number, month: number, day: number): JieQi {
  validateSolar(year, month, day);
  const query = jdFromDate(day, month, year);
  let best: JieQi | null = null;
  let bestJd = -Infinity;
  for (let y = year - 1; y <= year + 1; y++) {
    for (let i = 0; i < 24; i++) {
      const target = i * 15;
      const jdUtc = solarLongitudeToJd(target, y);
      const local = jdUtc + VN_TZ / 24;
      const z = Math.floor(local + 0.5);
      const f = local + 0.5 - z;
      const solarDate = jdToDate(z);
      const seconds = Math.round(f * 86400);
      const termJd = jdFromDate(solarDate.day, solarDate.month, solarDate.year);
      if (termJd <= query && termJd >= bestJd) {
        bestJd = termJd;
        best = {
          name: JIEQI[((i % 24) + 24) % 24],
          solarDate,
          exactTime: {
            hour: Math.floor(seconds / 3600),
            minute: Math.floor((seconds % 3600) / 60),
            second: seconds % 60,
          },
        };
      }
    }
  }
  if (!best) throw new Error("solar term calculation failed");
  return best;
}

const HOLIDAY_VI: Record<string, string> = {
  TetNguyenDan: "Tết Nguyên Đán",
  TetNguyenTieu: "Tết Nguyên Tiêu",
  GioToHungVuong: "Giỗ Tổ Hùng Vương",
  LeNgayQuocTe: "Ngày Quốc tế Lao động",
  LeGiaiPhong: "Ngày Giải phóng miền Nam",
  QuocKhanh: "Ngày Quốc khánh",
  VuLan: "Vu Lan (Lễ báo hiếu)",
  TrungThu: "Tết Trung Thu",
  PhatDan: "Lễ Phật Đản",
  Christmas: "Giáng sinh",
  NewYear: "Năm mới dương lịch",
  Valentine: "Valentine",
  PhuNuVietNam: "Ngày Phụ nữ Việt Nam",
  GiaoThua: "Giao thừa",
};

function holiday(key: keyof typeof HOLIDAY_VI, solar: SolarDate, isPublic: boolean): Holiday {
  return {
    name: HOLIDAY_VI[key],
    solarDate: solar,
    lunarDate: solarToLunar(solar.year, solar.month, solar.day),
    isPublicHoliday: isPublic,
  };
}

export function getVietnamHolidays(year: number): Holiday[] {
  if (year < MIN_YEAR || year > MAX_YEAR) {
    throw new Error(`year out of supported range (1900-2100): ${year}`);
  }
  const holidays: Holiday[] = [];
  const solarFixed: Array<[number, number, keyof typeof HOLIDAY_VI, boolean]> = [
    [1, 1, "NewYear", true],
    [2, 14, "Valentine", false],
    [4, 30, "LeGiaiPhong", true],
    [5, 1, "LeNgayQuocTe", true],
    [9, 2, "QuocKhanh", true],
    [10, 20, "PhuNuVietNam", false],
    [12, 25, "Christmas", false],
  ];
  for (const [m, d, name, pub] of solarFixed) {
    holidays.push(holiday(name, { year, month: m, day: d }, pub));
  }
  for (let d = 1; d <= 3; d++) {
    const solar = lunarToSolar(year, 1, d, false);
    if (solar.year === year) holidays.push(holiday("TetNguyenDan", solar, true));
  }
  try {
    let giaoThua: SolarDate;
    try {
      giaoThua = lunarToSolar(year - 1, 12, 30, false);
    } catch {
      giaoThua = lunarToSolar(year - 1, 12, 29, false);
    }
    if (giaoThua.year === year) holidays.push(holiday("GiaoThua", giaoThua, true));
  } catch {
    /* 1900 edge */
  }
  const lunarEvents: Array<[number, number, keyof typeof HOLIDAY_VI, boolean]> = [
    [1, 15, "TetNguyenTieu", false],
    [3, 10, "GioToHungVuong", true],
    [4, 15, "PhatDan", false],
    [7, 15, "VuLan", false],
    [8, 15, "TrungThu", false],
  ];
  for (const [m, d, name, pub] of lunarEvents) {
    const solar = lunarToSolar(year, m, d, false);
    if (solar.year === year) holidays.push(holiday(name, solar, pub));
  }
  holidays.sort(
    (a, b) =>
      a.solarDate.year - b.solarDate.year ||
      a.solarDate.month - b.solarDate.month ||
      a.solarDate.day - b.solarDate.day,
  );
  return holidays;
}

const HOANG_DAO: Record<number, number[]> = {
  1: [0, 1, 5, 7],
  7: [0, 1, 5, 7],
  2: [2, 3, 6, 8],
  8: [2, 3, 6, 8],
  3: [4, 5, 8, 10],
  9: [4, 5, 8, 10],
  4: [6, 7, 9, 11],
  10: [6, 7, 9, 11],
  5: [8, 9, 11, 1],
  11: [8, 9, 11, 1],
  6: [10, 11, 2, 4],
  12: [10, 11, 2, 4],
};

export function getGoodDays(lunarYear: number, lunarMonth: number): GoodDay[] {
  if (lunarYear < MIN_YEAR || lunarYear > MAX_YEAR) {
    throw new Error(`year out of supported range (1900-2100): ${lunarYear}`);
  }
  if (lunarMonth < 1 || lunarMonth > 12) throw new Error(`invalid lunar month: ${lunarMonth}`);
  const days: GoodDay[] = [];
  const branches = HOANG_DAO[lunarMonth] ?? [];
  for (let day = 1; day <= 30; day++) {
    let solar: SolarDate;
    try {
      solar = lunarToSolar(lunarYear, lunarMonth, day, false);
    } catch {
      break;
    }
    const lunar = solarToLunar(solar.year, solar.month, solar.day);
    if (lunar.month !== lunarMonth || lunar.leap || lunar.year !== lunarYear) break;
    const branch = remEuclid(jdFromDate(solar.day, solar.month, solar.year) + 1, 12);
    if (branches.includes(branch)) {
      days.push({
        lunarDate: lunar,
        solarDate: solar,
        category: "Hoàng đạo",
        description: `Ngày ${lunar.vietnameseName} — Hoàng đạo`,
      });
    }
  }
  return days;
}

export class LunarCalendar {
  solarToLunar(year: number, month: number, day: number): LunarDate {
    return solarToLunar(year, month, day);
  }

  lunarToSolar(year: number, month: number, day: number, leap = false): SolarDate {
    return lunarToSolar(year, month, day, leap);
  }

  leapMonth(lunarYear: number): number | null {
    return leapMonth(lunarYear);
  }

  getJieqi(year: number, month: number, day: number): JieQi {
    return getJieqi(year, month, day);
  }

  getVietnamHolidays(year: number): Holiday[] {
    return getVietnamHolidays(year);
  }

  getGoodDays(lunarYear: number, lunarMonth: number): GoodDay[] {
    return getGoodDays(lunarYear, lunarMonth);
  }

  getZodiac(lunarYear: number): string {
    return getZodiac(lunarYear);
  }

  getStemBranch(lunarYear: number): string {
    return getStemBranch(lunarYear);
  }
}
