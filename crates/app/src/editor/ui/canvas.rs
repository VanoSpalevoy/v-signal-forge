use eframe::egui;

use super::{node_color, panel_frame};
use crate::editor::{Connection, NodeWidget, SignalForgeApp, SocketKind, SocketRef};

const NODE_SIZE: egui::Vec2 = egui::Vec2::new(160.0, 90.0);

pub(super) fn show(app: &mut SignalForgeApp, ui: &mut egui::Ui) {
    egui::CentralPanel::default()
        .frame(panel_frame(super::CANVAS_BG))
        .show(ui, |ui| {
            let canvas_rect = ui.max_rect();
            let pointer = ui.ctx().pointer_hover_pos();
            finish_palette_drop(app, ui, canvas_rect, pointer);
            ui.painter().rect_filled(canvas_rect, 8.0, super::CANVAS_BG);

            handle_node_interactions(app, ui);

            let painter = ui.painter();
            paint_palette_preview(app, painter, canvas_rect, pointer);
            paint_connections(&app.connections, &app.nodes, painter);
            paint_nodes(&app.nodes, painter, app.language);
        });
}

fn finish_palette_drop(
    app: &mut SignalForgeApp,
    ui: &egui::Ui,
    canvas_rect: egui::Rect,
    pointer: Option<egui::Pos2>,
) {
    if let Some(kind) = app.dragged_palette_kind {
        if ui.ctx().input(|input| input.pointer.any_released()) {
            if let Some(pos) = pointer.filter(|pos| canvas_rect.contains(*pos)) {
                app.add_node_at(kind, pos - NODE_SIZE * 0.5);
            }
            app.dragged_palette_kind = None;
        }
    }
}

fn handle_node_interactions(app: &mut SignalForgeApp, ui: &mut egui::Ui) {
    for node_index in 0..app.nodes.len() {
        let node = &mut app.nodes[node_index];
        let drag_response = ui.allocate_rect(node.rect(), egui::Sense::drag());
        if drag_response.dragged() {
            node.pos += drag_response.drag_delta();
        }

        let input_rect = socket_rect(node, SocketKind::Input);
        let output_rect = socket_rect(node, SocketKind::Output);
        let input_response = ui.allocate_rect(input_rect, egui::Sense::click());
        let output_response = ui.allocate_rect(output_rect, egui::Sense::click());

        if output_response.clicked_by(egui::PointerButton::Primary) {
            app.pending_source = Some(SocketRef {
                node_id: node.id,
                side: SocketKind::Output,
            });
        }

        if input_response.clicked_by(egui::PointerButton::Primary) {
            if let Some(from) = app.pending_source {
                if from.node_id != node.id && from.side == SocketKind::Output {
                    app.connections.push(Connection {
                        from,
                        to: SocketRef {
                            node_id: node.id,
                            side: SocketKind::Input,
                        },
                    });
                }
                app.pending_source = None;
            }
        }

        if output_response.clicked_by(egui::PointerButton::Secondary) {
            app.connections.retain(|connection| {
                !(connection.from.node_id == node.id && connection.from.side == SocketKind::Output)
            });
        }

        if input_response.clicked_by(egui::PointerButton::Secondary) {
            app.connections.retain(|connection| {
                !(connection.to.node_id == node.id && connection.to.side == SocketKind::Input)
            });
        }
    }
}

fn paint_palette_preview(
    app: &SignalForgeApp,
    painter: &egui::Painter,
    canvas_rect: egui::Rect,
    pointer: Option<egui::Pos2>,
) {
    if let (Some(kind), Some(pos)) = (app.dragged_palette_kind, pointer) {
        if canvas_rect.contains(pos) {
            let preview_rect = egui::Rect::from_center_size(pos, NODE_SIZE);
            painter.rect_filled(preview_rect, 10.0, node_color(kind).gamma_multiply(0.65));
            painter.rect_stroke(
                preview_rect,
                10.0,
                egui::Stroke::new(1.2, egui::Color32::WHITE),
                egui::StrokeKind::Inside,
            );
            painter.text(
                preview_rect.center(),
                egui::Align2::CENTER_CENTER,
                kind.label(app.language),
                egui::FontId::proportional(15.0),
                egui::Color32::WHITE,
            );
        }
    }
}

fn paint_connections(connections: &[Connection], nodes: &[NodeWidget], painter: &egui::Painter) {
    for connection in connections {
        let (Some(from_node), Some(to_node)) = (
            nodes.iter().find(|node| node.id == connection.from.node_id),
            nodes.iter().find(|node| node.id == connection.to.node_id),
        ) else {
            continue;
        };

        let start = socket_pos(from_node, connection.from.side);
        let end = socket_pos(to_node, connection.to.side);
        let handle = (end.x - start.x).abs() * 0.5;
        let control1 = start + egui::vec2(handle, 0.0);
        let control2 = end - egui::vec2(handle, 0.0);

        painter.add(egui::epaint::CubicBezierShape::from_points_stroke(
            [start, control1, control2, end],
            false,
            egui::Color32::TRANSPARENT,
            egui::Stroke::new(2.0, egui::Color32::from_rgb(130, 200, 255)),
        ));
    }
}

fn paint_nodes(nodes: &[NodeWidget], painter: &egui::Painter, language: super::Language) {
    for node in nodes {
        let rect = node.rect();
        painter.rect_filled(rect, 10.0, node_color(node.kind));
        painter.rect_stroke(
            rect,
            10.0,
            egui::Stroke::new(1.2, egui::Color32::WHITE),
            egui::StrokeKind::Inside,
        );
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            format!("{}{}", node.kind.label(language), node.id),
            egui::FontId::proportional(15.0),
            egui::Color32::WHITE,
        );
        painter.circle_filled(
            socket_pos(node, SocketKind::Input),
            7.0,
            egui::Color32::WHITE,
        );
        painter.circle_filled(
            socket_pos(node, SocketKind::Output),
            7.0,
            egui::Color32::WHITE,
        );
    }
}

fn socket_pos(node: &NodeWidget, side: SocketKind) -> egui::Pos2 {
    match side {
        SocketKind::Input => node.input_pos(),
        SocketKind::Output => node.output_pos(),
    }
}

fn socket_rect(node: &NodeWidget, side: SocketKind) -> egui::Rect {
    egui::Rect::from_center_size(socket_pos(node, side), egui::vec2(12.0, 12.0))
}
