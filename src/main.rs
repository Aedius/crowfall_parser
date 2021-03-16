mod dps;
mod heal;

#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::{fs, io};
use std::fs::{File};
use std::io::BufReader;
use std::io::prelude::*;
use std::time::{SystemTime, Duration};
use dps::{Dps, RE_DPS, parse_dps};
use heal::{Heal, RE_HEAL, parse_heal};


fn main() -> io::Result<()> {
    let path = "C:\\Users\\Admin\\AppData\\LocalLow\\Art+Craft\\Crowfall\\CombatLogs";

    let month_ago = SystemTime::now().checked_sub(Duration::new(3 * 30 * 24 * 60 * 60, 0)).unwrap();

    let re_event = Regex::new(r"Event=\[(.*)\] ").unwrap();

    let entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    let mut data = Data {
        dps: Default::default(),
        heal: Default::default(),
    };

    for entry in entries {
        if entry.is_file() {
            let file = File::open(entry.as_path())?;

            if file.metadata()?.created()? > month_ago {
                let mut buf_reader = BufReader::new(file);
                let mut contents = String::new();
                buf_reader.read_to_string(&mut contents)?;

                let lines = contents.lines();

                for line in lines {
                    for cap in re_event.captures_iter(line) {
                        if !data.parse_row(&cap[1]) {
                            println!("cannot parse : {:?}", &cap[1]);
                            panic!()
                        }
                    }
                }
            }
        }
    }

    Ok(())
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
    fn parse_row(&mut self, row: &str) -> bool {
        if RE_FOOD.is_match(row) {
            return true;
        }
        if RE_RESOURCE.is_match(row) {
            return true;
        }

        if RE_SELF_RESOURCE.is_match(row) {
            return true;
        }
        if RE_DPS.is_match(row) {
            let dps = parse_dps(row).unwrap();
            self.dps.push(dps);
            return true;
        }
        if RE_HEAL.is_match(row) {
            let heal = parse_heal(row).unwrap();
            self.heal.push(heal);
            return true;
        }

        println!("{:?}", row);
        return false;
    }
}
