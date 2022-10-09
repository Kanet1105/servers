mod features;
mod pages;

pub mod prelude {}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct WebApp;

impl WebApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl Default for WebApp {
    fn default() -> Self {
        Self
    }
}

impl eframe::App for WebApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        
    }
}

#[cfg(target_arch = "wasm32")]
pub fn run() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();
    eframe::start_web(
        "main_canvas", // hardcode it
        web_options,
        Box::new(|cc| Box::new(WebApp::new(cc))),
    )
    .expect("failed to start eframe");
}
