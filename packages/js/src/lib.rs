use vn_lunar_core::{
    get_stem_branch as core_stem_branch, get_zodiac as core_zodiac, LunarCalendar, LunarDate,
    SolarDate,
};
use wasm_bindgen::prelude::*;

fn js_err(e: vn_lunar_core::LunarError) -> JsValue {
    JsValue::from_str(&e.to_string())
}

#[wasm_bindgen(js_name = SolarDate)]
pub struct JsSolarDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

impl From<SolarDate> for JsSolarDate {
    fn from(d: SolarDate) -> Self {
        Self {
            year: d.year,
            month: d.month,
            day: d.day,
        }
    }
}

#[wasm_bindgen(js_name = LunarDate)]
pub struct JsLunarDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub leap: bool,
}

impl From<LunarDate> for JsLunarDate {
    fn from(d: LunarDate) -> Self {
        Self {
            year: d.year,
            month: d.month,
            day: d.day,
            leap: d.leap,
        }
    }
}

#[wasm_bindgen(js_class = LunarDate)]
impl JsLunarDate {
    #[wasm_bindgen(getter, js_name = vietnameseName)]
    pub fn vietnamese_name(&self) -> String {
        LunarDate {
            year: self.year,
            month: self.month,
            day: self.day,
            leap: self.leap,
            zodiac: core_zodiac(self.year),
            stem_branch: core_stem_branch(self.year),
        }
        .vietnamese_name()
    }

    #[wasm_bindgen(getter)]
    pub fn zodiac(&self) -> String {
        core_zodiac(self.year).vietnamese().to_string()
    }

    #[wasm_bindgen(getter, js_name = stemBranch)]
    pub fn stem_branch(&self) -> String {
        core_stem_branch(self.year).vietnamese()
    }

    #[wasm_bindgen(getter, js_name = isTet)]
    pub fn is_tet(&self) -> bool {
        self.month == 1 && self.day <= 3 && !self.leap
    }
}

#[wasm_bindgen(js_name = LunarCalendar)]
pub struct JsLunarCalendar {
    inner: LunarCalendar,
}

#[wasm_bindgen(js_class = LunarCalendar)]
impl JsLunarCalendar {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: LunarCalendar::new(),
        }
    }

    #[wasm_bindgen(js_name = solarToLunar)]
    pub fn solar_to_lunar(&self, year: i32, month: u8, day: u8) -> Result<JsLunarDate, JsValue> {
        self.inner
            .solar_to_lunar(year, month, day)
            .map(Into::into)
            .map_err(js_err)
    }

    #[wasm_bindgen(js_name = lunarToSolar)]
    pub fn lunar_to_solar(
        &self,
        year: i32,
        month: u8,
        day: u8,
        leap: bool,
    ) -> Result<JsSolarDate, JsValue> {
        self.inner
            .lunar_to_solar(year, month, day, leap)
            .map(Into::into)
            .map_err(js_err)
    }

    #[wasm_bindgen(js_name = leapMonth)]
    pub fn leap_month(&self, lunar_year: i32) -> Option<u8> {
        self.inner.leap_month(lunar_year)
    }

    #[wasm_bindgen(js_name = getJieqi)]
    pub fn get_jieqi(&self, year: i32, month: u8, day: u8) -> Result<JsValue, JsValue> {
        let j = self.inner.get_jieqi(year, month, day).map_err(js_err)?;
        let obj = js_sys::Object::new();
        js_sys::Reflect::set(&obj, &"name".into(), &j.name.vietnamese().into())?;
        let date = js_sys::Object::new();
        js_sys::Reflect::set(&date, &"year".into(), &j.solar_date.year.into())?;
        js_sys::Reflect::set(&date, &"month".into(), &j.solar_date.month.into())?;
        js_sys::Reflect::set(&date, &"day".into(), &j.solar_date.day.into())?;
        js_sys::Reflect::set(&obj, &"solarDate".into(), &date)?;
        Ok(obj.into())
    }

    #[wasm_bindgen(js_name = getVietnamHolidays)]
    pub fn get_vietnam_holidays(&self, year: i32) -> Result<JsValue, JsValue> {
        let holidays = self.inner.get_vietnam_holidays(year).map_err(js_err)?;
        let arr = js_sys::Array::new();
        for h in holidays {
            let obj = js_sys::Object::new();
            js_sys::Reflect::set(&obj, &"name".into(), &h.name.vietnamese().into())?;
            js_sys::Reflect::set(&obj, &"year".into(), &h.solar_date.year.into())?;
            js_sys::Reflect::set(&obj, &"month".into(), &h.solar_date.month.into())?;
            js_sys::Reflect::set(&obj, &"day".into(), &h.solar_date.day.into())?;
            js_sys::Reflect::set(
                &obj,
                &"isPublicHoliday".into(),
                &h.is_public_holiday.into(),
            )?;
            arr.push(&obj);
        }
        Ok(arr.into())
    }

    #[wasm_bindgen(js_name = getGoodDays)]
    pub fn get_good_days(&self, lunar_year: i32, lunar_month: u8) -> Result<JsValue, JsValue> {
        let days = self
            .inner
            .get_good_days(lunar_year, lunar_month)
            .map_err(js_err)?;
        let arr = js_sys::Array::new();
        for d in days {
            let obj = js_sys::Object::new();
            js_sys::Reflect::set(&obj, &"year".into(), &d.solar_date.year.into())?;
            js_sys::Reflect::set(&obj, &"month".into(), &d.solar_date.month.into())?;
            js_sys::Reflect::set(&obj, &"day".into(), &d.solar_date.day.into())?;
            js_sys::Reflect::set(&obj, &"category".into(), &d.category.vietnamese().into())?;
            js_sys::Reflect::set(&obj, &"description".into(), &d.description.into())?;
            arr.push(&obj);
        }
        Ok(arr.into())
    }

    #[wasm_bindgen(js_name = getZodiac)]
    pub fn get_zodiac(&self, lunar_year: i32) -> String {
        self.inner.get_zodiac(lunar_year).vietnamese().to_string()
    }

    #[wasm_bindgen(js_name = getStemBranch)]
    pub fn get_stem_branch(&self, lunar_year: i32) -> String {
        self.inner.get_stem_branch(lunar_year).vietnamese()
    }
}

impl Default for JsLunarCalendar {
    fn default() -> Self {
        Self::new()
    }
}
