extern crate stpsyr;
use stpsyr::*;

fn main() {
    let mut s = Stpsyr::new("data/standard.csv");
    s.add_order(Power::from("Italy"), Province::from("ven"), Action::Move { to: Province::from("tyr"), convoyed: false });
    s.add_order(Power::from("Austria"), Province::from("tri"), Action::Move { to: Province::from("tyr"), convoyed: false });
    s.apply_orders();
}
