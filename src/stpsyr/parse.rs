use stpsyr::types::*;

impl Stpsyr {

    // parse orders as a string and apply them
    pub fn parse_orders(&mut self, orders: String) {
        let mut power = Power::from(String::new());

        for line in orders.lines() {
            let line = line.to_lowercase()
                .replace('-', " ")
                .replace(" m ", " ")
                .replace(" move ", " ")
                .replace(" move to ", " ")
                .replace(" moves ", " ")
                .replace(" moves to ", " ")
                .replace('(', " ")
                .replace(')', " ")
                .replace(" support ", " s ")
                .replace(" supports ", " s ")
                .replace("via convoy", "vc")
                .replace(" convoy ", " c ")
                .replace(" convoys ", " c ");
            let tokens: Vec<&str> = line.split_whitespace().collect();
            if tokens.is_empty() { continue; }
            else if tokens.len() == 1 {
                power = Power::from(tokens.into_iter().next().unwrap());
                continue;
            } else {
                let mut tokens_iter = tokens.iter().filter(|&token|
                    *token != "a" &&
                    *token != "army" &&
                    *token != "f" &&
                    *token != "fleet" &&
                    *token != "h" &&
                    *token != "hold" &&
                    *token != "holds" &&
                    *token != "stand" &&
                    *token != "stands");
                let province = Province::from(*tokens_iter.next().unwrap());
                match tokens_iter.next() {
                    None => {}, // hold
                    Some(token2) => { match *token2 {
                        "s" => {
                            // support
                            let a = tokens_iter.next().unwrap();
                            if let Some(b) = tokens_iter.next() {
                                // support move
                                self.add_order(power.clone(), province, Action::SupportMove {
                                    from: Province::from(*a), to: Province::from(*b)
                                });
                            } else {
                                // support hold
                                self.add_order(power.clone(), province, Action::SupportHold {
                                    to: Province::from(*a)
                                });
                            }
                        },
                        "c" => {
                            // convoy
                            let from = tokens_iter.next().unwrap();
                            let to = tokens_iter.next().unwrap();
                            self.add_order(power.clone(), province, Action::Convoy {
                                from: Province::from(*from), to: Province::from(*to)
                            });
                        },
                        _ => {
                            // regular move
                            let vc = tokens_iter.next().map_or(false, |token|
                                *token == "vc");
                            self.add_order(power.clone(), province, Action::Move {
                                to: Province::from(*token2), convoyed: vc
                            });
                        }
                    } }
                }
            }
        }
    }

}
