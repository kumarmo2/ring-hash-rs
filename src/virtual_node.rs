use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::{borrow::Borrow, sync::Arc};

use crate::{node::Node, utils};

#[derive(Debug)]
pub struct VirtualNode<'id, T>
where
    T: Eq + Hash + ?Sized,
{
    pub node: Arc<Node<'id, T>>,
    pub id_per_node: usize,
    pub(crate) hash: String,
}

impl<'id, T> Borrow<str> for VirtualNode<'id, T>
where
    T: Eq + Hash + ?Sized,
{
    fn borrow(&self) -> &str {
        self.hash.as_str()
    }
}

impl<'id, T> VirtualNode<'id, T>
where
    T: Eq + Hash + ?Sized,
{
    pub(crate) fn from_node(node: Arc<Node<'id, T>>, id_per_node: usize) -> Result<Self, String> {
        // TODO: check if the default hasher can be re-used multiple time.
        let mut hasher = DefaultHasher::new();
        node.as_ref().identifier.hash(&mut hasher);
        let hashed = hasher.finish();
        let mut identifier = hashed.to_string();
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

impl<'id, T> PartialEq for VirtualNode<'id, T>
where
    T: Eq + Hash + ?Sized,
{
    fn eq(&self, other: &Self) -> bool {
        self.hash.eq(&other.hash)
    }
}

impl<'id, T> Eq for VirtualNode<'id, T> where T: Eq + Hash + ?Sized {}

impl<'id, T> PartialOrd for VirtualNode<'id, T>
where
    T: Eq + Hash + ?Sized,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.hash.cmp(&other.hash))
    }
}

impl<'id, T> Ord for VirtualNode<'id, T>
where
    T: Eq + Hash + ?Sized,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hash.cmp(&other.hash)
    }
}
