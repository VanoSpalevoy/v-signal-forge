use eframe::egui;

mod localization;
mod ui;

use localization::{Language, Text};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeKind {
    Source,
    Gain,
    Sink,
}

impl NodeKind {
    fn label(self, language: Language) -> &'static str {
        match self {
            Self::Source => language.text(Text::Source),
            Self::Gain => language.text(Text::Gain),
            Self::Sink => language.text(Text::Sink),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SocketKind {
    Input,
    Output,
}

impl SocketKind {
    fn label(self, language: Language) -> &'static str {
        match self {
            Self::Input => language.text(Text::Input),
            Self::Output => language.text(Text::Output),
        }
    }
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
    kind: NodeKind,
    pos: egui::Pos2,
}

impl NodeWidget {
    fn new(id: usize, kind: NodeKind, pos: egui::Pos2) -> Self {
        Self { id, kind, pos }
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
    dragged_palette_kind: Option<NodeKind>,
    language: Language,
}

impl Default for SignalForgeApp {
    fn default() -> Self {
        let mut app = Self {
            nodes: Vec::new(),
            connections: Vec::new(),
            next_id: 0,
            pending_source: None,
            dragged_palette_kind: None,
            language: Language::English,
        };

        app.nodes.push(NodeWidget::new(
            app.next_id,
            NodeKind::Source,
            egui::pos2(80.0, 120.0),
        ));
        app.next_id += 1;
        app.nodes.push(NodeWidget::new(
            app.next_id,
            NodeKind::Sink,
            egui::pos2(420.0, 120.0),
        ));
        app.next_id += 1;
        app
    }
}

impl SignalForgeApp {
    fn add_node_at(&mut self, kind: NodeKind, pos: egui::Pos2) {
        self.nodes.push(NodeWidget::new(self.next_id, kind, pos));
        self.next_id += 1;
    }

    fn render(&mut self, ui: &mut egui::Ui) {
        ui::render(self, ui);
    }
}

impl eframe::App for SignalForgeApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.render(ui);
    }
}
