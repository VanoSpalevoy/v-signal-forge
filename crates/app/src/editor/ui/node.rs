use eframe::egui;

use super::super::localization::{Language, Text};

const NODE_WIDTH: f32 = 160.0;
const MIN_NODE_HEIGHT: f32 = 90.0;
const ROW_HEIGHT: f32 = 22.0;
const CONTENT_TOP: f32 = 38.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeKind {
    Source,
    Gain,
    Mixer,
    Sink,
}

impl NodeKind {
    pub(super) fn label(self, language: Language) -> &'static str {
        match self {
            Self::Source => language.text(Text::Source),
            Self::Gain => language.text(Text::Gain),
            Self::Mixer => language.text(Text::Mixer),
            Self::Sink => language.text(Text::Sink),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SocketKind {
    Input,
    Output,
}

impl SocketKind {
    pub(super) fn label(self, language: Language) -> &'static str {
        match self {
            Self::Input => language.text(Text::Input),
            Self::Output => language.text(Text::Output),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SocketRef {
    pub(super) node_id: usize,
    pub(super) side: SocketKind,
    pub(super) index: usize,
}

#[derive(Clone, Debug)]
pub struct NodeConfig {
    pub(super) kind: NodeKind,
    pub(super) title: String,
    pub(super) inputs: Vec<String>,
    pub(super) outputs: Vec<String>,
    pub(super) content: Vec<String>,
}

impl NodeConfig {
    pub(in crate::editor) fn for_kind(kind: NodeKind, language: Language) -> Self {
        let (inputs, outputs, content) = match kind {
            NodeKind::Source => (vec![], vec!["Audio"], vec!["Signal: sine"]),
            NodeKind::Gain => (vec!["Audio in"], vec!["Audio out"], vec!["Gain: 1.0"]),
            NodeKind::Mixer => (
                vec!["Left", "Right", "Side"],
                vec!["Mixed"],
                vec!["Mode: stereo", "Gain: 1.0", "Pan: center"],
            ),
            NodeKind::Sink => (vec!["Audio"], vec![], vec!["Channels: stereo"]),
        };

        Self {
            kind,
            title: kind.label(language).to_owned(),
            inputs: inputs.into_iter().map(str::to_owned).collect(),
            outputs: outputs.into_iter().map(str::to_owned).collect(),
            content: content.into_iter().map(str::to_owned).collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct NodeWidget {
    pub(super) id: usize,
    pub(super) config: NodeConfig,
    pub(super) pos: egui::Pos2,
}

impl NodeWidget {
    pub(in crate::editor) fn new(id: usize, config: NodeConfig, pos: egui::Pos2) -> Self {
        Self { id, config, pos }
    }

    pub(super) fn rect(&self) -> egui::Rect {
        egui::Rect::from_min_size(self.pos, egui::vec2(NODE_WIDTH, self.height()))
    }

    pub(super) fn sockets(&self, side: SocketKind) -> &[String] {
        match side {
            SocketKind::Input => &self.config.inputs,
            SocketKind::Output => &self.config.outputs,
        }
    }

    pub(super) fn socket_pos(&self, side: SocketKind, index: usize) -> egui::Pos2 {
        let sockets = self.sockets(side);
        let row = if index < sockets.len() { index } else { 0 };
        egui::pos2(
            match side {
                SocketKind::Input => self.pos.x,
                SocketKind::Output => self.pos.x + NODE_WIDTH,
            },
            self.pos.y + CONTENT_TOP + ROW_HEIGHT * (row as f32 + 0.5),
        )
        .clamp(
            egui::pos2(self.pos.x, self.pos.y),
            egui::pos2(self.pos.x + NODE_WIDTH, self.pos.y + self.height()),
        )
    }

    fn height(&self) -> f32 {
        let row_count = self
            .config
            .inputs
            .len()
            .max(self.config.outputs.len())
            .max(self.config.content.len())
            .max(1);
        MIN_NODE_HEIGHT.max(CONTENT_TOP + ROW_HEIGHT * row_count as f32 + 8.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_geometry_expands_for_custom_sockets_and_content() {
        let config = NodeConfig::for_kind(NodeKind::Mixer, Language::English);
        let node = NodeWidget::new(0, config, egui::Pos2::ZERO);

        assert!(node.rect().height() > MIN_NODE_HEIGHT);
        assert_eq!(node.config.inputs.len(), 3);
        assert_eq!(node.config.outputs.len(), 1);
        assert_ne!(
            node.socket_pos(SocketKind::Input, 0),
            node.socket_pos(SocketKind::Input, 2),
        );
    }
}
