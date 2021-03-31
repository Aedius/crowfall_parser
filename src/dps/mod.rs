use regex::Regex;

use chrono::prelude::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::iter::FromIterator;

const SELF_EMITTER: &str = "Your";
const SELF_RECEIVER: &str = "You";

lazy_static! {
    pub static ref RE_DPS: Regex = Regex::new("^([^ ]+) ?(.+)? hit (.+) for ([0-9]+) ?(\\(([0-9]+) absorbed\\))? ?(([^\\(]+) damage)? ?(\\(Critical\\))?.$").unwrap();
}

#[derive(Debug, PartialEq)]
pub struct Dps {
    pub date: DateTime<FixedOffset>,
    pub emitter: String,
    pub spell: String,
    pub receiver: String,
    pub damage: u32,
    pub kind: String,
    pub absorbed: u32,
    pub critical: bool,
}

pub fn parse_dps(row: &str, dt: DateTime<FixedOffset>) -> Option<Dps> {
    for cap in RE_DPS.captures_iter(row) {
        let kind = match cap.get(8) {
            Some(_) => {
                cap[8].to_string()
            }
            None => {
                "".to_string()
            }
        };

        let absorbed = match cap.get(6) {
            Some(_) => {
                cap[6].parse::<u32>().unwrap()
            }
            None => {
                0
            }
        };

        let critical = match cap.get(9) {
            Some(_) => {
                true
            }
            None => {
                false
            }
        };

        let spell = match cap.get(2) {
            Some(_) => {
                cap[2].to_string()
            }
            None => {
                "".to_string()
            }
        };

        return Some(Dps {
            date: dt,
            emitter: cap[1].to_string(),
            spell,
            receiver: cap[3].to_string(),
            damage: cap[4].parse::<u32>().unwrap(),
            kind,
            absorbed,
            critical,
        });
    }

    return None;
}

#[cfg(test)]
mod parse_tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn assert_dps_simple() {
        let tt = "Your Freezing Storm hit Major Thrall of Dark for 101 Ice damage.";
        let dt = DateTime::from(Utc::now());
        assert!(RE_DPS.is_match(tt));
        assert_eq!(
            parse_dps(tt, dt).unwrap(),
            Dps {
                date: dt,
                emitter: "Your".to_string(),
                spell: "Freezing Storm".to_string(),
                receiver: "Major Thrall of Dark".to_string(),
                damage: 101,
                kind: "Ice".to_string(),
                absorbed: 0,
                critical: false,
            }
        )
    }

    #[test]
    fn assert_dps_full_absorbed() {
        let tt = "Your Shatter Storm hit RexAlchy for 0 (51 absorbed).";
        let dt = DateTime::from(Utc::now());
        assert!(RE_DPS.is_match(tt));
        assert_eq!(
            parse_dps(tt, dt).unwrap(),
            Dps {
                date: dt,
                emitter: "Your".to_string(),
                spell: "Shatter Storm".to_string(),
                receiver: "RexAlchy".to_string(),
                damage: 0,
                kind: "".to_string(),
                absorbed: 51,
                critical: false,
            }
        )
    }

    #[test]
    fn assert_dps_partially_absorbed() {
        let tt = "Your Spiral Cast hit Thrall Soul for 272 (12 absorbed) Ice damage.";
        let dt = DateTime::from(Utc::now());
        assert!(RE_DPS.is_match(tt));
        assert_eq!(
            parse_dps(tt, dt).unwrap(),
            Dps {
                date: dt,
                emitter: "Your".to_string(),
                spell: "Spiral Cast".to_string(),
                receiver: "Thrall Soul".to_string(),
                damage: 272,
                kind: "Ice".to_string(),
                absorbed: 12,
                critical: false,
            }
        )
    }

    #[test]
    fn assert_dps_critical() {
        let tt = "Your Coalesce Forestry hit Urgu Myrmidon Chief for 311 Nature damage (Critical).";
        let dt = DateTime::from(Utc::now());
        assert!(RE_DPS.is_match(tt));
        assert_eq!(
            parse_dps(tt, dt).unwrap(),
            Dps {
                date: dt,
                emitter: "Your".to_string(),
                spell: "Coalesce Forestry".to_string(),
                receiver: "Urgu Myrmidon Chief".to_string(),
                damage: 311,
                kind: "Nature".to_string(),
                absorbed: 0,
                critical: true,
            }
        )
    }

    #[test]
    fn assert_dps_partially_absorbed_critical() {
        let tt = "Your Retaliate hit UDeadPRO for 292 (233 absorbed) Nature damage (Critical).";
        let dt = DateTime::from(Utc::now());
        assert!(RE_DPS.is_match(tt));
        assert_eq!(
            parse_dps(tt, dt).unwrap(),
            Dps {
                date: dt,
                emitter: "Your".to_string(),
                spell: "Retaliate".to_string(),
                receiver: "UDeadPRO".to_string(),
                damage: 292,
                kind: "Nature".to_string(),
                absorbed: 233,
                critical: true,
            }
        )
    }

    #[test]
    fn assert_dps_none() {
        let tt = "Your Holy Symbol hit Zankara for 0 (Critical).";
        let dt = DateTime::from(Utc::now());
        assert!(RE_DPS.is_match(tt));
        assert_eq!(
            parse_dps(tt, dt).unwrap(),
            Dps {
                date: dt,
                emitter: "Your".to_string(),
                spell: "Holy Symbol".to_string(),
                receiver: "Zankara".to_string(),
                damage: 0,
                kind: "".to_string(),
                absorbed: 0,
                critical: true,
            }
        )
    }

    #[test]
    fn assert_hit_none_critical() {
        let tt = "Gamako Fervor hit You for 0 (Critical).";
        let dt = DateTime::from(Utc::now());
        assert!(RE_DPS.is_match(tt));
        assert_eq!(
            parse_dps(tt, dt).unwrap(),
            Dps {
                date: dt,
                emitter: "Gamako".to_string(),
                spell: "Fervor".to_string(),
                receiver: "You".to_string(),
                damage: 0,
                kind: "".to_string(),
                absorbed: 0,
                critical: true,
            }
        )
    }

    #[test]
    fn assert_fire_dps() {
        let tt = "Sun Elf Confessor Fire Aura hit You for 26 Fire damage.";
        let dt = DateTime::from(Utc::now());
        assert!(RE_DPS.is_match(tt));
        assert_eq!(
            parse_dps(tt, dt).unwrap(),
            Dps {
                date: dt,
                emitter: "Sun".to_string(),
                spell: "Elf Confessor Fire Aura".to_string(),
                receiver: "You".to_string(),
                damage: 26,
                kind: "Fire".to_string(),
                absorbed: 0,
                critical: false,
            }
        )
    }

    #[test]
    fn assert_no_spell() {
        let tt = "Swoop hit You for 46 Piercing damage.";
        let dt = DateTime::from(Utc::now());
        assert!(RE_DPS.is_match(tt));
        assert_eq!(
            parse_dps(tt, dt).unwrap(),
            Dps {
                date: dt,
                emitter: "Swoop".to_string(),
                spell: "".to_string(),
                receiver: "You".to_string(),
                damage: 46,
                kind: "Piercing".to_string(),
                absorbed: 0,
                critical: false,
            }
        )
    }

    #[test]
    fn assert_partially_absorbed() {
        let tt = "Urgu Myrmidon Chief Slash hit You for 206 (198 absorbed) Crushing damage.";
        let dt = DateTime::from(Utc::now());
        assert!(RE_DPS.is_match(tt));
        assert_eq!(
            parse_dps(tt, dt).unwrap(),
            Dps {
                date: dt,
                emitter: "Urgu".to_string(),
                spell: "Myrmidon Chief Slash".to_string(),
                receiver: "You".to_string(),
                damage: 206,
                kind: "Crushing".to_string(),
                absorbed: 198,
                critical: false,
            }
        )
    }
}

#[derive(Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct DpsStats {
    pub received_by_kind: HashMap<String, u32>,
    pub emit_by_kind: HashMap<String, u32>,
    pub received_by_enemy: HashMap<String, u32>,
    pub emit_by_enemy: HashMap<String, u32>,
    pub emit_by_seconds: Vec<u32>,
    pub emit_by_seconds_absorbed: Vec<u32>,
    pub received_by_seconds: Vec<u32>,
    pub received_by_seconds_absorbed: Vec<u32>,
}

pub fn stats_dps(list: &Vec<Dps>, start: Option<i64>, end: Option<i64>) -> (DpsStats, Vec<String>) {
    let mut received_by_kind = HashMap::new();
    let mut emit_by_kind = HashMap::new();
    let mut received_by_enemy = HashMap::new();
    let mut emit_by_enemy = HashMap::new();
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

    for dps in list.iter() {
        if dps.date.timestamp() < start.unwrap_or(0) || dps.date.timestamp() > end.unwrap_or(i64::max_value()) {
            continue;
        }

        if dps.kind != "" {
            if dps.receiver == SELF_RECEIVER {
                let rec = received_by_kind.entry(dps.kind.to_string()).or_insert(0);
                *rec += dps.damage + dps.absorbed;
            }

            if dps.emitter == SELF_EMITTER {
                let emit = emit_by_kind.entry(dps.kind.to_string()).or_insert(0);
                *emit += dps.damage + dps.absorbed;
            }
        }

        if dps.receiver == SELF_RECEIVER {
            let rec = received_by_enemy.entry(dps.emitter.to_string()).or_insert(0);
            *rec += dps.damage + dps.absorbed;
            if take_seconds {
                received_by_seconds[(dps.date.timestamp() - start.unwrap()) as usize] += dps.damage;
                received_by_seconds_absorbed[(dps.date.timestamp() - start.unwrap()) as usize] += dps.absorbed;
            }
        }

        if dps.emitter == SELF_EMITTER {
            let emit = emit_by_enemy.entry(dps.receiver.to_string()).or_insert(0);
            *emit += dps.damage + dps.absorbed;
            if take_seconds {
                emit_by_seconds[(dps.date.timestamp() - start.unwrap()) as usize] += dps.damage;
                emit_by_seconds_absorbed[(dps.date.timestamp() - start.unwrap()) as usize] += dps.absorbed;
            }
        }
    }

    let mut opponent = vec!();
    for e in Vec::from_iter(received_by_enemy.keys().clone()){
        opponent.push(e.to_lowercase());
    }
    for e in Vec::from_iter(emit_by_enemy.keys().clone()){
        opponent.push(e.to_lowercase());
    }

    opponent.sort();
    opponent.dedup();

    (DpsStats {
        received_by_kind,
        emit_by_kind,
        received_by_enemy,
        emit_by_enemy,
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
    fn assert_received_by_kind_empty() {
        let list = vec![
            Dps {
                date: DateTime::from(Utc::now()),
                emitter: "John".to_string(),
                spell: "Spell".to_string(),
                receiver: "SomeoneElse".to_string(),
                damage: 100,
                kind: "Ice".to_string(),
                absorbed: 100,
                critical: false,
            },
            Dps {
                date: DateTime::from(Utc::now()),
                emitter: "John".to_string(),
                spell: "Spell".to_string(),
                receiver: "You".to_string(),
                damage: 100,
                kind: "".to_string(),
                absorbed: 100,
                critical: false,
            }
        ];
        assert_eq!(
            stats_dps(&list, None, None).0.received_by_kind,
            HashMap::new()
        )
    }

    #[test]
    fn assert_received_by_kind_sum() {
        let list = vec![
            Dps {
                date: DateTime::from(Utc::now()),
                emitter: "John".to_string(),
                spell: "Spell".to_string(),
                receiver: "You".to_string(),
                damage: 1000,
                kind: "Ice".to_string(),
                absorbed: 100,
                critical: false,
            },
            Dps {
                date: DateTime::from(Utc::now()),
                emitter: "John".to_string(),
                spell: "Spell".to_string(),
                receiver: "You".to_string(),
                damage: 100,
                kind: "Ice".to_string(),
                absorbed: 10,
                critical: false,
            },
            Dps {
                date: DateTime::from(Utc::now()),
                emitter: "John".to_string(),
                spell: "Spell".to_string(),
                receiver: "You".to_string(),
                damage: 123,
                kind: "Fire".to_string(),
                absorbed: 2000,
                critical: false,
            }
        ];

        let mut res: HashMap<String, u32> = HashMap::new();
        res.insert("Ice".to_string(), 1210);
        res.insert("Fire".to_string(), 2123);
        assert_eq!(
            stats_dps(&list, None, None).0.received_by_kind,
            res
        )
    }

    #[test]
    fn assert_emit_by_kind_empty() {
        let list = vec![
            Dps {
                date: DateTime::from(Utc::now()),
                emitter: "SomeoneElse".to_string(),
                spell: "Spell".to_string(),
                receiver: "John".to_string(),
                damage: 100,
                kind: "Ice".to_string(),
                absorbed: 100,
                critical: false,
            },
            Dps {
                date: DateTime::from(Utc::now()),
                emitter: "You".to_string(),
                spell: "Spell".to_string(),
                receiver: "John".to_string(),
                damage: 100,
                kind: "".to_string(),
                absorbed: 100,
                critical: false,
            }
        ];
        assert_eq!(
            stats_dps(&list, None, None).0.emit_by_kind,
            HashMap::new()
        )
    }

    #[test]
    fn assert_emit_by_kind_sum() {
        let list = vec![
            Dps {
                date: DateTime::from(Utc::now()),
                emitter: "Your".to_string(),
                spell: "Spell".to_string(),
                receiver: "SomeoneElse".to_string(),
                damage: 10,
                kind: "Ice".to_string(),
                absorbed: 200,
                critical: false,
            },
            Dps {
                date: DateTime::from(Utc::now()),
                emitter: "Your".to_string(),
                spell: "Spell".to_string(),
                receiver: "SomeoneElse".to_string(),
                damage: 600,
                kind: "Fire".to_string(),
                absorbed: 0,
                critical: false,
            },
            Dps {
                date: DateTime::from(Utc::now()),
                emitter: "Your".to_string(),
                spell: "Spell".to_string(),
                receiver: "SomeoneElse".to_string(),
                damage: 900,
                kind: "Fire".to_string(),
                absorbed: 25,
                critical: false,
            }
        ];

        let mut res: HashMap<String, u32> = HashMap::new();
        res.insert("Ice".to_string(), 210);
        res.insert("Fire".to_string(), 1525);
        assert_eq!(
            stats_dps(&list, None, None).0.emit_by_kind,
            res
        )
    }

    #[test]
    fn assert_received_by_enemy_empty() {
        let list = vec![
            Dps {
                date: DateTime::from(Utc::now()),
                emitter: "John".to_string(),
                spell: "Spell".to_string(),
                receiver: "SomeoneElse".to_string(),
                damage: 100,
                kind: "Ice".to_string(),
                absorbed: 100,
                critical: false,
            },
        ];
        assert_eq!(
            stats_dps(&list, None, None).0.received_by_enemy,
            HashMap::new()
        )
    }

    #[test]
    fn assert_received_by_enemy_sum() {
        let list = vec![
            Dps {
                date: DateTime::from(Utc::now()),
                emitter: "John".to_string(),
                spell: "Spell".to_string(),
                receiver: "You".to_string(),
                damage: 123,
                kind: "Ice".to_string(),
                absorbed: 0,
                critical: false,
            },
            Dps {
                date: DateTime::from(Utc::now()),
                emitter: "John".to_string(),
                spell: "Spell".to_string(),
                receiver: "You".to_string(),
                damage: 1000,
                kind: "Ice".to_string(),
                absorbed: 5,
                critical: false,
            },
            Dps {
                date: DateTime::from(Utc::now()),
                emitter: "Lennon".to_string(),
                spell: "Spell".to_string(),
                receiver: "You".to_string(),
                damage: 3500,
                kind: "Fire".to_string(),
                absorbed: 0,
                critical: false,
            }
        ];

        let mut res: HashMap<String, u32> = HashMap::new();
        res.insert("John".to_string(), 1128);
        res.insert("Lennon".to_string(), 3500);
        assert_eq!(
            stats_dps(&list, None, None).0.received_by_enemy,
            res
        )
    }

    #[test]
    fn assert_emit_by_enemy_empty() {
        let list = vec![
            Dps {
                date: DateTime::from(Utc::now()),
                emitter: "SomeoneElse".to_string(),
                spell: "Spell".to_string(),
                receiver: "You".to_string(),
                damage: 100,
                kind: "Ice".to_string(),
                absorbed: 100,
                critical: false,
            },
        ];
        assert_eq!(
            stats_dps(&list, None, None).0.emit_by_enemy,
            HashMap::new()
        )
    }

    #[test]
    fn assert_emit_by_enemy_sum() {
        let list = vec![
            Dps {
                date: DateTime::from(Utc::now()),
                emitter: "Your".to_string(),
                spell: "Spell".to_string(),
                receiver: "Paul".to_string(),
                damage: 800,
                kind: "Ice".to_string(),
                absorbed: 100,
                critical: false,
            },
            Dps {
                date: DateTime::from(Utc::now()),
                emitter: "Your".to_string(),
                spell: "Spell".to_string(),
                receiver: "Jacques".to_string(),
                damage: 352,
                kind: "Ice".to_string(),
                absorbed: 48,
                critical: false,
            },
            Dps {
                date: DateTime::from(Utc::now()),
                emitter: "Your".to_string(),
                spell: "Spell".to_string(),
                receiver: "Paul".to_string(),
                damage: 88,
                kind: "Fire".to_string(),
                absorbed: 1000,
                critical: false,
            }
        ];

        let mut res: HashMap<String, u32> = HashMap::new();
        res.insert("Jacques".to_string(), 400);
        res.insert("Paul".to_string(), 1988);
        assert_eq!(
            stats_dps(&list, None, None).0.emit_by_enemy,
            res
        )
    }

    #[test]
    fn assert_emit_by_enemy_clenup_by_time() {
        let list = vec![
            Dps {
                date: DateTime::parse_from_rfc3339("2021-03-17T20:20:45.111Z").unwrap(),
                emitter: "Your".to_string(),
                spell: "Spell".to_string(),
                receiver: "Paul".to_string(),
                damage: 800,
                kind: "Ice".to_string(),
                absorbed: 100,
                critical: false,
            },
            Dps {
                date: DateTime::parse_from_rfc3339("2021-03-17T20:50:45.111Z").unwrap(),
                emitter: "Your".to_string(),
                spell: "Spell".to_string(),
                receiver: "Jacques".to_string(),
                damage: 352,
                kind: "Ice".to_string(),
                absorbed: 48,
                critical: false,
            },
            Dps {
                date: DateTime::parse_from_rfc3339("2021-03-17T20:40:45.111Z").unwrap(),
                emitter: "Your".to_string(),
                spell: "Spell".to_string(),
                receiver: "Paul".to_string(),
                damage: 88,
                kind: "Fire".to_string(),
                absorbed: 1000,
                critical: false,
            }
        ];

        let mut res: HashMap<String, u32> = HashMap::new();
        res.insert("Paul".to_string(), 1088);

        let stats = stats_dps(&list, Some(DateTime::parse_from_rfc3339("2021-03-17T20:40:00.111Z").unwrap().timestamp()), Some(DateTime::parse_from_rfc3339("2021-03-17T20:42:00.111Z").unwrap().timestamp())).0;
        assert_eq!(
            stats.emit_by_enemy,
            res
        );
        let mut seconds = vec![0; 121];
        seconds[45] = 88;
        assert_eq!(
            stats.emit_by_seconds,
            seconds
        );
        let mut seconds_absorbed = vec![0; 121];
        seconds_absorbed[45] = 1000;
        assert_eq!(
            stats.emit_by_seconds_absorbed,
            seconds_absorbed
        )
    }
}