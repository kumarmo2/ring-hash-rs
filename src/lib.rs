pub mod ring;

pub mod node;
mod utils;
pub mod virtual_node;

use node::Node;
use ring::Ring;

/*
* TODO:
* 1. provide correct access modifiers, right now most of those are public.
*
* */

#[test]
fn lets_see_if_it_works() {
    let r = Ring::create_ring(10);

    let mut ring = r.unwrap(); // here we are just asseting that r cannot be an Err.

    let node = Node::from_str("10");
    ring.add_node(node);

    let node = Node::from_str("20");
    ring.add_node(node);

    let node = Node::from_str("30");
    ring.add_node(node);

    for vn in &ring.virtual_nodes {
        println!("vn: {:?}", vn);
    }

    let input = "30";
    println!("input hash: {}", utils::get_hash(input));

    assert_eq!(ring.assign_node(input).unwrap().identifier, "30");

    let input = "40";
    println!("input hash: {}", utils::get_hash(input));

    assert_eq!(ring.assign_node(input).unwrap().identifier, "30");

    let input = "50";
    println!("input hash: {}", utils::get_hash(input));

    assert_eq!(ring.assign_node(input).unwrap().identifier, "10");

    let input = "-1df";
    println!("input hash: {}", utils::get_hash(input));

    assert_eq!(ring.assign_node(input).unwrap().identifier, "10");
}
