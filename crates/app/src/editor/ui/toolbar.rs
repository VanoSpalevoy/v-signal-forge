// Toolbar for the editor

use eframe::egui;

use super::{Language, panel_frame};
use crate::editor::{SignalForgeApp, Text};

pub(super) fn show(app: &mut SignalForgeApp, ui: &mut egui::Ui) {
    egui::Panel::top("toolbar")
        .resizable(false)
        .frame(panel_frame(super::PANEL_BG))
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.set_min_height(34.0);
                ui.add_space(12.0);
                ui.label(app.language.text(Text::Tools));
                ui.add_space(12.0);
                if ui.button(app.language.text(Text::Format)).clicked() {
                    for node in &mut app.nodes {
                        node.pos.x = (node.pos.x / 100.0).round() * 100.0;
                        node.pos.y = (node.pos.y / 100.0).round() * 100.0;
                    }
                }
                ui.separator();
                ui.label(app.language.text(Text::Settings));
                ui.add_space(12.0);
                ui.label(app.language.text(Text::Language));
                egui::ComboBox::from_id_salt("language")
                    .selected_text(app.language.native_name())
                    .show_ui(ui, |ui| {
                        for language in Language::ALL {
                            ui.selectable_value(
                                &mut app.language,
                                language,
                                language.native_name(),
                            );
                        }
                    });
            });
        });
}
