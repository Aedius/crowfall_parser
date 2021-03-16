use regex::Regex;

lazy_static! {
    pub static ref RE_DPS: Regex = Regex::new("^([^ ]+) (.+)? ?hit (.+) for ([0-9]+) ?(\\(([0-9]+) absorbed\\))? ?(([^\\(]+) damage)? ?(\\(Critical\\))?.$").unwrap();
}

#[derive(Debug, PartialEq)]
pub struct Dps {
    pub emitter: String,
    pub spell: String,
    pub receiver: String,
    pub damage: u16,
    pub kind: String,
    pub absorbed: u16,
    pub critical: bool,
}

pub fn parse_dps(row: &str) -> Option<Dps> {
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
                cap[6].parse::<u16>().unwrap()
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
            emitter: cap[1].to_string(),
            spell,
            receiver: cap[3].to_string(),
            damage: cap[4].parse::<u16>().unwrap(),
            kind,
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
    fn assert_dps_simple() {
        let tt = "Your Freezing Storm hit Major Thrall of Dark for 101 Ice damage.";
        assert!(RE_DPS.is_match(tt));
        assert_eq!(
            parse_dps(tt).unwrap(),
            Dps {
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
        assert!(RE_DPS.is_match(tt));
        assert_eq!(
            parse_dps(tt).unwrap(),
            Dps {
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
        assert!(RE_DPS.is_match(tt));
        assert_eq!(
            parse_dps(tt).unwrap(),
            Dps {
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
        assert!(RE_DPS.is_match(tt));
        assert_eq!(
            parse_dps(tt).unwrap(),
            Dps {
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
        assert!(RE_DPS.is_match(tt));
        assert_eq!(
            parse_dps(tt).unwrap(),
            Dps {
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
        assert!(RE_DPS.is_match(tt));
        assert_eq!(
            parse_dps(tt).unwrap(),
            Dps {
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
        assert!(RE_DPS.is_match(tt));
        assert_eq!(
            parse_dps(tt).unwrap(),
            Dps {
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
        assert!(RE_DPS.is_match(tt));
        assert_eq!(
            parse_dps(tt).unwrap(),
            Dps {
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
        assert!(RE_DPS.is_match(tt));
        assert_eq!(
            parse_dps(tt).unwrap(),
            Dps {
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
        assert!(RE_DPS.is_match(tt));
        assert_eq!(
            parse_dps(tt).unwrap(),
            Dps {
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