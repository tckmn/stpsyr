extern crate stpsyr;
use stpsyr::*;

fn main() {
    let mut s = Stpsyr::new("data/standard.csv");
    s.add_order(String::from("Italy"), String::from("ven"), Action::Move { to: String::from("tyr"), convoyed: false });
    s.add_order(String::from("Austria"), String::from("tri"), Action::Move { to: String::from("tyr"), convoyed: false });
    s.apply_orders();
}
