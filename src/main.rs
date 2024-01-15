use msg_extractor::ExtractorApp;


fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

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