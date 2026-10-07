use eframe::egui;
use std::collections::HashMap;

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

    fn has_input(self) -> bool {
        matches!(self, Self::Gain | Self::Sink)
    }

    fn has_output(self) -> bool {
        matches!(self, Self::Source | Self::Gain)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct SocketRef {
    node_id: usize,
    side: SocketKind,
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
    connections: HashMap<SocketRef, SocketRef>,
    free_node_ids: Vec<usize>,
    pending_source: Option<SocketRef>,
    dragged_palette_kind: Option<NodeKind>,
    language: Language,
}

impl Default for SignalForgeApp {
    fn default() -> Self {
        let app = Self {
            nodes: Vec::new(),
            connections: HashMap::new(),
            free_node_ids: (0..=255).rev().collect(),
            pending_source: None,
            dragged_palette_kind: None,
            language: Language::English,
        };

        app
    }
}

impl SignalForgeApp {
    fn add_node_at(&mut self, kind: NodeKind, pos: egui::Pos2) {
        let Some(id) = self.free_node_ids.pop() else {
            return;
        };
        self.nodes.push(NodeWidget::new(id, kind, pos));
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
