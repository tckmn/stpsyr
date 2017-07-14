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

fn test_from_file(filename: &str) {
    let err_msg = "error parsing test cases";
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
