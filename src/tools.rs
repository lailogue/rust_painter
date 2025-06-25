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
    // HSV値を内部で管理
    pub hue: f32,        // 0.0 - 360.0
    pub saturation: f32, // 0.0 - 1.0
    pub value: f32,      // 0.0 - 1.0
}

impl Default for ToolSettings {
    fn default() -> Self {
        Self {
            current_tool: Tool::Pen,
            brush_size: 10.0,
            brush_opacity: 1.0,
            brush_color: Color::BLACK,
            background_color: Color::WHITE,
            hue: 0.0,        // 黒色のHSV値
            saturation: 0.0,
            value: 0.0,
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
    
    // HSV値を設定してRGB色を更新
    pub fn set_hsv(&mut self, hue: f32, saturation: f32, value: f32) {
        self.hue = hue.clamp(0.0, 360.0);
        self.saturation = saturation.clamp(0.0, 1.0);
        self.value = value.clamp(0.0, 1.0);
        self.brush_color = hsv_to_rgb(self.hue, self.saturation, self.value);
    }
    
    // 色相のみを設定
    pub fn set_hue(&mut self, hue: f32) {
        self.set_hsv(hue, self.saturation, self.value);
    }
    
    // 彩度のみを設定
    pub fn set_saturation(&mut self, saturation: f32) {
        self.set_hsv(self.hue, saturation, self.value);
    }
    
    // 明度のみを設定
    pub fn set_value(&mut self, value: f32) {
        self.set_hsv(self.hue, self.saturation, value);
    }
    
    // RGB色を設定してHSV値を更新
    pub fn set_brush_color(&mut self, color: Color) {
        self.brush_color = color;
        let (h, s, v) = rgb_to_hsv(color.r, color.g, color.b);
        self.hue = h;
        self.saturation = s;
        self.value = v;
    }
}

// HSVからRGBへの変換
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Color {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;
    
    let (r_prime, g_prime, b_prime) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    
    Color::from_rgb(r_prime + m, g_prime + m, b_prime + m)
}

// RGBからHSVへの変換
fn rgb_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g.max(b));
    let min = r.min(g.min(b));
    let delta = max - min;
    
    let hue = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * ((b - r) / delta + 2.0)
    } else {
        60.0 * ((r - g) / delta + 4.0)
    };
    
    let hue = if hue < 0.0 { hue + 360.0 } else { hue };
    
    let saturation = if max == 0.0 { 0.0 } else { delta / max };
    let value = max;
    
    (hue, saturation, value)
}