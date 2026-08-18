//! Vietnamese Lunar Calendar core.
//!
//! Civil conversions use Hồ Ngọc Đức's algorithm at UTC+7, the timezone of
//! the Vietnamese lunisolar calendar.

#![deny(unsafe_code)]

use core::fmt;

pub mod algorithms;
#[cfg(feature = "c-api")]
#[allow(unsafe_code)]
pub mod ffi;

pub use algorithms::{
    get_good_days, get_jieqi, get_stem_branch, get_vietnam_holidays, get_zodiac, leap_month,
    lunar_to_solar, solar_to_lunar, VN_TZ,
};

/// Main calendar interface. Cheap to copy; holds no cache.
#[derive(Debug, Default, Clone, Copy)]
pub struct LunarCalendar;

impl LunarCalendar {
    /// Create a calendar using Vietnam Standard Time (UTC+7).
    pub fn new() -> Self {
        Self
    }

    /// Gregorian → lunar (Vietnam civil date).
    pub fn solar_to_lunar(&self, year: i32, month: u8, day: u8) -> Result<LunarDate, LunarError> {
        algorithms::solar_to_lunar(year, month, day)
    }

    /// Lunar → Gregorian (Vietnam civil date).
    pub fn lunar_to_solar(
        &self,
        year: i32,
        month: u8,
        day: u8,
        leap: bool,
    ) -> Result<SolarDate, LunarError> {
        algorithms::lunar_to_solar(year, month, day, leap)
    }

    /// Leap month (1..=12) of a lunar year, if that year is intercalary.
    pub fn leap_month(&self, lunar_year: i32) -> Option<u8> {
        algorithms::leap_month(lunar_year)
    }

    /// Solar term in effect on a Gregorian date.
    pub fn get_jieqi(&self, year: i32, month: u8, day: u8) -> Result<JieQi, LunarError> {
        algorithms::get_jieqi(year, month, day)
    }

    /// Holidays whose Gregorian date falls in `year`.
    pub fn get_vietnam_holidays(&self, year: i32) -> Result<Vec<Holiday>, LunarError> {
        algorithms::get_vietnam_holidays(year)
    }

    /// Hoàng đạo days in a (non-leap) lunar month.
    pub fn get_good_days(
        &self,
        lunar_year: i32,
        lunar_month: u8,
    ) -> Result<Vec<GoodDay>, LunarError> {
        algorithms::get_good_days(lunar_year, lunar_month)
    }

    /// Zodiac animal of a lunar year.
    pub fn get_zodiac(&self, lunar_year: i32) -> Zodiac {
        algorithms::get_zodiac(lunar_year)
    }

    /// Can-chi of a lunar year.
    pub fn get_stem_branch(&self, lunar_year: i32) -> StemBranch {
        algorithms::get_stem_branch(lunar_year)
    }
}

/// Gregorian date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SolarDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

impl SolarDate {
    pub fn new(year: i32, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }
}

impl fmt::Display for SolarDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

/// Lunar date with year-level Vietnamese metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LunarDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub leap: bool,
    pub zodiac: Zodiac,
    pub stem_branch: StemBranch,
}

impl LunarDate {
    /// Display name such as `"Mùng 1 Tết Ất Tỵ"`.
    pub fn vietnamese_name(&self) -> String {
        let day_name = match self.day {
            15 => "Rằm".to_string(),
            d if d <= 10 => format!("Mùng {d}"),
            d => format!("Ngày {d}"),
        };
        let leap_str = if self.leap { " (nhuận)" } else { "" };
        let month_name = if self.month == 1 && self.day <= 3 && !self.leap {
            "Tết".to_string()
        } else {
            format!("Tháng {}{leap_str}", self.month)
        };
        format!(
            "{day_name} {month_name} {}",
            self.stem_branch.vietnamese()
        )
    }

    /// Tết Nguyên Đán (1–3/1 âm lịch, not a leap month).
    pub fn is_tet(&self) -> bool {
        self.month == 1 && self.day <= 3 && !self.leap
    }

    pub fn is_full_moon(&self) -> bool {
        self.day == 15
    }

    pub fn is_new_moon(&self) -> bool {
        self.day == 1
    }
}

impl fmt::Display for LunarDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02}{}",
            self.year,
            self.month,
            self.day,
            if self.leap { "L" } else { "" }
        )
    }
}

/// Solar term (tiết khí).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JieQi {
    pub name: JieQiName,
    pub solar_date: SolarDate,
    pub exact_time: Option<TimeOfDay>,
}

impl fmt::Display for JieQi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name.vietnamese(), self.solar_date)
    }
}

/// 24 solar terms. `from_longitude_index(0)` is Xuân Phân (0°).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JieQiName {
    LapXuan,
    VuThuy,
    KinhTrich,
    XuanPhan,
    ThanhMinh,
    CocVu,
    LapHa,
    TieuMan,
    MangChung,
    HaChi,
    TieuThu,
    DaiThu,
    LapThu,
    XySu,
    BachLo,
    ThuPhan,
    HanLo,
    SuongGiang,
    LapDong,
    TieuTuyet,
    DaiTuyet,
    DongChi,
    TieuHan,
    DaiHan,
}

impl JieQiName {
    pub fn vietnamese(&self) -> &'static str {
        match self {
            JieQiName::LapXuan => "Lập Xuân",
            JieQiName::VuThuy => "Vũ Thủy",
            JieQiName::KinhTrich => "Kinh Trập",
            JieQiName::XuanPhan => "Xuân Phân",
            JieQiName::ThanhMinh => "Thanh Minh",
            JieQiName::CocVu => "Cốc Vũ",
            JieQiName::LapHa => "Lập Hạ",
            JieQiName::TieuMan => "Tiểu Mãn",
            JieQiName::MangChung => "Mang Chủng",
            JieQiName::HaChi => "Hạ Chí",
            JieQiName::TieuThu => "Tiểu Thử",
            JieQiName::DaiThu => "Đại Thử",
            JieQiName::LapThu => "Lập Thu",
            JieQiName::XySu => "Xử Thử",
            JieQiName::BachLo => "Bạch Lộ",
            JieQiName::ThuPhan => "Thu Phân",
            JieQiName::HanLo => "Hàn Lộ",
            JieQiName::SuongGiang => "Sương Giáng",
            JieQiName::LapDong => "Lập Đông",
            JieQiName::TieuTuyet => "Tiểu Tuyết",
            JieQiName::DaiTuyet => "Đại Tuyết",
            JieQiName::DongChi => "Đông Chí",
            JieQiName::TieuHan => "Tiểu Hàn",
            JieQiName::DaiHan => "Đại Hàn",
        }
    }

    /// Index 0..=23 where 0° solar longitude is Xuân Phân.
    pub fn from_longitude_index(idx: u8) -> Self {
        match idx % 24 {
            0 => JieQiName::XuanPhan,
            1 => JieQiName::ThanhMinh,
            2 => JieQiName::CocVu,
            3 => JieQiName::LapHa,
            4 => JieQiName::TieuMan,
            5 => JieQiName::MangChung,
            6 => JieQiName::HaChi,
            7 => JieQiName::TieuThu,
            8 => JieQiName::DaiThu,
            9 => JieQiName::LapThu,
            10 => JieQiName::XySu,
            11 => JieQiName::BachLo,
            12 => JieQiName::ThuPhan,
            13 => JieQiName::HanLo,
            14 => JieQiName::SuongGiang,
            15 => JieQiName::LapDong,
            16 => JieQiName::TieuTuyet,
            17 => JieQiName::DaiTuyet,
            18 => JieQiName::DongChi,
            19 => JieQiName::TieuHan,
            20 => JieQiName::DaiHan,
            21 => JieQiName::LapXuan,
            22 => JieQiName::VuThuy,
            23 => JieQiName::KinhTrich,
            _ => unreachable!(),
        }
    }
}

/// Time of day (Vietnam local, after converting the astronomical instant).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeOfDay {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

/// Vietnamese holiday.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Holiday {
    pub name: HolidayName,
    pub solar_date: SolarDate,
    pub lunar_date: LunarDate,
    pub is_public_holiday: bool,
}

impl fmt::Display for Holiday {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} - {} (ÂM: {})",
            self.name.vietnamese(),
            self.solar_date,
            self.lunar_date
        )
    }
}

/// Holiday names (civil + traditional).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HolidayName {
    TetNguyenDan,
    TetNguyenTieu,
    GioToHungVuong,
    LeNgayQuocTe,
    LeGiaiPhong,
    QuocKhanh,
    VuLan,
    TrungThu,
    PhatDan,
    Christmas,
    NewYear,
    Valentine,
    PhuNuVietNam,
    GiaoThua,
}

impl HolidayName {
    pub fn vietnamese(&self) -> &'static str {
        match self {
            HolidayName::TetNguyenDan => "Tết Nguyên Đán",
            HolidayName::TetNguyenTieu => "Tết Nguyên Tiêu",
            HolidayName::GioToHungVuong => "Giỗ Tổ Hùng Vương",
            HolidayName::LeNgayQuocTe => "Ngày Quốc tế Lao động",
            HolidayName::LeGiaiPhong => "Ngày Giải phóng miền Nam",
            HolidayName::QuocKhanh => "Ngày Quốc khánh",
            HolidayName::VuLan => "Vu Lan (Lễ báo hiếu)",
            HolidayName::TrungThu => "Tết Trung Thu",
            HolidayName::PhatDan => "Lễ Phật Đản",
            HolidayName::Christmas => "Giáng sinh",
            HolidayName::NewYear => "Năm mới dương lịch",
            HolidayName::Valentine => "Valentine",
            HolidayName::PhuNuVietNam => "Ngày Phụ nữ Việt Nam",
            HolidayName::GiaoThua => "Giao thừa",
        }
    }
}

/// Auspicious-day annotation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoodDay {
    pub lunar_date: LunarDate,
    pub solar_date: SolarDate,
    pub category: GoodDayCategory,
    pub description: String,
}

/// Category of auspicious day.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GoodDayCategory {
    HoangDao,
    ThienDe,
    NguyetDe,
    SanhQi,
    ThienEn,
    PhucDuc,
}

impl GoodDayCategory {
    pub fn vietnamese(&self) -> &'static str {
        match self {
            GoodDayCategory::HoangDao => "Hoàng đạo",
            GoodDayCategory::ThienDe => "Thiên Đệ",
            GoodDayCategory::NguyetDe => "Nguyệt Đệ",
            GoodDayCategory::SanhQi => "Sinh Khí",
            GoodDayCategory::ThienEn => "Thiên Ân",
            GoodDayCategory::PhucDuc => "Phúc Đức",
        }
    }
}

/// 12 con giáp.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Zodiac {
    Rat,
    Ox,
    Tiger,
    Rabbit,
    Dragon,
    Snake,
    Horse,
    Goat,
    Monkey,
    Rooster,
    Dog,
    Pig,
}

impl Zodiac {
    pub fn vietnamese(&self) -> &'static str {
        match self {
            Zodiac::Rat => "Tý",
            Zodiac::Ox => "Sửu",
            Zodiac::Tiger => "Dần",
            Zodiac::Rabbit => "Mão",
            Zodiac::Dragon => "Thìn",
            Zodiac::Snake => "Tỵ",
            Zodiac::Horse => "Ngọ",
            Zodiac::Goat => "Mùi",
            Zodiac::Monkey => "Thân",
            Zodiac::Rooster => "Dậu",
            Zodiac::Dog => "Tuất",
            Zodiac::Pig => "Hợi",
        }
    }

    pub fn from_lunar_year(year: i32) -> Self {
        Self::from_index(((year - 1984).rem_euclid(12)) as u8)
    }

    pub fn from_index(idx: u8) -> Self {
        match idx % 12 {
            0 => Zodiac::Rat,
            1 => Zodiac::Ox,
            2 => Zodiac::Tiger,
            3 => Zodiac::Rabbit,
            4 => Zodiac::Dragon,
            5 => Zodiac::Snake,
            6 => Zodiac::Horse,
            7 => Zodiac::Goat,
            8 => Zodiac::Monkey,
            9 => Zodiac::Rooster,
            10 => Zodiac::Dog,
            _ => Zodiac::Pig,
        }
    }
}

/// 10 can.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HeavenlyStem {
    Giap,
    At,
    Binh,
    Dinh,
    Mau,
    Ky,
    Canh,
    Tan,
    Nham,
    Quy,
}

impl HeavenlyStem {
    pub fn from_index(idx: u8) -> Self {
        match idx % 10 {
            0 => HeavenlyStem::Giap,
            1 => HeavenlyStem::At,
            2 => HeavenlyStem::Binh,
            3 => HeavenlyStem::Dinh,
            4 => HeavenlyStem::Mau,
            5 => HeavenlyStem::Ky,
            6 => HeavenlyStem::Canh,
            7 => HeavenlyStem::Tan,
            8 => HeavenlyStem::Nham,
            _ => HeavenlyStem::Quy,
        }
    }

    pub fn vietnamese(&self) -> &'static str {
        match self {
            HeavenlyStem::Giap => "Giáp",
            HeavenlyStem::At => "Ất",
            HeavenlyStem::Binh => "Bính",
            HeavenlyStem::Dinh => "Đinh",
            HeavenlyStem::Mau => "Mậu",
            HeavenlyStem::Ky => "Kỷ",
            HeavenlyStem::Canh => "Canh",
            HeavenlyStem::Tan => "Tân",
            HeavenlyStem::Nham => "Nhâm",
            HeavenlyStem::Quy => "Quý",
        }
    }

    pub fn element(&self) -> Element {
        match self {
            HeavenlyStem::Giap | HeavenlyStem::At => Element::Wood,
            HeavenlyStem::Binh | HeavenlyStem::Dinh => Element::Fire,
            HeavenlyStem::Mau | HeavenlyStem::Ky => Element::Earth,
            HeavenlyStem::Canh | HeavenlyStem::Tan => Element::Metal,
            HeavenlyStem::Nham | HeavenlyStem::Quy => Element::Water,
        }
    }

    pub fn yin_yang(&self) -> YinYang {
        match self {
            HeavenlyStem::Giap
            | HeavenlyStem::Binh
            | HeavenlyStem::Mau
            | HeavenlyStem::Canh
            | HeavenlyStem::Nham => YinYang::Yang,
            _ => YinYang::Yin,
        }
    }
}

/// 12 chi.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EarthlyBranch {
    Ty,
    Suu,
    Dan,
    Mao,
    Thin,
    Ty2,
    Ngo,
    Mui,
    Than,
    Dau,
    Tuat,
    Hoi,
}

impl EarthlyBranch {
    pub fn from_index(idx: u8) -> Self {
        match idx % 12 {
            0 => EarthlyBranch::Ty,
            1 => EarthlyBranch::Suu,
            2 => EarthlyBranch::Dan,
            3 => EarthlyBranch::Mao,
            4 => EarthlyBranch::Thin,
            5 => EarthlyBranch::Ty2,
            6 => EarthlyBranch::Ngo,
            7 => EarthlyBranch::Mui,
            8 => EarthlyBranch::Than,
            9 => EarthlyBranch::Dau,
            10 => EarthlyBranch::Tuat,
            _ => EarthlyBranch::Hoi,
        }
    }

    pub fn vietnamese(&self) -> &'static str {
        match self {
            EarthlyBranch::Ty => "Tý",
            EarthlyBranch::Suu => "Sửu",
            EarthlyBranch::Dan => "Dần",
            EarthlyBranch::Mao => "Mão",
            EarthlyBranch::Thin => "Thìn",
            EarthlyBranch::Ty2 => "Tỵ",
            EarthlyBranch::Ngo => "Ngọ",
            EarthlyBranch::Mui => "Mùi",
            EarthlyBranch::Than => "Thân",
            EarthlyBranch::Dau => "Dậu",
            EarthlyBranch::Tuat => "Tuất",
            EarthlyBranch::Hoi => "Hợi",
        }
    }

    pub fn zodiac(&self) -> Zodiac {
        Zodiac::from_index(*self as u8)
    }
}

/// Ngũ hành.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Element {
    Wood,
    Fire,
    Earth,
    Metal,
    Water,
}

impl Element {
    pub fn vietnamese(&self) -> &'static str {
        match self {
            Element::Wood => "Mộc",
            Element::Fire => "Hỏa",
            Element::Earth => "Thổ",
            Element::Metal => "Kim",
            Element::Water => "Thủy",
        }
    }
}

/// Âm / Dương.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum YinYang {
    Yang,
    Yin,
}

impl YinYang {
    pub fn vietnamese(&self) -> &'static str {
        match self {
            YinYang::Yang => "Dương",
            YinYang::Yin => "Âm",
        }
    }
}

/// Can-chi pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StemBranch {
    pub stem: HeavenlyStem,
    pub branch: EarthlyBranch,
}

impl StemBranch {
    pub fn vietnamese(&self) -> String {
        format!("{} {}", self.stem.vietnamese(), self.branch.vietnamese())
    }

    pub fn element(&self) -> Element {
        self.stem.element()
    }

    pub fn yin_yang(&self) -> YinYang {
        self.stem.yin_yang()
    }
}

/// Library errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LunarError {
    InvalidDate(i32, u8, u8),
    YearOutOfRange(i32),
    InvalidLunarMonth(u8),
    InvalidLunarDay(u8),
    LeapMonthError,
    NewMoonError,
    SolarTermError,
}

impl fmt::Display for LunarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LunarError::InvalidDate(y, m, d) => write!(f, "invalid date: {y}-{m}-{d}"),
            LunarError::YearOutOfRange(y) => {
                write!(f, "year out of supported range (1900-2100): {y}")
            }
            LunarError::InvalidLunarMonth(m) => write!(f, "invalid lunar month: {m}"),
            LunarError::InvalidLunarDay(d) => write!(f, "invalid lunar day: {d}"),
            LunarError::LeapMonthError => {
                write!(f, "requested leap month does not exist in that year")
            }
            LunarError::NewMoonError => write!(f, "new moon calculation failed"),
            LunarError::SolarTermError => write!(f, "solar term calculation failed"),
        }
    }
}

impl std::error::Error for LunarError {}

impl LunarError {
    /// Stable C ABI error code.
    pub fn code(self) -> i32 {
        match self {
            LunarError::InvalidDate(_, _, _) => 1,
            LunarError::YearOutOfRange(_) => 2,
            LunarError::InvalidLunarMonth(_) => 3,
            LunarError::InvalidLunarDay(_) => 4,
            LunarError::LeapMonthError => 5,
            LunarError::NewMoonError => 6,
            LunarError::SolarTermError => 7,
        }
    }
}

/// Inclusive minimum supported Gregorian/lunar year.
pub const MIN_YEAR: i32 = 1900;
/// Inclusive maximum supported Gregorian/lunar year.
pub const MAX_YEAR: i32 = 2100;

/// Validate a Gregorian date in the supported range.
pub fn validate_solar_date(year: i32, month: u8, day: u8) -> Result<(), LunarError> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(LunarError::YearOutOfRange(year));
    }
    if !(1..=12).contains(&month) {
        return Err(LunarError::InvalidDate(year, month, day));
    }
    let max_day = days_in_month(year, month);
    if day < 1 || day > max_day {
        return Err(LunarError::InvalidDate(year, month, day));
    }
    Ok(())
}

fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tet_wikipedia_dates() {
        let cal = LunarCalendar::new();
        let tet = [
            (2020, 1, 25, Zodiac::Rat, "Canh Tý"),
            (2021, 2, 12, Zodiac::Ox, "Tân Sửu"),
            (2022, 2, 1, Zodiac::Tiger, "Nhâm Dần"),
            (2023, 1, 22, Zodiac::Rabbit, "Quý Mão"),
            (2024, 2, 10, Zodiac::Dragon, "Giáp Thìn"),
            (2025, 1, 29, Zodiac::Snake, "Ất Tỵ"),
            (2026, 2, 17, Zodiac::Horse, "Bính Ngọ"),
            (2027, 2, 6, Zodiac::Goat, "Đinh Mùi"),
            (2028, 1, 26, Zodiac::Monkey, "Mậu Thân"),
            (2029, 2, 13, Zodiac::Rooster, "Kỷ Dậu"),
            (2030, 2, 2, Zodiac::Dog, "Canh Tuất"),
            (2031, 1, 23, Zodiac::Pig, "Tân Hợi"),
        ];
        for (y, m, d, zodiac, can_chi) in tet {
            let lunar = cal.solar_to_lunar(y, m, d).unwrap();
            assert_eq!((lunar.year, lunar.month, lunar.day, lunar.leap), (y, 1, 1, false), "Tết {y}");
            assert!(lunar.is_tet());
            assert_eq!(lunar.zodiac, zodiac);
            assert_eq!(lunar.stem_branch.vietnamese(), can_chi);
            let solar = cal.lunar_to_solar(y, 1, 1, false).unwrap();
            assert_eq!(solar, SolarDate::new(y, m, d));
        }
    }

    #[test]
    fn vietnam_tet_2007_not_china() {
        let cal = LunarCalendar::new();
        let lunar = cal.solar_to_lunar(2007, 2, 17).unwrap();
        assert_eq!((lunar.year, lunar.month, lunar.day), (2007, 1, 1));
        let before = cal.solar_to_lunar(2007, 2, 16).unwrap();
        assert_ne!(before.day, 1);
    }

    #[test]
    fn day_before_tet_is_previous_year() {
        let cal = LunarCalendar::new();
        let lunar = cal.solar_to_lunar(2025, 1, 28).unwrap();
        assert_eq!(lunar.year, 2024);
        assert_eq!(lunar.zodiac, Zodiac::Dragon);
    }

    #[test]
    fn leap_months() {
        let cal = LunarCalendar::new();
        assert_eq!(cal.leap_month(2020), Some(4));
        assert_eq!(cal.leap_month(2023), Some(2));
        assert_eq!(cal.leap_month(2025), Some(6));
        let leap = cal.solar_to_lunar(2023, 3, 22).unwrap();
        assert_eq!(leap.month, 2);
        assert!(leap.leap);
        let solar = cal.lunar_to_solar(2023, 2, 1, true).unwrap();
        let back = cal.solar_to_lunar(solar.year, solar.month, solar.day).unwrap();
        assert!(back.leap && back.month == 2);
    }

    #[test]
    fn round_trip_sample() {
        let cal = LunarCalendar::new();
        for (y, m, d) in [
            (1901, 2, 19),
            (1985, 4, 5),
            (2000, 1, 1),
            (2007, 2, 17),
            (2025, 6, 15),
            (2099, 12, 31),
        ] {
            let lunar = cal.solar_to_lunar(y, m, d).unwrap();
            let solar = cal
                .lunar_to_solar(lunar.year, lunar.month, lunar.day, lunar.leap)
                .unwrap();
            assert_eq!(solar, SolarDate::new(y, m, d), "round-trip {y}-{m}-{d}");
        }
    }

    #[test]
    fn jieqi_lap_xuan_2025() {
        let cal = LunarCalendar::new();
        let jieqi = cal.get_jieqi(2025, 2, 3).unwrap();
        assert_eq!(jieqi.name, JieQiName::LapXuan);
        assert_eq!(jieqi.solar_date, SolarDate::new(2025, 2, 3));
        let next_day = cal.get_jieqi(2025, 2, 4).unwrap();
        assert_eq!(next_day.name, JieQiName::LapXuan);
        assert_eq!(next_day.solar_date, SolarDate::new(2025, 2, 3));
    }

    #[test]
    fn holidays_2025_contain_tet_and_hung_kings() {
        let cal = LunarCalendar::new();
        let holidays = cal.get_vietnam_holidays(2025).unwrap();
        assert!(holidays.iter().any(|h| h.name == HolidayName::TetNguyenDan));
        assert!(holidays.iter().any(|h| h.name == HolidayName::VuLan));
        assert!(holidays.iter().any(|h| {
            h.name == HolidayName::GioToHungVuong && h.solar_date == SolarDate::new(2025, 4, 7)
        }));
    }

    #[test]
    fn good_days_are_hoang_dao() {
        let cal = LunarCalendar::new();
        let days = cal.get_good_days(2025, 1).unwrap();
        assert!(!days.is_empty());
        assert!(days.iter().all(|d| d.category == GoodDayCategory::HoangDao));
    }

    #[test]
    fn invalid_leap_rejected() {
        let cal = LunarCalendar::new();
        assert!(cal.lunar_to_solar(2026, 4, 1, true).is_err());
    }
}
