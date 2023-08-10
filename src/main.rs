// #![warn(clippy::all, rust_2018_idioms)]
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    // 注册到 `stderr`
    env_logger::init();

    // 原生 gui 设置
    let native_options = eframe::NativeOptions {
        ..Default::default()
    };

    // 运行 egui
    return eframe::run_native(
        "gallery",
        native_options,
        Box::new(|cc| Box::new(gallery::TemplateApp::new(cc))),
    );
}
