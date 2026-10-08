use eframe::egui;
use std::collections::HashMap;

mod localization;
mod ui;

use localization::{Language, Text};
use ui::NodeConfig;
pub(super) use ui::{NodeKind, NodeWidget, SocketKind, SocketRef};

pub(super) const MAX_NODE_COUNT: usize = 256;

pub struct SignalForgeApp {
    nodes: Vec<NodeWidget>,
    connections: HashMap<SocketRef, SocketRef>,
    free_node_ids: Vec<usize>,
    node_limit_reached: bool,
    pending_source: Option<SocketRef>,
    dragged_palette_kind: Option<NodeKind>,
    language: Language,
    scale: f32,
}

impl Default for SignalForgeApp {
    fn default() -> Self {
        let app = Self {
            nodes: Vec::new(),
            connections: HashMap::new(),
            free_node_ids: (0..MAX_NODE_COUNT).rev().collect(),
            node_limit_reached: false,
            pending_source: None,
            dragged_palette_kind: None,
            language: Language::English,
            scale: 1.0,
        };

        app
    }
}

impl SignalForgeApp {
    fn add_node_at(&mut self, kind: NodeKind, pos: egui::Pos2) {
        let Some(id) = self.free_node_ids.pop() else {
            self.node_limit_reached = true;
            return;
        };
        self.node_limit_reached = false;
        self.nodes.push(NodeWidget::new(
            id,
            NodeConfig::for_kind(kind, self.language),
            pos,
        ));
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
