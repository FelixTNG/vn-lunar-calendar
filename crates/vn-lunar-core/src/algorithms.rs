//! Vietnamese lunisolar conversion (Hồ Ngọc Đức, UTC+7).
//!
//! Month 11 always contains Đông Chí. A leap month is the first month after
//! that Đông Chí which contains no major solar term (trung khí).

use crate::{
    validate_solar_date, EarthlyBranch, GoodDay, GoodDayCategory, HeavenlyStem, Holiday,
    HolidayName, JieQi, JieQiName, LunarDate, LunarError, SolarDate, StemBranch, TimeOfDay,
    Zodiac, MAX_YEAR, MIN_YEAR,
};

/// Vietnam Standard Time offset used by the official civil calendar.
pub const VN_TZ: f64 = 7.0;

const SYNODIC_MONTH: f64 = 29.530588853;
const NEW_MOON_EPOCH: f64 = 2_415_021.076_998_695;

fn floor_i(x: f64) -> i32 {
    x.floor() as i32
}

/// Julian day number for a Gregorian (or Julian, if before 1582-10-15) date.
pub fn jd_from_date(day: u8, month: u8, year: i32) -> i32 {
    let a = floor_i((14.0 - f64::from(month)) / 12.0);
    let y = year + 4800 - a;
    let m = i32::from(month) + 12 * a - 3;
    let mut jd = i32::from(day) + floor_i((153.0 * f64::from(m) + 2.0) / 5.0) + 365 * y
        + floor_i(f64::from(y) / 4.0)
        - floor_i(f64::from(y) / 100.0)
        + floor_i(f64::from(y) / 400.0)
        - 32045;
    if jd < 2_299_161 {
        jd = i32::from(day) + floor_i((153.0 * f64::from(m) + 2.0) / 5.0) + 365 * y
            + floor_i(f64::from(y) / 4.0)
            - 32083;
    }
    jd
}

/// Gregorian date for a Julian day number.
pub fn jd_to_date(jd: i32) -> SolarDate {
    let (b, c) = if jd > 2_299_160 {
        let a = jd + 32044;
        let b = floor_i((4.0 * f64::from(a) + 3.0) / 146097.0);
        let c = a - floor_i((f64::from(b) * 146097.0) / 4.0);
        (b, c)
    } else {
        (0, jd + 32082)
    };
    let d = floor_i((4.0 * f64::from(c) + 3.0) / 1461.0);
    let e = c - floor_i((1461.0 * f64::from(d)) / 4.0);
    let m = floor_i((5.0 * f64::from(e) + 2.0) / 153.0);
    let day = (e - floor_i((153.0 * f64::from(m) + 2.0) / 5.0) + 1) as u8;
    let month = (m + 3 - 12 * floor_i(f64::from(m) / 10.0)) as u8;
    let year = b * 100 + d - 4800 + floor_i(f64::from(m) / 10.0);
    SolarDate::new(year, month, day)
}

fn new_moon(k: i32) -> f64 {
    let k = f64::from(k);
    let t = k / 1236.85;
    let t2 = t * t;
    let t3 = t2 * t;
    let dr = std::f64::consts::PI / 180.0;
    let mut jd1 = 2_415_020.759_33 + 29.530_588_68 * k + 0.000_117_8 * t2 - 0.000_000_155 * t3;
    jd1 += 0.000_33 * ((166.56 + 132.87 * t - 0.009_173 * t2) * dr).sin();
    let m = 359.2242 + 29.105_356_08 * k - 0.000_033_3 * t2 - 0.000_003_47 * t3;
    let mpr = 306.0253 + 385.816_918_06 * k + 0.010_730_6 * t2 + 0.000_012_36 * t3;
    let f = 21.2964 + 390.670_506_46 * k - 0.001_652_8 * t2 - 0.000_002_39 * t3;
    let mut c1 = (0.1734 - 0.000_393 * t) * (m * dr).sin() + 0.0021 * (2.0 * dr * m).sin();
    c1 = c1 - 0.4068 * (mpr * dr).sin() + 0.0161 * (dr * 2.0 * mpr).sin();
    c1 -= 0.0004 * (dr * 3.0 * mpr).sin();
    c1 = c1 + 0.0104 * (dr * 2.0 * f).sin() - 0.0051 * (dr * (m + mpr)).sin();
    c1 = c1 - 0.0074 * (dr * (m - mpr)).sin() + 0.0004 * (dr * (2.0 * f + m)).sin();
    c1 = c1 - 0.0004 * (dr * (2.0 * f - m)).sin() - 0.0006 * (dr * (2.0 * f + mpr)).sin();
    c1 = c1 + 0.0010 * (dr * (2.0 * f - mpr)).sin() + 0.0005 * (dr * (2.0 * mpr + m)).sin();
    let deltat = if t < -11.0 {
        0.001 + 0.000_839 * t + 0.000_226_1 * t2 - 0.000_008_45 * t3 - 0.000_000_081 * t * t3
    } else {
        -0.000_278 + 0.000_265 * t + 0.000_262 * t2
    };
    jd1 + c1 - deltat
}

fn new_moon_day(k: i32, time_zone: f64) -> i32 {
    floor_i(new_moon(k) + 0.5 + time_zone / 24.0)
}

/// Sun true longitude in radians, range `[0, 2π)`.
fn sun_longitude_rad(jdn: f64) -> f64 {
    let t = (jdn - 2_451_545.0) / 36525.0;
    let t2 = t * t;
    let dr = std::f64::consts::PI / 180.0;
    let m = 357.52910 + 35999.05030 * t - 0.000_155_9 * t2 - 0.000_000_48 * t * t2;
    let l0 = 280.46645 + 36000.76983 * t + 0.000_303_2 * t2;
    let mut dl = (1.914600 - 0.004_817 * t - 0.000_014 * t2) * (dr * m).sin();
    dl += (0.019_993 - 0.000_101 * t) * (dr * 2.0 * m).sin() + 0.000_290 * (dr * 3.0 * m).sin();
    let mut l = (l0 + dl) * dr;
    l -= std::f64::consts::PI * 2.0 * (l / (std::f64::consts::PI * 2.0)).floor();
    l
}

/// Sun true longitude in degrees, range `[0, 360)`.
pub fn sun_longitude_deg(jdn: f64) -> f64 {
    sun_longitude_rad(jdn) * 180.0 / std::f64::consts::PI
}

/// Major solar-term index 0..=11 (0 = Xuân Phân, 9 = Đông Chí).
fn major_term_index(day_number: i32, time_zone: f64) -> i32 {
    // `/ π * 6` maps `[0, 2π)` onto `[0, 12)` so Đông Chí (270°) is 9.
    floor_i(sun_longitude_rad(f64::from(day_number) - 0.5 - time_zone / 24.0) / std::f64::consts::PI * 6.0)
}

fn lunar_month_11(year: i32, time_zone: f64) -> i32 {
    let off = f64::from(jd_from_date(31, 12, year) - 2_415_021);
    let k = floor_i(off / SYNODIC_MONTH);
    let nm = new_moon_day(k, time_zone);
    if major_term_index(nm, time_zone) >= 9 {
        new_moon_day(k - 1, time_zone)
    } else {
        nm
    }
}

fn leap_month_offset(a11: i32, time_zone: f64) -> i32 {
    let k = floor_i(0.5 + (f64::from(a11) - NEW_MOON_EPOCH) / SYNODIC_MONTH);
    let mut i = 1;
    let mut arc = major_term_index(new_moon_day(k + i, time_zone), time_zone);
    loop {
        let last = arc;
        i += 1;
        arc = major_term_index(new_moon_day(k + i, time_zone), time_zone);
        if arc == last || i >= 14 {
            break;
        }
    }
    i - 1
}

/// Leap month number (1..=12) of a lunar year, if any.
pub fn leap_month(lunar_year: i32) -> Option<u8> {
    let a11 = lunar_month_11(lunar_year - 1, VN_TZ);
    let b11 = lunar_month_11(lunar_year, VN_TZ);
    if b11 - a11 <= 365 {
        return None;
    }
    let mut month = leap_month_offset(a11, VN_TZ) - 2;
    if month < 0 {
        month += 12;
    }
    Some(month as u8)
}

fn lunar_date_parts(
    year: i32,
    month: u8,
    day: u8,
    leap: bool,
) -> Result<LunarDate, LunarError> {
    Ok(LunarDate {
        year,
        month,
        day,
        leap,
        zodiac: Zodiac::from_lunar_year(year),
        stem_branch: get_stem_branch(year),
    })
}

/// Convert a Gregorian date (Vietnam civil day) to a lunar date.
pub fn solar_to_lunar(year: i32, month: u8, day: u8) -> Result<LunarDate, LunarError> {
    validate_solar_date(year, month, day)?;
    let time_zone = VN_TZ;
    let day_number = jd_from_date(day, month, year);
    let k = floor_i((f64::from(day_number) - NEW_MOON_EPOCH) / SYNODIC_MONTH);
    let mut month_start = new_moon_day(k + 1, time_zone);
    if month_start > day_number {
        month_start = new_moon_day(k, time_zone);
    }
    let mut a11 = lunar_month_11(year, time_zone);
    let mut b11 = a11;
    let mut lunar_year = if a11 >= month_start {
        a11 = lunar_month_11(year - 1, time_zone);
        year
    } else {
        b11 = lunar_month_11(year + 1, time_zone);
        year + 1
    };
    let lunar_day = (day_number - month_start + 1) as u8;
    let diff = floor_i(f64::from(month_start - a11) / 29.0);
    let mut lunar_leap = false;
    let mut lunar_month = diff + 11;
    if b11 - a11 > 365 {
        let leap_diff = leap_month_offset(a11, time_zone);
        if diff >= leap_diff {
            lunar_month = diff + 10;
            if diff == leap_diff {
                lunar_leap = true;
            }
        }
    }
    if lunar_month > 12 {
        lunar_month -= 12;
    }
    if lunar_month >= 11 && diff < 4 {
        lunar_year -= 1;
    }
    lunar_date_parts(lunar_year, lunar_month as u8, lunar_day, lunar_leap)
}

/// Convert a lunar date to Gregorian (Vietnam civil day).
pub fn lunar_to_solar(
    year: i32,
    month: u8,
    day: u8,
    leap: bool,
) -> Result<SolarDate, LunarError> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(LunarError::YearOutOfRange(year));
    }
    if !(1..=12).contains(&month) {
        return Err(LunarError::InvalidLunarMonth(month));
    }
    if !(1..=30).contains(&day) {
        return Err(LunarError::InvalidLunarDay(day));
    }

    let time_zone = VN_TZ;
    let (a11, b11) = if month < 11 {
        (
            lunar_month_11(year - 1, time_zone),
            lunar_month_11(year, time_zone),
        )
    } else {
        (
            lunar_month_11(year, time_zone),
            lunar_month_11(year + 1, time_zone),
        )
    };
    let mut off = i32::from(month) - 11;
    if off < 0 {
        off += 12;
    }
    if b11 - a11 > 365 {
        let leap_off = leap_month_offset(a11, time_zone);
        let mut leap_month_n = leap_off - 2;
        if leap_month_n < 0 {
            leap_month_n += 12;
        }
        if leap && i32::from(month) != leap_month_n {
            return Err(LunarError::LeapMonthError);
        }
        if leap || off >= leap_off {
            off += 1;
        }
    } else if leap {
        return Err(LunarError::LeapMonthError);
    }
    let k = floor_i(0.5 + (f64::from(a11) - NEW_MOON_EPOCH) / SYNODIC_MONTH);
    let month_start = new_moon_day(k + off, time_zone);
    let solar = jd_to_date(month_start + i32::from(day) - 1);
    if solar.year < MIN_YEAR || solar.year > MAX_YEAR {
        return Err(LunarError::YearOutOfRange(solar.year));
    }
    Ok(solar)
}

/// Instant (Julian day, UTC) when the sun reaches `target_deg`.
fn solar_longitude_to_jd(target_deg: f64, year: i32) -> f64 {
    let month_guess = ((target_deg + 90.0) % 360.0) / 30.0;
    let month = ((month_guess as u8) % 12) + 1;
    let mut jd = f64::from(jd_from_date(1, month, year));
    for _ in 0..20 {
        let lon = sun_longitude_deg(jd);
        let mut diff = target_deg - lon;
        while diff > 180.0 {
            diff -= 360.0;
        }
        while diff < -180.0 {
            diff += 360.0;
        }
        jd += diff / 0.985_647;
    }
    jd
}

fn vietnam_civil_from_jd_utc(jd_utc: f64) -> (SolarDate, TimeOfDay) {
    let local = jd_utc + VN_TZ / 24.0;
    let z = (local + 0.5).floor();
    let f = local + 0.5 - z;
    let date = jd_to_date(z as i32);
    let seconds = (f * 86400.0).round() as u32;
    let hour = (seconds / 3600) as u8;
    let minute = ((seconds % 3600) / 60) as u8;
    let second = (seconds % 60) as u8;
    (date, TimeOfDay { hour, minute, second })
}

/// Solar term in effect on a Gregorian date (Vietnam civil day).
pub fn get_jieqi(year: i32, month: u8, day: u8) -> Result<JieQi, LunarError> {
    validate_solar_date(year, month, day)?;
    let query = jd_from_date(day, month, year);
    let mut best: Option<JieQi> = None;
    for y in (year - 1)..=(year + 1) {
        for i in 0..24 {
            let target = f64::from(i) * 15.0;
            let jd_utc = solar_longitude_to_jd(target, y);
            let (solar_date, exact_time) = vietnam_civil_from_jd_utc(jd_utc);
            let term_jd = jd_from_date(solar_date.day, solar_date.month, solar_date.year);
            if term_jd <= query {
                let candidate = JieQi {
                    name: JieQiName::from_longitude_index(floor_i(target / 15.0).rem_euclid(24) as u8),
                    solar_date,
                    exact_time: Some(exact_time),
                };
                let better = best.as_ref().map_or(true, |b| {
                    jd_from_date(b.solar_date.day, b.solar_date.month, b.solar_date.year) <= term_jd
                });
                if better {
                    best = Some(candidate);
                }
            }
        }
    }
    best.ok_or(LunarError::SolarTermError)
}

fn holiday(
    name: HolidayName,
    solar: SolarDate,
    is_public: bool,
) -> Result<Holiday, LunarError> {
    let lunar = solar_to_lunar(solar.year, solar.month, solar.day)?;
    Ok(Holiday {
        name,
        solar_date: solar,
        lunar_date: lunar,
        is_public_holiday: is_public,
    })
}

/// Vietnamese civil and traditional holidays whose solar date falls in `year`.
pub fn get_vietnam_holidays(year: i32) -> Result<Vec<Holiday>, LunarError> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(LunarError::YearOutOfRange(year));
    }
    let mut holidays = Vec::new();

    let solar_fixed = [
        (1_u8, 1_u8, HolidayName::NewYear, true),
        (2, 14, HolidayName::Valentine, false),
        (4, 30, HolidayName::LeGiaiPhong, true),
        (5, 1, HolidayName::LeNgayQuocTe, true),
        (9, 2, HolidayName::QuocKhanh, true),
        (10, 20, HolidayName::PhuNuVietNam, false),
        (12, 25, HolidayName::Christmas, false),
    ];
    for (m, d, name, public) in solar_fixed {
        holidays.push(holiday(name, SolarDate::new(year, m, d), public)?);
    }

    for d in 1..=3 {
        let solar = lunar_to_solar(year, 1, d, false)?;
        if solar.year == year {
            holidays.push(holiday(HolidayName::TetNguyenDan, solar, true)?);
        }
    }

    let giao_thua = lunar_to_solar(year - 1, 12, 30, false)
        .or_else(|_| lunar_to_solar(year - 1, 12, 29, false))?;
    if giao_thua.year == year {
        holidays.push(holiday(HolidayName::GiaoThua, giao_thua, true)?);
    }

    let lunar_events = [
        (1_u8, 15_u8, HolidayName::TetNguyenTieu, false),
        (3, 10, HolidayName::GioToHungVuong, true),
        (4, 15, HolidayName::PhatDan, false),
        (7, 15, HolidayName::VuLan, false),
        (8, 15, HolidayName::TrungThu, false),
    ];
    for (m, d, name, public) in lunar_events {
        let solar = lunar_to_solar(year, m, d, false)?;
        if solar.year == year {
            holidays.push(holiday(name, solar, public)?);
        }
    }

    holidays.sort_by_key(|h| (h.solar_date.year, h.solar_date.month, h.solar_date.day));
    Ok(holidays)
}

fn day_branch_from_jd(jd: i32) -> EarthlyBranch {
    EarthlyBranch::from_index(((jd + 1).rem_euclid(12)) as u8)
}

fn is_hoang_dao(lunar_month: u8, branch: EarthlyBranch) -> bool {
    use EarthlyBranch::*;
    let set: &[EarthlyBranch] = match lunar_month {
        1 | 7 => &[Ty, Suu, Ty2, Mui],
        2 | 8 => &[Dan, Mao, Ngo, Than],
        3 | 9 => &[Thin, Ty2, Than, Tuat],
        4 | 10 => &[Ngo, Mui, Dau, Hoi],
        5 | 11 => &[Than, Dau, Hoi, Suu],
        6 | 12 => &[Tuat, Hoi, Dan, Thin],
        _ => return false,
    };
    set.contains(&branch)
}

/// Hoàng đạo days in a lunar month (non-leap month).
pub fn get_good_days(lunar_year: i32, lunar_month: u8) -> Result<Vec<GoodDay>, LunarError> {
    if lunar_year < MIN_YEAR || lunar_year > MAX_YEAR {
        return Err(LunarError::YearOutOfRange(lunar_year));
    }
    if !(1..=12).contains(&lunar_month) {
        return Err(LunarError::InvalidLunarMonth(lunar_month));
    }

    let mut good_days = Vec::new();
    for day in 1..=30 {
        let Ok(solar) = lunar_to_solar(lunar_year, lunar_month, day, false) else {
            break;
        };
        let lunar = solar_to_lunar(solar.year, solar.month, solar.day)?;
        if lunar.month != lunar_month || lunar.leap || lunar.year != lunar_year {
            break;
        }
        let branch = day_branch_from_jd(jd_from_date(solar.day, solar.month, solar.year));
        if is_hoang_dao(lunar_month, branch) {
            good_days.push(GoodDay {
                lunar_date: lunar.clone(),
                solar_date: solar,
                category: GoodDayCategory::HoangDao,
                description: format!(
                    "Ngày {} — {}",
                    lunar.vietnamese_name(),
                    GoodDayCategory::HoangDao.vietnamese()
                ),
            });
        }
    }
    Ok(good_days)
}

/// Zodiac animal of a lunar year (1984 = Tý).
pub fn get_zodiac(lunar_year: i32) -> Zodiac {
    Zodiac::from_lunar_year(lunar_year)
}

/// Can-chi of a lunar year (1984 = Giáp Tý).
pub fn get_stem_branch(lunar_year: i32) -> StemBranch {
    let cycle_idx = (lunar_year - 1984).rem_euclid(60) as u8;
    StemBranch {
        stem: HeavenlyStem::from_index(cycle_idx % 10),
        branch: EarthlyBranch::from_index(cycle_idx % 12),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jd_round_trip_j2000() {
        let jd = jd_from_date(1, 1, 2000);
        let d = jd_to_date(jd);
        assert_eq!(d, SolarDate::new(2000, 1, 1));
    }

    #[test]
    fn stem_branch_cycle() {
        let sb1984 = get_stem_branch(1984);
        assert_eq!(sb1984.stem, HeavenlyStem::Giap);
        assert_eq!(sb1984.branch, EarthlyBranch::Ty);
        assert_eq!(get_stem_branch(2044).stem, HeavenlyStem::Giap);
    }
}
