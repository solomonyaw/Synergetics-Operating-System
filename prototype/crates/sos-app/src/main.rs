use eframe::NativeOptions;

use sos_app::app::SynergeticsApp;

fn main() -> eframe::Result<()> {

    tracing_subscriber::fmt::init();

    let options = NativeOptions {

        viewport: egui::ViewportBuilder::default()

            .with_title("Synergetics Operating System")

            .with_inner_size([1600.0, 900.0])

            .with_resizable(true)

            .with_maximized(true),

        ..Default::default()
    };

    eframe::run_native(
        "Synergetics OS",
        options,
        Box::new(|cc| {

            Ok(Box::new(SynergeticsApp::new(cc)))

        }),
    )
}