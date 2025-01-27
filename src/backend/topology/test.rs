#![cfg(test)]

use crate::backend::topology::Topology;

#[test]
fn node_id() {
    let node_id = 11;
    let topology = Topology::new(node_id);
    assert_eq!(node_id, topology.start_id);
}

#[test]
fn fresh_id() {
    let mut topology = Topology::new(11);
    let base = topology.take_fresh_flood_id();

    for i in 1..=10 {
        assert_eq!(base + i, topology.take_fresh_flood_id());
    }
}

#[test]
fn simple_path() {
    let mut topology = Topology::new(11);
    let path = vec![11, 1, 2, 3];
    topology.add_path(path.clone());

    let routing = topology.get_routing_for(3);
    assert_eq!(path, routing.unwrap().hops);
}

#[test]
fn remove_node() {
    let mut topology = Topology::new(11);
    let path = vec![11, 1, 2, 3];

    topology.add_path(path);
    topology.remove_node(2);
    assert!(!topology.graph.contains_node(2));

    let routing = topology.get_routing_for(3);
    assert_eq!(None, routing);
}

#[test]
fn readd_node() {
    let mut topology = Topology::new(11);
    let path = vec![11, 1, 2, 3];

    topology.add_path(path.clone());
    topology.remove_node(2);
    assert!(!topology.graph.contains_node(2));

    topology.add_path(path.clone());
    assert!(topology.graph.contains_node(2));

    let routing = topology.get_routing_for(3);
    assert_eq!(path, routing.unwrap().hops);
}

#[test]
fn shortest_path() {
    let mut topology = Topology::new(11);
    let path1 = vec![11, 1, 2, 3, 4, 5];
    let path2 = vec![11, 1, 4, 2, 3, 5];
    let expected = vec![11, 1, 4, 5];

    topology.add_path(path1);
    topology.add_path(path2);

    let routing = topology.get_routing_for(5);
    assert_eq!(expected, routing.unwrap().hops);
}
