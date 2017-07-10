use std::collections::HashSet;

use stpsyr::types::*;

impl Stpsyr {

    // the publicly exposed function to modify self.retreats
    pub fn add_retreat(&mut self, owner: Power, province: Province, action: RetreatAction) {
        // TODO refactor this method to get rid of repetition from verification
        //   used in add_order
        // TODO can't retreat to the place that attacked you

        match self.phase {
            Phase::SpringRetreats | Phase::FallRetreats => {},
            _ => panic!("add_retreat called during non-retreat phase")
        };

        // there has to be a unit that was dislodged here to order it
        let unit = if let Some(unit) = self.dislodged.iter().find(|&&(ref p, _)|
                p == &province).map(|&(_, ref u)| u.clone()) { unit }
            else { return; };

        // can't order a unit that's not yours
        if unit.owner != owner { return; }

        // can't order to a province you can't reach or a province that was
        //   contested during the last diplomacy phase
        if match &action {
            &RetreatAction::Move { ref to } => {
                let r = self.get_region(&province).unwrap();
                self.contested.contains(to) || !match unit.unit_type {
                    UnitType::Army => r.army_borders.clone(),
                    UnitType::Fleet => r.fleet_borders.clone().into_iter()
                        .filter(|p|
                            p.from_coast == r.province.coast &&
                            p.coast == to.coast)
                        .collect()
                }.contains(&to)
            },
            _ => false
        } { return; }

        self.retreats.push(Retreat {
            owner: owner,
            province: province,
            action: action
        });
    }

    // the publicly exposed function that is called once all retreats have been
    //   added
    // TODO this breaks if multiple retreat orders are submitted for a single
    //   province. and yes, that should definitely be handled in add_retreat,
    //   but I'm lazy and don't wanna scroll all the way back up there to add
    //   the comment in the right place
    pub fn apply_retreats(&mut self) {
        self.dislodged = vec![];

        {
            // we need a new scope for these to release the borrows later
            let (mut attempts, mut conflicts) = (HashSet::new(), HashSet::new());

            for retreat in self.retreats.iter() {
                match &retreat.action {
                    &RetreatAction::Move { ref to } => {
                        if attempts.contains(to) { conflicts.insert(to); }
                        else { attempts.insert(to); }
                    },
                    _ => {}
                }
            }

            for retreat in self.retreats.iter() {
                match &retreat.action {
                    &RetreatAction::Move { ref to } => {
                        if !conflicts.contains(to) {
                            // process the retreat
                            let from_idx = self.map.iter()
                                .position(|r| r.province == retreat.province).unwrap();
                            let to_idx = self.map.iter()
                                .position(|r| r.province == *to).unwrap();
                            assert!(self.map[to_idx].unit.is_none());
                            self.map[to_idx].unit = self.map[from_idx].unit.clone();
                        }
                    },
                    // handle disbands as if they were NMRs - no difference anyway
                    &RetreatAction::Disband => {}
                }
            }
        }

        self.next_phase();
        self.retreats = vec![];
    }

}
