use tiny_skia::{Pixmap, Paint, Path, PathBuilder, Point, Stroke, Color as SkiaColor};
use iced::Color;
use crate::tools::ToolSettings;
use crate::layer_system::LayerManager;

#[derive(Debug, Clone)]
pub struct PaintStroke {
    pub points: Vec<Point>,
    pub color: iced::Color,
    pub stroke_width: f32,
}

impl PaintStroke {
    pub fn new(color: Color, width: f32) -> Self {
        Self {
            points: Vec::new(),
            color,
            stroke_width: width,
        }
    }
    
    pub fn add_point(&mut self, x: f32, y: f32) {
        self.points.push(Point::from_xy(x, y));
    }
    
    pub fn draw_to_pixmap(&self, pixmap: &mut Pixmap) {
        if self.points.is_empty() {
            return;
        }
        
        let mut paint = Paint::default();
        paint.set_color(SkiaColor::from_rgba(
            self.color.r,
            self.color.g,
            self.color.b,
            self.color.a,
        ).unwrap_or(SkiaColor::BLACK));
        paint.anti_alias = true;
        
        // 円形ブラシ実装：各点に円を描画
        for point in &self.points {
            let mut path = PathBuilder::new();
            path.push_circle(point.x, point.y, self.stroke_width / 2.0);
            if let Some(path) = path.finish() {
                pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, tiny_skia::Transform::identity(), None);
            }
        }
        
        // 点の間を補間して滑らかな描画を実現
        if self.points.len() > 1 {
            for window in self.points.windows(2) {
                let p1 = window[0];
                let p2 = window[1];
                
                // 2点間の距離を計算
                let dx = p2.x - p1.x;
                let dy = p2.y - p1.y;
                let distance = (dx * dx + dy * dy).sqrt();
                
                // ブラシサイズの半分の間隔で補間点を生成
                let step_size = self.stroke_width / 4.0;
                let steps = (distance / step_size).ceil() as i32;
                
                if steps > 1 {
                    for i in 1..steps {
                        let t = i as f32 / steps as f32;
                        let x = p1.x + dx * t;
                        let y = p1.y + dy * t;
                        
                        let mut path = PathBuilder::new();
                        path.push_circle(x, y, self.stroke_width / 2.0);
                        if let Some(path) = path.finish() {
                            pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, tiny_skia::Transform::identity(), None);
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct PaintEngine {
    pub width: u32,
    pub height: u32,
    pub current_stroke: Option<PaintStroke>,
    pub is_drawing: bool,
}

impl PaintEngine {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            current_stroke: None,
            is_drawing: false,
        }
    }
    
    pub fn start_stroke(&mut self, x: f32, y: f32, tools: &ToolSettings) {
        let color = tools.get_current_color();
        let mut stroke = PaintStroke::new(color, tools.brush_size);
        stroke.add_point(x, y);
        
        self.current_stroke = Some(stroke);
        self.is_drawing = true;
    }
    
    pub fn continue_stroke(&mut self, x: f32, y: f32) {
        if let Some(ref mut stroke) = self.current_stroke {
            stroke.add_point(x, y);
        }
    }
    
    pub fn end_stroke(&mut self, layer_manager: &mut LayerManager) {
        if let Some(stroke) = self.current_stroke.take() {
            if let Some(active_layer) = layer_manager.get_active_layer_mut() {
                // アクティブレイヤーにストロークを追加（Pixmap描画とストロークリスト保存）
                active_layer.add_stroke(stroke);
            }
        }
        self.is_drawing = false;
    }
    
    pub fn cancel_stroke(&mut self) {
        self.current_stroke = None;
        self.is_drawing = false;
    }
    
    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
    
    /// プレビュー用：現在のストロークを含む一時的な画像を生成
    pub fn render_preview(&self, layer_manager: &LayerManager) -> Option<Pixmap> {
        let mut preview = layer_manager.composite()?;
        
        // 現在描画中のストロークを上に描画
        if let Some(ref stroke) = self.current_stroke {
            stroke.draw_to_pixmap(&mut preview);
        }
        
        Some(preview)
    }
    
    /// 現在描画中のストロークを取得（プレビュー用）
    pub fn get_current_stroke(&self) -> Option<&PaintStroke> {
        self.current_stroke.as_ref()
    }
    
    /// デバッグ用：キャンバスに格子を描画
    pub fn draw_grid(&self, pixmap: &mut Pixmap, grid_size: f32) {
        let mut paint = Paint::default();
        paint.set_color(SkiaColor::from_rgba(0.8, 0.8, 0.8, 0.4).unwrap_or(SkiaColor::BLACK));
        paint.anti_alias = true;
        
        let mut stroke = Stroke::default();
        stroke.width = 1.0;
        
        // 縦線
        let mut x = 0.0;
        while x <= self.width as f32 {
            let mut path = PathBuilder::new();
            path.move_to(x, 0.0);
            path.line_to(x, self.height as f32);
            if let Some(path) = path.finish() {
                pixmap.stroke_path(&path, &paint, &stroke, tiny_skia::Transform::identity(), None);
            }
            x += grid_size;
        }
        
        // 横線
        let mut y = 0.0;
        while y <= self.height as f32 {
            let mut path = PathBuilder::new();
            path.move_to(0.0, y);
            path.line_to(self.width as f32, y);
            if let Some(path) = path.finish() {
                pixmap.stroke_path(&path, &paint, &stroke, tiny_skia::Transform::identity(), None);
            }
            y += grid_size;
        }
    }
}