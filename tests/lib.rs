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

#[test]
fn test_datc_6a2() {
    let mut s = Stpsyr::new("data/standard.csv");
    s.add_order(String::from("England"), String::from("lvp"), Action::Move { to: String::from("iri"), convoyed: false });
    s.apply_orders();
    assert!(s.get_unit(&String::from("iri")).is_none());
}

#[test]
fn test_datc_6a3() {
    let mut s = Stpsyr::new("data/standard.csv");
    s.add_order(String::from("Germany"), String::from("kie"), Action::Move { to: String::from("ruh"), convoyed: false });
    s.apply_orders();
    assert!(s.get_unit(&String::from("ruh")).is_none());
}

#[test]
fn test_datc_6a4() {
    let mut s = Stpsyr::new("data/standard.csv");
    s.add_order(String::from("Germany"), String::from("kie"), Action::Move { to: String::from("kie"), convoyed: false });
    s.apply_orders();
    assert!(s.get_unit(&String::from("kie")).is_some());
}

#[test]
fn test_datc_6a6() {
    let mut s = Stpsyr::new("data/standard.csv");
    s.add_order(String::from("Germany"), String::from("lon"), Action::Move { to: String::from("nth"), convoyed: false });
    s.apply_orders();
    assert!(s.get_unit(&String::from("nth")).is_none());
}

#[test]
fn test_datc_6a8() {
    let mut s = Stpsyr::new("data/standard.csv");
    s.add_order(String::from("Italy"), String::from("rom"), Action::Move { to: String::from("ven"), convoyed: false });
    s.add_order(String::from("Italy"), String::from("ven"), Action::Move { to: String::from("tyr"), convoyed: false });
    s.apply_orders();
    s.add_order(String::from("Austria"), String::from("tri"), Action::SupportHold { to: String::from("tri") });
    s.add_order(String::from("Italy"), String::from("ven"), Action::Move { to: String::from("tri"), convoyed: false });
    s.add_order(String::from("Italy"), String::from("tyr"), Action::SupportMove { from: String::from("ven"), to: String::from("tri") });
    let dislodged = s.apply_orders();
    assert_eq!(dislodged.len(), 1);
    assert_eq!(dislodged[0].0, "tri");
}

#[test]
fn test_datc_6a9() {
    let mut s = Stpsyr::new("data/standard.csv");
    s.add_order(String::from("Turkey"), String::from("con"), Action::Move { to: String::from("bul"), convoyed: false });
    s.add_order(String::from("Turkey"), String::from("smy"), Action::Move { to: String::from("con"), convoyed: false });
    s.add_order(String::from("Turkey"), String::from("ank"), Action::Move { to: String::from("smy"), convoyed: false });
    s.apply_orders();
    assert!(s.get_unit(&String::from("smy")).is_none());
}

#[test]
fn test_datc_6a10() {
    let mut s = Stpsyr::new("data/standard.csv");
    s.add_order(String::from("Italy"), String::from("rom"), Action::Move { to: String::from("apu"), convoyed: false });
    s.add_order(String::from("Italy"), String::from("nap"), Action::Move { to: String::from("rom"), convoyed: false });
    s.add_order(String::from("Italy"), String::from("ven"), Action::Move { to: String::from("tyr"), convoyed: false });
    s.add_order(String::from("Austria"), String::from("tri"), Action::Move { to: String::from("ven"), convoyed: false });
    s.apply_orders();
    s.add_order(String::from("Italy"), String::from("rom"), Action::SupportMove { from: String::from("apu"), to: String::from("ven") });
    s.add_order(String::from("Italy"), String::from("apu"), Action::Move { to: String::from("ven"), convoyed: false });
    s.apply_orders();
    assert_eq!(s.get_unit(&String::from("ven")).unwrap().owner, "Austria");
}
