extern crate stpsyr;
use stpsyr::*;

macro_rules! move_order {
    ($s:ident, $power:expr, $from:expr, $to:expr, $convoyed:expr) => (
        $s.add_order(Power::from($power), Province::from($from), Action::Move { to: Province::from($to), convoyed: $convoyed });
    )
}

macro_rules! support_hold_order {
    ($s:ident, $power:expr, $from:expr, $to:expr) => (
        $s.add_order(Power::from($power), Province::from($from), Action::SupportHold { to: Province::from($to) });
    )
}

macro_rules! support_move_order {
    ($s:ident, $power:expr, $from:expr, $from2:expr, $to:expr) => (
        $s.add_order(Power::from($power), Province::from($from), Action::SupportMove { from: Province::from($from2), to: Province::from($to) });
    )
}

macro_rules! convoy_order {
    ($s:ident, $power:expr, $from:expr, $from2:expr, $to:expr) => (
        $s.add_order(Power::from($power), Province::from($from), Action::Convoy { from: Province::from($from2), to: Province::from($to) });
    )
}

macro_rules! order {
    ($s:ident, $orders:expr) => (
        $s.parse_orders(String::from($orders));
    )
}

macro_rules! assert_empty {
    ($s:ident, $x:expr) => (
        assert!($s.get_unit(&Province::from($x)).is_none());
    )
}

macro_rules! assert_nonempty {
    ($s:ident, $x:expr) => (
        assert!($s.get_unit(&Province::from($x)).is_some());
    )
}

macro_rules! assert_unit {
    ($s:ident, $province:expr, $unit:expr) => (
        assert_eq!(format!("{:?}", $s.get_unit(&Province::from($province)).unwrap()), $unit);
    )
}

#[test]
fn test_datc_6a1() {
    let mut s = Stpsyr::new("data/standard.csv");
    order!(s, "
    England
        F lon-pic
    Italy
        A rom-tun
    ");
    assert_empty!(s, "pic");
    assert_empty!(s, "tun");
}

#[test]
fn test_datc_6a2() {
    let mut s = Stpsyr::new("data/standard.csv");
    order!(s, "
    England
        A lvp-iri
    ");
    assert_empty!(s, "iri");
}

#[test]
fn test_datc_6a3() {
    let mut s = Stpsyr::new("data/standard.csv");
    order!(s, "
    Germany
        F kie-ruh
    ");
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
fn test_datc_6a5() {
    let mut s = Stpsyr::new("data/standard.csv");
    move_order!(s, "Italy", "rom", "ven", false);
    move_order!(s, "Italy", "ven", "tyr", false);
    move_order!(s, "Austria", "bud", "tri", false);
    move_order!(s, "Austria", "tri", "adr", false);
    s.apply_orders();
    move_order!(s, "Italy", "ven", "tri", false);
    support_move_order!(s, "Italy", "tyr", "ven", "tri");
    convoy_order!(s, "Austria", "adr", "tri", "tri");
    move_order!(s, "Austria", "tri", "tri", true);
    support_move_order!(s, "Austria", "vie", "tri", "tri");
    s.apply_orders();
    assert_empty!(s, "ven");
}

#[test]
fn test_datc_6a6() {
    let mut s = Stpsyr::new("data/standard.csv");
    move_order!(s, "Germany", "lon", "nth", false);
    s.apply_orders();
    assert_empty!(s, "nth");
}

#[test]
fn test_datc_6a7() {
    let mut s = Stpsyr::new("data/standard.csv");
    move_order!(s, "England", "edi", "nth", false);
    s.apply_orders();
    convoy_order!(s, "England", "nth", "lon", "bel");
    move_order!(s, "England", "lon", "bel", true);
    s.apply_orders();
    assert_empty!(s, "bel");
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
    assert_eq!(dislodged[0].0, Province::from("tri"));
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
    assert_unit!(s, "ven", "Fleet Austria");
}

#[test]
fn test_datc_6a11() {
    let mut s = Stpsyr::new("data/standard.csv");
    move_order!(s, "Italy", "ven", "tyr", false);
    move_order!(s, "Austria", "vie", "tyr", false);
    s.apply_orders();
    assert_empty!(s, "tyr");
}

#[test]
fn test_datc_6a12() {
    let mut s = Stpsyr::new("data/standard.csv");
    move_order!(s, "Italy", "ven", "tyr", false);
    move_order!(s, "Austria", "vie", "tyr", false);
    move_order!(s, "Germany", "mun", "tyr", false);
    s.apply_orders();
    assert_empty!(s, "tyr");
}

#[test]
fn test_datc_6b1() {
    let mut s = Stpsyr::new("data/standard.csv");
    move_order!(s, "Turkey", "con", "bul", false);
    move_order!(s, "Turkey", "ank", "con", false);
    s.apply_orders();
    move_order!(s, "Turkey", "bul", "ser", false);
    move_order!(s, "Turkey", "con", "bul", false);
    s.apply_orders();
    assert_empty!(s, "bul");
}

#[test]
fn test_datc_6b2() {
    // NOTE: THIS TEST CASE DIFFERS FROM DATC RECOMMENDATION!!!
    let mut s = Stpsyr::new("data/standard.csv");
    move_order!(s, "Turkey", "ank", "bla", false);
    s.apply_orders();
    move_order!(s, "Turkey", "bla", "bul", false);
    s.apply_orders();
    assert_empty!(s, "bul");
}

#[test]
fn test_datc_6b3() {
    let mut s = Stpsyr::new("data/standard.csv");
    move_order!(s, "Turkey", "ank", "bla", false);
    s.apply_orders();
    move_order!(s, "Turkey", "bla", "bul/sc", false);
    s.apply_orders();
    assert_empty!(s, "bul");
}

#[test]
fn test_datc_6b4() {
    let mut s = Stpsyr::new("data/standard.csv");
    move_order!(s, "Turkey", "ank", "con", false);
    move_order!(s, "Turkey", "con", "smy", false);
    move_order!(s, "Turkey", "smy", "syr", false);
    move_order!(s, "Russia", "sev", "bla", false);
    move_order!(s, "Austria", "bud", "rum", false);
    s.apply_orders();
    move_order!(s, "Turkey", "con", "bul/sc", false);
    support_move_order!(s, "Russia", "bla", "con", "bul/sc");
    move_order!(s, "Austria", "rum", "bul", false);
    s.apply_orders();
    assert_empty!(s, "con");
}

#[test]
fn test_datc_6b5() {
    let mut s = Stpsyr::new("data/standard.csv");
    move_order!(s, "Turkey", "ank", "con", false);
    move_order!(s, "Turkey", "con", "bul", false);
    s.apply_orders();
    move_order!(s, "Turkey", "con", "bul/sc", false);
    move_order!(s, "Turkey", "bul", "gre", false);
    s.apply_orders();
    support_move_order!(s, "Turkey", "bul", "bud", "rum");
    move_order!(s, "Austria", "bud", "rum", false);
    move_order!(s, "Russia", "sev", "rum", false);
    s.apply_orders();
    assert_empty!(s, "rum");
}

#[test]
fn test_datc_6b6() {
    let mut s = Stpsyr::new("data/standard.csv");
    move_order!(s, "Turkey", "ank", "con", false);
    move_order!(s, "Turkey", "con", "bul", false);
    move_order!(s, "Italy", "nap", "ion", false);
    s.apply_orders();
    move_order!(s, "Turkey", "con", "bul/nc", false);
    move_order!(s, "Turkey", "bul", "gre", false);
    move_order!(s, "Italy", "ion", "aeg", false);
    s.apply_orders();
    support_move_order!(s, "Turkey", "bul", "bud", "rum");
    move_order!(s, "Austria", "bud", "rum", false);
    move_order!(s, "Russia", "sev", "rum", false);
    move_order!(s, "Italy", "aeg", "bul/sc", false);
    s.apply_orders();
    assert_empty!(s, "rum");
}

#[test]
fn test_datc_6b7() {
    let mut s = Stpsyr::new("data/standard.csv");
    move_order!(s, "France", "bre", "mao", false);
    move_order!(s, "France", "mar", "spa", false);
    move_order!(s, "Italy", "ven", "pie", false);
    move_order!(s, "Italy", "nap", "tys", false);
    s.apply_orders();
    move_order!(s, "France", "spa", "por", false);
    move_order!(s, "Italy", "pie", "mar", false);
    move_order!(s, "Italy", "tys", "lyo", false);
    s.apply_orders();
    support_move_order!(s, "France", "por", "mao", "spa");
    move_order!(s, "France", "mao", "spa/nc", false);
    support_move_order!(s, "Italy", "mar", "lyo", "spa/sc");
    move_order!(s, "Italy", "lyo", "spa/sc", false);
    s.apply_orders();
    assert_empty!(s, "spa");
}

// test 6b7 makes test 6b8 extraneous

#[test]
fn test_datc_6b9() {
    // NOTE: THIS TEST CASE DIFFERS FROM DATC RECOMMENDATION!!!
    let mut s = Stpsyr::new("data/standard.csv");
    move_order!(s, "France", "bre", "mao", false);
    move_order!(s, "France", "mar", "spa", false);
    move_order!(s, "Italy", "ven", "pie", false);
    move_order!(s, "Italy", "nap", "tys", false);
    s.apply_orders();
    move_order!(s, "France", "spa", "por", false);
    move_order!(s, "Italy", "pie", "mar", false);
    move_order!(s, "Italy", "tys", "lyo", false);
    s.apply_orders();
    support_move_order!(s, "France", "por", "mao", "spa/sc");
    move_order!(s, "France", "mao", "spa/nc", false);
    support_move_order!(s, "Italy", "mar", "lyo", "spa/sc");
    move_order!(s, "Italy", "lyo", "spa/sc", false);
    s.apply_orders();
    assert_empty!(s, "spa");
}

#[test]
fn test_datc_6b10() {
    let mut s = Stpsyr::new("data/standard.csv");
    move_order!(s, "France", "bre", "mao", false);
    s.apply_orders();
    move_order!(s, "France", "mao", "spa/sc", false);
    s.apply_orders();
    move_order!(s, "France", "spa/nc", "lyo", false);
    s.apply_orders();
    assert_nonempty!(s, "lyo");
}

#[test]
fn test_datc_6b11() {
    let mut s = Stpsyr::new("data/standard.csv");
    move_order!(s, "France", "bre", "mao", false);
    s.apply_orders();
    move_order!(s, "France", "mao", "spa/nc", false);
    s.apply_orders();
    move_order!(s, "France", "spa/sc", "lyo", false);
    s.apply_orders();
    assert_empty!(s, "lyo");
}

#[test]
fn test_datc_6b12() {
    let mut s = Stpsyr::new("data/standard.csv");
    move_order!(s, "France", "mar", "spa/nc", false);
    s.apply_orders();
    assert_nonempty!(s, "spa");
}

#[test]
fn test_datc_6b13() {
    let mut s = Stpsyr::new("data/standard.csv");
    move_order!(s, "Turkey", "ank", "con", false);
    move_order!(s, "Turkey", "con", "bul", false);
    move_order!(s, "Russia", "sev", "rum", false);
    s.apply_orders();
    move_order!(s, "Turkey", "bul", "gre", false);
    move_order!(s, "Russia", "rum", "bul/nc", false);
    s.apply_orders();
    move_order!(s, "Turkey", "con", "bul/sc", false);
    move_order!(s, "Russia", "bul/nc", "con", false);
    s.apply_orders();
    assert_unit!(s, "con", "Fleet Turkey");
}

// TODO 6b14 (pending builds)

#[test]
fn test_datc_6c1() {
    let mut s = Stpsyr::new("data/standard.csv");
    move_order!(s, "Turkey", "ank", "con", false);
    move_order!(s, "Turkey", "con", "smy", false);
    move_order!(s, "Turkey", "smy", "ank", false);
    s.apply_orders();
    assert_unit!(s, "con", "Fleet Turkey");
}

#[test]
fn test_datc_6c2() {
    let mut s = Stpsyr::new("data/standard.csv");
    move_order!(s, "Russia", "sev", "bla", false);
    s.apply_orders();
    support_move_order!(s, "Russia", "bla", "smy", "ank");
    move_order!(s, "Turkey", "ank", "con", false);
    move_order!(s, "Turkey", "con", "smy", false);
    move_order!(s, "Turkey", "smy", "ank", false);
    s.apply_orders();
    assert_unit!(s, "con", "Fleet Turkey");
}

#[test]
fn test_datc_6c3() {
    let mut s = Stpsyr::new("data/standard.csv");
    move_order!(s, "Russia", "sev", "bla", false);
    s.apply_orders();
    move_order!(s, "Russia", "bla", "ank", false);
    move_order!(s, "Turkey", "ank", "con", false);
    move_order!(s, "Turkey", "con", "smy", false);
    move_order!(s, "Turkey", "smy", "ank", false);
    s.apply_orders();
    assert_unit!(s, "ank", "Fleet Turkey");
}

#[test]
fn test_datc_6c4() {
    let mut s = Stpsyr::new("data/standard.csv");
    move_order!(s, "Turkey", "ank", "bla", false);
    move_order!(s, "Turkey", "smy", "con", false);
    move_order!(s, "Turkey", "con", "bul", false);
    move_order!(s, "Austria", "bud", "rum", false);
    s.apply_orders();
    move_order!(s, "Russia", "sev", "bla", false);
    move_order!(s, "Turkey", "con", "bul", false);
    move_order!(s, "Turkey", "bul", "rum", false);
    convoy_order!(s, "Turkey", "bla", "rum", "con");
    move_order!(s, "Austria", "rum", "con", true);
    s.apply_orders();
    assert_unit!(s, "con", "Army Austria");
}

#[test]
fn test_datc_6c5() {
    let mut s = Stpsyr::new("data/standard.csv");
    move_order!(s, "France", "bre", "eng", false);
    move_order!(s, "France", "par", "bre", false);
    move_order!(s, "France", "mar", "bur", false);
    move_order!(s, "Germany", "mun", "ruh", false);
    move_order!(s, "England", "edi", "nth", false);
    s.apply_orders();
    move_order!(s, "France", "bur", "pic", false);
    move_order!(s, "Germany", "ruh", "bel", false);
    s.apply_orders();
    move_order!(s, "Germany", "bel", "pic", false);
    move_order!(s, "France", "pic", "bre", false);
    convoy_order!(s, "France", "eng", "bre", "bel");
    move_order!(s, "France", "bre", "bel", true);
    move_order!(s, "England", "nth", "eng", false);
    support_move_order!(s, "England", "lon", "nth", "eng");
    s.apply_orders();
    assert_unit!(s, "pic", "Army France");
}

#[test]
fn test_datc_6c6() {
    let mut s = Stpsyr::new("data/standard.csv");
    move_order!(s, "France", "bre", "eng", false);
    move_order!(s, "France", "par", "pic", false);
    move_order!(s, "England", "lon", "nth", false);
    move_order!(s, "England", "lvp", "yor", false);
    s.apply_orders();
    move_order!(s, "France", "pic", "bel", false);
    move_order!(s, "England", "yor", "lon", false);
    s.apply_orders();
    convoy_order!(s, "France", "eng", "bel", "lon");
    move_order!(s, "France", "bel", "lon", true);
    convoy_order!(s, "England", "nth", "lon", "bel");
    move_order!(s, "England", "lon", "bel", true);
    s.apply_orders();
    assert_unit!(s, "bel", "Army England");
}

#[test]
fn test_datc_6c7() {
    let mut s = Stpsyr::new("data/standard.csv");
    move_order!(s, "France", "mar", "bur", false);
    move_order!(s, "France", "bre", "eng", false);
    move_order!(s, "France", "par", "pic", false);
    move_order!(s, "England", "lon", "nth", false);
    move_order!(s, "England", "lvp", "yor", false);
    s.apply_orders();
    move_order!(s, "France", "pic", "bel", false);
    move_order!(s, "England", "yor", "lon", false);
    s.apply_orders();
    move_order!(s, "France", "bur", "bel", false);
    convoy_order!(s, "France", "eng", "bel", "lon");
    move_order!(s, "France", "bel", "lon", true);
    convoy_order!(s, "England", "nth", "lon", "bel");
    move_order!(s, "England", "lon", "bel", true);
    s.apply_orders();
    assert_unit!(s, "lon", "Army England");
}

#[test]
fn test_convoy() {
    let mut s = Stpsyr::new("data/standard.csv");
    move_order!(s, "Italy", "nap", "ion", false);
    move_order!(s, "Italy", "rom", "apu", false);
    s.apply_orders();
    convoy_order!(s, "Italy", "ion", "apu", "tun");
    move_order!(s, "Italy", "apu", "tun", true);
    s.apply_orders();
    assert_nonempty!(s, "tun");
}

#[test]
fn test_coast() {
    let mut s = Stpsyr::new("data/standard.csv");
    move_order!(s, "Turkey", "ank", "con", false);
    move_order!(s, "Turkey", "con", "bul", false);
    s.apply_orders();
    move_order!(s, "Turkey", "con", "bul/nc", false);
    move_order!(s, "Turkey", "bul", "gre", false);
    s.apply_orders();
    support_move_order!(s, "Turkey", "bul", "bud", "rum");
    move_order!(s, "Austria", "bud", "rum", false);
    move_order!(s, "Russia", "sev", "rum", false);
    s.apply_orders();
    assert_nonempty!(s, "rum");
}
