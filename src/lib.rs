pub mod ring;

pub mod node;
mod utils;
pub mod virtual_node;

use std::rc::Rc;

use node::Node;
use ring::Ring;

/*
* TODO:
* 1. Make ring as thread safe so that can be used in concurrent environment, after the setup phase.
* 2. provide correct access modifiers, right now most of those are public.
* 3. cache the result for assign_node, so that if the same output is asked to be assigned,
*    result can be send using cache.
* 4. In the end, add the delete node as well.
*
*
* */

#[test]
fn lets_see_if_it_works() {
    let r = Ring::create_ring(10);

    let mut ring = r.unwrap(); // here we are just asseting that r cannot be an Err.

    let node = Node::new(10);
    ring.add_node(node).unwrap();

    let node = Node::new(20);
    ring.add_node(node).unwrap();

    let node = Node::new(30);
    ring.add_node(node).unwrap();

    let lock = ring.virtual_nodes.read().unwrap();

    // NOTE: these println are here so to know the hash for the inputs and nodes.
    for vn in lock.iter() {
        println!("vn: {:?}", vn);
    }

    let input = "30";
    println!("input hash: {}", utils::get_hash(input));

    assert_eq!(ring.assign_node(input).unwrap(), 20);

    let input = "40";
    println!("input hash: {}", utils::get_hash(input));

    assert_eq!(ring.assign_node(input).unwrap(), 30);

    let input = "50";
    println!("input hash: {}", utils::get_hash(input));

    assert_eq!(ring.assign_node(input).unwrap(), 20);

    let input = "-1df";
    println!("input hash: {}", utils::get_hash(input));

    assert_eq!(ring.assign_node(input).unwrap(), 30);
}

#[test]
fn multiple_nodes_with_identity_added_returns_error() {
    let mut ring = Ring::create_ring(3).unwrap();
    let n1 = Node::new(101);
    let n2 = Node::new(101);
    ring.add_node(n1).unwrap();
    assert_eq!(
        ring.add_node(n2).err().unwrap().as_str(),
        "Node with same identity has already been added"
    )
}

#[test]
fn assign_node_in_multi_threaded_env() {
    let mut ring = Ring::create_ring(3).unwrap();

    let n1 = Node::new(101);
    ring.add_node(n1).unwrap();

    let n1 = Node::new(102);
    ring.add_node(n1).unwrap();

    std::thread::scope(|s| {
        let n1 = s.spawn(|| ring.assign_node("sdfsdf").unwrap());

        let n2 = s.spawn(|| {
            // ring.add_node(Node::from_str("sdfsf sldfklsf")); // this will fail, as above in first spawn it has already been borrowed immutably.

            return ring.assign_node("sdfsdf").unwrap();
        });
        let x = n1.join();
        let y = n2.join();

        assert_eq!(x.unwrap(), y.unwrap());
    });
}
