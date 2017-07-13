[![travis build status](https://travis-ci.org/KeyboardFire/stpsyr.svg?branch=master)](https://travis-ci.org/KeyboardFire/stpsyr) [![current crates.io version](https://img.shields.io/crates/v/stpsyr.svg)](https://crates.io/crates/stpsyr)

**stpsyr** is an adjudicator for the Diplomacy board game, written in Rust.
Currently, it fully supports adjudication of human-readable orders, and it can
generate maps of the current state of the board in SVG format. The DATC test
cases are partially implemented.

Planned features include variant maps, better map drawing (e.g. with arrows
that show the moves from the previous phase), and a web-based client/server
that allows Backstabbr-style input of orders.

The adjudication algorithm is taken from Lucas Kruijswijk's
[The Math of Adjudication](http://www.diplomatic-pouch.org/Zine/S2009M/Kruijswijk/DipMath_Chp1.htm).

stpsyr is licensed under the GNU GPL 3.0.
