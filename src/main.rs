use eframe::egui;
use egui::*;
use std::sync::Arc;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "ペイントアプリ",
        options,
        Box::new(|cc| Ok(Box::new(PaintApp::new(cc)))),
    )
}

struct PaintApp {
    // 描画データ
    strokes: Vec<Stroke>,
    current_stroke: Option<Stroke>,
    
    // ツール設定
    brush_size: f32,
    brush_color: Color32,
    is_eraser: bool,
    
    // キャンバス設定
    #[allow(dead_code)]
    canvas_size: Vec2,
    background_color: Color32,
}

#[derive(Clone)]
struct Stroke {
    points: Vec<Pos2>,
    color: Color32,
    width: f32,
}

impl Default for PaintApp {
    fn default() -> Self {
        Self {
            strokes: Vec::new(),
            current_stroke: None,
            brush_size: 2.0,
            brush_color: Color32::BLACK,
            is_eraser: false,
            canvas_size: Vec2::new(600.0, 400.0),
            background_color: Color32::WHITE,
        }
    }
}

impl PaintApp {
    // 2点間を補間して滑らかな円で描画する
    fn draw_smooth_stroke(painter: &egui::Painter, p1: Pos2, p2: Pos2, width: f32, color: Color32) {
        let distance = (p2 - p1).length();
        let step_size = (width / 4.0).max(1.0); // ブラシサイズに応じたステップサイズ
        let steps = (distance / step_size).ceil() as usize;
        
        if steps <= 1 {
            painter.circle_filled(p2, width / 2.0, color);
        } else {
            for i in 0..=steps {
                let t = i as f32 / steps as f32;
                let interpolated_pos = p1 + t * (p2 - p1);
                painter.circle_filled(interpolated_pos, width / 2.0, color);
            }
        }
    }

    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // フォント設定（日本語対応）
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "noto_sans_jp".to_owned(),
            Arc::new(egui::FontData::from_static(include_bytes!("../fonts/NotoSansJP-Regular.ttf"))),
        );
        
        fonts.families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "noto_sans_jp".to_owned());
        
        cc.egui_ctx.set_fonts(fonts);
        
        Self::default()
    }
    
    fn clear_canvas(&mut self) {
        self.strokes.clear();
        self.current_stroke = None;
    }
}

impl eframe::App for PaintApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // トップパネル - ツールバー
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("ブラシサイズ:");
                ui.add(egui::Slider::new(&mut self.brush_size, 1.0..=20.0));
                
                ui.separator();
                
                // カラーピッカー
                ui.label("色:");
                ui.color_edit_button_srgba(&mut self.brush_color);
                
                ui.separator();
                
                // ツール選択
                ui.selectable_value(&mut self.is_eraser, false, "ペン");
                ui.selectable_value(&mut self.is_eraser, true, "消しゴム");
                
                ui.separator();
                
                if ui.button("クリア").clicked() {
                    self.clear_canvas();
                }
                
                ui.separator();
                
                // 保存/読み込みボタン（実装は省略）
                if ui.button("保存").clicked() {
                    // TODO: 画像として保存
                }
                
                if ui.button("開く").clicked() {
                    // TODO: 画像を開く
                }
            });
        });
        
        // サイドパネル - レイヤーやその他のオプション
        egui::SidePanel::left("layers").show(ctx, |ui| {
            ui.heading("レイヤー");
            ui.separator();
            
            // レイヤー機能の実装は省略
            ui.label("レイヤー1");
            
            ui.separator();
            ui.heading("背景色");
            ui.color_edit_button_srgba(&mut self.background_color);
        });
        
        // メインキャンバス
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_size = ui.available_size();
            
            // キャンバスの描画エリア
            let (response, painter) = ui.allocate_painter(
                available_size,
                Sense::drag(),
            );
            
            // 背景を描画
            painter.rect_filled(
                response.rect,
                0.0,
                self.background_color,
            );
            
            // マウス入力の処理
            if response.dragged() {
                if let Some(pos) = response.interact_pointer_pos() {
                    let canvas_pos = (pos - response.rect.min).to_pos2();
                    
                    if self.current_stroke.is_none() {
                        self.current_stroke = Some(Stroke {
                            points: vec![canvas_pos],
                            color: if self.is_eraser { 
                                self.background_color 
                            } else { 
                                self.brush_color 
                            },
                            width: self.brush_size,
                        });
                    } else if let Some(ref mut stroke) = self.current_stroke {
                        stroke.points.push(canvas_pos);
                    }
                }
            }
            
            // ドラッグが終了したらストロークを確定
            if response.drag_stopped() {
                if let Some(stroke) = self.current_stroke.take() {
                    if stroke.points.len() > 1 {
                        self.strokes.push(stroke);
                    }
                }
            }
            
            // すべてのストロークを描画
            let offset = response.rect.min.to_vec2();
            
            for stroke in &self.strokes {
                if stroke.points.len() == 1 {
                    // 単一点の場合
                    painter.circle_filled(
                        stroke.points[0] + offset,
                        stroke.width / 2.0,
                        stroke.color,
                    );
                } else if stroke.points.len() > 1 {
                    // 最初の点を描画
                    painter.circle_filled(
                        stroke.points[0] + offset,
                        stroke.width / 2.0,
                        stroke.color,
                    );
                    // 連続する点間を補間描画
                    for i in 1..stroke.points.len() {
                        Self::draw_smooth_stroke(
                            &painter,
                            stroke.points[i - 1] + offset,
                            stroke.points[i] + offset,
                            stroke.width,
                            stroke.color,
                        );
                    }
                }
            }
            
            // 現在描画中のストロークを描画
            if let Some(ref stroke) = self.current_stroke {
                if stroke.points.len() == 1 {
                    // 単一点の場合
                    painter.circle_filled(
                        stroke.points[0] + offset,
                        stroke.width / 2.0,
                        stroke.color,
                    );
                } else if stroke.points.len() > 1 {
                    // 最初の点を描画
                    painter.circle_filled(
                        stroke.points[0] + offset,
                        stroke.width / 2.0,
                        stroke.color,
                    );
                    // 連続する点間を補間描画
                    for i in 1..stroke.points.len() {
                        Self::draw_smooth_stroke(
                            &painter,
                            stroke.points[i - 1] + offset,
                            stroke.points[i] + offset,
                            stroke.width,
                            stroke.color,
                        );
                    }
                }
            }
            
            // ステータスバー
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.label(format!(
                    "キャンバスサイズ: {:.0}x{:.0} | ストローク数: {}",
                    available_size.x,
                    available_size.y,
                    self.strokes.len()
                ));
            });
        });
    }
}