//! C ABI so other languages can call the same conversions without Rust.

#![allow(unsafe_code)]

use crate::{LunarCalendar, LunarError, MAX_YEAR, MIN_YEAR};

/// Gregorian date for C callers.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct VnSolarDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub _pad: u8,
}

/// Lunar date for C callers (`leap` is 0 or 1).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct VnLunarDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub leap: u8,
}

fn write_err(out_err: *mut i32, code: i32) {
    if !out_err.is_null() {
        // Safety: caller-provided out pointer, documented as optional.
        unsafe { *out_err = code };
    }
}

/// Convert Gregorian (Vietnam civil) to lunar.
///
/// Returns 0 on success. `out` must be non-null. `out_err` may be null.
#[no_mangle]
pub extern "C" fn vn_lunar_solar_to_lunar(
    year: i32,
    month: u8,
    day: u8,
    out: *mut VnLunarDate,
    out_err: *mut i32,
) -> i32 {
    if out.is_null() {
        write_err(out_err, 8);
        return 8;
    }
    match LunarCalendar::new().solar_to_lunar(year, month, day) {
        Ok(d) => {
            unsafe {
                *out = VnLunarDate {
                    year: d.year,
                    month: d.month,
                    day: d.day,
                    leap: u8::from(d.leap),
                };
            }
            write_err(out_err, 0);
            0
        }
        Err(e) => {
            let code = e.code();
            write_err(out_err, code);
            code
        }
    }
}

/// Convert lunar to Gregorian (Vietnam civil).
///
/// `leap` is non-zero for a leap month. Returns 0 on success.
#[no_mangle]
pub extern "C" fn vn_lunar_lunar_to_solar(
    year: i32,
    month: u8,
    day: u8,
    leap: u8,
    out: *mut VnSolarDate,
    out_err: *mut i32,
) -> i32 {
    if out.is_null() {
        write_err(out_err, 8);
        return 8;
    }
    match LunarCalendar::new().lunar_to_solar(year, month, day, leap != 0) {
        Ok(d) => {
            unsafe {
                *out = VnSolarDate {
                    year: d.year,
                    month: d.month,
                    day: d.day,
                    _pad: 0,
                };
            }
            write_err(out_err, 0);
            0
        }
        Err(e) => {
            let code = e.code();
            write_err(out_err, code);
            code
        }
    }
}

/// Leap month of `lunar_year`, or 0 if none. Negative values are error codes.
#[no_mangle]
pub extern "C" fn vn_lunar_leap_month(lunar_year: i32) -> i32 {
    if lunar_year < MIN_YEAR || lunar_year > MAX_YEAR {
        return LunarError::YearOutOfRange(lunar_year).code();
    }
    i32::from(LunarCalendar::new().leap_month(lunar_year).unwrap_or(0))
}

/// Library version string (UTF-8, NUL-terminated, static).
#[no_mangle]
pub extern "C" fn vn_lunar_version() -> *const core::ffi::c_char {
    static VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "\0");
    VERSION.as_ptr().cast()
}
