#[derive(Debug, PartialEq, Eq)]
pub enum RingError {
    NodeAlreadyExists,
    InvalidVirtualNodesPerNode,
}
