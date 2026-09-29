use crate::model::node_socket::NodeSocket;

// Node structure for the core of the application. It contains the main logic and
// data structures for the application.
pub struct Node {
    pub name: String,
    pub inputs: Vec<NodeSocket>,
    pub outputs: Vec<NodeSocket>,
}