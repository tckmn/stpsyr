/*
 * stpsyr - a Diplomacy adjudicator in Rust
 * Copyright (C) 2017  Keyboard Fire <andy@keyboardfire.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

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
        $s.parse(String::from($orders));
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
    let mut s = Stpsyr::new("data/standard.csv");
    let mut power = None;
    for line in file.lines() {
        let line = line.unwrap();
        match line.chars().next() {
            Some('/') => {},
            Some('#') => {
                title = line.chars().skip(2).collect();
                println!("begin test for test case \"{}\"", title);
                s = Stpsyr::new("data/standard.csv");
            },
            None => {
                if power.is_some() {
                    s.apply();
                    power = None;
                }
            },
            Some(ch) => {
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
                } else if ch == ' ' {
                    s.parse(power.as_ref().unwrap(), line);
                } else {
                    power = Some(Power::from(line));
                }
            }
        }
    }
}

#[test]
fn test_datc_6a() { test_from_file("tests/datc-6.a.txt"); }
#[test]
fn test_datc_6b() { test_from_file("tests/datc-6.b.txt"); }
#[test]
fn test_datc_6c() { test_from_file("tests/datc-6.c.txt"); }
#[test]
fn test_datc_6d() { test_from_file("tests/datc-6.d.txt"); }
#[test]
fn test_datc_6e() { test_from_file("tests/datc-6.e.txt"); }

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
    move_order!(s, "Turkey", "con", "bul/ec", false);
    move_order!(s, "Turkey", "bul", "gre", false);
    s.apply_orders();
    s.apply_adjusts();
    support_move_order!(s, "Turkey", "bul", "bud", "rum");
    move_order!(s, "Austria", "bud", "rum", false);
    move_order!(s, "Russia", "sev", "rum", false);
    s.apply_orders();
    assert_nonempty!(s, "rum");
}
