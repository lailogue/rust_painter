use eframe::egui;
use crate::tools::ToolSettings;
use crate::ui::canvas::CanvasHandler;
use crate::ui::{render_toolbar, render_sidebar};
use crate::font::setup_fonts;

pub struct PaintApp {
    tools: ToolSettings,
    canvas: CanvasHandler,
}

impl Default for PaintApp {
    fn default() -> Self {
        Self {
            tools: ToolSettings::default(),
            canvas: CanvasHandler::default(),
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
        egui::SidePanel::left("layers").show(ctx, |ui| {
            render_sidebar(ui, &mut self.tools);
        });
        
        // メインキャンバス
        egui::CentralPanel::default().show(ctx, |ui| {
            self.canvas.render(ui, &self.tools);
        });
    }
}