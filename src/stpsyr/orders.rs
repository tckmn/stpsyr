use stpsyr::types::*;

impl Stpsyr {

    // the publicly exposed function to modify self.orders
    pub fn add_order(&mut self, owner: Power, province: Province, action: Action) {
        match self.phase {
            Phase::SpringDiplomacy | Phase::FallDiplomacy => {},
            _ => panic!("add_order called during non-diplomacy phase")
        };

        // there has to be a unit here to order it
        let unit = if let Some(unit) = self.get_unit(&province) { unit }
            else { return; };

        let (is_move, convoyed) = match action {
            Action::Move { ref to, convoyed } => {
                // let's do a quick check here: unit can't move to itself
                if province == *to { return; }
                (true, convoyed)
            },
            Action::SupportMove { ref from, ref to } => {
                // another quick check: can't support yourself or a non-move
                if province == *from || province == *to || *from == *to { return; }
                (false, false)
            }
            _ => (false, false)
        }; // NOTE use this better

        // can't convoy a fleet
        if convoyed && unit.unit_type == UnitType::Fleet { return; }

        // can't order a unit that's not yours
        if unit.owner != owner { return; }

        // can't order to a province you can't reach
        if !convoyed && match &action {
            &Action::Move { ref to, convoyed: _ } |
            &Action::SupportHold { ref to } |
            &Action::SupportMove { from: _, ref to } => {
                let r = self.get_region(&province).unwrap();
                !match unit.unit_type {
                    UnitType::Army => r.army_borders.clone(),
                    UnitType::Fleet => r.fleet_borders.clone().into_iter()
                        .filter(|p|
                            p.from_coast == r.province.coast &&
                            (!is_move || p.coast == to.coast))
                        .collect()
                }.contains(&to)
            },
            _ => false
        } { return; }

        // all checks pass
        let id = self.orders.len();
        self.orders.push(Order {
            owner: owner,
            province: province,
            action: action,
            resolution: false,
            state: OrderState::UNRESOLVED,
            id: id
        });
    }

    // this is the publicly exposed function that is called once all orders
    //   have been added
    // TODO support retreats and builds
    // TODO clear self.contested
    pub fn apply_orders(&mut self) -> Vec<(Province, Unit)> {
        // resolve all orders
        for i in 0..self.orders.len() {
            self.resolve(i);
            assert!(self.orders[i].state == OrderState::RESOLVED);
            println!("{:?}", self.orders[i]);
        }

        // do the moves that were successfully resolved
        self.apply_resolved();

        // update phase data
        self.phase = match self.phase {
            Phase::SpringDiplomacy => if self.dislodged.is_empty() {
                Phase::FallDiplomacy
            } else {
                Phase::SpringRetreats
            },
            Phase::SpringRetreats => Phase::FallDiplomacy,
            Phase::FallDiplomacy | Phase::FallRetreats =>
                if self.phase == Phase::FallRetreats || self.dislodged.is_empty() {
                    if self.sc_counts() != self.unit_counts() {
                        Phase::Builds
                    } else {
                        Phase::SpringDiplomacy
                    }
                } else {
                    Phase::FallRetreats
                },
            Phase::Builds => { self.year += 1; Phase::SpringDiplomacy }
        };

        println!("{:?} {}: {:?}", self.phase, self.year, self.map);

        // clear out orders, return dislodged units
        self.orders = vec![];
        self.dislodged.clone()
    }

}
