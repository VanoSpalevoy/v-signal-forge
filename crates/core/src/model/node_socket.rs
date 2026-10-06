#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SocketDirection {
    Input,
    Output,
}

// Structure for a node socket in the core of the application. It contains the main
// logic and data structures for the node socket.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeSocket {
    pub name: String,
    pub direction: SocketDirection,
}

impl NodeSocket {
    pub fn input(name: &str) -> Self {
        Self {
            name: name.to_string(),
            direction: SocketDirection::Input,
        }
    }

    pub fn output(name: &str) -> Self {
        Self {
            name: name.to_string(),
            direction: SocketDirection::Output,
        }
    }
}
