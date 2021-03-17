mod dps;
mod heal;

use wasm_bindgen::prelude::*;

#[macro_use]
extern crate lazy_static;

use regex::Regex;
use dps::*;
use heal::*;
use chrono::prelude::{DateTime, FixedOffset};
use std::collections::HashMap;

struct Data {
    pub dps: Vec<Dps>,
    pub heal: Vec<Heal>,
}

#[wasm_bindgen]
pub struct ExportedData {
    dps_stats: ExportedDpsStats
}

#[wasm_bindgen]
struct ExportedDpsStats {
     received_by_kind: Vec<Stat>,
     emit_by_kind: Vec<Stat>,
     received_by_enemy: Vec<Stat>,
     emit_by_enemy: Vec<Stat>,
}

#[wasm_bindgen]
struct Stat {
     name: String,
     value: u16,
}


#[wasm_bindgen]
pub fn parse(contents: &str) -> ExportedData {
    let re_event = Regex::new("([-0-9T:\\.]+Z).*Event=\\[(.*)\\] ").unwrap();

    let mut data = Data {
        dps: Default::default(),
        heal: Default::default(),
    };

    let mut nb = 0;

    let lines = contents.lines();

    for line in lines {
        for cap in re_event.captures_iter(line) {
            let d = DateTime::parse_from_rfc3339(&cap[1]).unwrap();

            if !data.parse_row(&cap[2], d) {
                println!("cannot parse : {:?}", &cap[2]);
                panic!()
            }
            nb = nb + 1;
        }
    }

    let dps_stats = stats_dps(data.dps);

    return ExportedData {
        dps_stats: ExportedDpsStats {
            received_by_kind: to_vec_stat(dps_stats.received_by_kind),
            emit_by_kind: to_vec_stat(dps_stats.emit_by_kind),
            received_by_enemy: to_vec_stat(dps_stats.received_by_enemy),
            emit_by_enemy: to_vec_stat(dps_stats.emit_by_enemy),
        }
    };
}




lazy_static! {
    static ref RE_FOOD: Regex = Regex::new("^Your meal restored You for ([0-9]+) food.$").unwrap();

    static ref RE_SELF_RESOURCE: Regex = Regex::new("^Your (.+) (restored|drained) You for ([0-9]+) (.+).$").unwrap();

    static ref RE_RESOURCE: Regex = Regex::new("^(.+) (restored|drained) You for ([0-9]+) (.+).$").unwrap();
}



impl Data {
    fn parse_row(&mut self, row: &str, dt: DateTime<FixedOffset>) -> bool {
        if RE_FOOD.is_match(row) {
            //TODO
            return true;
        }

        if RE_RESOURCE.is_match(row) {
            //TODO
            return true;
        }

        if RE_SELF_RESOURCE.is_match(row) {
            //TODO
            return true;
        }

        if RE_DPS.is_match(row) {
            let dps = parse_dps(row, dt).unwrap();
            self.dps.push(dps);
            return true;
        }

        if RE_HEAL.is_match(row) {
            let heal = parse_heal(row, dt).unwrap();
            self.heal.push(heal);
            return true;
        }

        println!("{:?}", row);
        return false;
    }
}


fn to_vec_stat(hash: HashMap<String, u16>) -> Vec<Stat> {
    let mut vec = vec![];

    for (k, v) in hash.into_iter() {
        vec.push(Stat {
            name: k,
            value: v,
        })
    }

    vec
}