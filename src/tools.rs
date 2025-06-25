use iced::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tool {
    Pen,
    Eraser,
}

impl Default for Tool {
    fn default() -> Self {
        Tool::Pen
    }
}

#[derive(Debug, Clone)]
pub struct ToolSettings {
    pub current_tool: Tool,
    pub brush_size: f32,
    pub brush_opacity: f32,
    pub brush_color: Color,
    pub background_color: Color,
}

impl Default for ToolSettings {
    fn default() -> Self {
        Self {
            current_tool: Tool::Pen,
            brush_size: 10.0,
            brush_opacity: 1.0,
            brush_color: Color::BLACK,
            background_color: Color::WHITE,
        }
    }
}

impl ToolSettings {
    pub fn set_tool(&mut self, tool: Tool) {
        self.current_tool = tool;
    }
    
    pub fn set_brush_size(&mut self, size: f32) {
        self.brush_size = size.clamp(1.0, 200.0);
    }
    
    pub fn set_brush_opacity(&mut self, opacity: f32) {
        self.brush_opacity = opacity.clamp(0.0, 1.0);
    }
    
    pub fn set_brush_color(&mut self, color: Color) {
        self.brush_color = color;
    }
    
    pub fn get_current_color(&self) -> Color {
        match self.current_tool {
            Tool::Pen => {
                Color {
                    r: self.brush_color.r,
                    g: self.brush_color.g,
                    b: self.brush_color.b,
                    a: self.brush_opacity,
                }
            },
            Tool::Eraser => self.background_color,
        }
    }
    
    pub fn is_eraser(&self) -> bool {
        self.current_tool == Tool::Eraser
    }
}