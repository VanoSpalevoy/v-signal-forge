// Structure for a node socket in the core of the application. It contains the main
// logic and data structures for the node socket.
pub struct NodeSocket {
    pub name: String,
    pub socket_type: String,
    pub target_signal_type: String,
    pub connected_socket: Option<Box<NodeSocket>>,
}