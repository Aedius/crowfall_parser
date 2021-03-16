
use regex::Regex;

lazy_static! {
    pub static ref RE_DPS: Regex = Regex::new("^Your (.+) hit (.+) for ([0-9]+) ?(\\(([0-9]+) absorbed\\))? ?(([^\\(]+) damage)? ?(\\(Critical\\))?.$").unwrap();

}

#[derive(Debug, PartialEq)]
pub struct Dps{
    pub spell : String,
    pub target : String,
    pub damage : u16,
    pub kind : String,
    pub absorbed: u16,
    pub critical : bool,
}

pub fn parse_dps(row: &str) -> Option<Dps> {
    for cap in RE_DPS.captures_iter(row) {
        println!("{:?}", &cap);

        let kind = match cap.get(7) {
            Some(_) => {
                cap[7].to_string()
            }
            None => {
                "".to_string()
            }
        };

        let absorbed = match cap.get(5){
            Some(_) => {
                cap[5].parse::<u16>().unwrap()
            }
            None => {
                0
            }
        };

        let critical = match cap.get(8){
            Some(_) => {
                true
            }
            None => {
                false
            }
        };

        return Some(Dps {
            spell: cap[1].to_string(),
            target: cap[2].to_string(),
            damage:  cap[3].parse::<u16>().unwrap(),
            kind,
            absorbed,
            critical
        });
    }

    return None
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn assert_dps_simple() {
        let tt = "Your Freezing Storm hit Major Thrall of Dark for 101 Ice damage.";
        assert!(RE_DPS.is_match("Your Freezing Storm hit Major Thrall of Dark for 101 Ice damage."));
        assert_eq!(
            parse_dps(tt).unwrap(),
            Dps{
                spell: "Freezing Storm".to_string(),
                target: "Major Thrall of Dark".to_string(),
                damage: 101,
                kind: "Ice".to_string(),
                absorbed: 0,
                critical: false
            }
        )
    }
    #[test]
    fn assert_dps_full_absorbed() {
        let tt = "Your Shatter Storm hit RexAlchy for 0 (51 absorbed).";
        assert!(RE_DPS.is_match(tt));
        assert_eq!(
            parse_dps(tt).unwrap(),
            Dps{
                spell: "Shatter Storm".to_string(),
                target: "RexAlchy".to_string(),
                damage: 0,
                kind: "".to_string(),
                absorbed: 51,
                critical: false
            }
        )
    }
    #[test]
    fn assert_dps_partially_absorbed() {
        let tt = "Your Spiral Cast hit Thrall Soul for 272 (12 absorbed) Ice damage.";
        assert!(RE_DPS.is_match(tt));
        assert_eq!(
            parse_dps(tt).unwrap(),
            Dps{
                spell: "Spiral Cast".to_string(),
                target: "Thrall Soul".to_string(),
                damage: 272,
                kind: "Ice".to_string(),
                absorbed: 12,
                critical: false
            }
        )
    }
    #[test]
    fn assert_dps_critical() {
        let tt = "Your Coalesce Forestry hit Urgu Myrmidon Chief for 311 Nature damage (Critical).";
        assert!(RE_DPS.is_match(tt));
        assert_eq!(
            parse_dps(tt).unwrap(),
            Dps{
                spell: "Coalesce Forestry".to_string(),
                target: "Urgu Myrmidon Chief".to_string(),
                damage: 311,
                kind: "Nature".to_string(),
                absorbed: 0,
                critical: true
            }
        )
    }
    #[test]
    fn assert_dps_partially_absorbed_critical() {
        let tt = "Your Retaliate hit UDeadPRO for 292 (233 absorbed) Nature damage (Critical).";
        assert!(RE_DPS.is_match(tt));
        assert_eq!(
            parse_dps(tt).unwrap(),
            Dps{
                spell: "Retaliate".to_string(),
                target: "UDeadPRO".to_string(),
                damage: 292,
                kind: "Nature".to_string(),
                absorbed: 233,
                critical: true
            }
        )
    }
    #[test]
    fn assert_dps_none() {
        let tt = "Your Holy Symbol hit Zankara for 0 (Critical).";
        assert!(RE_DPS.is_match(tt));
        assert_eq!(
            parse_dps(tt).unwrap(),
            Dps{
                spell: "Holy Symbol".to_string(),
                target: "Zankara".to_string(),
                damage: 0,
                kind: "".to_string(),
                absorbed: 0,
                critical: true
            }
        )
    }
}