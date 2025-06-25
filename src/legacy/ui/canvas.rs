use eframe::egui::{self, Sense};
use crate::stroke::Stroke;
use crate::tools::ToolSettings;
use crate::layer::LayerManager;

pub struct CanvasHandler {
    pub current_stroke: Option<Stroke>,
}

impl CanvasHandler {
    pub fn new() -> Self {
        Self {
            current_stroke: None,
        }
    }
    
    pub fn clear(&mut self) {
        self.current_stroke = None;
    }
    
    pub fn render(&mut self, ui: &mut egui::Ui, tools: &ToolSettings, layer_manager: &mut LayerManager) -> bool {
        let available_size = ui.available_size();
        
        // キャンバスの描画エリア
        let (response, mut painter) = ui.allocate_painter(
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
        let mut stroke_completed = false;
        if response.drag_stopped() {
            if let Some(stroke) = self.current_stroke.take() {
                if stroke.len() > 1 {
                    if let Some(active_layer) = layer_manager.get_active_layer_mut() {
                        active_layer.add_stroke(stroke);
                        stroke_completed = true;
                    }
                }
            }
        }
        
        // すべてのレイヤーのストロークを描画
        let offset = response.rect.min.to_vec2();
        
        // レイヤーを正しい順序で描画（下から上へ）
        for (_, layer) in layer_manager.get_layers_for_rendering() {
            if (layer.opacity - 1.0).abs() < 0.001 {
                // 完全不透明レイヤー：通常描画
                for stroke in &layer.strokes {
                    stroke.draw(&painter, offset);
                }
            } else {
                // 透明度があるレイヤー：Painterの透明度機能を使用
                // eGUIの制限により、この実装は近似的なものです
                let original_opacity = painter.opacity();
                painter.set_opacity(original_opacity * layer.opacity);
                
                for stroke in &layer.strokes {
                    stroke.draw(&painter, offset);
                }
                
                // 透明度を元に戻す
                painter.set_opacity(original_opacity);
            }
        }
        
        // 現在描画中のストロークを描画
        if let Some(ref stroke) = self.current_stroke {
            stroke.draw(&painter, offset);
        }
        
        // ステータスバー
        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            ui.label(format!(
                "キャンバスサイズ: {:.0}x{:.0} | 総ストローク数: {} | アクティブレイヤー: {}",
                available_size.x,
                available_size.y,
                layer_manager.total_stroke_count(),
                layer_manager.get_active_layer().map_or("なし".to_string(), |l| l.name.clone())
            ));
        });
        
        stroke_completed
    }
}

impl Default for CanvasHandler {
    fn default() -> Self {
        Self::new()
    }
}