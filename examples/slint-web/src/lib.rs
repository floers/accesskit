
#[cfg(not(feature = "server"))]
slint::include_modules!();

#[cfg(not(feature = "server"))]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn run() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Started");
    let window = MainWindow::new().unwrap();
    window.run().unwrap();
}
