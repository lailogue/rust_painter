use iced::widget::canvas::{self, Geometry, Path, Stroke, Frame};
use iced::{mouse, Color, Point, Rectangle, Renderer, Size};
// use crate::paint_engine::PaintEngine;
// use crate::tools::ToolSettings;
use crate::Message;

#[derive(Debug, Default)]
pub struct PaintCanvas {
    cache: canvas::Cache,
}

impl PaintCanvas {
    // pub fn new(_paint_engine: &PaintEngine, _tools: &ToolSettings) -> Self {
    //     Self {
    //         cache: canvas::Cache::default(),
    //     }
    // }
    
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

impl canvas::Program<Message> for PaintCanvas {
    type State = CanvasState;

    fn update(
        &self,
        state: &mut Self::State,
        event: canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (canvas::event::Status, Option<Message>) {
        let cursor_position = cursor.position_in(bounds);

        match event {
            canvas::Event::Mouse(mouse_event) => {
                match mouse_event {
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        if let Some(position) = cursor_position {
                            state.is_drawing = true;
                            state.last_position = Some(position);
                            return (
                                canvas::event::Status::Captured,
                                Some(Message::CanvasMessage(event))
                            );
                        }
                    }
                    mouse::Event::CursorMoved { .. } => {
                        if state.is_drawing {
                            if let Some(position) = cursor_position {
                                state.last_position = Some(position);
                                return (
                                    canvas::event::Status::Captured,
                                    Some(Message::CanvasMessage(event))
                                );
                            }
                        }
                    }
                    mouse::Event::ButtonReleased(mouse::Button::Left) => {
                        if state.is_drawing {
                            state.is_drawing = false;
                            state.last_position = None;
                            return (
                                canvas::event::Status::Captured,
                                Some(Message::CanvasMessage(event))
                            );
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
        state: &Self::State,
        renderer: &Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let canvas = self.cache.draw(renderer, bounds.size(), |frame: &mut Frame| {
            // 背景を描画
            frame.fill_rectangle(
                Point::ORIGIN,
                bounds.size(),
                Color::WHITE,
            );
            
            // 簡易版：マウスカーソル位置に円を描画
            if let Some(position) = state.last_position {
                frame.fill(
                    &Path::circle(position, 10.0),
                    Color::BLACK,
                );
            }
            
            // グリッドを描画（デバッグ用）
            self.draw_grid(frame, bounds.size(), 50.0);
        });

        vec![canvas]
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if cursor.is_over(bounds) {
            mouse::Interaction::Crosshair
        } else {
            mouse::Interaction::default()
        }
    }
}

impl PaintCanvas {
    fn draw_grid(&self, frame: &mut Frame, size: Size, grid_size: f32) {
        let grid_color = Color::from_rgba(0.9, 0.9, 0.9, 0.5);
        let stroke = Stroke::default().with_width(1.0).with_color(grid_color);
        
        // 縦線
        let mut x = 0.0;
        while x <= size.width {
            let path = Path::line(Point::new(x, 0.0), Point::new(x, size.height));
            frame.stroke(&path, stroke.clone());
            x += grid_size;
        }
        
        // 横線
        let mut y = 0.0;
        while y <= size.height {
            let path = Path::line(Point::new(0.0, y), Point::new(size.width, y));
            frame.stroke(&path, stroke.clone());
            y += grid_size;
        }
    }
}

#[derive(Debug, Default)]
pub struct CanvasState {
    pub is_drawing: bool,
    pub last_position: Option<Point>,
}