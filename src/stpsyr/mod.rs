extern crate csv;

use std::collections::HashSet;

mod types;
pub use self::types::*;
mod adjudicate;
pub use self::adjudicate::*;
mod adjusts;
pub use self::adjusts::*;
mod orders;
pub use self::orders::*;
mod parse;
pub use self::parse::*;
mod retreats;
pub use self::retreats::*;
mod util;
pub use self::util::*;

impl Stpsyr {

    pub fn new(mapfile: &'static str) -> Stpsyr {
        // parse input file as CSV to generate the map
        let mut reader = csv::Reader::from_file(mapfile).unwrap();

        let mut map: Vec<MapRegion> = Vec::new();
        for region in reader.decode::<(
                    String,          // 0 name
                    bool,            // 1 SC?
                    Option<String>,  // 2 starting owner
                    Option<String>,  // 3 starting unit type
                    String,          // 4 bordering provinces (fleets)
                    String           // 5 bordering provinces (armies)
                )>() {
            let region = region.unwrap();
            let province = Province::from(region.0.clone());

            let fleet_borders: Vec<Province> = region.4.split_whitespace().map(|p| {
                let mut border = Province::from(p);
                if let Some(coast) = province.coast {
                    border.from_coast = Some(coast);
                }
                border
            }).collect();
            let army_borders = region.5.split_whitespace().map(Province::from)
                .collect();

            if let Some(existing_region) = map.iter_mut()
                    .find(|r| r.province == province) {
                existing_region.fleet_borders.extend(fleet_borders.iter().cloned());
                continue;
            }

            map.push(MapRegion {
                province: province,
                sc: region.1,

                owner: region.2.clone().map(Power::from),
                home_power: region.2.clone().map(Power::from),
                unit: region.3.as_ref().map(|unit_type| Unit {
                    owner: Power::from(region.2.clone().unwrap()),
                    unit_type: match &unit_type[..] {
                        "Army" => UnitType::Army,
                        "Fleet" => UnitType::Fleet,
                        _ => panic!("unit type must be Army or Fleet")
                    }
                }),

                fleet_borders: fleet_borders,
                army_borders: army_borders
            });
        };

        Stpsyr {
            map: map,
            orders: vec![],
            retreats: vec![],
            adjusts: vec![],
            dependencies: vec![],
            dislodged: vec![],
            contested: HashSet::new(),
            phase: Phase::SpringDiplomacy,
            year: 1901
        }
    }

}
