use crate::backend::network::test::{new_net_handler, Net, TIMEOUT};
use common_structs::leaf::LeafEvent::ControllerShortcut;
use rusty_tester::utils::data::{new_flood_request, new_test_fragment_packet, new_test_nack};
use wg_2024::packet::NackType::{Dropped, ErrorInRouting, UnexpectedRecipient};

#[test]
fn test_flood_for_routing_err() {
    let flood = new_flood_request(10, 100, 7, true);
    assert_eq!(None, Net::find_routing_error(8, &flood));

    let flood = new_flood_request(10, 100, 7, false);
    assert_eq!(None, Net::find_routing_error(8, &flood));
}

#[test]
fn test_for_routing_err_fine() {
    let packet = new_test_fragment_packet(&[10, 20], 11);
    assert_eq!(None, Net::find_routing_error(20, &packet));
}

#[test]
fn test_for_routing_err_unexpected() {
    let packet = new_test_fragment_packet(&[10, 30], 11);
    // TODO check id to return
    assert_eq!(
        Some(UnexpectedRecipient(20)),
        Net::find_routing_error(20, &packet)
    );
}

#[test]
fn test_for_routing_err_next_hop() {
    let packet = new_test_fragment_packet(&[10, 20, 30], 11);
    assert_eq!(
        Some(ErrorInRouting(30)),
        Net::find_routing_error(20, &packet)
    );
}

#[test]
fn test_handle_err_routing() {
    let (mut net, conn) = new_net_handler(20, 10);
    let packet = new_test_fragment_packet(&[10, 20, 30], 11);
    let error = ErrorInRouting(30);

    net.handle_error(packet, error);
    let expected = new_test_nack(&[20, 10], error, 11, 1);
    assert_eq!(Ok(expected), conn.pck_out.recv_timeout(TIMEOUT));
}

#[test]
fn test_handle_err_unexpected() {
    let (mut net, conn) = new_net_handler(20, 10);
    let packet = new_test_fragment_packet(&[10, 40, 50], 11);
    let error = UnexpectedRecipient(40);

    net.handle_error(packet, error);
    let expected = new_test_nack(&[20, 10], error, 11, 1);
    assert_eq!(Ok(expected), conn.pck_out.recv_timeout(TIMEOUT));
}

#[test]
fn test_handle_err_undroppable() {
    let (mut net, conn) = new_net_handler(20, 10);
    let packet = new_test_nack(&[10, 20, 40], Dropped, 11, 1);

    net.handle_error(packet.clone(), UnexpectedRecipient(40));
    assert_eq!(
        Ok(ControllerShortcut(packet)),
        conn.leaf_event_out.recv_timeout(TIMEOUT)
    );
}
