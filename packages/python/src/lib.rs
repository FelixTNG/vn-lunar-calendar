use pyo3::prelude::*;
use vn_lunar_core::{
    get_stem_branch, get_zodiac, EarthlyBranch, Element, GoodDay, GoodDayCategory, HeavenlyStem,
    Holiday, HolidayName, JieQi, JieQiName, LunarCalendar, LunarDate, SolarDate, StemBranch,
    TimeOfDay, YinYang, Zodiac,
};

#[pymodule]
fn vn_lunar(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PySolarDate>()?;
    m.add_class::<PyLunarDate>()?;
    m.add_class::<PyJieQi>()?;
    m.add_class::<PyTimeOfDay>()?;
    m.add_class::<PyHoliday>()?;
    m.add_class::<PyGoodDay>()?;
    m.add_class::<PyZodiac>()?;
    m.add_class::<PyStemBranch>()?;
    m.add_class::<PyHeavenlyStem>()?;
    m.add_class::<PyEarthlyBranch>()?;
    m.add_class::<PyElement>()?;
    m.add_class::<PyYinYang>()?;
    m.add_class::<PyLunarCalendar>()?;
    m.add_class::<PyJieQiName>()?;
    m.add_class::<PyHolidayName>()?;
    m.add_class::<PyGoodDayCategory>()?;
    m.add_function(wrap_pyfunction!(get_vietnam_holidays_py, m)?)?;
    m.add_function(wrap_pyfunction!(get_good_days_py, m)?)?;
    m.add_function(wrap_pyfunction!(get_jieqi_py, m)?)?;
    m.add_function(wrap_pyfunction!(get_zodiac_py, m)?)?;
    m.add_function(wrap_pyfunction!(get_stem_branch_py, m)?)?;
    Ok(())
}

fn lunar_error(err: vn_lunar_core::LunarError) -> PyErr {
    pyo3::exceptions::PyValueError::new_err(err.to_string())
}

#[pyclass(name = "SolarDate")]
#[derive(Clone)]
struct PySolarDate {
    #[pyo3(get)]
    year: i32,
    #[pyo3(get)]
    month: u8,
    #[pyo3(get)]
    day: u8,
}

#[pymethods]
impl PySolarDate {
    #[new]
    fn new(year: i32, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }

    fn __repr__(&self) -> String {
        format!("SolarDate({}-{:02}-{:02})", self.year, self.month, self.day)
    }
}

impl From<SolarDate> for PySolarDate {
    fn from(d: SolarDate) -> Self {
        Self {
            year: d.year,
            month: d.month,
            day: d.day,
        }
    }
}

#[pyclass(name = "LunarDate")]
#[derive(Clone)]
struct PyLunarDate {
    #[pyo3(get)]
    year: i32,
    #[pyo3(get)]
    month: u8,
    #[pyo3(get)]
    day: u8,
    #[pyo3(get)]
    leap: bool,
    #[pyo3(get)]
    zodiac: PyZodiac,
    #[pyo3(get)]
    stem_branch: PyStemBranch,
}

#[pymethods]
impl PyLunarDate {
    fn vietnamese_name(&self) -> String {
        LunarDate {
            year: self.year,
            month: self.month,
            day: self.day,
            leap: self.leap,
            zodiac: Zodiac::from_lunar_year(self.year),
            stem_branch: get_stem_branch(self.year),
        }
        .vietnamese_name()
    }

    fn is_tet(&self) -> bool {
        self.month == 1 && self.day <= 3 && !self.leap
    }

    fn __repr__(&self) -> String {
        format!(
            "LunarDate({}-{:02}-{:02}{})",
            self.year,
            self.month,
            self.day,
            if self.leap { "L" } else { "" }
        )
    }

    fn __str__(&self) -> String {
        self.vietnamese_name()
    }
}

impl From<LunarDate> for PyLunarDate {
    fn from(d: LunarDate) -> Self {
        Self {
            year: d.year,
            month: d.month,
            day: d.day,
            leap: d.leap,
            zodiac: PyZodiac::from(d.zodiac),
            stem_branch: PyStemBranch::from(d.stem_branch),
        }
    }
}

#[pyclass(name = "JieQi")]
#[derive(Clone)]
struct PyJieQi {
    #[pyo3(get)]
    name: PyJieQiName,
    #[pyo3(get)]
    solar_date: PySolarDate,
    #[pyo3(get)]
    exact_time: Option<PyTimeOfDay>,
}

impl From<JieQi> for PyJieQi {
    fn from(j: JieQi) -> Self {
        Self {
            name: PyJieQiName::from(j.name),
            solar_date: PySolarDate::from(j.solar_date),
            exact_time: j.exact_time.map(PyTimeOfDay::from),
        }
    }
}

#[pyclass(name = "TimeOfDay")]
#[derive(Clone)]
struct PyTimeOfDay {
    #[pyo3(get)]
    hour: u8,
    #[pyo3(get)]
    minute: u8,
    #[pyo3(get)]
    second: u8,
}

impl From<TimeOfDay> for PyTimeOfDay {
    fn from(t: TimeOfDay) -> Self {
        Self {
            hour: t.hour,
            minute: t.minute,
            second: t.second,
        }
    }
}

#[pyclass(name = "Holiday")]
#[derive(Clone)]
struct PyHoliday {
    #[pyo3(get)]
    name: PyHolidayName,
    #[pyo3(get)]
    solar_date: PySolarDate,
    #[pyo3(get)]
    lunar_date: PyLunarDate,
    #[pyo3(get)]
    is_public_holiday: bool,
}

impl From<Holiday> for PyHoliday {
    fn from(h: Holiday) -> Self {
        Self {
            name: PyHolidayName::from(h.name),
            solar_date: PySolarDate::from(h.solar_date),
            lunar_date: PyLunarDate::from(h.lunar_date),
            is_public_holiday: h.is_public_holiday,
        }
    }
}

#[pyclass(name = "GoodDay")]
#[derive(Clone)]
struct PyGoodDay {
    #[pyo3(get)]
    lunar_date: PyLunarDate,
    #[pyo3(get)]
    solar_date: PySolarDate,
    #[pyo3(get)]
    category: PyGoodDayCategory,
    #[pyo3(get)]
    description: String,
}

impl From<GoodDay> for PyGoodDay {
    fn from(g: GoodDay) -> Self {
        Self {
            lunar_date: PyLunarDate::from(g.lunar_date),
            solar_date: PySolarDate::from(g.solar_date),
            category: PyGoodDayCategory::from(g.category),
            description: g.description,
        }
    }
}

#[pyclass(name = "JieQiName", eq, eq_int)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum PyJieQiName {
    LapXuan = 0,
    VuThuy = 1,
    KinhTrich = 2,
    XuanPhan = 3,
    ThanhMinh = 4,
    CocVu = 5,
    LapHa = 6,
    TieuMan = 7,
    MangChung = 8,
    HaChi = 9,
    TieuThu = 10,
    DaiThu = 11,
    LapThu = 12,
    XySu = 13,
    BachLo = 14,
    ThuPhan = 15,
    HanLo = 16,
    SuongGiang = 17,
    LapDong = 18,
    TieuTuyet = 19,
    DaiTuyet = 20,
    DongChi = 21,
    TieuHan = 22,
    DaiHan = 23,
}

impl From<JieQiName> for PyJieQiName {
    fn from(n: JieQiName) -> Self {
        match n {
            JieQiName::LapXuan => Self::LapXuan,
            JieQiName::VuThuy => Self::VuThuy,
            JieQiName::KinhTrich => Self::KinhTrich,
            JieQiName::XuanPhan => Self::XuanPhan,
            JieQiName::ThanhMinh => Self::ThanhMinh,
            JieQiName::CocVu => Self::CocVu,
            JieQiName::LapHa => Self::LapHa,
            JieQiName::TieuMan => Self::TieuMan,
            JieQiName::MangChung => Self::MangChung,
            JieQiName::HaChi => Self::HaChi,
            JieQiName::TieuThu => Self::TieuThu,
            JieQiName::DaiThu => Self::DaiThu,
            JieQiName::LapThu => Self::LapThu,
            JieQiName::XySu => Self::XySu,
            JieQiName::BachLo => Self::BachLo,
            JieQiName::ThuPhan => Self::ThuPhan,
            JieQiName::HanLo => Self::HanLo,
            JieQiName::SuongGiang => Self::SuongGiang,
            JieQiName::LapDong => Self::LapDong,
            JieQiName::TieuTuyet => Self::TieuTuyet,
            JieQiName::DaiTuyet => Self::DaiTuyet,
            JieQiName::DongChi => Self::DongChi,
            JieQiName::TieuHan => Self::TieuHan,
            JieQiName::DaiHan => Self::DaiHan,
        }
    }
}

#[pyclass(name = "HolidayName", eq, eq_int)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum PyHolidayName {
    TetNguyenDan = 0,
    TetNguyenTieu = 1,
    GioToHungVuong = 2,
    LeNgayQuocTe = 3,
    LeGiaiPhong = 4,
    QuocKhanh = 5,
    VuLan = 6,
    TrungThu = 7,
    PhatDan = 8,
    Christmas = 9,
    NewYear = 10,
    Valentine = 11,
    PhuNuVietNam = 12,
    GiaoThua = 13,
}

impl From<HolidayName> for PyHolidayName {
    fn from(n: HolidayName) -> Self {
        match n {
            HolidayName::TetNguyenDan => Self::TetNguyenDan,
            HolidayName::TetNguyenTieu => Self::TetNguyenTieu,
            HolidayName::GioToHungVuong => Self::GioToHungVuong,
            HolidayName::LeNgayQuocTe => Self::LeNgayQuocTe,
            HolidayName::LeGiaiPhong => Self::LeGiaiPhong,
            HolidayName::QuocKhanh => Self::QuocKhanh,
            HolidayName::VuLan => Self::VuLan,
            HolidayName::TrungThu => Self::TrungThu,
            HolidayName::PhatDan => Self::PhatDan,
            HolidayName::Christmas => Self::Christmas,
            HolidayName::NewYear => Self::NewYear,
            HolidayName::Valentine => Self::Valentine,
            HolidayName::PhuNuVietNam => Self::PhuNuVietNam,
            HolidayName::GiaoThua => Self::GiaoThua,
        }
    }
}

#[pyclass(name = "GoodDayCategory", eq, eq_int)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum PyGoodDayCategory {
    HoangDao = 0,
    ThienDe = 1,
    NguyetDe = 2,
    SanhQi = 3,
    ThienEn = 4,
    PhucDuc = 5,
}

impl From<GoodDayCategory> for PyGoodDayCategory {
    fn from(c: GoodDayCategory) -> Self {
        match c {
            GoodDayCategory::HoangDao => Self::HoangDao,
            GoodDayCategory::ThienDe => Self::ThienDe,
            GoodDayCategory::NguyetDe => Self::NguyetDe,
            GoodDayCategory::SanhQi => Self::SanhQi,
            GoodDayCategory::ThienEn => Self::ThienEn,
            GoodDayCategory::PhucDuc => Self::PhucDuc,
        }
    }
}

#[pyclass(name = "Zodiac", eq, eq_int)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum PyZodiac {
    Rat = 0,
    Ox = 1,
    Tiger = 2,
    Rabbit = 3,
    Dragon = 4,
    Snake = 5,
    Horse = 6,
    Goat = 7,
    Monkey = 8,
    Rooster = 9,
    Dog = 10,
    Pig = 11,
}

impl From<Zodiac> for PyZodiac {
    fn from(z: Zodiac) -> Self {
        match z {
            Zodiac::Rat => Self::Rat,
            Zodiac::Ox => Self::Ox,
            Zodiac::Tiger => Self::Tiger,
            Zodiac::Rabbit => Self::Rabbit,
            Zodiac::Dragon => Self::Dragon,
            Zodiac::Snake => Self::Snake,
            Zodiac::Horse => Self::Horse,
            Zodiac::Goat => Self::Goat,
            Zodiac::Monkey => Self::Monkey,
            Zodiac::Rooster => Self::Rooster,
            Zodiac::Dog => Self::Dog,
            Zodiac::Pig => Self::Pig,
        }
    }
}

#[pymethods]
impl PyZodiac {
    fn vietnamese(&self) -> &'static str {
        Zodiac::from_index(*self as u8).vietnamese()
    }
}

#[pyclass(name = "StemBranch")]
#[derive(Clone)]
struct PyStemBranch {
    #[pyo3(get)]
    stem: PyHeavenlyStem,
    #[pyo3(get)]
    branch: PyEarthlyBranch,
}

#[pymethods]
impl PyStemBranch {
    fn vietnamese(&self) -> String {
        format!("{} {}", self.stem.vietnamese(), self.branch.vietnamese())
    }
}

impl From<StemBranch> for PyStemBranch {
    fn from(s: StemBranch) -> Self {
        Self {
            stem: PyHeavenlyStem::from(s.stem),
            branch: PyEarthlyBranch::from(s.branch),
        }
    }
}

#[pyclass(name = "HeavenlyStem", eq, eq_int)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum PyHeavenlyStem {
    Giap = 0,
    At = 1,
    Binh = 2,
    Dinh = 3,
    Mau = 4,
    Ky = 5,
    Canh = 6,
    Tan = 7,
    Nham = 8,
    Quy = 9,
}

impl From<HeavenlyStem> for PyHeavenlyStem {
    fn from(s: HeavenlyStem) -> Self {
        match s {
            HeavenlyStem::Giap => Self::Giap,
            HeavenlyStem::At => Self::At,
            HeavenlyStem::Binh => Self::Binh,
            HeavenlyStem::Dinh => Self::Dinh,
            HeavenlyStem::Mau => Self::Mau,
            HeavenlyStem::Ky => Self::Ky,
            HeavenlyStem::Canh => Self::Canh,
            HeavenlyStem::Tan => Self::Tan,
            HeavenlyStem::Nham => Self::Nham,
            HeavenlyStem::Quy => Self::Quy,
        }
    }
}

#[pymethods]
impl PyHeavenlyStem {
    fn vietnamese(&self) -> &'static str {
        HeavenlyStem::from_index(*self as u8).vietnamese()
    }
}

#[pyclass(name = "EarthlyBranch", eq, eq_int)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum PyEarthlyBranch {
    Ty = 0,
    Suu = 1,
    Dan = 2,
    Mao = 3,
    Thin = 4,
    Ty2 = 5,
    Ngo = 6,
    Mui = 7,
    Than = 8,
    Dau = 9,
    Tuat = 10,
    Hoi = 11,
}

impl From<EarthlyBranch> for PyEarthlyBranch {
    fn from(b: EarthlyBranch) -> Self {
        match b {
            EarthlyBranch::Ty => Self::Ty,
            EarthlyBranch::Suu => Self::Suu,
            EarthlyBranch::Dan => Self::Dan,
            EarthlyBranch::Mao => Self::Mao,
            EarthlyBranch::Thin => Self::Thin,
            EarthlyBranch::Ty2 => Self::Ty2,
            EarthlyBranch::Ngo => Self::Ngo,
            EarthlyBranch::Mui => Self::Mui,
            EarthlyBranch::Than => Self::Than,
            EarthlyBranch::Dau => Self::Dau,
            EarthlyBranch::Tuat => Self::Tuat,
            EarthlyBranch::Hoi => Self::Hoi,
        }
    }
}

#[pymethods]
impl PyEarthlyBranch {
    fn vietnamese(&self) -> &'static str {
        EarthlyBranch::from_index(*self as u8).vietnamese()
    }
}

#[pyclass(name = "Element", eq, eq_int)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum PyElement {
    Wood = 0,
    Fire = 1,
    Earth = 2,
    Metal = 3,
    Water = 4,
}

impl From<Element> for PyElement {
    fn from(e: Element) -> Self {
        match e {
            Element::Wood => Self::Wood,
            Element::Fire => Self::Fire,
            Element::Earth => Self::Earth,
            Element::Metal => Self::Metal,
            Element::Water => Self::Water,
        }
    }
}

#[pyclass(name = "YinYang", eq, eq_int)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum PyYinYang {
    Yang = 0,
    Yin = 1,
}

impl From<YinYang> for PyYinYang {
    fn from(y: YinYang) -> Self {
        match y {
            YinYang::Yang => Self::Yang,
            YinYang::Yin => Self::Yin,
        }
    }
}

#[pyclass(name = "LunarCalendar")]
struct PyLunarCalendar {
    inner: LunarCalendar,
}

#[pymethods]
impl PyLunarCalendar {
    #[new]
    fn new() -> Self {
        Self {
            inner: LunarCalendar::new(),
        }
    }

    fn solar_to_lunar(&self, year: i32, month: u8, day: u8) -> PyResult<PyLunarDate> {
        self.inner
            .solar_to_lunar(year, month, day)
            .map(PyLunarDate::from)
            .map_err(lunar_error)
    }

    fn lunar_to_solar(&self, year: i32, month: u8, day: u8, leap: bool) -> PyResult<PySolarDate> {
        self.inner
            .lunar_to_solar(year, month, day, leap)
            .map(PySolarDate::from)
            .map_err(lunar_error)
    }

    fn leap_month(&self, lunar_year: i32) -> Option<u8> {
        self.inner.leap_month(lunar_year)
    }

    fn get_jieqi(&self, year: i32, month: u8, day: u8) -> PyResult<PyJieQi> {
        self.inner
            .get_jieqi(year, month, day)
            .map(PyJieQi::from)
            .map_err(lunar_error)
    }

    fn get_vietnam_holidays(&self, year: i32) -> PyResult<Vec<PyHoliday>> {
        self.inner
            .get_vietnam_holidays(year)
            .map(|v| v.into_iter().map(PyHoliday::from).collect())
            .map_err(lunar_error)
    }

    fn get_good_days(&self, year: i32, month: u8) -> PyResult<Vec<PyGoodDay>> {
        self.inner
            .get_good_days(year, month)
            .map(|v| v.into_iter().map(PyGoodDay::from).collect())
            .map_err(lunar_error)
    }
}

#[pyfunction]
fn get_vietnam_holidays_py(year: i32) -> PyResult<Vec<PyHoliday>> {
    LunarCalendar::new()
        .get_vietnam_holidays(year)
        .map(|v| v.into_iter().map(PyHoliday::from).collect())
        .map_err(lunar_error)
}

#[pyfunction]
fn get_good_days_py(year: i32, month: u8) -> PyResult<Vec<PyGoodDay>> {
    LunarCalendar::new()
        .get_good_days(year, month)
        .map(|v| v.into_iter().map(PyGoodDay::from).collect())
        .map_err(lunar_error)
}

#[pyfunction]
fn get_jieqi_py(year: i32, month: u8, day: u8) -> PyResult<PyJieQi> {
    LunarCalendar::new()
        .get_jieqi(year, month, day)
        .map(PyJieQi::from)
        .map_err(lunar_error)
}

#[pyfunction]
fn get_zodiac_py(year: i32) -> PyZodiac {
    PyZodiac::from(get_zodiac(year))
}

#[pyfunction]
fn get_stem_branch_py(year: i32) -> PyStemBranch {
    PyStemBranch::from(get_stem_branch(year))
}
