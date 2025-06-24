use eframe::egui::{self, Sense};
use crate::stroke::Stroke;
use crate::tools::ToolSettings;

pub struct CanvasHandler {
    pub strokes: Vec<Stroke>,
    pub current_stroke: Option<Stroke>,
}

impl CanvasHandler {
    pub fn new() -> Self {
        Self {
            strokes: Vec::new(),
            current_stroke: None,
        }
    }
    
    pub fn clear(&mut self) {
        self.strokes.clear();
        self.current_stroke = None;
    }
    
    pub fn render(&mut self, ui: &mut egui::Ui, tools: &ToolSettings) {
        let available_size = ui.available_size();
        
        // キャンバスの描画エリア
        let (response, painter) = ui.allocate_painter(
            available_size,
            Sense::drag(),
        );
        
        // 背景を描画
        painter.rect_filled(
            response.rect,
            0.0,
            tools.background_color,
        );
        
        // マウス入力の処理
        if response.dragged() {
            if let Some(pos) = response.interact_pointer_pos() {
                let canvas_pos = (pos - response.rect.min).to_pos2();
                
                if self.current_stroke.is_none() {
                    self.current_stroke = Some(Stroke::new(
                        tools.get_current_color(),
                        tools.brush_size,
                    ));
                }
                
                if let Some(ref mut stroke) = self.current_stroke {
                    stroke.add_point(canvas_pos);
                }
            }
        }
        
        // ドラッグが終了したらストロークを確定
        if response.drag_stopped() {
            if let Some(stroke) = self.current_stroke.take() {
                if stroke.len() > 1 {
                    self.strokes.push(stroke);
                }
            }
        }
        
        // すべてのストロークを描画
        let offset = response.rect.min.to_vec2();
        
        for stroke in &self.strokes {
            stroke.draw(&painter, offset);
        }
        
        // 現在描画中のストロークを描画
        if let Some(ref stroke) = self.current_stroke {
            stroke.draw(&painter, offset);
        }
        
        // ステータスバー
        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            ui.label(format!(
                "キャンバスサイズ: {:.0}x{:.0} | ストローク数: {}",
                available_size.x,
                available_size.y,
                self.strokes.len()
            ));
        });
    }
}

impl Default for CanvasHandler {
    fn default() -> Self {
        Self::new()
    }
}