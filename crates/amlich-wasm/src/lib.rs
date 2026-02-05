// amlich-wasm - WASM bindings for web usage
//
// TODO: Implement in Phase 4

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_day_info(_day: u32, _month: u32, _year: i32) -> JsValue {
    // TODO: Phase 4
    JsValue::from_str("{\"todo\": \"Phase 4\"}")
}

#[wasm_bindgen]
pub fn convert_solar_to_lunar(_day: u32, _month: u32, _year: i32) -> JsValue {
    // TODO: Phase 4
    JsValue::from_str("{\"todo\": \"Phase 4\"}")
}

#[wasm_bindgen]
pub fn convert_lunar_to_solar(_day: u32, _month: u32, _year: i32, _is_leap: bool) -> JsValue {
    // TODO: Phase 4
    JsValue::from_str("{\"todo\": \"Phase 4\"}")
}
