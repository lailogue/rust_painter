use iced::widget::{canvas, column, container, row, slider, text, button};
use iced::{window, Application, Color, Element, Length, Settings, Theme};

mod canvas_widget;
mod font;
mod paint_engine;
mod layer_system;
mod tools;

use canvas_widget::PaintCanvas;
use paint_engine::PaintEngine;
use layer_system::{LayerManager, LayerAction};
use tools::{Tool, ToolSettings};

pub fn main() -> iced::Result {
    PaintApp::run(Settings {
        window: window::Settings {
            size: iced::Size::new(1200.0, 800.0),
            ..Default::default()
        },
        fonts: vec![font::setup_fonts().into()],
        default_font: font::default_font(),
        ..Default::default()
    })
}

#[derive(Debug, Clone)]
pub enum Message {
    // ツール関連
    ToolChanged(Tool),
    BrushSizeChanged(f32),
    BrushOpacityChanged(f32),
    ColorChanged(Color),
    
    // レイヤー関連
    LayerAction(LayerAction),
    
    // キャンバス関連
    CanvasMessage(canvas::Event),
    
    // 描画関連
    StartStroke(iced::Point),
    ContinueStroke(iced::Point),
    EndStroke,
}

pub struct PaintApp {
    tools: ToolSettings,
    layer_manager: LayerManager,
    paint_engine: PaintEngine,
    should_redraw: bool,
}

impl Application for PaintApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, iced::Command<Message>) {
        (
            Self {
                tools: ToolSettings::default(),
                layer_manager: LayerManager::with_size(800, 600),
                paint_engine: PaintEngine::new(800, 600),
                should_redraw: false,
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Rust Painter - Iced + Tiny Skia")
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::ToolChanged(tool) => {
                self.tools.set_tool(tool);
            }
            Message::BrushSizeChanged(size) => {
                self.tools.set_brush_size(size);
            }
            Message::BrushOpacityChanged(opacity) => {
                self.tools.set_brush_opacity(opacity);
            }
            Message::ColorChanged(color) => {
                self.tools.set_brush_color(color);
            }
            Message::LayerAction(action) => {
                self.layer_manager.handle_action(action);
            }
            Message::CanvasMessage(event) => {
                // キャンバスイベントの処理
                use iced::widget::canvas;
                match event {
                    canvas::Event::Mouse(mouse_event) => {
                        match mouse_event {
                            iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left) => {
                                // マウス左ボタンが押された：ストローク開始
                                // この処理はcanvas_widgetで座標を取得済み
                            }
                            iced::mouse::Event::CursorMoved { .. } => {
                                // マウス移動：描画中ならストローク継続
                                // この処理もcanvas_widgetで座標を取得済み
                            }
                            iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left) => {
                                // マウス左ボタンが離された：ストローク終了
                                // この処理もcanvas_widgetで座標を取得済み
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            Message::StartStroke(point) => {
                self.paint_engine.start_stroke(point.x, point.y, &self.tools);
                self.should_redraw = true;
            }
            Message::ContinueStroke(point) => {
                self.paint_engine.continue_stroke(point.x, point.y);
                self.should_redraw = true;
            }
            Message::EndStroke => {
                self.paint_engine.end_stroke(&mut self.layer_manager);
                self.should_redraw = true;
            }
        }
        iced::Command::none()
    }

    fn view(&self) -> Element<Message> {
        let toolbar = self.create_toolbar();
        let layer_panel = self.create_layer_panel();
        let canvas = self.create_canvas();

        let main_content = row![
            container(layer_panel).width(250),
            container(canvas).width(Length::Fill),
        ];

        column![
            container(toolbar).height(60),
            container(main_content).height(Length::Fill),
        ]
        .into()
    }
}

impl PaintApp {
    fn create_toolbar(&self) -> Element<Message> {
        let brush_size_slider = row![
            text("ブラシサイズ:"),
            slider(1.0..=200.0, self.tools.brush_size, Message::BrushSizeChanged)
                .step(1.0)
                .width(150),
            text(format!("{:.0}", self.tools.brush_size))
        ]
        .spacing(10);

        let opacity_slider = row![
            text("透明度:"),
            slider(0.0..=1.0, self.tools.brush_opacity, Message::BrushOpacityChanged)
                .step(0.01)
                .width(150),
            text(format!("{:.0}%", self.tools.brush_opacity * 100.0))
        ]
        .spacing(10);

        let tool_buttons = row![
            button("ペン").on_press(Message::ToolChanged(Tool::Pen)),
            button("消しゴム").on_press(Message::ToolChanged(Tool::Eraser)),
        ]
        .spacing(10);

        row![brush_size_slider, opacity_slider, tool_buttons]
            .spacing(20)
            .padding(10)
            .into()
    }

    fn create_layer_panel(&self) -> Element<Message> {
        let layer_buttons = row![
            button("追加").on_press(Message::LayerAction(LayerAction::Add)),
            button("削除").on_press(Message::LayerAction(LayerAction::Delete)),
        ]
        .spacing(10);

        column![
            text("レイヤー").size(20),
            layer_buttons,
            text(format!("レイヤー数: {}", self.layer_manager.layer_count())),
            text(format!("アクティブ: {}", self.layer_manager.active_layer_index() + 1)),
        ]
        .spacing(10)
        .padding(10)
        .into()
    }

    fn create_canvas(&self) -> Element<Message> {
        canvas(PaintCanvas::new(&self.paint_engine, &self.layer_manager, &self.tools))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}