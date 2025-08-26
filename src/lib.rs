use js_sys::{Array, Function, Reflect};
use serde::Serialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
struct ToggleMsg {
    action: &'static str,
    enabled: bool,
}

#[derive(Serialize)]
struct IntervalMsg {
    action: &'static str,
    min: u32,
    max: u32,
}

fn send_message(msg: &JsValue) {
    let global = js_sys::global();
    if let Ok(chrome) = Reflect::get(&global, &JsValue::from_str("chrome")) {
        if let Ok(runtime) = Reflect::get(&chrome, &JsValue::from_str("runtime")) {
            if let Ok(send_fn) = Reflect::get(&runtime, &JsValue::from_str("sendMessage")) {
                if let Ok(func) = send_fn.dyn_into::<Function>() {
                    let _ = func.apply(&runtime, &Array::of1(msg));
                }
            }
        }
    }
}

#[wasm_bindgen]
pub fn toggle(enabled: bool) {
    let msg = ToggleMsg {
        action: "toggle",
        enabled,
    };
    let js_msg = to_value(&msg).unwrap();
    send_message(&js_msg);
}

#[wasm_bindgen]
pub fn set_interval(min: u32, max: u32) {
    let msg = IntervalMsg {
        action: "set_interval",
        min,
        max,
    };
    let js_msg = to_value(&msg).unwrap();
    send_message(&js_msg);
}
