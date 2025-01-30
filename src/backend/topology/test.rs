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
    topology.add_path(&path, true).unwrap();

    let routing = topology.get_routing_for(3);
    assert_eq!(path, routing.unwrap().hops);
}

#[test]
fn inverse_path_fail() {
    let mut topology = Topology::new(11);
    let path = vec![3, 2, 1, 11];
    assert!(topology.add_path(&path, true).is_err());
}

#[test]
fn last_hop_undirectional() {
    let mut topology = Topology::new(11);
    let path1 = vec![11, 1, 2, 3, 4, 5, 7, 9];
    let path2 = vec![11, 8];
    let path3 = vec![11, 1, 2, 3, 4, 5, 6, 7, 8];
    let expected = vec![11, 8, 7, 9];

    topology.add_path(&path1, true).unwrap();
    topology.add_path(&path2, false).unwrap();
    topology.add_path(&path3, false).unwrap();

    let routing = topology.get_routing_for(9);
    assert_eq!(expected, routing.unwrap().hops);
}

#[test]
fn last_hop_directional() {
    let mut topology = Topology::new(11);
    let path1 = vec![11, 1, 2, 3, 4, 5, 7, 9];
    let path2 = vec![11, 8];
    let path3 = vec![11, 1, 2, 3, 4, 5, 6, 7, 8];

    topology.add_path(&path1, true).unwrap();
    topology.add_path(&path2, true).unwrap();
    topology.add_path(&path3, true).unwrap();

    let routing = topology.get_routing_for(9);
    assert_eq!(path1, routing.unwrap().hops);
}

#[test]
fn remove_node() {
    let mut topology = Topology::new(11);
    let path = vec![11, 1, 2, 3];

    topology.add_path(&path, true).unwrap();
    topology.remove_node(2);
    assert!(!topology.graph.contains_node(2));

    assert!(topology.get_routing_for(3).is_none());
}

#[test]
fn readd_node() {
    let mut topology = Topology::new(11);
    let path = vec![11, 1, 2, 3];

    topology.add_path(&path, true).unwrap();
    topology.remove_node(2);
    assert!(!topology.graph.contains_node(2));

    topology.add_path(&path, true).unwrap();
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

    topology.add_path(&path1, true).unwrap();
    topology.add_path(&path2, true).unwrap();

    let routing = topology.get_routing_for(5);
    assert_eq!(expected, routing.unwrap().hops);
}

#[test]
fn avoid_dead() {
    let mut topology = Topology::new(11);
    let path1 = vec![11, 1, 5, 8, 9];
    let path2 = vec![11, 1, 2, 3, 4, 6, 7, 8];
    let expected = vec![11, 1, 2, 3, 4, 6, 7, 8, 9];

    topology.add_path(&path1, true).unwrap();
    topology.add_path(&path2, false).unwrap();

    for _ in 0..10 {
        topology.mark_drop(5);
    }

    let routing = topology.get_routing_for(9);
    assert_eq!(expected, routing.unwrap().hops);
}

#[test]
fn max_weight_decrease() {
    let mut topology = Topology::new(11);
    let path1 = vec![11, 1, 5, 8, 9];
    topology.add_path(&path1, true).unwrap();

    for _ in 0..100 {
        topology.mark_drop(5);
    }
    assert!(*topology.weights.get(&5).unwrap() > 0);
    for _ in 0..100 {
        topology.update_weight(5, true);
    }
    assert_eq!(0u8, *topology.weights.get(&5).unwrap());
}
