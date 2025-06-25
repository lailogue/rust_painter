use eframe::egui::{self, Color32, Pos2};

#[derive(Clone, Debug)]
pub struct Stroke {
    pub points: Vec<Pos2>,
    pub color: Color32,
    pub width: f32,
}

impl Stroke {
    pub fn new(color: Color32, width: f32) -> Self {
        Self {
            points: Vec::new(),
            color,
            width,
        }
    }
    
    pub fn add_point(&mut self, point: Pos2) {
        self.points.push(point);
    }
    
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }
    
    pub fn len(&self) -> usize {
        self.points.len()
    }
    
    
    /// ストロークを描画する（レイヤー透明度適用版）
    pub fn draw_with_layer_opacity(&self, painter: &egui::Painter, offset: egui::Vec2, layer_opacity: f32) {
        let final_color = self.apply_layer_opacity(self.color, layer_opacity);
        self.draw_internal(painter, offset, final_color);
    }
    
    /// ストロークを描画する
    pub fn draw(&self, painter: &egui::Painter, offset: egui::Vec2) {
        self.draw_internal(painter, offset, self.color);
    }
    
    /// 内部描画メソッド（円形補間で滑らかな描画）
    fn draw_internal(&self, painter: &egui::Painter, offset: egui::Vec2, color: Color32) {
        if self.points.len() == 1 {
            // 単一点の場合：円を描画
            painter.circle_filled(
                self.points[0] + offset,
                self.width / 2.0,
                color,
            );
        } else if self.points.len() > 1 {
            // 複数点の場合：円形ブラシで滑らかに描画
            // 各点に円を描画
            for point in &self.points {
                painter.circle_filled(*point + offset, self.width / 2.0, color);
            }
            
            // 点間を補間して滑らかにする
            for i in 1..self.points.len() {
                let p1 = self.points[i - 1] + offset;
                let p2 = self.points[i] + offset;
                
                let distance = (p2 - p1).length();
                let step_size = (self.width / 3.0).max(1.0);
                let steps = (distance / step_size).ceil() as usize;
                
                for j in 1..steps {
                    let t = j as f32 / steps as f32;
                    let interpolated_pos = p1 + t * (p2 - p1);
                    painter.circle_filled(interpolated_pos, self.width / 2.0, color);
                }
            }
        }
    }
    
    /// レイヤー透明度を色に適用する
    fn apply_layer_opacity(&self, color: Color32, layer_opacity: f32) -> Color32 {
        let [r, g, b, a] = color.to_array();
        // レイヤー透明度を適用（0.0-1.0の範囲でクランプ）
        let clamped_opacity = layer_opacity.clamp(0.0, 1.0);
        let final_alpha = ((a as f32 / 255.0) * clamped_opacity * 255.0) as u8;
        Color32::from_rgba_unmultiplied(r, g, b, final_alpha)
    }
}