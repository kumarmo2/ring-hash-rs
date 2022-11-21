use std::{borrow::Borrow, rc::Rc};

use crate::{node::Node, utils};

#[derive(Debug)]
pub struct VirtualNode {
    pub node: Rc<Node>,
    pub id_per_node: usize,
    pub(crate) hash: String,
}

impl Borrow<str> for VirtualNode {
    fn borrow(&self) -> &str {
        self.hash.as_str()
    }
}

impl VirtualNode {
    pub(crate) fn from_node(node: Rc<Node>, id_per_node: usize) -> Result<Self, String> {
        let mut identifier = String::to_owned(&node.as_ref().identifier);
        identifier.push('-');
        identifier.push_str(&id_per_node.to_string());

        let hash = utils::get_hash(&identifier);

        Ok(Self {
            node,
            id_per_node,
            hash,
        })
    }
}

// TODO: understand the below traits: Eq, PartialOrd, PartialEq, Ord

impl PartialEq for VirtualNode {
    fn eq(&self, other: &Self) -> bool {
        self.hash.eq(&other.hash)
    }
}

impl Eq for VirtualNode {}

impl PartialOrd for VirtualNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.hash.cmp(&other.hash))
    }
}

impl Ord for VirtualNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hash.cmp(&other.hash)
    }
}
