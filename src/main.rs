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

fn main() {
    let mut s = Stpsyr::new("data/standard.csv");

    // let (italy, austria, germany) =
    //     (Power::from("Italy"), Power::from("Austria"), Power::from("Germany"));

    // // spring
    // s.parse(&italy, "
    //     F Nap-ION
    //     A Rom-Ven
    //     A Ven-Tyr".to_string());
    // s.parse(&austria, "
    //     A Vie-Tyr".to_string());
    // s.parse(&germany, "
    //     A Mun S A Ven-Tyr".to_string());
    // s.apply();

    // // autumn
    // s.parse(&italy, "
    //     A Tyr-Tri
    //     A Ven S A Tyr-Tri
    //     F ION-Tun".to_string());
    // s.apply();

    // s.parse(&austria, "
    //     Tri-Alb".to_string());
    // s.apply();

    // // winter
    // s.parse(&austria, "
    //     D Alb".to_string());
    // s.parse(&italy, "
    //     B A Rom
    //     B F Nap".to_string());

    s.render_svg("meems.svg".to_string()).unwrap();
}
