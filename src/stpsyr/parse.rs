use stpsyr::types::*;

impl Stpsyr {

    // parse orders as a string and apply them
    pub fn parse(&mut self, power: &Power, orders: String) {
        match self.phase {
            Phase::SpringDiplomacy | Phase::FallDiplomacy =>
                self.parse_orders(power, orders),
            Phase::SpringRetreats | Phase::FallRetreats =>
                self.parse_retreats(power, orders),
            Phase::Builds =>
                self.parse_adjusts(power, orders)
        }
    }

    pub fn apply(&mut self) {
        match self.phase {
            Phase::SpringDiplomacy | Phase::FallDiplomacy =>
                self.apply_orders(),
            Phase::SpringRetreats | Phase::FallRetreats =>
                self.apply_retreats(),
            Phase::Builds =>
                self.apply_adjusts()
        }
    }

    fn parse_orders(&mut self, power: &Power, orders: String) {
        for line in orders.lines() {
            let line = line.to_lowercase()
                .replace('(', "/")
                .replace(" /", "/");
            let tokens: Vec<&str> = line
                .split(|c: char| !(c.is_lowercase() || c == '/') )
                .collect();
            let mut tokens_iter = tokens.iter().filter(|&token|
                    (token.len() >= 3 ||
                     *token == "s" || *token == "c" || *token == "vc") &&
                    *token != "army" && *token != "fleet" &&
                    *token != "hold" && *token != "holds" &&
                    *token != "stand" && *token != "stands" &&
                    *token != "move" && *token != "moves" &&
                    *token != "the" && *token != "coast" && *token != "via"
                ).map(|&token| match token {
                    "support" | "supports" => "s",
                    "convoy" | "convoys" | "vc" => "c",
                    "north" => "nc",
                    "south" => "sc",
                    "east" => "ec",
                    "west" => "wc",
                    _ => token
                });
            let province = if let Some(p) = tokens_iter.next() {
                Province::from(p)
            } else { continue };
            match tokens_iter.next() {
                None => {}, // hold
                Some(token2) => { match token2 {
                    "s" => {
                        // support
                        let a = tokens_iter.next().unwrap();
                        if let Some(b) = tokens_iter.next() {
                            // support move
                            self.add_order(power.clone(), province,
                            Action::SupportMove {
                                from: Province::from(a), to: Province::from(b)
                            });
                        } else {
                            // support hold
                            self.add_order(power.clone(), province,
                            Action::SupportHold {
                                to: Province::from(a)
                            });
                        }
                    },
                    "c" => {
                        // convoy
                        let from = tokens_iter.next().unwrap();
                        let to = tokens_iter.next().unwrap();
                        self.add_order(power.clone(), province, Action::Convoy {
                            from: Province::from(from), to: Province::from(to)
                        });
                    },
                    _ => {
                        // regular move
                        let vc = tokens_iter.next().map_or(false, |token|
                            token == "c");
                        self.add_order(power.clone(), province, Action::Move {
                            to: Province::from(token2), convoyed: vc
                        });
                    }
                } }
            }
        }
    }

    fn parse_retreats(&mut self, power: &Power, orders: String) {
        unimplemented!();
    }

    fn parse_adjusts(&mut self, power: &Power, orders: String) {
        unimplemented!();
    }

}
