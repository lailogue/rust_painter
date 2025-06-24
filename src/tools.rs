use eframe::egui::Color32;

#[derive(Debug, Clone, PartialEq)]
pub enum Tool {
    Pen,
    Eraser,
}

impl Default for Tool {
    fn default() -> Self {
        Tool::Pen
    }
}

pub struct ToolSettings {
    pub current_tool: Tool,
    pub brush_size: f32,
    pub brush_color: Color32,
    pub brush_opacity: f32,  // 0.0 - 1.0
    pub background_color: Color32,
}

impl Default for ToolSettings {
    fn default() -> Self {
        Self {
            current_tool: Tool::Pen,
            brush_size: 10.0,  // デフォルト10に変更
            brush_color: Color32::BLACK,
            brush_opacity: 1.0,  // 完全不透明
            background_color: Color32::WHITE,
        }
    }
}

impl ToolSettings {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn is_eraser(&self) -> bool {
        self.current_tool == Tool::Eraser
    }
    
    pub fn get_current_color(&self) -> Color32 {
        match self.current_tool {
            Tool::Pen => {
                let [r, g, b, _] = self.brush_color.to_array();
                let alpha = (self.brush_opacity * 255.0) as u8;
                Color32::from_rgba_unmultiplied(r, g, b, alpha)
            },
            Tool::Eraser => self.background_color,
        }
    }
    
    pub fn set_tool(&mut self, tool: Tool) {
        self.current_tool = tool;
    }
    
    pub fn set_brush_size(&mut self, size: f32) {
        self.brush_size = size.clamp(1.0, 200.0);  // 最大200に変更
    }
    
    pub fn set_brush_opacity(&mut self, opacity: f32) {
        self.brush_opacity = opacity.clamp(0.0, 1.0);
    }
    
    pub fn set_brush_color(&mut self, color: Color32) {
        self.brush_color = color;
    }
    
    pub fn set_background_color(&mut self, color: Color32) {
        self.background_color = color;
    }
}