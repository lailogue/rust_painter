// レイヤー管理に必要な基本的な型のみを使用
use crate::stroke::Stroke;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Layer {
    pub id: Uuid,
    pub name: String,
    pub strokes: Vec<Stroke>,
    pub visible: bool,
    pub opacity: f32, // 0.0 - 1.0
}

impl Layer {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            strokes: Vec::new(),
            visible: true,
            opacity: 1.0,
        }
    }
    
    pub fn add_stroke(&mut self, stroke: Stroke) {
        self.strokes.push(stroke);
    }
    
    pub fn clear(&mut self) {
        self.strokes.clear();
    }
    
    pub fn stroke_count(&self) -> usize {
        self.strokes.len()
    }
    
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
    
    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity.clamp(0.0, 1.0);
    }
    
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

#[derive(Debug)]
pub struct LayerManager {
    layers: Vec<Layer>,
    active_layer_index: usize,
}

impl LayerManager {
    pub fn new() -> Self {
        let mut manager = Self {
            layers: Vec::new(),
            active_layer_index: 0,
        };
        
        // デフォルトレイヤーを作成
        manager.add_layer("Layer 1".to_string());
        manager
    }
    
    pub fn add_layer(&mut self, name: String) -> usize {
        let layer = Layer::new(name);
        self.layers.push(layer);
        let new_index = self.layers.len() - 1;
        self.active_layer_index = new_index;
        new_index
    }
    
    pub fn remove_layer(&mut self, index: usize) -> bool {
        if self.layers.len() <= 1 || index >= self.layers.len() {
            return false; // 最後のレイヤーは削除できない
        }
        
        self.layers.remove(index);
        
        // アクティブレイヤーのインデックスを調整
        if self.active_layer_index >= self.layers.len() {
            self.active_layer_index = self.layers.len() - 1;
        } else if self.active_layer_index > index {
            self.active_layer_index -= 1;
        }
        
        true
    }
    
    // レイヤーを上に移動（描画順序で後になる = インデックスが増える）
    pub fn move_layer_up(&mut self, index: usize) -> bool {
        if index >= self.layers.len() - 1 {
            return false;
        }
        
        self.layers.swap(index, index + 1);
        
        // アクティブレイヤーのインデックスを調整
        if self.active_layer_index == index {
            self.active_layer_index = index + 1;
        } else if self.active_layer_index == index + 1 {
            self.active_layer_index = index;
        }
        
        true
    }
    
    // レイヤーを下に移動（描画順序で前になる = インデックスが減る）
    pub fn move_layer_down(&mut self, index: usize) -> bool {
        if index == 0 || index >= self.layers.len() {
            return false;
        }
        
        self.layers.swap(index - 1, index);
        
        // アクティブレイヤーのインデックスを調整
        if self.active_layer_index == index {
            self.active_layer_index = index - 1;
        } else if self.active_layer_index == index - 1 {
            self.active_layer_index = index;
        }
        
        true
    }
    
    pub fn set_active_layer(&mut self, index: usize) -> bool {
        if index < self.layers.len() {
            self.active_layer_index = index;
            true
        } else {
            false
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
    
    pub fn total_stroke_count(&self) -> usize {
        self.layers.iter().map(|layer| layer.stroke_count()).sum()
    }
    
    pub fn clear_active_layer(&mut self) {
        if let Some(layer) = self.get_active_layer_mut() {
            layer.clear();
        }
    }
    
    pub fn clear_all_layers(&mut self) {
        for layer in &mut self.layers {
            layer.clear();
        }
    }
    
    // レイヤーを描画順序で取得（下から上へ）
    pub fn get_layers_for_rendering(&self) -> impl Iterator<Item = (usize, &Layer)> {
        self.layers.iter().enumerate().filter(|(_, layer)| layer.visible)
    }
}

impl Default for LayerManager {
    fn default() -> Self {
        Self::new()
    }
}