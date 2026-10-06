pub mod error;
pub mod model;
mod traits;
// mod real_time_processing;
// mod async_processing;
pub use crate::error::AppError;
pub use model::{Node, NodeKind, NodeSocket, SampleBlock, SocketDirection};

// #[cfg(test)]
