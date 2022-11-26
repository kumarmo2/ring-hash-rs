use std::sync::{Arc, RwLock};

use crate::{errors::RingError, node::Node, utils, virtual_node::VirtualNode};
use std::hash::Hash;

pub struct Ring<'id, T>
where
    T: Eq + Hash + ?Sized,
{
    pub(crate) virtual_nodes: RwLock<Vec<VirtualNode<'id, T>>>,
    // TODO: ideally `num_of_nodes` needs to be protected by a lock.
    num_of_nodes: usize,
    virtual_nodes_per_node: usize,
}
/*
 * TODO:
*  -
 * - remove heap allocations if possible
 * */

impl<'id, T> Ring<'id, T>
where
    T: Eq + Hash + ?Sized,
{
    pub fn create_ring(virtual_nodes_per_node: usize) -> Result<Self, RingError> {
        if virtual_nodes_per_node < 1 {
            return Err(RingError::InvalidVirtualNodesPerNode);
        }
        Ok(Self {
            virtual_nodes: RwLock::new(vec![]),
            num_of_nodes: 0,
            virtual_nodes_per_node,
        })
    }

    pub fn assign_node<K>(&self, input: &K) -> Option<&T>
    where
        K: AsRef<[u8]> + ?Sized,
    {
        let target_hash = utils::get_hash(input);
        let index: usize;
        {
            // unwrapping will fail for the case, when the lock has been "poisoned".
            // I think that it is good to "not continue" once the lock has been poisoned.
            // In those cases, let the user of the library make the decision how to continue after
            // this.
            let virtual_nodes = self.virtual_nodes.read().unwrap();
            let x = utils::cyclic_binary_search(virtual_nodes.as_ref(), target_hash.as_str());
            index = x;
        }
        // read().unwrap() is intentional, as we don't want to continue if lock is poisoned.
        if let Some(item) = self.virtual_nodes.read().unwrap().get(index) {
            return Some(item.node.identifier);
        }
        // ideally this should never be reached.
        unreachable!()
    }

    pub fn add_node(&mut self, node: Node<'id, T>) -> Result<(), RingError> {
        let cloned_node = Arc::new(node);
        // To check if the node has already been added, we create the first VirtualNode
        // for that node i.e Virtual Node with id_per_node as 0 and check if that virtual
        // Node is already present in the ring.
        let virtual_node = VirtualNode::from_node(Arc::clone(&cloned_node), 0).unwrap();

        if let Some(_) = utils::binary_search(&self.virtual_nodes.read().unwrap(), &virtual_node) {
            // TODO: instead of using Strings for Errors, use Enums
            return Err(RingError::NodeAlreadyExists);
        }

        let cloned = Arc::clone(&cloned_node);
        let mut virtual_nodes = self.virtual_nodes.write().unwrap();
        for index in 0..self.virtual_nodes_per_node {
            // For each node, we make ring.virtual_nodes_per_node number of virtual nodes.
            // Unique Identifier for each VirtualNode is the Identifier of the corresponding
            // Node appended with the index of that virtual node at the node level.
            // eg: Node (id1): VirtualNode(id1-0), VirtualNode(id1-1) ...
            // eg: Node (id2): VirtualNode(id2-0), VirtualNode(id2-1) ...
            let node = Arc::clone(&cloned);
            virtual_nodes.push(VirtualNode::from_node(node, index).unwrap());
        }
        virtual_nodes.sort();
        self.num_of_nodes += 1;
        Ok(())
    }
}
