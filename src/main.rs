mod app;
mod stroke;
mod tools;
mod font;
mod ui;

use eframe::egui;
use app::PaintApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "ペイントアプリ",
        options,
        Box::new(|cc| Ok(Box::new(PaintApp::new(cc)))),
    )
}