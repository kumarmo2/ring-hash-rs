use std::sync::{Arc, RwLock};

use crate::{node::Node, utils, virtual_node::VirtualNode};
use std::hash::Hash;

/// Operations that mutate the Ring itself, i.e adding nodes and deleting nodes
/// are not thread safe as of now. It implies that once the ring has been setup,
/// that is adding nodes are completed, passing to multiple threads will fail to compile
/// , if we try to add/remove nodes from it. Will need some locking and interior mutability for that .
pub struct Ring<T>
where
    T: Copy + Eq + Hash + ?Sized,
{
    pub(crate) virtual_nodes: RwLock<Vec<VirtualNode<T>>>,
    // TODO: ideally `num_of_nodes` needs to be protected by a lock.
    num_of_nodes: usize,
    virtual_nodes_per_node: usize,
}
/*
 * TODO:
*  -
 * - remove heap allocations if possible
 * */

impl<T> Ring<T>
where
    T: Copy + Eq + Hash + ?Sized,
{
    pub fn create_ring(virtual_nodes_per_node: usize) -> Result<Self, String> {
        if virtual_nodes_per_node < 1 {
            return Err("virtual_nodes_per_node cannot be less than 1".to_string());
        }
        Ok(Self {
            virtual_nodes: RwLock::new(vec![]),
            num_of_nodes: 0,
            virtual_nodes_per_node,
        })
    }

    pub fn assign_node<K>(&self, input: &K) -> Option<T>
    where
        K: AsRef<[u8]> + ?Sized,
    {
        let target_hash = utils::get_hash(input);
        // TODO: remove the unwrap.
        let index: usize;
        {
            let virtual_nodes = self.virtual_nodes.read().unwrap();
            let x = utils::cyclic_binary_search(virtual_nodes.as_ref(), target_hash.as_str());
            index = x;
        }
        Some(
            self.virtual_nodes
                .read()
                .unwrap()
                .get(index)
                .unwrap()
                .node
                .identifier,
        )
    }

    // TODO: remove the unwrap.
    pub fn add_node(&mut self, node: Node<T>) -> Result<(), String> {
        let cloned = Arc::new(node);
        let virtual_node = VirtualNode::from_node(Arc::clone(&cloned), 0).unwrap();

        if let Some(_) = utils::binary_search(&self.virtual_nodes.read().unwrap(), &virtual_node) {
            // TODO: instead of using Strings for Errors, use Enums
            return Err("Node with same identity has already been added".to_owned());
        }

        let cloned = Arc::clone(&cloned);
        let mut virtual_nodes = self.virtual_nodes.write().unwrap();
        for index in 0..self.virtual_nodes_per_node {
            let node = Arc::clone(&cloned);

            virtual_nodes.push(VirtualNode::from_node(node, index).unwrap());
        }
        virtual_nodes.sort();
        self.num_of_nodes += 1;
        Ok(())
    }
}
