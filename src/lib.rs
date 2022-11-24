pub mod ring;

pub mod node;
mod utils;
pub mod virtual_node;

use std::net::Ipv4Addr;
use std::rc::Rc;
use std::str::FromStr;

use node::Node;
use ring::Ring;

/*
* TODO:
* - Make ring as thread safe so that can be used in concurrent environment, after the setup
*    phase.[Done]
* - cache the result for assign_node, so that if the same output is asked to be assigned,
*    result can be send using cache.
* - In the end, add the delete node as well.
* - See if I can remove the trait bound on T for "Copy" in Node<T>. This got introduced when I
*    made the `ring` thread safe. I had to introduce "Lock" for the ring. Because of which I
*    couldn't return references from the ring. Since, I couldn't expose the references, I
*    introduced the Copy constraint on T, so that it can be returned easily. [Done]
* - provide correct access modifiers, right now most of those are public.
*
*
* */

#[test]
fn lets_see_if_it_works_for_ipv4() {
    let r = Ring::create_ring(10);

    let mut ring = r.unwrap(); // here we are just asseting that r cannot be an Err.
    let ip1 = Ipv4Addr::new(192, 1, 1, 1);
    let node = Node::new(&ip1);
    ring.add_node(node).unwrap();

    let ip2 = Ipv4Addr::new(192, 1, 1, 2);
    let node = Node::new(&ip2);
    ring.add_node(node).unwrap();

    let ip3 = Ipv4Addr::new(192, 1, 1, 3);
    let node = Node::new(&ip3);
    ring.add_node(node).unwrap();

    let lock = ring.virtual_nodes.read().unwrap();

    // NOTE: these println are here so to know the hash for the inputs and nodes.
    for vn in lock.iter() {
        println!("vn: {:?}", vn);
    }

    let input = "30";
    println!("input hash: {}", utils::get_hash(input));

    assert_eq!(
        *ring.assign_node(input).unwrap(),
        ip2 // Ipv4Addr::new(192, 1, 1, 2)
    );

    let input = "40";
    println!("input hash: {}", utils::get_hash(input));

    assert_eq!(
        *ring.assign_node(input).unwrap(),
        ip3,
        // Ipv4Addr::new(192, 1, 1, 3)
    );

    let input = "50";
    println!("input hash: {}", utils::get_hash(input));

    assert_eq!(
        *ring.assign_node(input).unwrap(),
        ip1 // Ipv4Addr::new(192, 1, 1, 1)
    );

    let input = "-1df";
    println!("input hash: {}", utils::get_hash(input));

    assert_eq!(
        *ring.assign_node(input).unwrap(),
        ip3 // Ipv4Addr::new(192, 1, 1, 3)
    );
}

#[test]
fn lets_see_if_it_works_for_str() {
    let r = Ring::create_ring(10);

    let mut ring = r.unwrap(); // here we are just asseting that r cannot be an Err.

    let id_10 = "10";
    let node = Node::new(id_10);
    ring.add_node(node).unwrap();

    let id_20 = "20";
    let node = Node::new(id_20);
    ring.add_node(node).unwrap();

    let id_30 = "30";
    let node = Node::new(id_30);
    ring.add_node(node).unwrap();

    let lock = ring.virtual_nodes.read().unwrap();

    // NOTE: these println are here so to know the hash for the inputs and nodes.
    for vn in lock.iter() {
        println!("vn: {:?}", vn);
    }

    let input = "30";
    println!("input hash: {}", utils::get_hash(input));

    assert_eq!(ring.assign_node(input).unwrap(), id_30);

    let input = "40";
    println!("input hash: {}", utils::get_hash(input));

    assert_eq!(ring.assign_node(input).unwrap(), id_30);

    let input = "50";
    println!("input hash: {}", utils::get_hash(input));

    assert_eq!(ring.assign_node(input).unwrap(), id_30);

    let input = "-1df";
    println!("input hash: {}", utils::get_hash(input));

    assert_eq!(ring.assign_node(input).unwrap(), id_10);
}

#[test]
fn lets_see_if_it_works() {
    let r = Ring::create_ring(10);

    let mut ring = r.unwrap(); // here we are just asseting that r cannot be an Err.

    let id_10 = 10;
    let node = Node::new(&id_10);
    ring.add_node(node).unwrap();

    let id_20 = 20;
    let node = Node::new(&id_20);
    ring.add_node(node).unwrap();

    let id_30 = 30;
    let node = Node::new(&id_30);
    ring.add_node(node).unwrap();

    let lock = ring.virtual_nodes.read().unwrap();

    // NOTE: these println are here so to know the hash for the inputs and nodes.
    for vn in lock.iter() {
        println!("vn: {:?}", vn);
    }

    let input = "30";
    println!("input hash: {}", utils::get_hash(input));

    assert_eq!(*ring.assign_node(input).unwrap(), id_20);

    let input = "40";
    println!("input hash: {}", utils::get_hash(input));

    assert_eq!(*ring.assign_node(input).unwrap(), id_30);

    let input = "50";
    println!("input hash: {}", utils::get_hash(input));

    assert_eq!(*ring.assign_node(input).unwrap(), id_20);

    let input = "-1df";
    println!("input hash: {}", utils::get_hash(input));

    assert_eq!(*ring.assign_node(input).unwrap(), id_30);
}

#[test]
fn multiple_nodes_with_same_identity_added_returns_error() {
    let mut ring = Ring::create_ring(3).unwrap();
    let id = 101;
    let n1 = Node::new(&id);
    let n2 = Node::new(&id);
    ring.add_node(n1).unwrap();
    assert_eq!(
        ring.add_node(n2).err().unwrap().as_str(),
        "Node with same identity has already been added"
    )
}

#[test]
fn multiple_nodes_with_same_ipv4_identity_added_returns_error() {
    let mut ring = Ring::create_ring(3).unwrap();

    let ip1 = Ipv4Addr::new(127, 0, 0, 1);
    let ip2 = Ipv4Addr::new(127, 0, 0, 1);

    let n1 = Node::new(&ip1);

    ring.add_node(n1).unwrap();

    let n2 = Node::new(&ip2);
    assert_eq!(
        ring.add_node(n2).err().unwrap().as_str(),
        "Node with same identity has already been added"
    )
}

#[test]
fn assign_node_in_multi_threaded_env() {
    let mut ring = Ring::create_ring(3).unwrap();

    let id_101 = 101;
    let n1 = Node::new(&id_101);
    ring.add_node(n1).unwrap();

    let id_102 = 102;
    let n1 = Node::new(&id_102);
    ring.add_node(n1).unwrap();

    std::thread::scope(|s| {
        let n1 = s.spawn(|| ring.assign_node("sdfsdf").unwrap());

        let n2 = s.spawn(|| {
            return ring.assign_node("sdfsdf").unwrap();
        });
        let x = n1.join();
        let y = n2.join();

        assert_eq!(x.unwrap(), y.unwrap());
    });
}
