use std::collections::HashMap;

use stpsyr::types::*;

impl Stpsyr {

    // get the unit currently in a province
    pub fn get_unit(&self, province: &Province) -> Option<Unit> {
        self.get_region(province).and_then(|r| r.unit.clone())
    }

    // get the MapRegion corresponding to a provence
    pub fn get_region(&self, province: &Province) -> Option<&MapRegion> {
        self.map.iter().find(|r| r.province == *province)
    }

    // get counts of SCs owned for each power
    pub fn sc_counts(&self) -> HashMap<Power, u32> {
        let mut counts = HashMap::new();
        for ref r in self.map.iter() {
            if r.sc {
                if let Some(ref p) = r.owner {
                    if let Some(count) = counts.get_mut(p) {
                        *count += 1;
                        continue;
                    }
                    counts.insert(p.clone(), 1);
                }
            }
        }
        counts
    }

    // get counts of units for each power
    pub fn unit_counts(&self) -> HashMap<Power, u32> {
        let mut counts = HashMap::new();
        for ref r in self.map.iter() {
            if let Some(ref u) = r.unit {
                if let Some(count) = counts.get_mut(&u.owner) {
                    *count += 1;
                    continue;
                }
                counts.insert(u.owner.clone(), 1);
            }
        }
        counts
    }

}
