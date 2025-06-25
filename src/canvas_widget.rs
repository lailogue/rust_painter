use iced::widget::canvas::{self, Geometry, Path, Stroke, Frame};
use iced::{mouse, Color, Point, Rectangle, Renderer, Size};
use crate::paint_engine::PaintEngine;
use crate::layer_system::LayerManager;
use crate::tools::ToolSettings;
use crate::Message;

#[derive(Debug)]
pub struct PaintCanvas<'a> {
    paint_engine: &'a PaintEngine,
    layer_manager: &'a LayerManager,
    tools: &'a ToolSettings,
    cache: canvas::Cache,
}

impl<'a> PaintCanvas<'a> {
    pub fn new(paint_engine: &'a PaintEngine, layer_manager: &'a LayerManager, tools: &'a ToolSettings) -> Self {
        Self {
            paint_engine,
            layer_manager,
            tools,
            cache: canvas::Cache::default(),
        }
    }
    
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

impl<'a> canvas::Program<Message> for PaintCanvas<'a> {
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
                            state.needs_redraw = true;
                            return (
                                canvas::event::Status::Captured,
                                Some(Message::StartStroke(position))
                            );
                        }
                    }
                    mouse::Event::CursorMoved { .. } => {
                        if state.is_drawing {
                            if let Some(position) = cursor_position {
                                state.last_position = Some(position);
                                state.needs_redraw = true;
                                return (
                                    canvas::event::Status::Captured,
                                    Some(Message::ContinueStroke(position))
                                );
                            }
                        }
                    }
                    mouse::Event::ButtonReleased(mouse::Button::Left) => {
                        if state.is_drawing {
                            state.is_drawing = false;
                            state.last_position = None;
                            state.needs_redraw = true;
                            return (
                                canvas::event::Status::Captured,
                                Some(Message::EndStroke)
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
        // 再描画が必要な場合はキャッシュをクリア
        let cache = &self.cache;
        if state.needs_redraw {
            // Note: ここでキャッシュをクリアする方法を調査する必要がある
        }
        
        let canvas = cache.draw(renderer, bounds.size(), |frame: &mut Frame| {
            // 背景を描画
            frame.fill_rectangle(
                Point::ORIGIN,
                bounds.size(),
                Color::WHITE,
            );
            
            // tiny_skiaで描画された内容を表示（プレビュー）
            if let Some(pixmap) = self.paint_engine.render_preview(self.layer_manager) {
                self.draw_pixmap_to_frame(frame, &pixmap, bounds.size());
            }
            
            // 現在のカーソル位置にブラシのプレビューを表示
            if let Some(position) = state.last_position {
                frame.stroke(
                    &Path::circle(position, self.tools.brush_size / 2.0),
                    Stroke::default()
                        .with_width(1.0)
                        .with_color(self.tools.get_current_color()),
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

impl<'a> PaintCanvas<'a> {
    fn draw_pixmap_to_frame(&self, frame: &mut Frame, pixmap: &tiny_skia::Pixmap, canvas_size: Size) {
        // 簡単な実装：tiny_skiaで描画した内容を可視化
        // 実際の描画内容を点で表現（デモ用）
        let data = pixmap.data();
        let width = pixmap.width();
        let height = pixmap.height();
        
        // サンプリングして描画（パフォーマンスのため）
        let sample_rate = 4;
        for y in (0..height as usize).step_by(sample_rate) {
            for x in (0..width as usize).step_by(sample_rate) {
                let index = (y * width as usize + x) * 4;
                if index + 3 < data.len() {
                    let a = data[index + 3] as f32 / 255.0;
                    
                    if a > 0.1 { // 描画されたピクセルのみ表示
                        let r = data[index] as f32 / 255.0;
                        let g = data[index + 1] as f32 / 255.0;
                        let b = data[index + 2] as f32 / 255.0;
                        let color = Color::from_rgba(r, g, b, a);
                        
                        let point = Point::new(x as f32, y as f32);
                        frame.fill(
                            &Path::circle(point, 2.0),
                            color,
                        );
                    }
                }
            }
        }
    }
    
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
    pub needs_redraw: bool,
}