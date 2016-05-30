extern crate csv;

use std::fmt;

#[derive(Clone,Copy,Debug)]
pub enum UnitType { Army, Fleet }

#[derive(Clone)]
pub struct Unit {
    pub owner: String,
    pub unit_type: UnitType
}
impl fmt::Debug for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {}", self.unit_type, self.owner)
    }
}

#[derive(Clone)]
struct Province {
    name: String,
    sc: bool,
    owner: Option<String>,
    unit: Option<Unit>,
    fleet_borders: Vec<String>,
    army_borders: Vec<String>
}
impl fmt::Debug for Province {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}{}",
               self.name,
               if self.sc { "*" } else { "" },
               self.owner.as_ref().map_or(String::new(), |o| format!(" ({})", o)),
               self.unit.as_ref().map_or(String::new(), |o| format!(" [{:?}]", o)))
    }
}

#[derive(Clone,Debug)]
enum OrderState { UNRESOLVED, GUESSING, RESOLVED }
#[derive(Clone,Debug)]
pub enum Action {
    Hold,
    Move { to: String, convoyed: bool },
    SupportHold { to: String },
    SupportMove { from: String, to: String },
    Convoy { from: String, to: String }
}

#[derive(Clone,Debug)]
struct Order {
    owner: String,
    province: String,
    action: Action,
    resolution: bool,
    state: OrderState,
    id: usize
}

pub enum RetreatAction {
    Disband,
    Move { to: String }
}

struct Retreat {
    owner: String,
    province: String,
    action: RetreatAction
}

pub struct Stpsyr {
    map: Vec<Province>,
    orders: Vec<Order>,
    dependencies: Vec<usize>,
    dislodged: Vec<(String, Unit)>
}

impl Stpsyr {
    pub fn new(mapfile: &'static str) -> Stpsyr {
        let mut reader = csv::Reader::from_file(mapfile).unwrap();

        let map: Vec<Province> = reader.decode::<(
                    String,          // 0 name
                    bool,            // 1 SC?
                    Option<String>,  // 2 starting owner
                    Option<String>,  // 3 starting unit type
                    String,          // 4 bordering provinces (fleets)
                    String           // 5 bordering provinces (armies)
                )>().map(|province| {
            let province = province.unwrap();
            Province {
                name: province.0.clone(),
                sc: province.1,
                owner: province.2.clone(),
                unit: province.3.as_ref().map(|unit_type| Unit {
                    owner: province.2.clone().unwrap(),
                    unit_type: match &unit_type[..] {
                        "Army" => UnitType::Army,
                        "Fleet" => UnitType::Fleet,
                        _ => panic!("unit type must be Army or Fleet")
                    }
                }),
                fleet_borders: province.4.split_whitespace().map(String::from).collect(),
                army_borders: province.5.split_whitespace().map(String::from).collect()
            }
        }).collect();

        Stpsyr { map: map, orders: vec![], dependencies: vec![], dislodged: vec![] }
    }

    pub fn add_order(&mut self, owner: String, province: String, action: Action) {
        let unit = if let Some(unit) = self.get_unit(&province) { unit }
            else { return; };
        // TODO use this
        let convoyed = match action {
            Action::Move { ref to, convoyed } => {
                if province == *to { return; }
                convoyed
            },
            Action::SupportMove { ref from, ref to } => {
                if *from == *to { return; }
                false
            }
            _ => false
        };
        if unit.owner == owner &&
                match &action {
                    &Action::Move { ref to, convoyed: _ } |
                    &Action::SupportHold { ref to } |
                    &Action::SupportMove { from: _, ref to } => {
                        let p = self.get_province(&province).unwrap();
                        match unit.unit_type {
                            UnitType::Army => &p.army_borders,
                            UnitType::Fleet => &p.fleet_borders
                        }.contains(&to)
                    },
                    _ => true
                } {
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
    }

    pub fn apply_orders(&mut self) -> Vec<(String, Unit)> {
        for i in 0..self.orders.len() {
            self.resolve(i);
            println!("{:?}", self.orders[i]);
        }

        let mut dislodged: Vec<(String, Unit)> = vec![];
        let mut moved_away: Vec<String> = vec![];
        for order in self.orders.iter() { if order.resolution {
            match order.action { Action::Move { ref to, convoyed: _ } => {
                let from_idx = self.map.iter().position(|p| p.name == order.province).unwrap();
                let to_idx = self.map.iter().position(|p| p.name == *to).unwrap();
                if let Some(ref unit) = self.map[to_idx].unit {
                    dislodged.push((to.clone(), unit.clone()));
                }
                self.map[to_idx].unit = self.map[from_idx].unit.clone();
                self.map[to_idx].owner = self.map[from_idx].owner.clone();
                moved_away.push(order.province.clone());
            }, _ => {} }
        } }
        self.orders = vec![];

        for province in self.map.iter_mut() {
            let p_dislodged = dislodged.iter().find(|d| d.0 == province.name);
            let p_moved_away = moved_away.contains(&province.name);
            if let Some(dislodgement) = p_dislodged {
                if !p_moved_away {
                    self.dislodged.push(dislodgement.clone());
                }
            } else if p_moved_away {
                province.unit = None;
            }
        }

        println!("{:?}", self.map);

        self.dislodged.clone()
    }

    pub fn get_unit(&self, province: &String) -> Option<Unit> {
        self.get_province(province).and_then(|p| p.unit.clone())
    }

    fn get_province(&self, province: &String) -> Option<&Province> {
        self.map.iter().find(|p| p.name == *province)
    }

    fn resolve(&mut self, id: usize) -> bool {
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

                // TODO backup rule

                // start over in case backup rule hasn't resolved all orders
                self.resolve(id)
            }
        }
    }

    fn adjudicate(&mut self, id: usize) -> bool {
        let province = self.orders[id].province.clone();
        match self.orders[id].action.clone() {

            Action::Hold => {
                // a hold order never fails (what would that even mean)
                true
            },

            Action::Move { to, convoyed } => {
                let attack_strength = self.attack_strength(province.clone());

                let counter_strength = if self.orders.iter().find(|o|
                        match o.action {
                            Action::Move { to: ref move_to, convoyed: _ } =>
                                province == *move_to,
                            _ => false
                        } && o.province == to).is_some() {
                    // head to head battle
                    self.defend_strength(to.clone())
                } else {
                    // no head to head
                    self.hold_strength(to.clone())
                };

                let contesting_orders = self.orders.iter().filter(|o|
                    match o.action {
                        Action::Move { to: ref move_to, convoyed: _ } =>
                            to == *move_to,
                        _ => false
                    } && o.province != province).map(|o| o.province.clone())
                    .collect::<Vec<String>>();

                attack_strength > counter_strength && contesting_orders.iter()
                    .all(|p| attack_strength > self.prevent_strength(p.clone()))
            },

            Action::SupportHold { to } | Action::SupportMove { from: _, to } => {
                self.orders.iter().find(|o|
                    match o.action {
                        Action::Move { to: ref move_to, convoyed: _ } =>
                            province == *move_to,
                        _ => false
                    } && o.province != to
                    && o.owner != self.orders[id].owner).is_none()
            },

            Action::Convoy { from, to } => {
                // TODO
                true
            },

        }
    }

    fn hold_strength(&mut self, province: String) -> usize {
        if self.get_unit(&province).is_some() {
            let move_id = self.orders.iter().find(|o|
                match o.action {
                    Action::Move { to: _, convoyed: _ } => true, _ => false
                } && o.province == province).map(|o| o.id);

            if let Some(move_id) = move_id {
                if self.resolve(move_id) { 0 } else { 1 }
            } else {
                1 + self.orders.clone().iter().filter(|o|
                    match o.action {
                        Action::SupportHold { ref to } => *to == province,
                        _ => false
                    } && self.resolve(o.id)).count()
            }
        } else {
            0
        }
    }

    // TODO check path (for convoys)
    // (and add that to prevent_strength, support checks in adjudicate also)
    fn attack_strength(&mut self, province: String) -> usize {
        let move_order = if let Some(move_order) = self.orders.iter().find(|o|
                match o.action {
                    Action::Move { to: _, convoyed: _ } => true, _ => false
                } && o.province == province) { move_order }
            else { return 0; }.clone();
        let dest = match move_order.action {
            Action::Move { ref to, convoyed: _ } => to.clone(),
            _ => unreachable!()
        };

        let move_id = self.orders.iter().find(|o|
            match o.action {
                Action::Move { ref to, convoyed: _ } => *to != province, _ => false
            } && o.province == dest).map(|o| o.id);
        let moved_away = move_id.map_or(false, |id| self.resolve(id));
        let attacked_power = if moved_away {
            None
        } else {
            self.get_province(&dest).and_then(|p| p.owner.clone())
        };

        if attacked_power == Some(move_order.owner) { return 0; }

        1 + self.orders.clone().iter().filter(|o|
            match o.action {
                Action::SupportMove { ref from, ref to } =>
                    *from == province && *to == dest,
                _ => false
            } && attacked_power.as_ref().map_or(true, |attacked| *attacked != o.owner)
            && self.resolve(o.id)).count()
    }

    fn defend_strength(&mut self, province: String) -> usize {
        let move_order = if let Some(move_order) = self.orders.iter().find(|o|
                match o.action {
                    Action::Move { to: _, convoyed: _ } => true, _ => false
                } && o.province == province) { move_order }
            else { return 0; }.clone();
        let dest = match move_order.action {
            Action::Move { ref to, convoyed: _ } => to.clone(),
            _ => unreachable!()
        };

        1 + self.orders.clone().iter().filter(|o|
            match o.action {
                Action::SupportMove { ref from, ref to } =>
                    *from == province && *to == dest,
                _ => false
            } && self.resolve(o.id)).count()
    }

    fn prevent_strength(&mut self, province: String) -> usize {
        let move_order = if let Some(move_order) = self.orders.iter().find(|o|
                match o.action {
                    Action::Move { to: _, convoyed: _ } => true, _ => false
                } && o.province == province) { move_order }
            else { return 0; }.clone();
        let dest = match move_order.action {
            Action::Move { ref to, convoyed: _ } => to.clone(),
            _ => unreachable!()
        };

        let move_id = self.orders.iter().find(|o|
            match o.action {
                Action::Move { ref to, convoyed: _ } => *to == province, _ => false
            } && o.province == dest).map(|o| o.id);
        if let Some(move_id) = move_id {
            if self.resolve(move_id) { return 0; }
        }

        1 + self.orders.clone().iter().filter(|o|
            match o.action {
                Action::SupportMove { ref from, ref to } =>
                    *from == province && *to == dest,
                _ => false
            } && self.resolve(o.id)).count()
    }
}
