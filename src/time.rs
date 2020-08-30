use crate::model::UtcMillis;
use chrono::prelude::*;

pub fn js_utc_now() -> UtcMillis {
    UtcMillis(js_sys::Date::now() as u64)
}

/// computes a western-biased FixedOffset using the JS runtime
pub fn js_local_offset() -> FixedOffset {
    FixedOffset::west(js_local_offset_seconds())
}

const JS_CHRONO_OFFSET_COEFF: i32 = 60;
fn js_local_offset_seconds() -> i32 {
    js_sys::Date::new_0().get_timezone_offset() as i32 * JS_CHRONO_OFFSET_COEFF
}
