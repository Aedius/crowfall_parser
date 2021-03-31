use regex::Regex;

use chrono::prelude::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::iter::FromIterator;

const SELF_EMITTER: &str = "Your";
const SELF_RECEIVER: &str = "You";

lazy_static! {
    pub static ref RE_HEAL: Regex = Regex::new("^([^ ]+) (.+) healed (.+) for ([0-9]+)( \\(([0-9]+) absorbed\\))?( hit points)?( \\(Critical\\))?.$").unwrap();
}

#[derive(Debug, PartialEq)]
pub struct Heal {
    pub date: DateTime<FixedOffset>,
    pub emitter: String,
    pub spell: String,
    pub receiver: String,
    pub heal: u32,
    pub absorbed: u32,
    pub critical: bool,
}

pub fn parse_heal(row: &str, dt: DateTime<FixedOffset>) -> Option<Heal> {
    for cap in RE_HEAL.captures_iter(row) {
        let absorbed = match cap.get(6) {
            Some(_) => {
                cap[6].parse::<u32>().unwrap()
            }
            None => {
                0
            }
        };

        let critical = match cap.get(8) {
            Some(_) => {
                true
            }
            None => {
                false
            }
        };

        return Some(Heal {
            date: dt,
            emitter: cap[1].to_string(),
            spell: cap[2].to_string(),
            receiver: cap[3].to_string(),
            heal: cap[4].parse::<u32>().unwrap(),
            absorbed,
            critical,
        });
    }

    return None;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn assert_self_heal_simple() {
        let tt = "Your Electrogenesis healed You for 486 hit points.";
        let dt = DateTime::from(Utc::now());
        assert!(RE_HEAL.is_match(tt));
        assert_eq!(
            parse_heal(tt, dt).unwrap(),
            Heal {
                date: dt,
                emitter: "Your".to_string(),
                spell: "Electrogenesis".to_string(),
                receiver: "You".to_string(),
                heal: 486,
                absorbed: 0,
                critical: false,
            }
        )
    }

    #[test]
    fn assert_self_heal_critical() {
        let tt = "Your Retaliate healed You for 162 hit points (Critical).";
        let dt = DateTime::from(Utc::now());
        assert!(RE_HEAL.is_match(tt));
        assert_eq!(
            parse_heal(tt, dt).unwrap(),
            Heal {
                date: dt,
                emitter: "Your".to_string(),
                spell: "Retaliate".to_string(),
                receiver: "You".to_string(),
                heal: 162,
                absorbed: 0,
                critical: true,
            }
        )
    }

    #[test]
    fn assert_self_heal_absorbed() {
        let tt = "Your Coalesce Life healed patibulaire for 0 (401 absorbed).";
        let dt = DateTime::from(Utc::now());
        assert!(RE_HEAL.is_match(tt));
        assert_eq!(
            parse_heal(tt, dt).unwrap(),
            Heal {
                date: dt,
                emitter: "Your".to_string(),
                spell: "Coalesce Life".to_string(),
                receiver: "patibulaire".to_string(),
                heal: 0,
                absorbed: 401,
                critical: false,
            }
        )
    }

    #[test]
    fn assert_heal_received() {
        let tt = "royo Divine Light healed You for 518 hit points (Critical).";
        let dt = DateTime::from(Utc::now());
        assert!(RE_HEAL.is_match(tt));
        assert_eq!(
            parse_heal(tt, dt).unwrap(),
            Heal {
                date: dt,
                emitter: "royo".to_string(),
                spell: "Divine Light".to_string(),
                receiver: "You".to_string(),
                heal: 518,
                absorbed: 0,
                critical: true,
            }
        )
    }
}

#[derive(Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct HealStats {
    pub received_by_ally: HashMap<String, u32>,
    pub emit_by_ally: HashMap<String, u32>,
    pub emit_by_seconds: Vec<u32>,
    pub emit_by_seconds_absorbed: Vec<u32>,
    pub received_by_seconds: Vec<u32>,
    pub received_by_seconds_absorbed: Vec<u32>,
}

pub fn stats_heal(list: &Vec<Heal>, start: Option<i64>, end: Option<i64>) -> (HealStats, Vec<String>) {

    let mut received_by_ally = HashMap::new();
    let mut emit_by_ally = HashMap::new();
    let mut emit_by_seconds = vec![];
    let mut emit_by_seconds_absorbed = vec![];
    let mut received_by_seconds = vec![];
    let mut received_by_seconds_absorbed = vec![];
    let mut take_seconds = false;

    if start != None && end != None {
        let s = (end.unwrap() - start.unwrap() +1) as usize ;

        emit_by_seconds = vec![0; s];
        emit_by_seconds_absorbed = vec![0; s];
        received_by_seconds = vec![0; s];
        received_by_seconds_absorbed = vec![0; s];
        take_seconds = true;
    }

    for heal in list.iter(){

        if heal.date.timestamp() < start.unwrap_or(0) || heal.date.timestamp() > end.unwrap_or(i64::max_value()) {
            continue;
        }

        if heal.receiver == SELF_RECEIVER {
            let rec = received_by_ally.entry(heal.emitter.to_string()).or_insert(0);
            *rec += heal.heal + heal.absorbed;
            if take_seconds {
                received_by_seconds[(heal.date.timestamp() - start.unwrap()) as usize] += heal.heal;
                received_by_seconds_absorbed[(heal.date.timestamp() - start.unwrap()) as usize] += heal.absorbed;
            }
        }

        if heal.emitter == SELF_EMITTER {
            let emit = emit_by_ally.entry(heal.receiver.to_string()).or_insert(0);
            *emit += heal.heal + heal.absorbed;
            if take_seconds {
                emit_by_seconds[(heal.date.timestamp() - start.unwrap()) as usize] += heal.heal;
                emit_by_seconds_absorbed[(heal.date.timestamp() - start.unwrap()) as usize] += heal.absorbed;
            }
        }

    }

    let mut opponent = vec!();
    for e in Vec::from_iter(received_by_ally.keys().clone()){
        opponent.push(e.to_lowercase());
    }
    for e in Vec::from_iter(emit_by_ally.keys().clone()){
        opponent.push(e.to_lowercase());
    }

    opponent.sort();
    opponent.dedup();

    (HealStats {
        received_by_ally,
        emit_by_ally,
        emit_by_seconds,
        emit_by_seconds_absorbed,
        received_by_seconds,
        received_by_seconds_absorbed,
    }, opponent)
}

#[cfg(test)]
mod stats_tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;



    #[test]
    fn assert_ally_received_sum() {
        let list = vec![
            Heal {
                date: DateTime::from(Utc::now()),
                emitter: "John".to_string(),
                spell: "Spell".to_string(),
                receiver: "You".to_string(),
                absorbed: 0,
                critical: false,
                heal: 150
            },
            Heal {
                date: DateTime::from(Utc::now()),
                emitter: "John".to_string(),
                spell: "Spell".to_string(),
                receiver: "You".to_string(),
                absorbed: 5,
                critical: false,
                heal: 800
            },
            Heal {
                date: DateTime::from(Utc::now()),
                emitter: "Lennon".to_string(),
                spell: "Spell".to_string(),
                receiver: "You".to_string(),
                absorbed: 0,
                critical: true,
                heal: 1000
            }
        ];

        let mut res: HashMap<String, u32> = HashMap::new();
        res.insert("Lennon".to_string(), 1000);
        res.insert("John".to_string(), 955);
        assert_eq!(
            stats_heal(&list, None, None).0.received_by_ally,
            res
        )
    }

}