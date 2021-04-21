mod dps;
mod heal;
mod split;

use wasm_bindgen::prelude::*;

#[macro_use]
extern crate lazy_static;

use regex::Regex;
use dps::*;
use heal::*;
use chrono::prelude::{DateTime, FixedOffset};
use serde::{Serialize, Deserialize};
use crate::split::{split_in_fight, FightTimer};

struct Data {
    pub dps: Vec<Dps>,
    pub heal: Vec<Heal>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExportedData {
    pub dps_stats: DpsStats,
    pub heal_stats: HealStats,
    pub errors : Vec<String>,
    pub fights: Vec<Fight>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Fight{
    pub opponent: Vec<String>,
    pub time : FightTimer,
    pub dps_stats: DpsStats,
    pub heal_stats: HealStats,
}

#[wasm_bindgen]
pub fn parse(contents: &str, time_between: i64, minimum_time: i64) -> JsValue {
    let to_export = parse_rust(contents, time_between, minimum_time);

    JsValue::from_serde(&to_export).unwrap()
}

fn parse_rust(contents: &str, time_between: i64, minimum_time: i64) -> ExportedData {
    let re_event = Regex::new("([-0-9T:\\.]+Z).*Event=\\[(.*)\\]").unwrap();

    let mut data = Data {
        dps: Default::default(),
        heal: Default::default(),
    };

    let mut date_list = vec![];

    let mut nb = 0;

    let lines = contents.lines();
    let mut errors = vec![];

    for line in lines {

        if !re_event.is_match(line){
            println!("NO MATCH : {:?}",line);
        }

        for cap in re_event.captures_iter(line) {
            let d = DateTime::parse_from_rfc3339(&cap[1]).unwrap();

            if data.parse_row(&cap[2], d) {
                date_list.push(d);
            } else {
                errors.push(cap[2].to_string());
            }
            nb = nb + 1;
        }
    }

    let ( dps_stats, _) = stats_dps(&data.dps, None, None);
    let ( heal_stats, _) = stats_heal(&data.heal, None, None);

    let fight_timers = split_in_fight(date_list, time_between, minimum_time);
    let mut fight = vec![];

    for timer in fight_timers {

        let (dps_stats, mut opponent ) = stats_dps(&data.dps, Some(timer.start), Some(timer.end));
        let (heal_stats, mut opponent_heal ) = stats_heal(&data.heal, Some(timer.start), Some(timer.end));
        opponent.append(&mut opponent_heal);

        opponent.sort();
        opponent.dedup();

        fight.push(Fight {
            time: timer.clone(),
            dps_stats,
            heal_stats,
            opponent
        })
    }

    let to_export = ExportedData {
        dps_stats,
        heal_stats,
        errors,
        fights: fight
    };
    to_export
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use std::fs::{File};
    use std::io::BufReader;
    use std::io::prelude::*;

    #[test]
    fn assert_parse() {
        let file = File::open("./fixtures/file1.txt").unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();

        let calc = parse_rust(contents.as_str(), 30, 0);

        assert_eq!(calc.errors.len(), 0);
        assert_eq!(calc.fights.len(), 13);

        println!("{:?}", calc)
    }

    #[test]
    fn assert_parse_with_minimum() {
        let file = File::open("./fixtures/file1.txt").unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();

        let calc = parse_rust(contents.as_str(), 30, 30);

        assert_eq!(calc.errors.len(), 0);
        assert_eq!(calc.fights.len(), 7);

        println!("{:?}", calc)
    }

}