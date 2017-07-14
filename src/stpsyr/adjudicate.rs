use stpsyr::types::*;

impl Stpsyr {

    // this is the function that actually moves units when their resolution is
    //   successful
    pub fn apply_resolved(&mut self) {
        // anything that got moved on top of (but maybe it also moved away)
        let mut dislodged: Vec<(Province, Unit)> = vec![];
        // anything that left an empty space (but maybe something also moved in)
        let mut moved_away: Vec<&Province> = vec![];

        let old_map = self.map.clone();
        for order in &self.orders {
            if let Action::Move { ref to, .. } = order.action {
                if order.resolution {
                    // we have a successful move
                    let from_idx = self.map.iter()
                        .position(|r| r.province == order.province).unwrap();
                    let to_idx = self.map.iter()
                        .position(|r| r.province == *to).unwrap();

                    if let Some(ref unit) = self.map[to_idx].unit {
                        dislodged.push((to.clone(), unit.clone()));
                    }

                    self.map[to_idx].unit = old_map[from_idx].unit.clone();

                    if to.coast.is_some() {
                        self.map[to_idx].province.coast =
                            self.map[to_idx].province.coast.and(to.coast);
                    }

                    moved_away.push(&order.province);
                } else {
                    self.contested.insert(order.province.clone());
                }
            }
        }

        // now we can do processing for dislodged and moved_away
        for region in &mut self.map {
            let p_dislodged = dislodged.iter().find(|d| d.0 == region.province);
            let p_moved_away = moved_away.contains(&&region.province);
            if let Some(dislodgement) = p_dislodged {
                if !p_moved_away {
                    // dislodged and not moved away: add it to the list
                    self.dislodged.push(dislodgement.clone());
                }
            } else if p_moved_away {
                // moved away and not dislodged: clear from map
                region.unit = None;
            }
        }
    }

    // this is the recursive resolve function, almost directly copied from
    //   http://diplom.org/Zine/S2009M/Kruijswijk/DipMath_Chp6.htm
    // it takes the id of an order and returns whether it was successful
    pub fn resolve(&mut self, id: usize) -> bool {
        match self.orders[id].state {
            // if order is already resolved, just return the resolution
            OrderState::RESOLVED => self.orders[id].resolution,
            OrderState::GUESSING => {
                // if we're guessing, add the order to the dependency list
                // and return the guess
                if !self.dependencies.contains(&id) {
                    self.dependencies.push(id);
                }
                self.orders[id].resolution
            },
            OrderState::UNRESOLVED => {
                let old_dep_count = self.dependencies.len();

                // start guessing
                self.orders[id].resolution = false;
                self.orders[id].state = OrderState::GUESSING;

                // adjudicate the order with the first guess
                let first_result = self.adjudicate(id);

                if self.dependencies.len() == old_dep_count {
                    // result is not dependent on a guess
                    match self.orders[id].state {
                        OrderState::RESOLVED => {},
                        _ => { self.orders[id].resolution = first_result; }
                    }
                    self.orders[id].state = OrderState::RESOLVED;
                    return first_result;
                }

                if self.dependencies[old_dep_count] != id {
                    // result is dependent on guess, but not our own
                    self.dependencies.push(id);
                    self.orders[id].resolution = first_result;
                    return first_result;
                }

                // result is dependent on our own guess, so let's guess again
                for dep in self.dependencies.drain(old_dep_count..) {
                    self.orders[dep].state = OrderState::UNRESOLVED;
                }
                self.orders[id].resolution = true;
                self.orders[id].state = OrderState::GUESSING;

                // adjudicate with the second guess
                let second_result = self.adjudicate(id);

                if first_result == second_result {
                    // only one resolution!
                    for dep in self.dependencies.drain(old_dep_count..) {
                        self.orders[dep].state = OrderState::UNRESOLVED;
                    }
                    self.orders[id].resolution = first_result;
                    self.orders[id].state = OrderState::RESOLVED;
                    return first_result;
                }

                // we have circular dependencies; use the backup rule
                self.backup_rule(old_dep_count);

                // start over in case backup rule hasn't resolved all orders
                self.resolve(id)
            }
        }
    }

    // this is what we call from resolve() to tell whether an order follows
    //   the equations
    fn adjudicate(&mut self, id: usize) -> bool {
        // the province being adjudicated
        let province = self.orders[id].province.clone();
        match self.orders[id].action.clone() {

            Action::Hold => {
                // a hold order never fails (what would that even mean)
                true
            },

            Action::Move { to, convoyed } => {
                let attack_strength = self.attack_strength(&province);

                // the attack strength (above) needs to be greater than this
                let counter_strength = if !convoyed && self.orders.iter().any(|o|
                        match o.action {
                            Action::Move { to: ref move_to, convoyed } =>
                                province == *move_to && !convoyed,
                            _ => false
                        } && o.province == to) {
                    // head-to-head battle
                    self.defend_strength(&to)
                } else {
                    // no head-to-head battle
                    self.hold_strength(&to)
                };

                // it also needs to be greater than the prevent strength of all
                //   units moving to the same space
                let contesting_orders = self.orders.iter().filter(|o|
                    match o.action {
                        Action::Move { to: ref move_to, .. } =>
                            to == *move_to,
                        _ => false
                    } && o.province != province).map(|o| o.province.clone())
                    .collect::<Vec<Province>>();

                // return whether it satisfies both these conditions
                attack_strength > counter_strength && contesting_orders.iter()
                    .all(|p| attack_strength > self.prevent_strength(p))
            },

            Action::SupportHold { to } | Action::SupportMove { to, .. } => {
                // a support is cut when...
                !self.orders.clone().iter().any(|o|
                    match o.action {
                        Action::Move { to: ref move_to, convoyed } =>
                            // ... something with a valid path attacks it...
                            province == *move_to && if convoyed {
                                !self.convoy_paths(o).is_empty()
                            } else { true },
                        _ => false
                    } &&
                    // ... and it's not the thing being supported (in)to...
                    o.province != to &&
                    // ... , and you can't cut your own support
                    o.owner != self.orders[id].owner)
            },

            Action::Convoy { .. } => {
                // a convoy only fails when it is dislodged
                !self.orders.clone().iter().any(|o|
                    match o.action {
                        Action::Move { to: ref move_to, .. } => {
                            province == *move_to
                        },
                        _ => false
                    } && self.resolve(o.id))
            },

        }
    }

    // this returns all valid paths a convoyed army can go through to get to
    //   its destination, taking into account dislodged fleets
    fn convoy_paths(&mut self, order: &Order) -> Vec<Vec<Province>> {
        match order.action {
            Action::Move { ref to, convoyed } => { if convoyed {

                // first, find all paths at all through water that get from
                //   the province of the order to the destination
                let paths: Vec<Vec<Province>> = self.find_paths(
                    vec![self.get_region(&order.province).unwrap()], to)
                    .iter().map(|path|
                        path.iter().map(|r| r.province.clone()).collect()
                    ).collect();

                // now filter those paths for the ones that are actually valid
                paths.iter().filter(|path| {
                    path.iter().skip(1).all(|&ref p|
                        // for every convoying fleet...
                        self.orders.clone().iter().any(|o|
                            o.province == *p && match o.action {
                                // ... there has to be a convoy order
                                Action::Convoy { ref from, to: ref c_to } => {
                                    *from == order.province && *to == *c_to
                                }, _ => false
                            } && self.resolve(o.id)  // ... and it must succeed
                        )
                    )
                }).cloned().collect()

            } else { panic!("convoy_paths called on non-convoyed Move"); } },
            _ => panic!("convoy_paths called on non-Move")
        }
    }

    // utility function used from convoy_paths (see above)
    fn find_paths<'a>(&'a self, path: Vec<&'a MapRegion>, target: &Province)
            -> Vec<Vec<&MapRegion>> {
        // the "end" of the current chain
        let region = path.last().unwrap().clone();
        // if we've made it already, return
        if region.fleet_borders.contains(target) { return vec![path]; }
        // otherwise, find the next fleet in the chain
        self.map.iter().filter(|&r|
                // it's empty water if we can move to it as a fleet but can't
                // move to it as an army
                region.fleet_borders.contains(&r.province) &&
                !region.army_borders.contains(&r.province) &&
                // check for the presence of the appropriate order
                self.orders.iter().any(|o|
                    o.province == r.province && match o.action {
                        Action::Convoy { ref from, ref to } => {
                            *from == path[0].province && *to == *target
                        }, _ => false
                    }
                ) &&
                // we also need to make sure we don't get in an infinite loop
                !path.contains(&r)).flat_map(|r| {
                    // add the next fleet to the path
                    let mut new_path = path.clone();
                    new_path.push(r);
                    // and recurse
                    self.find_paths(new_path, target)
                }).collect()
    }

    fn hold_strength(&mut self, province: &Province) -> usize {
        if self.get_unit(province).is_some() {
            // figure out if the unit in this region is moving away
            let move_id = self.orders.iter().find(|o|
                match o.action {
                    Action::Move { .. } => true, _ => false
                } && o.province == *province).map(|o| o.id);

            if let Some(move_id) = move_id {
                // if the unit moves away successfully, we treat the province
                //   as empty. otherwise, it always has hold strength of 1,
                //   regardless of support
                if self.resolve(move_id) { 0 } else { 1 }
            } else {
                // hold strength is 1 plus the number of successful orders to
                //   support hold
                1 + self.orders.clone().iter().filter(|o|
                    match o.action {
                        Action::SupportHold { ref to } => *to == *province,
                        _ => false
                    } && self.resolve(o.id)).count()
            }
        } else {
            // the hold strength of an empty province is always 0
            0
        }
    }

    fn attack_strength(&mut self, province: &Province) -> usize {
        // first, if there's no move order, attack strength doesn't make sense
        // otherwise, use it to find the destination and whether it's a convoy
        let move_order = if let Some(move_order) = self.orders.iter().find(|o|
                match o.action {
                    Action::Move { .. } => true, _ => false
                } && o.province == *province) { move_order }
            else { panic!("attack_strength called on non-Move"); }.clone();
        let (dest, convoyed) = match move_order.action {
            Action::Move { ref to, convoyed } => (to, convoyed),
            _ => unreachable!()
        };

        // attack strength is 0 if the path is invalid
        if convoyed && self.convoy_paths(&move_order).is_empty() { return 0; }

        // now we check to see whether the unit at the destination has moved
        //   away, given that it's not a head-to-head battle. this is important
        //   because we cannot call resolve if it is one, as that would cause
        //   the recursion to become infinite
        let move_id = self.orders.iter().find(|o|
            (convoyed || match o.action {
                Action::Move { ref to, convoyed } => convoyed || *to != *province,
                _ => false
            }) && o.province == *dest).map(|o| o.id);
        let moved_away = move_id.map_or(false, |id| self.resolve(id));

        // we also figure out which power we're attacking
        let attacked_power = if moved_away {
            None
        } else {
            self.get_region(dest)
                .and_then(|r| r.clone().unit.map(|u| u.owner.clone()))
        };

        // because if we attack ourselves, attack strength is always 0
        if attacked_power == Some(move_order.owner) { return 0; }

        // otherwise, attack strength is 1 plus the number of successful orders
        //   to support the move
        let supports: Vec<usize> = self.orders.iter().filter(|o|
            match o.action {
                Action::SupportMove { ref from, ref to } =>
                    *from == *province && *to == *dest,
                _ => false
            } &&
            attacked_power.as_ref().map_or(true, |power| *power != o.owner))
            .map(|o| o.id).collect();

        1 + supports.iter().filter(|&id| self.resolve(*id)).count()
    }

    fn defend_strength(&mut self, province: &Province) -> usize {
        // similar to attack strength, first find the move in question
        let move_order = if let Some(move_order) = self.orders.iter().find(|o|
                match o.action {
                    Action::Move { .. } => true, _ => false
                } && o.province == *province) { move_order }
            else { panic!("defend_strength called on non-Move"); }.clone();
        let dest = match move_order.action {
            Action::Move { ref to, .. } => to,
            _ => unreachable!()
        };

        // defend strength is just 1 plus number of successful support moves
        let supports: Vec<usize> = self.orders.iter().filter(|o|
            match o.action {
                Action::SupportMove { ref from, ref to } =>
                    *from == *province && *to == *dest,
                _ => false
            }).map(|o| o.id).collect();

        1 + supports.iter().filter(|&id| self.resolve(*id)).count()
    }

    fn prevent_strength(&mut self, province: &Province) -> usize {
        // same as always...
        let move_order = if let Some(move_order) = self.orders.iter().find(|o|
                match o.action {
                    Action::Move { .. } => true, _ => false
                } && o.province == *province) { move_order }
            else { panic!("prevent_strength called on non-Move"); }.clone();
        let (dest, convoyed) = match move_order.action {
            Action::Move { ref to, convoyed } => (to, convoyed),
            _ => unreachable!()
        };

        // prevent strength also requires a successful path in case of convoy
        if convoyed && self.convoy_paths(&move_order).is_empty() { return 0; }

        // if we're in a head-to-head battle and lose, prevent strength is 0
        let move_id = self.orders.iter().find(|o|
            match o.action {
                Action::Move { ref to, convoyed } => !convoyed && *to == *province,
                _ => false
            } && o.province == *dest).map(|o| o.id);
        if let Some(move_id) = move_id {
            if !convoyed && self.resolve(move_id) { return 0; }
        }

        // otherwise, 1 plus number of successful support moves
        let supports: Vec<usize> = self.orders.iter().filter(|o|
            match o.action {
                Action::SupportMove { ref from, ref to } =>
                    *from == *province && *to == *dest,
                _ => false
            }).map(|o| o.id).collect();

        1 + supports.iter().filter(|&id| self.resolve(*id)).count()
    }

    fn backup_rule(&mut self, old_dep_count: usize) {
        let dependencies = self.dependencies.drain(old_dep_count..)
            .collect::<Vec<usize>>();
        let (mut only_moves, mut convoys) = (true, false);

        for &dep in &dependencies {
            match self.orders[dep].action {
                Action::Move { .. } => {},
                Action::Convoy { .. } => {
                    only_moves = false;
                    convoys = true;
                },
                _ => only_moves = false
            }
        }

        for &dep in &dependencies {
            if only_moves {
                // circular movement---make everything succeed
                self.orders[dep].resolution = true;
                self.orders[dep].state = OrderState::RESOLVED;
            } else if convoys {
                // convoy paradox---make convoy fail as per Szykman
                let is_convoy = match self.orders[dep].action {
                    Action::Convoy { .. } => true, _ => false
                };
                if is_convoy {
                    self.orders[dep].resolution = false;
                    self.orders[dep].state = OrderState::RESOLVED;
                } else {
                    self.orders[dep].state = OrderState::UNRESOLVED;
                }
            } else {
                panic!("unknown circular dependency");
            }
        }
    }

}
