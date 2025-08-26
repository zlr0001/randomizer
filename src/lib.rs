use gloo_timers::future::TimeoutFuture;
use js_sys::{Array, Function, Math, Reflect};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

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
            }
        }
    });
}
