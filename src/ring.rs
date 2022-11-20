use std::rc::Rc;

use crate::{node::Node, utils, virtual_node::VirtualNode};

pub struct Ring {
    pub(crate) virtual_nodes: Vec<VirtualNode>,
    num_of_nodes: usize,
    virtual_nodes_per_node: usize,
}

/*
* NOTE:
* For first phase, lets assume the changes to the ring are done at the start only.
* Once its been setup, after that only its started to be used in the system.
* Reason being, right now, this is not threadSafe if being mutated from multiple
* threads.
*
* */

impl Ring {
    pub fn create_ring(virtual_nodes_per_node: usize) -> Result<Self, String> {
        if virtual_nodes_per_node < 1 {
            return Err("virtual_nodes_per_node cannot be less than 1".to_string());
        }
        Ok(Self {
            virtual_nodes: vec![],
            num_of_nodes: 0,
            virtual_nodes_per_node,
        })
    }

    pub fn assign_node(&self, input: &str) -> Option<&Node> {
        let target_hash = utils::get_hash(input);
        // TODO: remove the unwrap.
        Some(
            &self
                .virtual_nodes
                .get(utils::cyclic_binary_search(&self.virtual_nodes, &target_hash).unwrap())
                .unwrap()
                .node,
        )
    }

    pub fn add_node(&mut self, node: Node) {
        let cloned = Rc::new(node);
        for index in 0..self.virtual_nodes_per_node {
            let node = Rc::clone(&cloned);

            // TODO: remove the unwrap.
            self.virtual_nodes
                .push(VirtualNode::from_node(node, index).unwrap());
        }

        self.virtual_nodes.sort();

        self.num_of_nodes += 1;
    }
}
/*
 * TODO:
 * - if the node already has been added in the ring, return Err.
 * - remove heap allocations if possible
 *
 *
 * */
