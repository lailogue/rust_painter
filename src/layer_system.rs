use uuid::Uuid;
use tiny_skia::{Pixmap, Paint, Color as SkiaColor, BlendMode};
use crate::paint_engine::PaintStroke;

#[derive(Debug, Clone)]
pub struct Layer {
    pub id: Uuid,
    pub name: String,
    pub pixmap: Pixmap,
    pub strokes: Vec<PaintStroke>, // 確定済みストロークのリスト
    pub visible: bool,
    pub opacity: f32,
}

impl Layer {
    pub fn new(name: String, width: u32, height: u32) -> Option<Self> {
        let pixmap = Pixmap::new(width, height)?;
        Some(Self {
            id: Uuid::new_v4(),
            name,
            pixmap,
            strokes: Vec::new(),
            visible: true,
            opacity: 1.0,
        })
    }
    
    pub fn clear(&mut self) {
        self.pixmap.fill(SkiaColor::TRANSPARENT);
        self.strokes.clear();
    }
    
    pub fn add_stroke(&mut self, stroke: PaintStroke) {
        // Pixmapに描画
        stroke.draw_to_pixmap(&mut self.pixmap);
        // ストロークリストに追加（iced表示用）
        self.strokes.push(stroke);
    }
    
    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity.clamp(0.0, 1.0);
    }
    
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
    
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

#[derive(Debug, Clone)]
pub enum LayerAction {
    Add,
    Delete,
    MoveUp(usize),
    MoveDown(usize),
    SetOpacity(usize, f32),
    SetVisible(usize, bool),
    Rename(usize, String),
    SetActive(usize),
}

#[derive(Debug)]
pub struct LayerManager {
    layers: Vec<Layer>,
    active_layer_index: usize,
    canvas_width: u32,
    canvas_height: u32,
}

impl LayerManager {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            active_layer_index: 0,
            canvas_width: 800,
            canvas_height: 600,
        }
    }
    
    pub fn with_size(width: u32, height: u32) -> Self {
        let mut manager = Self {
            layers: Vec::new(),
            active_layer_index: 0,
            canvas_width: width,
            canvas_height: height,
        };
        
        // デフォルトレイヤーを作成
        manager.add_layer("Layer 1".to_string());
        manager
    }
    
    pub fn add_layer(&mut self, name: String) {
        if let Some(layer) = Layer::new(name, self.canvas_width, self.canvas_height) {
            self.layers.push(layer);
            self.active_layer_index = self.layers.len() - 1;
        }
    }
    
    pub fn remove_layer(&mut self, index: usize) {
        if self.layers.len() > 1 && index < self.layers.len() {
            self.layers.remove(index);
            if self.active_layer_index >= self.layers.len() {
                self.active_layer_index = self.layers.len() - 1;
            } else if self.active_layer_index > index {
                self.active_layer_index -= 1;
            }
        }
    }
    
    pub fn move_layer_up(&mut self, index: usize) {
        if index > 0 && index < self.layers.len() {
            self.layers.swap(index - 1, index);
            if self.active_layer_index == index {
                self.active_layer_index = index - 1;
            } else if self.active_layer_index == index - 1 {
                self.active_layer_index = index;
            }
        }
    }
    
    pub fn move_layer_down(&mut self, index: usize) {
        if index < self.layers.len() - 1 {
            self.layers.swap(index, index + 1);
            if self.active_layer_index == index {
                self.active_layer_index = index + 1;
            } else if self.active_layer_index == index + 1 {
                self.active_layer_index = index;
            }
        }
    }
    
    pub fn get_active_layer(&self) -> Option<&Layer> {
        self.layers.get(self.active_layer_index)
    }
    
    pub fn get_active_layer_mut(&mut self) -> Option<&mut Layer> {
        self.layers.get_mut(self.active_layer_index)
    }
    
    pub fn get_layer(&self, index: usize) -> Option<&Layer> {
        self.layers.get(index)
    }
    
    pub fn get_layer_mut(&mut self, index: usize) -> Option<&mut Layer> {
        self.layers.get_mut(index)
    }
    
    pub fn get_layers(&self) -> &Vec<Layer> {
        &self.layers
    }
    
    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }
    
    pub fn active_layer_index(&self) -> usize {
        self.active_layer_index
    }
    
    /// 表示可能なレイヤーとストロークのリストを取得（iced表示用）
    pub fn get_visible_strokes(&self) -> Vec<(&PaintStroke, f32)> {
        let mut strokes = Vec::new();
        
        // レイヤーを下から上へ（描画順）
        for layer in &self.layers {
            if layer.visible {
                for stroke in &layer.strokes {
                    strokes.push((stroke, layer.opacity));
                }
            }
        }
        
        strokes
    }
    
    pub fn handle_action(&mut self, action: LayerAction) {
        match action {
            LayerAction::Add => {
                let layer_count = self.layer_count();
                let new_name = format!("Layer {}", layer_count + 1);
                self.add_layer(new_name);
            }
            LayerAction::Delete => {
                if self.layer_count() > 1 {
                    self.remove_layer(self.active_layer_index);
                }
            }
            LayerAction::MoveUp(index) => {
                self.move_layer_up(index);
            }
            LayerAction::MoveDown(index) => {
                self.move_layer_down(index);
            }
            LayerAction::SetOpacity(index, opacity) => {
                if let Some(layer) = self.get_layer_mut(index) {
                    layer.set_opacity(opacity);
                }
            }
            LayerAction::SetVisible(index, visible) => {
                if let Some(layer) = self.get_layer_mut(index) {
                    layer.set_visible(visible);
                }
            }
            LayerAction::Rename(index, name) => {
                if let Some(layer) = self.get_layer_mut(index) {
                    layer.set_name(name);
                }
            }
            LayerAction::SetActive(index) => {
                if index < self.layer_count() {
                    self.active_layer_index = index;
                }
            }
        }
    }
    
    /// 全レイヤーを合成した最終画像を生成
    pub fn composite(&self) -> Option<Pixmap> {
        if self.layers.is_empty() {
            return None;
        }
        
        let mut result = Pixmap::new(self.canvas_width, self.canvas_height)?;
        result.fill(SkiaColor::WHITE); // 白背景
        
        // レイヤーを下から上へ合成
        for layer in &self.layers {
            if layer.visible {
                let pixmap_paint = tiny_skia::PixmapPaint {
                    opacity: layer.opacity,
                    blend_mode: BlendMode::SourceOver,
                    quality: tiny_skia::FilterQuality::Nearest,
                };
                
                result.draw_pixmap(
                    0, 0,
                    layer.pixmap.as_ref(),
                    &pixmap_paint,
                    tiny_skia::Transform::identity(),
                    None,
                );
            }
        }
        
        Some(result)
    }
    
    pub fn resize(&mut self, width: u32, height: u32) {
        self.canvas_width = width;
        self.canvas_height = height;
        
        // 既存レイヤーをリサイズ（簡単のため新しいレイヤーで置き換え）
        for layer in &mut self.layers {
            if let Some(new_pixmap) = Pixmap::new(width, height) {
                layer.pixmap = new_pixmap;
            }
        }
    }
}