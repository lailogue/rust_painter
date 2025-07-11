use iced::widget::{canvas, column, container, row, slider, text, button, Space, checkbox, scrollable};
use iced::{window, Application, Color, Element, Length, Settings, Theme};

mod canvas_widget;
mod font;
mod paint_engine;
mod layer_system;
mod tools;
mod color_picker;

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
    
    // HSV カラーピッカー関連
    HueChanged(f32),
    SaturationChanged(f32),
    ValueChanged(f32),
    
    // 2D カラーピッカー関連
    ColorPickerChanged(f32, f32, f32), // hue, saturation, value
    
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
            Message::HueChanged(hue) => {
                self.tools.set_hue(hue);
            }
            Message::SaturationChanged(saturation) => {
                self.tools.set_saturation(saturation);
            }
            Message::ValueChanged(value) => {
                self.tools.set_value(value);
            }
            Message::ColorPickerChanged(hue, saturation, value) => {
                self.tools.set_hsv(hue, saturation, value);
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
        let left_toolbar = self.create_left_toolbar();
        let layer_panel = self.create_layer_panel();
        let canvas = self.create_canvas();
        let color_picker_panel = self.create_color_picker_panel();

        let main_content = row![
            container(layer_panel).width(250),
            container(canvas).width(Length::Fill),
            container(color_picker_panel).width(280),
        ];

        column![
            container(left_toolbar).height(120),
            container(main_content).height(Length::Fill),
        ]
        .into()
    }
}

impl PaintApp {
    fn create_left_toolbar(&self) -> Element<Message> {
        // ツール設定
        let brush_size_slider = row![
            text("ブラシサイズ:"),
            slider(1.0..=200.0, self.tools.brush_size, Message::BrushSizeChanged)
                .step(1.0)
                .width(120),
            text(format!("{:.0}", self.tools.brush_size))
        ]
        .spacing(8);

        let opacity_slider = row![
            text("透明度:"),
            slider(0.0..=1.0, self.tools.brush_opacity, Message::BrushOpacityChanged)
                .step(0.01)
                .width(120),
            text(format!("{:.0}%", self.tools.brush_opacity * 100.0))
        ]
        .spacing(8);

        let tool_buttons = row![
            button("ペン").on_press(Message::ToolChanged(Tool::Pen)),
            button("消しゴム").on_press(Message::ToolChanged(Tool::Eraser)),
        ]
        .spacing(8);

        row![brush_size_slider, opacity_slider, tool_buttons]
            .spacing(15)
            .padding(10)
            .into()
    }

    fn create_color_picker_panel(&self) -> Element<Message> {
        let current_color = self.tools.brush_color;
        
        // カラープレビュー（現在の色を表示）
        let color_preview = container(
            Space::with_width(80).height(80)
        )
        .style(move |_theme: &Theme| {
            container::Appearance {
                background: Some(iced::Background::Color(current_color)),
                border: iced::Border {
                    color: Color::BLACK,
                    width: 2.0,
                    radius: 8.0.into(),
                },
                ..Default::default()
            }
        });

        // 2Dカラーピッカー（S-V平面）
        let color_picker_2d: Element<Message> = color_picker::ColorPicker2D::new(
            self.tools.hue, 
            self.tools.saturation, 
            self.tools.value
        ).size(220.0).into();

        // 色相スライダー
        let hue_slider: Element<Message> = color_picker::HueSlider::new(self.tools.hue)
            .size(220.0, 20.0)
            .into();

        column![
            text("カラーピッカー").size(18),
            Space::with_height(10),
            color_preview,
            Space::with_height(15),
            color_picker_2d,
            Space::with_height(15),
            text(format!("色相: {:.0}°", self.tools.hue)).size(12),
            hue_slider,
            Space::with_height(10),
            text(format!("彩度: {:.0}% / 明度: {:.0}%", 
                self.tools.saturation * 100.0, 
                self.tools.value * 100.0)).size(12)
        ]
        .spacing(5)
        .padding(15)
        .into()
    }

    fn create_layer_panel(&self) -> Element<Message> {
        let layer_buttons = row![
            button("追加").on_press(Message::LayerAction(LayerAction::Add)),
            button("削除").on_press(Message::LayerAction(LayerAction::Delete)),
        ]
        .spacing(10);

        // レイヤーリストを作成（上から下へ、逆順で表示）
        let mut layer_list = column![]
            .spacing(5);

        let layers = self.layer_manager.get_layers();
        let active_index = self.layer_manager.active_layer_index();

        // レイヤーを逆順で表示（上が最前面）
        for (index, layer) in layers.iter().enumerate().rev() {
            let is_active = index == active_index;
            let is_background = index == 0;
            
            // レイヤー選択ボタン
            let layer_button = if is_active {
                button(text(&layer.name))
                    .on_press(Message::LayerAction(LayerAction::SetActive(index)))
                    .style(iced::theme::Button::Primary)
                    .width(Length::Fill)
            } else {
                button(text(&layer.name))
                    .on_press(Message::LayerAction(LayerAction::SetActive(index)))
                    .style(iced::theme::Button::Secondary)
                    .width(Length::Fill)
            };

            // 表示/非表示チェックボックス
            let visibility_checkbox = checkbox("", layer.visible)
                .on_toggle(move |visible| Message::LayerAction(LayerAction::SetVisible(index, visible)));

            // 透明度スライダー（背景レイヤー以外）
            let opacity_control: Element<Message> = if !is_background {
                row![
                    text("透明度:").size(12),
                    slider(0.0..=1.0, layer.opacity, move |opacity| {
                        Message::LayerAction(LayerAction::SetOpacity(index, opacity))
                    })
                    .step(0.01)
                    .width(80),
                    text(format!("{:.0}%", layer.opacity * 100.0)).size(12)
                ]
                .spacing(5)
                .into()
            } else {
                Space::with_height(0).into()
            };

            // 上下移動ボタン（背景レイヤー以外）
            let move_buttons: Element<Message> = if !is_background {
                row![
                    button("↑")
                        .on_press(Message::LayerAction(LayerAction::MoveDown(index)))
                        .width(30),
                    button("↓")
                        .on_press(Message::LayerAction(LayerAction::MoveUp(index)))
                        .width(30),
                ]
                .spacing(5)
                .into()
            } else {
                Space::with_width(0).into()
            };

            // レイヤー項目全体
            let layer_item = container(
                column![
                    row![
                        visibility_checkbox,
                        layer_button,
                        move_buttons,
                    ]
                    .spacing(5)
                    .align_items(iced::Alignment::Center),
                    opacity_control,
                ]
                .spacing(5)
            )
            .style(move |_theme: &Theme| {
                container::Appearance {
                    background: if is_active {
                        Some(iced::Background::Color(Color::from_rgba(0.6, 0.8, 1.0, 0.3)))
                    } else {
                        Some(iced::Background::Color(Color::from_rgba(0.9, 0.9, 0.9, 0.5)))
                    },
                    border: iced::Border {
                        color: if is_active { Color::from_rgb(0.3, 0.6, 0.9) } else { Color::from_rgb(0.7, 0.7, 0.7) },
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    ..Default::default()
                }
            })
            .padding(8)
            .width(Length::Fill);

            layer_list = layer_list.push(layer_item);
        }

        // スクロール可能なレイヤーリスト
        let scrollable_layers = scrollable(layer_list)
            .height(300);

        column![
            text("レイヤー").size(20),
            layer_buttons,
            Space::with_height(10),
            scrollable_layers,
            Space::with_height(10),
            text(format!("レイヤー数: {}", self.layer_manager.layer_count())).size(12),
            text(format!("アクティブ: {}", layers.get(active_index).map_or("なし".to_string(), |l| l.name.clone()))).size(12),
        ]
        .spacing(5)
        .padding(10)
        .into()
    }

    fn create_canvas(&self) -> Element<Message> {
        // キャンバスを明確に区別するための境界線付きコンテナ
        container(
            canvas(PaintCanvas::new(&self.paint_engine, &self.layer_manager, &self.tools))
                .width(Length::Fill)
                .height(Length::Fill)
        )
        .style(|_theme: &Theme| {
            container::Appearance {
                background: Some(iced::Background::Color(Color::WHITE)),
                border: iced::Border {
                    color: Color::from_rgb(0.7, 0.7, 0.7),
                    width: 2.0,
                    radius: 4.0.into(),
                },
                shadow: iced::Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                    offset: iced::Vector::new(2.0, 2.0),
                    blur_radius: 4.0,
                },
                ..Default::default()
            }
        })
        .padding(8)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}