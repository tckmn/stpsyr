extern crate stpsyr;
use stpsyr::*;

#[test]
fn test_datc_6a1() {
    let mut s = Stpsyr::new("data/standard.csv");
    s.add_order(String::from("England"), String::from("lon"), Action::Move { to: String::from("pic"), convoyed: false });
    s.add_order(String::from("Italy"), String::from("rom"), Action::Move { to: String::from("tun"), convoyed: false });
    s.apply_orders();
    assert!(s.get_unit(&String::from("pic")).is_none());
    assert!(s.get_unit(&String::from("tun")).is_none());
}
