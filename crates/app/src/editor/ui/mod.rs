mod canvas;
mod debug;
mod node;
mod palette;
mod toolbar;

use eframe::egui;

use super::SignalForgeApp;
pub(super) use super::localization::Language;
pub use node::{NodeConfig, NodeKind, NodeWidget, SocketKind, SocketRef};

const WINDOW_BG: egui::Color32 = egui::Color32::from_rgb(10, 12, 16);
const PANEL_BG: egui::Color32 = egui::Color32::from_rgb(14, 16, 22);
const CANVAS_BG: egui::Color32 = egui::Color32::from_rgb(20, 24, 32);
const CONTROL_BG: egui::Color32 = egui::Color32::from_rgb(31, 36, 45);
const CONTROL_HOVER: egui::Color32 = egui::Color32::from_rgb(45, 53, 65);
const TEXT: egui::Color32 = egui::Color32::from_rgb(232, 235, 240);
const BORDER: egui::Color32 = egui::Color32::from_rgb(48, 54, 64);

pub(super) fn render(app: &mut SignalForgeApp, ui: &mut egui::Ui) {
    ui.ctx().set_visuals(visuals());
    toolbar::show(app, ui);
    palette::show(app, ui);
    debug::show(app, ui);
    canvas::show(app, ui);
}

pub(super) fn panel_frame(fill: egui::Color32) -> egui::Frame {
    egui::Frame::default()
        .fill(fill)
        .stroke(egui::Stroke::new(1.0, BORDER))
        .inner_margin(egui::Margin::same(8))
}

fn visuals() -> egui::Visuals {
    let mut visuals = egui::Visuals::dark();
    visuals.window_fill = PANEL_BG;
    visuals.panel_fill = PANEL_BG;
    visuals.extreme_bg_color = WINDOW_BG;
    visuals.faint_bg_color = CANVAS_BG;
    visuals.code_bg_color = WINDOW_BG;
    visuals.override_text_color = Some(TEXT);
    visuals.widgets.noninteractive.bg_fill = PANEL_BG;
    visuals.widgets.inactive.bg_fill = CONTROL_BG;
    visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, TEXT);
    visuals.widgets.hovered.bg_fill = CONTROL_HOVER;
    visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, TEXT);
    visuals.widgets.active.bg_fill = egui::Color32::from_rgb(59, 73, 91);
    visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, TEXT);
    visuals
}

fn node_color(kind: NodeKind) -> egui::Color32 {
    match kind {
        NodeKind::Source => egui::Color32::from_rgb(75, 142, 88),
        NodeKind::Gain => egui::Color32::from_rgb(110, 92, 188),
        NodeKind::Mixer => egui::Color32::from_rgb(56, 128, 145),
        NodeKind::Sink => egui::Color32::from_rgb(165, 90, 72),
    }
}
