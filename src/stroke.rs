use eframe::egui::{self, Color32, Pos2};

#[derive(Clone)]
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
    
    /// 2点間を補間して滑らかな円で描画する
    pub fn draw_smooth_stroke(painter: &egui::Painter, p1: Pos2, p2: Pos2, width: f32, color: Color32) {
        let distance = (p2 - p1).length();
        let step_size = (width / 4.0).max(1.0); // ブラシサイズに応じたステップサイズ
        let steps = (distance / step_size).ceil() as usize;
        
        if steps <= 1 {
            painter.circle_filled(p2, width / 2.0, color);
        } else {
            for i in 0..=steps {
                let t = i as f32 / steps as f32;
                let interpolated_pos = p1 + t * (p2 - p1);
                painter.circle_filled(interpolated_pos, width / 2.0, color);
            }
        }
    }
    
    /// ストロークを描画する
    pub fn draw(&self, painter: &egui::Painter, offset: egui::Vec2) {
        if self.points.len() == 1 {
            // 単一点の場合
            painter.circle_filled(
                self.points[0] + offset,
                self.width / 2.0,
                self.color,
            );
        } else if self.points.len() > 1 {
            // 最初の点を描画
            painter.circle_filled(
                self.points[0] + offset,
                self.width / 2.0,
                self.color,
            );
            // 連続する点間を補間描画
            for i in 1..self.points.len() {
                Self::draw_smooth_stroke(
                    painter,
                    self.points[i - 1] + offset,
                    self.points[i] + offset,
                    self.width,
                    self.color,
                );
            }
        }
    }
}