use eframe::egui;

use super::{node_color, panel_frame};
use crate::editor::{NodeKind, SignalForgeApp};

pub(super) fn show(app: &mut SignalForgeApp, ui: &mut egui::Ui) {
    egui::Panel::left("palette")
        .resizable(false)
        .frame(panel_frame(super::PANEL_BG))
        .show(ui, |ui| {
            ui.set_min_width(120.0);
            ui.set_max_width(120.0);
            ui.vertical(|ui| {
                ui.heading(app.language.text(crate::editor::Text::Nodes));
                ui.separator();
                for kind in [NodeKind::Source, NodeKind::Gain, NodeKind::Sink] {
                    let response = ui.add_sized(
                        [ui.available_width(), 36.0],
                        egui::Button::new(
                            egui::RichText::new(kind.label(app.language)).color(super::TEXT),
                        )
                        .fill(node_color(kind))
                        .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE))
                        .sense(egui::Sense::click_and_drag()),
                    );

                    if response.drag_started() {
                        app.dragged_palette_kind = Some(kind);
                    }

                    if app.dragged_palette_kind == Some(kind) {
                        ui.ctx().output_mut(|output| {
                            output.cursor_icon = egui::CursorIcon::Grab;
                        });
                    }
                }
            });
        });
}
