use eframe::egui;

use super::panel_frame;
use crate::editor::{SignalForgeApp, Text};

pub(super) fn show(app: &SignalForgeApp, ui: &mut egui::Ui) {
    egui::Panel::bottom("debug")
        .resizable(false)
        .frame(panel_frame(super::PANEL_BG))
        .show(ui, |ui| {
            ui.vertical(|ui| {
                ui.label(app.language.text(Text::Debug));
                if let Some(source) = app.pending_source {
                    ui.label(format!(
                        "{} {} {}",
                        app.language.text(Text::SelectedSourceSocket),
                        source.node_id,
                        source.side.label(app.language),
                    ));
                } else {
                    ui.label(app.language.text(Text::Ready));
                }
                for (to, from) in &app.connections {
                    ui.label(format!(
                        "{} {:?} -> {} {:?}",
                        from.node_id,
                        from.side.label(app.language),
                        to.node_id,
                        to.side.label(app.language),
                    ));
                }
            });
        });
}
