extern crate stpsyr;
use stpsyr::*;

macro_rules! move_order {
    ($s:ident, $power:expr, $from:expr, $to:expr, $convoyed:expr) => (
        $s.add_order(String::from($power), String::from($from), Action::Move { to: String::from($to), convoyed: $convoyed });
    )
}

macro_rules! support_hold_order {
    ($s:ident, $power:expr, $from:expr, $to:expr) => (
        $s.add_order(String::from($power), String::from($from), Action::SupportHold { to: String::from($to) });
    )
}

macro_rules! support_move_order {
    ($s:ident, $power:expr, $from:expr, $from2:expr, $to:expr) => (
        $s.add_order(String::from($power), String::from($from), Action::SupportMove { from: String::from($from2), to: String::from($to) });
    )
}

macro_rules! assert_empty {
    ($s:ident, $x:expr) => (
        assert!($s.get_unit(&String::from($x)).is_none());
    )
}

macro_rules! assert_nonempty {
    ($s:ident, $x:expr) => (
        assert!($s.get_unit(&String::from($x)).is_some());
    )
}

#[test]
fn test_datc_6a1() {
    let mut s = Stpsyr::new("data/standard.csv");
    move_order!(s, "England", "lon", "pic", false);
    move_order!(s, "Italy", "rom", "tun", false);
    s.apply_orders();
    assert_empty!(s, "pic");
    assert_empty!(s, "tun");
}

#[test]
fn test_datc_6a2() {
    let mut s = Stpsyr::new("data/standard.csv");
    move_order!(s, "England", "lvp", "iri", false);
    s.apply_orders();
    assert_empty!(s, "iri");
}

#[test]
fn test_datc_6a3() {
    let mut s = Stpsyr::new("data/standard.csv");
    move_order!(s, "Germany", "kie", "ruh", false);
    s.apply_orders();
    assert_empty!(s, "ruh");
}

#[test]
fn test_datc_6a4() {
    let mut s = Stpsyr::new("data/standard.csv");
    move_order!(s, "Germany", "kie", "kie", false);
    s.apply_orders();
    assert_nonempty!(s, "kie");
}

#[test]
fn test_datc_6a6() {
    let mut s = Stpsyr::new("data/standard.csv");
    move_order!(s, "Germany", "lon", "nth", false);
    s.apply_orders();
    assert_empty!(s, "nth");
}

#[test]
fn test_datc_6a8() {
    let mut s = Stpsyr::new("data/standard.csv");
    move_order!(s, "Italy", "rom", "ven", false);
    move_order!(s, "Italy", "ven", "tyr", false);
    s.apply_orders();
    support_hold_order!(s, "Austria", "tri", "tri");
    move_order!(s, "Italy", "ven", "tri", false);
    support_move_order!(s, "Italy", "tyr", "ven", "tri");
    let dislodged = s.apply_orders();
    assert_eq!(dislodged.len(), 1);
    assert_eq!(dislodged[0].0, "tri");
}

#[test]
fn test_datc_6a9() {
    let mut s = Stpsyr::new("data/standard.csv");
    move_order!(s, "Turkey", "con", "bul", false);
    move_order!(s, "Turkey", "smy", "con", false);
    move_order!(s, "Turkey", "ank", "smy", false);
    s.apply_orders();
    assert_empty!(s, "smy");
}

#[test]
fn test_datc_6a10() {
    let mut s = Stpsyr::new("data/standard.csv");
    move_order!(s, "Italy", "rom", "apu", false);
    move_order!(s, "Italy", "nap", "rom", false);
    move_order!(s, "Italy", "ven", "tyr", false);
    move_order!(s, "Austria", "tri", "ven", false);
    s.apply_orders();
    support_move_order!(s, "Italy", "rom", "apu", "ven");
    move_order!(s, "Italy", "apu", "ven", false);
    s.apply_orders();
    assert_eq!(s.get_unit(&String::from("ven")).unwrap().owner, "Austria");
}

#[test]
fn test_datc_6a11() {
    let mut s = Stpsyr::new("data/standard.csv");
    move_order!(s, "Italy", "ven", "tyr", false);
    move_order!(s, "Austria", "vie", "tyr", false);
    s.apply_orders();
    assert_empty!(s, "tyr");
}
