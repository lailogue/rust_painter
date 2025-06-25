use iced::widget::canvas::{self, Geometry, Path, Frame};
use iced::{mouse, Color, Point, Rectangle, Renderer, Size, Element};
use crate::Message;

// 2Dカラーピッカーウィジェット
#[derive(Debug, Clone)]
pub struct ColorPicker2D {
    hue: f32,          // 現在の色相 (0-360)
    saturation: f32,   // 現在の彩度 (0-1)
    value: f32,        // 現在の明度 (0-1)
    size: f32,         // ピッカーのサイズ
}

impl ColorPicker2D {
    pub fn new(hue: f32, saturation: f32, value: f32) -> Self {
        Self {
            hue: hue.clamp(0.0, 360.0),
            saturation: saturation.clamp(0.0, 1.0),
            value: value.clamp(0.0, 1.0),
            size: 200.0,
        }
    }
    
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
}

impl<'a> From<ColorPicker2D> for Element<'a, Message> {
    fn from(picker: ColorPicker2D) -> Self {
        let size = picker.size;
        iced::widget::canvas(picker)
            .width(size)
            .height(size)
            .into()
    }
}

impl canvas::Program<Message> for ColorPicker2D {
    type State = ColorPickerState;

    fn update(
        &self,
        state: &mut Self::State,
        event: canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (canvas::event::Status, Option<Message>) {
        match event {
            canvas::Event::Mouse(mouse_event) => {
                match mouse_event {
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        if let Some(position) = cursor.position_in(bounds) {
                            state.is_dragging = true;
                            let (s, v) = self.position_to_sv(position, bounds.size());
                            return (
                                canvas::event::Status::Captured,
                                Some(Message::ColorPickerChanged(self.hue, s, v))
                            );
                        }
                    }
                    mouse::Event::CursorMoved { .. } => {
                        if state.is_dragging {
                            if let Some(position) = cursor.position_in(bounds) {
                                let (s, v) = self.position_to_sv(position, bounds.size());
                                return (
                                    canvas::event::Status::Captured,
                                    Some(Message::ColorPickerChanged(self.hue, s, v))
                                );
                            }
                        }
                    }
                    mouse::Event::ButtonReleased(mouse::Button::Left) => {
                        if state.is_dragging {
                            state.is_dragging = false;
                            return (canvas::event::Status::Captured, None);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        (canvas::event::Status::Ignored, None)
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let canvas = iced::widget::canvas::Cache::default().draw(renderer, bounds.size(), |frame: &mut Frame| {
            self.draw_color_square(frame, bounds.size());
            self.draw_cursor(frame, bounds.size());
        });

        vec![canvas]
    }

    fn mouse_interaction(
        &self,
        state: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if cursor.is_over(bounds) || state.is_dragging {
            mouse::Interaction::Crosshair
        } else {
            mouse::Interaction::default()
        }
    }
}

impl ColorPicker2D {
    // マウス位置をS-V値に変換
    fn position_to_sv(&self, position: Point, size: Size) -> (f32, f32) {
        let s = (position.x / size.width).clamp(0.0, 1.0);
        let v = 1.0 - (position.y / size.height).clamp(0.0, 1.0); // Y軸を反転（上が明るい）
        (s, v)
    }
    
    // S-V値をマウス位置に変換
    fn sv_to_position(&self, size: Size) -> Point {
        Point::new(
            self.saturation * size.width,
            (1.0 - self.value) * size.height, // Y軸を反転
        )
    }
    
    // S-V平面の色グラデーションを描画
    fn draw_color_square(&self, frame: &mut Frame, size: Size) {
        let resolution = 50; // 描画解像度
        let step_x = size.width / resolution as f32;
        let step_y = size.height / resolution as f32;
        
        for i in 0..resolution {
            for j in 0..resolution {
                let s = i as f32 / (resolution - 1) as f32;
                let v = 1.0 - (j as f32 / (resolution - 1) as f32); // Y軸反転
                
                let color = hsv_to_rgb(self.hue, s, v);
                
                let rect = Rectangle::new(
                    Point::new(i as f32 * step_x, j as f32 * step_y),
                    Size::new(step_x + 1.0, step_y + 1.0), // 隙間を埋めるため+1
                );
                
                frame.fill_rectangle(rect.position(), rect.size(), color);
            }
        }
    }
    
    // 現在の選択位置にカーソルを描画
    fn draw_cursor(&self, frame: &mut Frame, size: Size) {
        let position = self.sv_to_position(size);
        let radius = 5.0;
        
        // 外側の白い円
        frame.stroke(
            &Path::circle(position, radius + 1.0),
            iced::widget::canvas::Stroke::default()
                .with_width(2.0)
                .with_color(Color::WHITE),
        );
        
        // 内側の黒い円
        frame.stroke(
            &Path::circle(position, radius),
            iced::widget::canvas::Stroke::default()
                .with_width(1.0)
                .with_color(Color::BLACK),
        );
    }
}

#[derive(Debug, Default)]
pub struct ColorPickerState {
    pub is_dragging: bool,
}

// HSVからRGBへの変換（color_picker.rs内で使用）
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

// 色相スライダーウィジェット
#[derive(Debug, Clone)]
pub struct HueSlider {
    hue: f32,
    width: f32,
    height: f32,
}

impl HueSlider {
    pub fn new(hue: f32) -> Self {
        Self {
            hue: hue.clamp(0.0, 360.0),
            width: 200.0,
            height: 20.0,
        }
    }
    
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }
}

impl<'a> From<HueSlider> for Element<'a, Message> {
    fn from(slider: HueSlider) -> Self {
        let width = slider.width;
        let height = slider.height;
        iced::widget::canvas(slider)
            .width(width)
            .height(height)
            .into()
    }
}

impl canvas::Program<Message> for HueSlider {
    type State = HueSliderState;

    fn update(
        &self,
        state: &mut Self::State,
        event: canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (canvas::event::Status, Option<Message>) {
        match event {
            canvas::Event::Mouse(mouse_event) => {
                match mouse_event {
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        if let Some(position) = cursor.position_in(bounds) {
                            state.is_dragging = true;
                            let hue = self.position_to_hue(position.x, bounds.width);
                            return (
                                canvas::event::Status::Captured,
                                Some(Message::HueChanged(hue))
                            );
                        }
                    }
                    mouse::Event::CursorMoved { .. } => {
                        if state.is_dragging {
                            if let Some(position) = cursor.position_in(bounds) {
                                let hue = self.position_to_hue(position.x, bounds.width);
                                return (
                                    canvas::event::Status::Captured,
                                    Some(Message::HueChanged(hue))
                                );
                            }
                        }
                    }
                    mouse::Event::ButtonReleased(mouse::Button::Left) => {
                        if state.is_dragging {
                            state.is_dragging = false;
                            return (canvas::event::Status::Captured, None);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        (canvas::event::Status::Ignored, None)
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let canvas = iced::widget::canvas::Cache::default().draw(renderer, bounds.size(), |frame: &mut Frame| {
            self.draw_hue_gradient(frame, bounds.size());
            self.draw_hue_cursor(frame, bounds.size());
        });

        vec![canvas]
    }

    fn mouse_interaction(
        &self,
        state: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if cursor.is_over(bounds) || state.is_dragging {
            mouse::Interaction::Crosshair
        } else {
            mouse::Interaction::default()
        }
    }
}

impl HueSlider {
    fn position_to_hue(&self, x: f32, width: f32) -> f32 {
        (x / width * 360.0).clamp(0.0, 360.0)
    }
    
    fn hue_to_position(&self, width: f32) -> f32 {
        (self.hue / 360.0) * width
    }
    
    fn draw_hue_gradient(&self, frame: &mut Frame, size: Size) {
        let steps = 360;
        let step_width = size.width / steps as f32;
        
        for i in 0..steps {
            let hue = i as f32;
            let color = hsv_to_rgb(hue, 1.0, 1.0); // 最大彩度・明度
            
            let rect = Rectangle::new(
                Point::new(i as f32 * step_width, 0.0),
                Size::new(step_width + 1.0, size.height),
            );
            
            frame.fill_rectangle(rect.position(), rect.size(), color);
        }
    }
    
    fn draw_hue_cursor(&self, frame: &mut Frame, size: Size) {
        let x = self.hue_to_position(size.width);
        
        // カーソル線を描画
        let path = Path::line(
            Point::new(x, 0.0),
            Point::new(x, size.height),
        );
        
        frame.stroke(
            &path,
            iced::widget::canvas::Stroke::default()
                .with_width(2.0)
                .with_color(Color::WHITE),
        );
        
        frame.stroke(
            &path,
            iced::widget::canvas::Stroke::default()
                .with_width(1.0)
                .with_color(Color::BLACK),
        );
    }
}

#[derive(Debug, Default)]
pub struct HueSliderState {
    pub is_dragging: bool,
}