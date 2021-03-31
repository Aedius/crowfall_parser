use regex::Regex;

use chrono::prelude::*;

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