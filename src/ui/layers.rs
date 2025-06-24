use eframe::egui;
use crate::layer::LayerManager;

#[derive(Default)]
pub struct LayerRenameState {
    pub editing_layer: Option<usize>,
    pub temp_name: String,
}

pub fn render_layer_panel(ui: &mut egui::Ui, layer_manager: &mut LayerManager, rename_state: &mut LayerRenameState) -> LayerPanelAction {
    let mut action = LayerPanelAction::None;
    
    ui.heading("レイヤー");
    ui.separator();
    
    // 新しいレイヤー追加ボタン
    ui.horizontal(|ui| {
        if ui.button("➕ 新規レイヤー").clicked() {
            action = LayerPanelAction::AddLayer;
        }
        
        if ui.button("🗑 削除").clicked() {
            if layer_manager.layer_count() > 1 {
                action = LayerPanelAction::DeleteLayer(layer_manager.active_layer_index());
            }
        }
    });
    
    ui.separator();
    
    // レイヤーリスト（上から下へ表示 = 描画順と逆）
    egui::ScrollArea::vertical()
        .max_height(300.0)
        .show(ui, |ui| {
            let layer_count = layer_manager.layer_count();
            let active_index = layer_manager.active_layer_index();
            
            for i in (0..layer_count).rev() { // 逆順で表示（上位レイヤーが上に）
                if let Some(layer) = layer_manager.get_layer(i) {
                    ui.horizontal(|ui| {
                        // アクティブレイヤーの表示
                        let is_active = i == active_index;
                        
                        // レイヤー名の表示・編集
                        if rename_state.editing_layer == Some(i) {
                            // 編集モード
                            let response = ui.text_edit_singleline(&mut rename_state.temp_name);
                            
                            if response.lost_focus() || ui.input(|input| input.key_pressed(egui::Key::Enter)) {
                                action = LayerPanelAction::RenameLayer(i, rename_state.temp_name.clone());
                                rename_state.editing_layer = None;
                            }
                            
                            if ui.input(|input| input.key_pressed(egui::Key::Escape)) {
                                rename_state.editing_layer = None;
                            }
                            
                            // 初回フォーカス
                            if response.gained_focus() {
                                response.request_focus();
                            }
                        } else {
                            // 通常表示モード
                            let button_text = if is_active {
                                format!("🎯 {}", layer.name)
                            } else {
                                format!("   {}", layer.name)
                            };
                            
                            let response = ui.selectable_label(is_active, button_text);
                            
                            if response.clicked() {
                                action = LayerPanelAction::SetActiveLayer(i);
                            }
                            
                            // ダブルクリックで名前編集開始
                            if response.double_clicked() {
                                rename_state.editing_layer = Some(i);
                                rename_state.temp_name = layer.name.clone();
                            }
                        }
                        
                        // 表示/非表示切り替え
                        let visibility_icon = if layer.visible { "👁" } else { "🙈" };
                        if ui.small_button(visibility_icon).clicked() {
                            action = LayerPanelAction::ToggleVisibility(i);
                        }
                        
                        // 移動ボタン（表示順序に合わせた論理）
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            // ⬆ボタン: より上のレイヤーになる（インデックスが増える）
                            if i < layer_count - 1 && ui.small_button("⬆").clicked() {
                                action = LayerPanelAction::MoveLayerUp(i);
                            }
                            // ⬇ボタン: より下のレイヤーになる（インデックスが減る）
                            if i > 0 && ui.small_button("⬇").clicked() {
                                action = LayerPanelAction::MoveLayerDown(i);
                            }
                        });
                    });
                    
                    // レイヤー詳細情報（アクティブレイヤーのみ）
                    if i == active_index {
                        ui.indent("layer_details", |ui| {
                            ui.small(format!("ストローク数: {}", layer.stroke_count()));
                            
                            // 不透明度スライダー
                            ui.horizontal(|ui| {
                                ui.small("不透明度:");
                                let mut opacity_percent = layer.opacity * 100.0;
                                if ui.add(egui::Slider::new(&mut opacity_percent, 0.0..=100.0)
                                    .suffix("%")
                                    .show_value(true)).changed() {
                                    action = LayerPanelAction::SetOpacity(i, opacity_percent / 100.0);
                                }
                            });
                        });
                    }
                    
                    ui.separator();
                }
            }
        });
    
    // 統計情報
    ui.separator();
    ui.small(format!("総レイヤー数: {}", layer_manager.layer_count()));
    ui.small(format!("総ストローク数: {}", layer_manager.total_stroke_count()));
    
    action
}

#[derive(Debug, Clone)]
pub enum LayerPanelAction {
    None,
    AddLayer,
    DeleteLayer(usize),
    SetActiveLayer(usize),
    ToggleVisibility(usize),
    MoveLayerUp(usize),
    MoveLayerDown(usize),
    RenameLayer(usize, String),
    SetOpacity(usize, f32),
}

// レイヤー名編集ダイアログ（将来の実装用）
#[allow(dead_code)]
pub fn show_layer_rename_dialog(
    _ctx: &egui::Context,
    _is_open: &mut bool,
    _layer_name: &mut String,
) -> bool {
    // 将来の実装用のプレースホルダー
    false
}

// レイヤー操作のためのヘルパー関数
impl LayerManager {
    pub fn handle_layer_action(&mut self, action: LayerPanelAction) {
        match action {
            LayerPanelAction::None => {}
            LayerPanelAction::AddLayer => {
                let layer_count = self.layer_count();
                let new_name = format!("Layer {}", layer_count + 1);
                self.add_layer(new_name);
            }
            LayerPanelAction::DeleteLayer(index) => {
                self.remove_layer(index);
            }
            LayerPanelAction::SetActiveLayer(index) => {
                self.set_active_layer(index);
            }
            LayerPanelAction::ToggleVisibility(index) => {
                if let Some(layer) = self.get_layer_mut(index) {
                    layer.set_visible(!layer.visible);
                }
            }
            LayerPanelAction::MoveLayerUp(index) => {
                self.move_layer_up(index);
            }
            LayerPanelAction::MoveLayerDown(index) => {
                self.move_layer_down(index);
            }
            LayerPanelAction::RenameLayer(index, new_name) => {
                if let Some(layer) = self.get_layer_mut(index) {
                    layer.set_name(new_name);
                }
            }
            LayerPanelAction::SetOpacity(index, opacity) => {
                if let Some(layer) = self.get_layer_mut(index) {
                    layer.set_opacity(opacity);
                }
            }
        }
    }
}