use stpsyr::types::*;

impl Stpsyr {

    // the publicly exposed function to modify self.adjusts
    pub fn add_adjust(&mut self, owner: Power, province: Province, action: AdjustAction) {
        match self.phase {
            Phase::Builds => {},
            _ => panic!("add_adjust called during non-build phase")
        }

        // find difference in SC and unit counts
        let delta =
            if let Some(count) = self.sc_counts().get(&owner) { *count as i32 }
                else { return; } -
            if let Some(count) = self.unit_counts().get(&owner) { *count as i32 }
                else { return; };

        // find existing number of adjust orders for this power
        let mut dup = false;
        let num = self.adjusts.iter()
            .filter(|&&Adjust { owner: ref o, province: ref p, action: _ }|
                    if owner == *o && province == *p {
                        dup = true; true
                    } else { owner == *o }).count() as i32;

        // fail if we're not allowed to build or destroy at all
        if dup || match action {
            AdjustAction::Disband => delta >= 0 || -num == delta,
            AdjustAction::Build { unit_type: _ } => delta <= 0 || num == delta
        } { return; }

        // now we have to check if the given province is a valid one to build/
        // destroy in
        let region = self.map.iter().find(|r| r.province == province).unwrap();
        if !match action {
            AdjustAction::Disband => region.unit.as_ref()
                .map_or(false, |u| u.owner == owner),
            AdjustAction::Build { unit_type: t } => region.unit.is_none() &&
                region.home_power.as_ref().map_or(false, |&ref p| *p == owner) &&
                match t {
                    UnitType::Army => !region.army_borders.is_empty(),
                    UnitType::Fleet => !region.fleet_borders.is_empty()
                }
        } { return; }

        // everything's good
        self.adjusts.push(Adjust {
            owner: owner,
            province: province,
            action: action
        });
    }

    // the publicly exposed function that is called once all adjusts have been
    //   added
    pub fn apply_adjusts(&mut self) {
        for adjust in self.adjusts.iter() {
            let region = self.map.iter_mut()
                .find(|r| r.province == adjust.province).unwrap();
            match adjust.action {
                AdjustAction::Disband => region.unit = None,
                AdjustAction::Build { unit_type } => region.unit = Some(Unit {
                    owner: adjust.owner.clone(), unit_type: unit_type
                })
            }
        }

        self.next_phase();
        self.adjusts = vec![];
    }

}
