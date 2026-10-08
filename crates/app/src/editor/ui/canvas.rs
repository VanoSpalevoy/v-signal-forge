use eframe::egui;
use std::collections::HashMap;

use super::{node_color, panel_frame};
use crate::editor::{NodeWidget, SignalForgeApp, SocketKind, SocketRef, Text};

const NODE_SIZE: egui::Vec2 = egui::Vec2::new(160.0, 90.0);
const DELETE_BUTTON_SIZE: f32 = 22.0;

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
    let mut deleted_node_id = None;

    for node_index in 0..app.nodes.len() {
        let node = &mut app.nodes[node_index];
        let drag_response = ui.allocate_rect(node.rect(), egui::Sense::drag());
        if drag_response.dragged() {
            node.pos += drag_response.drag_delta();
        }

        let input_responses: Vec<_> = (0..node.config.inputs.len())
            .map(|index| {
                ui.allocate_rect(
                    socket_rect(node, SocketKind::Input, index),
                    egui::Sense::click(),
                )
            })
            .collect();
        let output_responses: Vec<_> = (0..node.config.outputs.len())
            .map(|index| {
                ui.allocate_rect(
                    socket_rect(node, SocketKind::Output, index),
                    egui::Sense::click(),
                )
            })
            .collect();
        let delete_response = ui
            .allocate_rect(delete_button_rect(node), egui::Sense::click())
            .on_hover_text(app.language.text(Text::DeleteNode));

        if delete_response.clicked_by(egui::PointerButton::Primary) {
            deleted_node_id = Some(node.id);
            continue;
        }

        if let Some(index) = output_responses
            .iter()
            .position(|response| response.clicked_by(egui::PointerButton::Primary))
        {
            app.pending_source = Some(SocketRef {
                node_id: node.id,
                side: SocketKind::Output,
                index,
            });
        }

        if let Some(index) = input_responses
            .iter()
            .position(|response| response.clicked_by(egui::PointerButton::Primary))
        {
            if let Some(from) = app.pending_source {
                if from.node_id != node.id && from.side == SocketKind::Output {
                    let input = SocketRef {
                        node_id: node.id,
                        side: SocketKind::Input,
                        index,
                    };
                    if app.connections.contains_key(&input) {
                        app.pending_source = None;
                        continue;
                    }
                    app.connections.insert(input, from);
                }
                app.pending_source = None;
            }
        }

        if let Some(index) = output_responses
            .iter()
            .position(|response| response.clicked_by(egui::PointerButton::Secondary))
        {
            app.connections.retain(|_, from| {
                !(from.node_id == node.id && from.side == SocketKind::Output && from.index == index)
            });
        }

        if let Some(index) = input_responses
            .iter()
            .position(|response| response.clicked_by(egui::PointerButton::Secondary))
        {
            app.connections.remove(&SocketRef {
                node_id: node.id,
                side: SocketKind::Input,
                index,
            });
        }
    }

    if let Some(node_id) = deleted_node_id {
        app.nodes.retain(|node| node.id != node_id);
        app.connections
            .retain(|input, output| input.node_id != node_id && output.node_id != node_id);
        app.free_node_ids.push(node_id);
        app.node_limit_reached = false;
        if app
            .pending_source
            .is_some_and(|source| source.node_id == node_id)
        {
            app.pending_source = None;
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

fn paint_connections(
    connections: &HashMap<SocketRef, SocketRef>,
    nodes: &[NodeWidget],
    painter: &egui::Painter,
) {
    for (to, from) in connections {
        let (Some(from_node), Some(to_node)) = (
            nodes.iter().find(|node| node.id == from.node_id),
            nodes.iter().find(|node| node.id == to.node_id),
        ) else {
            continue;
        };

        let start = socket_pos(from_node, from.side, from.index);
        let end = socket_pos(to_node, to.side, to.index);
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

fn paint_nodes(nodes: &[NodeWidget], painter: &egui::Painter, _language: super::Language) {
    for node in nodes {
        let rect = node.rect();
        painter.rect_filled(rect, 10.0, node_color(node.config.kind));
        painter.rect_stroke(
            rect,
            10.0,
            egui::Stroke::new(1.2, egui::Color32::WHITE),
            egui::StrokeKind::Inside,
        );
        painter.text(
            rect.right_top() + egui::vec2(-6.0, 6.0),
            egui::Align2::RIGHT_TOP,
            format!("{} {}", node.config.title, node.id),
            egui::FontId::proportional(13.0),
            egui::Color32::WHITE,
        );
        for (index, content) in node.config.content.iter().enumerate() {
            painter.text(
                egui::pos2(rect.center().x, rect.top() + 50.0 + index as f32 * 22.0),
                egui::Align2::CENTER_CENTER,
                content,
                egui::FontId::proportional(12.0),
                egui::Color32::WHITE,
            );
        }
        for (side, sockets) in [
            (SocketKind::Input, &node.config.inputs),
            (SocketKind::Output, &node.config.outputs),
        ] {
            for (index, label) in sockets.iter().enumerate() {
                let pos = socket_pos(node, side, index);
                painter.circle_filled(pos, 7.0, egui::Color32::WHITE);
                let align = match side {
                    SocketKind::Input => egui::Align2::LEFT_CENTER,
                    SocketKind::Output => egui::Align2::RIGHT_CENTER,
                };
                let label_pos = pos
                    + egui::vec2(
                        if side == SocketKind::Input {
                            10.0
                        } else {
                            -10.0
                        },
                        0.0,
                    );
                painter.text(
                    label_pos,
                    align,
                    label,
                    egui::FontId::proportional(10.0),
                    egui::Color32::WHITE,
                );
            }
        }

        let delete_rect = delete_button_rect(node);
        painter.rect_filled(delete_rect, 4.0, egui::Color32::from_rgb(105, 48, 48));
        painter.text(
            delete_rect.center(),
            egui::Align2::CENTER_CENTER,
            "x",
            egui::FontId::proportional(15.0),
            egui::Color32::WHITE,
        );
    }
}

fn socket_pos(node: &NodeWidget, side: SocketKind, index: usize) -> egui::Pos2 {
    node.socket_pos(side, index)
}

fn socket_rect(node: &NodeWidget, side: SocketKind, index: usize) -> egui::Rect {
    egui::Rect::from_center_size(socket_pos(node, side, index), egui::vec2(12.0, 12.0))
}

fn delete_button_rect(node: &NodeWidget) -> egui::Rect {
    egui::Rect::from_min_size(
        node.rect().left_top() + egui::vec2(6.0, 6.0),
        egui::vec2(DELETE_BUTTON_SIZE, DELETE_BUTTON_SIZE),
    )
}
