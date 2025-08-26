use gloo_timers::future::TimeoutFuture;
use js_sys::{Array, Function, Math, Reflect};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

// global partagé entre rust et js
static ENABLED: AtomicBool = AtomicBool::new(true);
static MIN_MINUTES: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(1));
static MAX_MINUTES: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(5));

#[wasm_bindgen(start)]
pub fn start() {
    spawn_local(async move {
        loop {
            if ENABLED.load(Ordering::SeqCst) {
                // on récupère le min et max en minutes
                let min = *MIN_MINUTES.lock().unwrap();
                let max = *MAX_MINUTES.lock().unwrap();

                // convertion en millisecondes
                let min_ms = min * 60_000;
                let max_ms = max * 60_000;

                // choix de minute aléatoire
                let random_interval =
                    (Math::random() * (max_ms - min_ms) as f64 + min_ms as f64) as u32;

                // attendre un peu
                TimeoutFuture::new(random_interval).await;

                // communication avec background.js
                let global = js_sys::global();
                if let Ok(chrome) = Reflect::get(&global, &JsValue::from_str("chrome")) {
                    if let Ok(runtime) = Reflect::get(&chrome, &JsValue::from_str("runtime")) {
                        if let Ok(send_fn) =
                            Reflect::get(&runtime, &JsValue::from_str("sendMessage"))
                        {
                            if let Ok(func) = send_fn.dyn_into::<Function>() {
                                let msg = JsValue::from_str(r#"{"action":"reload_tab"}"#);
                                let _ = func.apply(&runtime, &Array::of1(&msg));
                            }
                        }
                    }
                }
            } else {
                // si désactivé, on attend 1 seconde avant de revérifier
                TimeoutFuture::new(1000).await;
            }
        }
    });
}

#[wasm_bindgen]
pub fn set_interval(min: u32, max: u32) {
    if min > 0 && max >= min {
        *MIN_MINUTES.lock().unwrap() = min;
        *MAX_MINUTES.lock().unwrap() = max;
    }
}

#[wasm_bindgen]
pub fn toggle(enabled: bool) {
    ENABLED.store(enabled, Ordering::SeqCst);
}
