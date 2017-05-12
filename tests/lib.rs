extern crate stpsyr;
use stpsyr::*;

use std::io::{BufRead, BufReader};
use std::fs::File;

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

fn test_from_file(filename: &str) {
    let err_msg = "bad thing happen ono"; // TODO ...
    let f = File::open(filename).expect(err_msg);
    let file = BufReader::new(&f);
    let mut title = String::new();
    let mut cache = String::new();
    let mut s = Stpsyr::new("data/standard.csv");
    for line in file.lines() {
        let line = line.unwrap();
        match line.chars().next() {
            Some('/') => {},
            Some('#') => {
                title = line.chars().skip(2).collect();
                s = Stpsyr::new("data/standard.csv");
            },
            None => {
                println!("CACHE: {:?}", cache);
                if !cache.is_empty() {
                    s.parse_orders(cache);
                    s.apply_orders();
                    cache = String::new();
                }
            },
            _ => {
                if line.contains(':') {
                    let mut parts = line.split(": ");
                    let province = parts.next().expect(err_msg);
                    let real_unit = s.get_unit(&Province::from(province))
                        .map_or(String::from("empty"), |u| format!("{:?}", u));
                    let assert_unit = parts.next().expect(err_msg);
                    if parts.next().is_some() { panic!(err_msg); }

                    if real_unit != assert_unit {
                        panic!("file {}, test \"{}\": in {}, expected {}, found {}",
                            filename, title, province, assert_unit, real_unit);
                    }
                } else {
                    cache = format!("{}{}\n", cache, line);
                }
            }
        }
    }
}

#[test]
fn test_datc_6a() {
    test_from_file("tests/datc-6.a.txt");
}

#[test]
fn test_datc_6b() {
    test_from_file("tests/datc-6.b.txt");
}

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
