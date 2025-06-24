use eframe::egui;
use crate::tools::ToolSettings;
use crate::ui::canvas::CanvasHandler;
use crate::ui::{render_toolbar, render_sidebar, render_layer_panel, LayerRenameState};
use crate::font::setup_fonts;
use crate::layer::LayerManager;

pub struct PaintApp {
    tools: ToolSettings,
    canvas: CanvasHandler,
    layer_manager: LayerManager,
    layer_rename_state: LayerRenameState,
}

impl Default for PaintApp {
    fn default() -> Self {
        Self {
            tools: ToolSettings::default(),
            canvas: CanvasHandler::default(),
            layer_manager: LayerManager::default(),
            layer_rename_state: LayerRenameState::default(),
        }
    }
}

impl PaintApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // フォント設定（日本語対応）
        setup_fonts(&cc.egui_ctx);
        
        Self::default()
    }
    
    fn clear_canvas(&mut self) {
        self.canvas.clear();
        self.layer_manager.clear_active_layer();
    }
}

impl eframe::App for PaintApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // トップパネル - ツールバー
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            if render_toolbar(ui, &mut self.tools) {
                self.clear_canvas();
            }
        });
        
        // サイドパネル - レイヤーやその他のオプション
        egui::SidePanel::left("layers")
            .resizable(true)
            .default_width(250.0)
            .width_range(200.0..=400.0)
            .show(ctx, |ui| {
                // レイヤーパネル
                let layer_action = render_layer_panel(ui, &mut self.layer_manager, &mut self.layer_rename_state);
                self.layer_manager.handle_layer_action(layer_action);
                
                ui.separator();
                
                // ツール設定
                render_sidebar(ui, &mut self.tools);
            });
        
        // メインキャンバス
        egui::CentralPanel::default().show(ctx, |ui| {
            let _stroke_completed = self.canvas.render(ui, &self.tools, &mut self.layer_manager);
        });
    }
}