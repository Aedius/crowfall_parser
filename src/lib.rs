mod dps;
mod heal;


use wasm_bindgen::prelude::*;

#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::io::prelude::*;
use dps::{Dps, RE_DPS, parse_dps};
use heal::{Heal, RE_HEAL, parse_heal};
use chrono::prelude::{DateTime, FixedOffset};


#[wasm_bindgen]
pub fn greet(contents: &str) -> String {

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
            nb = nb +1;
        }
    }

    return format!("total : {:?} ({} {})", nb, data.dps.len(), data.heal.len());


}


lazy_static! {
    static ref RE_FOOD: Regex = Regex::new("^Your meal restored You for ([0-9]+) food.$").unwrap();

    static ref RE_SELF_RESOURCE: Regex = Regex::new("^Your (.+) (restored|drained) You for ([0-9]+) (.+).$").unwrap();

    static ref RE_RESOURCE: Regex = Regex::new("^(.+) (restored|drained) You for ([0-9]+) (.+).$").unwrap();
}



struct Data {
    dps: Vec<Dps>,
    heal: Vec<Heal>,
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
