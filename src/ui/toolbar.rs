use eframe::egui;
use crate::tools::{Tool, ToolSettings};

pub fn render_toolbar(ui: &mut egui::Ui, tools: &mut ToolSettings) -> bool {
    let response = ui.horizontal(|ui| {
        ui.label("ブラシサイズ:");
        ui.add(egui::Slider::new(&mut tools.brush_size, 1.0..=200.0)
            .show_value(true));
        
        ui.separator();
        
        // カラーピッカー
        ui.label("色:");
        ui.color_edit_button_srgba(&mut tools.brush_color);
        
        ui.separator();
        
        // ブラシ透明度（ペンツールのみ）
        if tools.current_tool == Tool::Pen {
            ui.label("透明度:");
            let mut opacity_percent = tools.brush_opacity * 100.0;
            if ui.add(egui::Slider::new(&mut opacity_percent, 0.0..=100.0)
                .suffix("%")
                .show_value(true)).changed() {
                tools.set_brush_opacity(opacity_percent / 100.0);
            }
        }
        
        ui.separator();
        
        // ツール選択
        ui.selectable_value(&mut tools.current_tool, Tool::Pen, "ペン");
        ui.selectable_value(&mut tools.current_tool, Tool::Eraser, "消しゴム");
        
        ui.separator();
        
        let clear_clicked = ui.button("クリア").clicked();
        
        ui.separator();
        
        // 保存/読み込みボタン（実装は省略）
        if ui.button("保存").clicked() {
            // TODO: 画像として保存
        }
        
        if ui.button("開く").clicked() {
            // TODO: 画像を開く
        }
        
        clear_clicked
    });
    
    response.inner
}