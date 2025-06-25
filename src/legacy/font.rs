use eframe::egui;
use std::sync::Arc;

pub fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "noto_sans_jp".to_owned(),
        Arc::new(egui::FontData::from_static(include_bytes!("../fonts/NotoSansJP-Regular.ttf"))),
    );
    
    fonts.families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "noto_sans_jp".to_owned());
    
    ctx.set_fonts(fonts);
}