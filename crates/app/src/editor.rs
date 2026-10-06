use eframe::egui;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeKind {
    Source,
    Gain,
    Sink,
}

impl NodeKind {
    fn label(self) -> &'static str {
        match self {
            Self::Source => "Source",
            Self::Gain => "Gain",
            Self::Sink => "Sink",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SocketKind {
    Input,
    Output,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SocketRef {
    node_id: usize,
    side: SocketKind,
}

#[derive(Clone, Debug, PartialEq)]
struct Connection {
    from: SocketRef,
    to: SocketRef,
}

#[derive(Clone, Debug)]
struct NodeWidget {
    id: usize,
    label: String,
    kind: NodeKind,
    pos: egui::Pos2,
}

impl NodeWidget {
    fn new(id: usize, kind: NodeKind, pos: egui::Pos2) -> Self {
        Self {
            id,
            label: format!("{}{}", kind.label().to_lowercase(), id),
            kind,
            pos,
        }
    }

    fn rect(&self) -> egui::Rect {
        egui::Rect::from_min_size(self.pos, egui::vec2(160.0, 90.0))
    }

    fn input_pos(&self) -> egui::Pos2 {
        egui::pos2(self.pos.x, self.pos.y + 45.0)
    }

    fn output_pos(&self) -> egui::Pos2 {
        egui::pos2(self.pos.x + 160.0, self.pos.y + 45.0)
    }
}

pub struct SignalForgeApp {
    nodes: Vec<NodeWidget>,
    connections: Vec<Connection>,
    next_id: usize,
    pending_source: Option<SocketRef>,
}

impl Default for SignalForgeApp {
    fn default() -> Self {
        let mut app = Self {
            nodes: Vec::new(),
            connections: Vec::new(),
            next_id: 0,
            pending_source: None,
        };

        app.nodes.push(NodeWidget::new(app.next_id, NodeKind::Source, egui::pos2(80.0, 120.0)));
        app.next_id += 1;
        app.nodes.push(NodeWidget::new(app.next_id, NodeKind::Sink, egui::pos2(420.0, 120.0)));
        app.next_id += 1;
        app
    }
}

impl SignalForgeApp {
    fn add_node(&mut self, kind: NodeKind) {
        let x = 60.0 + (self.nodes.len() as f32 * 80.0) % 320.0;
        let y = 60.0 + (self.nodes.len() as f32 * 50.0) % 220.0;
        self.nodes.push(NodeWidget::new(self.next_id, kind, egui::pos2(x, y)));
        self.next_id += 1;
    }

    fn socket_rect(node: &NodeWidget, side: SocketKind) -> egui::Rect {
        let center = match side {
            SocketKind::Input => node.input_pos(),
            SocketKind::Output => node.output_pos(),
        };
        egui::Rect::from_center_size(center, egui::vec2(12.0, 12.0))
    }

    fn draw_wire(painter: &egui::Painter, from: &NodeWidget, to: &NodeWidget, from_side: SocketKind, to_side: SocketKind) {
        let start = match from_side {
            SocketKind::Input => from.input_pos(),
            SocketKind::Output => from.output_pos(),
        };
        let end = match to_side {
            SocketKind::Input => to.input_pos(),
            SocketKind::Output => to.output_pos(),
        };
        painter.line_segment([start, end], egui::Stroke::new(2.0, egui::Color32::from_rgb(130, 200, 255)));
    }

    fn render(&mut self, ui: &mut egui::Ui) {
        let toolbar = egui::Frame::default()
            .fill(egui::Color32::from_rgb(10, 12, 16))
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(30, 30, 32)))
            .corner_radius(8.0)
            .inner_margin(egui::Margin::same(8));

        toolbar.show(ui, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Add Source").clicked() {
                    self.add_node(NodeKind::Source);
                }
                if ui.button("Add Gain").clicked() {
                    self.add_node(NodeKind::Gain);
                }
                if ui.button("Add Sink").clicked() {
                    self.add_node(NodeKind::Sink);
                }
            });
        });

        ui.separator();

        let available = ui.available_size();
        let canvas_height = (available.y * 0.72).max(240.0);
        let debug_height = 120.0;

        let canvas_rect = egui::Rect::from_min_size(
            ui.min_rect().min + egui::vec2(0.0, 40.0),
            egui::vec2(available.x, canvas_height),
        );
        let debug_rect = egui::Rect::from_min_size(
            egui::pos2(ui.min_rect().min.x, ui.min_rect().max.y - debug_height),
            egui::vec2(available.x, debug_height),
        );

        let canvas_response = ui.allocate_rect(canvas_rect, egui::Sense::hover());
        let painter = ui.painter_at(canvas_rect);
        painter.rect_filled(canvas_rect, 8.0, egui::Color32::from_rgb(20, 24, 32));

        for connection in &self.connections {
            if let Some(from_node) = self.nodes.iter().find(|node| node.id == connection.from.node_id) {
                if let Some(to_node) = self.nodes.iter().find(|node| node.id == connection.to.node_id) {
                    Self::draw_wire(
                        &painter,
                        from_node,
                        to_node,
                        connection.from.side,
                        connection.to.side,
                    );
                }
            }
        }

        for node_index in 0..self.nodes.len() {
            let node = &mut self.nodes[node_index];
            let rect = node.rect();
            let drag_response = ui.allocate_rect(rect, egui::Sense::drag());
            if drag_response.dragged() {
                node.pos += drag_response.drag_delta();
            }

            let color = match node.kind {
                NodeKind::Source => egui::Color32::from_rgb(75, 142, 88),
                NodeKind::Gain => egui::Color32::from_rgb(110, 92, 188),
                NodeKind::Sink => egui::Color32::from_rgb(165, 90, 72),
            };

            let input_rect = Self::socket_rect(node, SocketKind::Input);
            let output_rect = Self::socket_rect(node, SocketKind::Output);

            let input_response = ui.allocate_rect(input_rect, egui::Sense::click());
            let output_response = ui.allocate_rect(output_rect, egui::Sense::click());

            let painter = ui.painter();
            painter.rect_filled(rect, 10.0, color);
            painter.rect_stroke(
                rect,
                10.0,
                egui::Stroke::new(1.2, egui::Color32::WHITE),
                egui::StrokeKind::Inside,
            );
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                node.label.as_str(),
                egui::FontId::proportional(15.0),
                egui::Color32::WHITE,
            );
            painter.circle_filled(input_rect.center(), 7.0, egui::Color32::WHITE);
            painter.circle_filled(output_rect.center(), 7.0, egui::Color32::WHITE);

            if output_response.clicked() {
                self.pending_source = Some(SocketRef { node_id: node.id, side: SocketKind::Output });
            }

            if input_response.clicked() {
                if let Some(from) = self.pending_source {
                    if from.node_id != node.id {
                        if from.side == SocketKind::Output {
                            self.connections.push(Connection {
                                from,
                                to: SocketRef { node_id: node.id, side: SocketKind::Input },
                            });
                        }
                    }
                    self.pending_source = None;
                }
            }
        }

        ui.allocate_rect(debug_rect, egui::Sense::hover());
        let debug_frame = egui::Frame::default()
            .fill(egui::Color32::from_rgb(10, 12, 16))
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(30, 30, 32)))
            .corner_radius(8.0)
            .inner_margin(egui::Margin::same(8));

        debug_frame.show(ui, |ui| {
            let debug_painter = ui.painter();
            let debug_area = ui.max_rect();
            debug_painter.text(
                debug_area.left_top() + egui::vec2(4.0, 4.0),
                egui::Align2::LEFT_TOP,
                "Debug",
                egui::FontId::proportional(13.0),
                egui::Color32::LIGHT_GRAY,
            );

            let debug_contents = if let Some(source) = self.pending_source {
                format!("Selected source socket: node {}", source.node_id)
            } else {
                "Ready".to_string()
            };

            debug_painter.text(
                debug_area.left_top() + egui::vec2(4.0, 26.0),
                egui::Align2::LEFT_TOP,
                &debug_contents,
                egui::FontId::proportional(12.5),
                egui::Color32::WHITE,
            );

            if !self.connections.is_empty() {
                let lines = self
                    .connections
                    .iter()
                    .map(|connection| {
                        format!(
                            "{} {:?} -> {} {:?}",
                            connection.from.node_id,
                            connection.from.side,
                            connection.to.node_id,
                            connection.to.side,
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                debug_painter.text(
                    debug_area.left_top() + egui::vec2(4.0, 50.0),
                    egui::Align2::LEFT_TOP,
                    &lines,
                    egui::FontId::proportional(12.0),
                    egui::Color32::WHITE,
                );
            }
        });

        let _ = canvas_response;
    }
}

impl eframe::App for SignalForgeApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.render(ui);
    }
}
