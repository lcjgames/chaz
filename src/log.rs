use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[cfg(not(target_arch = "wasm32"))]
fn log(s: &str) {
    println!(s);
}

macro_rules! console_log {
    ($($t:tt)*) => (crate::log::log(&format_args!($($t)*).to_string()))
}

pub(crate) use console_log;
