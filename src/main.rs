

#[cfg(target_os="macos")]
fn main() -> eframe::Result<()> {
    use msg_extractor::ExtractorApp;
    //env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([330.0, 280.0])
            .with_min_inner_size([300.0, 220.0]),
            
        ..Default::default()
    };
    eframe::run_native(
        "msg-extracor by rakurtz",
        native_options,
        Box::new(|cc| Box::new(ExtractorApp::new(cc))),
    )
}

#[cfg(target_os="windows")]
fn main() {
    extern crate native_windows_gui as nwg;
    extern crate native_windows_derive as nwd;

    use nwd::NwgUi;
    use nwg::NativeUi;
    use std::cell::RefCell;

    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = msg_extractor::NwmApp::build_ui(Default::default()).expect("Failed to build UI");
    
    nwg::dispatch_thread_events();

}