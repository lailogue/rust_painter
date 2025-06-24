use eframe::egui;
use crate::tools::ToolSettings;

pub fn render_sidebar(ui: &mut egui::Ui, tools: &mut ToolSettings) {
    ui.heading("レイヤー");
    ui.separator();
    
    // レイヤー機能の実装は省略
    ui.label("レイヤー1");
    
    ui.separator();
    ui.heading("背景色");
    ui.color_edit_button_srgba(&mut tools.background_color);
}