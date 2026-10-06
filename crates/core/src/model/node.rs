use crate::{AppError, model::node_socket::NodeSocket};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_gain_and_sink_pipeline_produces_expected_signal() {
        let source = Node::source("source", vec![1.0, 2.0, 3.0]);
        let gain = Node::gain("gain", 2.0);
        let sink = Node::sink("sink");

        let source_output = source.process(&[]).unwrap();
        let gain_output = gain.process(&[source_output.clone()]).unwrap();
        let sink_output = sink.process(&[gain_output.clone()]).unwrap();

        assert_eq!(source_output, vec![1.0, 2.0, 3.0]);
        assert_eq!(gain_output, vec![2.0, 4.0, 6.0]);
        assert_eq!(sink_output, vec![2.0, 4.0, 6.0]);
    }

    #[test]
    fn add_node_sums_multiple_inputs() {
        let add = Node::add("add");

        let result = add
            .process(&[vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]])
            .unwrap();

        assert_eq!(result, vec![9.0, 12.0]);
    }
}

// Node structure for the core of the application. It contains the main logic and
// data structures for the application.
pub struct Node {
    pub id: String,
    pub name: String,
    pub inputs: Vec<NodeSocket>,
    pub outputs: Vec<NodeSocket>,
    pub kind: NodeKind,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NodeKind {
    Source { samples: Vec<f32> },
    Gain { factor: f32 },
    Add,
    Sink,
}

impl Node {
    pub fn source(name: &str, samples: Vec<f32>) -> Self {
        Self {
            id: name.to_string(),
            name: name.to_string(),
            inputs: vec![],
            outputs: vec![NodeSocket::output("out")],
            kind: NodeKind::Source { samples },
        }
    }

    pub fn gain(name: &str, factor: f32) -> Self {
        Self {
            id: name.to_string(),
            name: name.to_string(),
            inputs: vec![NodeSocket::input("in")],
            outputs: vec![NodeSocket::output("out")],
            kind: NodeKind::Gain { factor },
        }
    }

    pub fn sink(name: &str) -> Self {
        Self {
            id: name.to_string(),
            name: name.to_string(),
            inputs: vec![NodeSocket::input("in")],
            outputs: vec![NodeSocket::output("out")],
            kind: NodeKind::Sink,
        }
    }

    pub fn add(name: &str) -> Self {
        Self {
            id: name.to_string(),
            name: name.to_string(),
            inputs: vec![NodeSocket::input("a"), NodeSocket::input("b")],
            outputs: vec![NodeSocket::output("out")],
            kind: NodeKind::Add,
        }
    }

    pub fn process(&self, inputs: &[Vec<f32>]) -> Result<Vec<f32>, AppError> {
        match &self.kind {
            NodeKind::Source { samples } => Ok(samples.clone()),
            NodeKind::Gain { factor } => {
                let input = inputs.first().cloned().ok_or_else(|| {
                    AppError::InvalidInput(format!("gain node '{}' has no input", self.name))
                })?;
                Ok(input.iter().map(|value| value * factor).collect())
            }
            NodeKind::Add => {
                if inputs.is_empty() {
                    return Err(AppError::InvalidInput(format!(
                        "add node '{}' has no inputs",
                        self.name
                    )));
                }

                let mut sum = inputs[0].clone();
                for values in inputs.iter().skip(1) {
                    if values.len() != sum.len() {
                        return Err(AppError::InvalidInput(format!(
                            "add node '{}' received vectors of different lengths",
                            self.name
                        )));
                    }
                    for (index, value) in values.iter().enumerate() {
                        sum[index] += value;
                    }
                }
                Ok(sum)
            }
            NodeKind::Sink => inputs.first().cloned().ok_or_else(|| {
                AppError::InvalidInput(format!("sink node '{}' has no input", self.name))
            }),
        }
    }
}
