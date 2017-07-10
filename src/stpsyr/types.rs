use std::fmt;
use std::cmp;

use std::collections::HashSet;

// the only information attached to a Unit is its owner and type
// ex. "Austrian fleet"
#[derive(Clone)]
pub struct Unit {
    pub owner: Power,
    pub unit_type: UnitType
}
impl fmt::Debug for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {:?}", self.unit_type, self.owner)
    }
}
#[derive(Clone,Copy,Debug,PartialEq)]
pub enum UnitType { Army, Fleet }

// a Province is an extension of a String, partially for semantics, but also
//   because we need to take coasts into account when enumerating borders
#[derive(Clone,Eq,Hash)]
pub struct Province {
    pub name: String,
    pub coast: Option<char>,
    pub from_coast: Option<char>
}
impl fmt::Debug for Province {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.name,
            self.coast.map_or(String::new(), |coast| format!("/{}c", coast)),
            self.from_coast.map_or(String::new(), |coast| format!(" [from {}c]", coast)))
    }
}
impl From<String> for Province {
    fn from(s: String) -> Province {
        if let Some(idx) = s.find('/') {
            let mut s = s;
            let coast = s.chars().nth(idx + 1);
            s.truncate(idx);
            Province { name: s, coast: coast, from_coast: None }
        } else {
            Province { name: s, coast: None, from_coast: None }
        }
    }
}
impl<'a> From<&'a str> for Province {
    fn from(s: &str) -> Province {
        Province::from(s.to_string())
    }
}
impl cmp::PartialEq for Province {
    fn eq(&self, other: &Province) -> bool {
        self.name == other.name
    }
}


// a Power is simply a wrapper around a String for semantics
// ex. Germany, Austria
#[derive(Clone,Eq,Hash)]
pub struct Power {
    pub name: String
}
impl fmt::Debug for Power {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl From<String> for Power {
    fn from(s: String) -> Power {
        Power { name: s }
    }
}
impl<'a> From<&'a str> for Power {
    fn from(s: &str) -> Power {
        Power::from(s.to_string())
    }
}
impl cmp::PartialEq for Power {
    fn eq(&self, other: &Power) -> bool {
        self.name.to_lowercase() == other.name.to_lowercase()
    }
}

// a MapRegion is a location on the map, storing the province, whether it's an
//   SC, its current owner, the unit in it (not necessarily with the same owner
//   as the region), and its borders (stored separately for fleets and armies)
#[derive(Clone)]
pub struct MapRegion {
    pub province: Province,
    pub sc: bool,
    pub owner: Option<Power>,
    pub home_power: Option<Power>,
    pub unit: Option<Unit>,
    pub fleet_borders: Vec<Province>,
    pub army_borders: Vec<Province>
}
impl fmt::Debug for MapRegion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}{}{}{}",
            self.province,
            if self.sc { "*" } else { "" },
            self.owner.as_ref().map_or(String::new(), |o| format!(" ({:?})", o)),
            self.unit.as_ref().map_or(String::new(), |o| format!(" [{:?}]", o)))
    }
}
impl cmp::PartialEq for MapRegion {
    fn eq(&self, other: &MapRegion) -> bool {
        self.province == other.province
    }
}

// here are some utility types for the Order struct
#[derive(Clone,Debug,PartialEq)]
pub enum OrderState { UNRESOLVED, GUESSING, RESOLVED }
#[derive(Clone,Debug)]
pub enum Action {
    Hold,
    Move { to: Province, convoyed: bool },
    SupportHold { to: Province },
    SupportMove { from: Province, to: Province },
    Convoy { from: Province, to: Province }
}

// an Order stores the power that ordered it, which province is being ordered,
//   the actual order (action), and some meta information for the resolve() and
//   adjudicate() functions
// it is separate from a Retreat and an Adjust
#[derive(Clone,Debug)]
pub struct Order {
    pub owner: Power,
    pub province: Province,
    pub action: Action,
    pub resolution: bool,
    pub state: OrderState,
    pub id: usize
}

// utility type for Retreat, corresponding to Action for Order
pub enum RetreatAction {
    Disband,
    Move { to: Province }
}

// a Retreat stores the power that ordered it, which province to retreat from,
//   and what to do with it (disband or move)
pub struct Retreat {
    pub owner: Power,
    pub province: Province,
    pub action: RetreatAction
}

pub enum AdjustAction {
    Disband,
    Build { unit_type: UnitType }
}

// a Adjust stores the power that ordered it, which province to build/destroy
// in, and what to do there (disband or build a unit)
pub struct Adjust {
    pub owner: Power,
    pub province: Province,
    pub action: AdjustAction
}

// fairly self-explanatory
#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Phase {
    SpringDiplomacy,
    SpringRetreats,
    FallDiplomacy,
    FallRetreats,
    Builds
}

// this is the main struct (duh)
pub struct Stpsyr {
    pub map: Vec<MapRegion>,
    pub orders: Vec<Order>,
    pub retreats: Vec<Retreat>,
    pub adjusts: Vec<Adjust>,
    pub dependencies: Vec<usize>,
    pub dislodged: Vec<(Province, Unit)>,
    pub contested: HashSet<Province>,
    pub phase: Phase,
    pub year: i32
}
