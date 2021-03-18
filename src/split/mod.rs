
use chrono::prelude::{DateTime, FixedOffset};
use chrono::Duration;


#[derive(Debug, PartialEq)]
pub struct Fight{
    start:DateTime<FixedOffset>,
    end:DateTime<FixedOffset>
}

pub fn split_in_fight(mut list: Vec<DateTime<FixedOffset>>, diff : i64)-> Vec<Fight>{

    if list.len() == 0 {
        return vec![]
    }
    if list.len() == 1 {
        return vec![Fight{
            start: *list.first().unwrap(),
            end: *list.first().unwrap(),
        }]
    }

    let mut res = vec![];

    list.sort();

    let mut start = *list.first().unwrap();
    let mut previous = start;

    for current in  list{
        if current - previous > Duration::seconds(diff) {
            res.push(Fight{
                 start,
                end: previous
            });
            start = current;
        }
        previous = current;
    }
    res.push(Fight{
        start,
        end: previous
    });

    return res
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn assert_one_date() {
        assert_eq!(
            split_in_fight(vec![
                DateTime::parse_from_rfc3339("2021-03-17T20:30:45.111Z").unwrap()
            ],60),
            vec![Fight{
                start: DateTime::parse_from_rfc3339("2021-03-17T20:30:45.111Z").unwrap(),
                end: DateTime::parse_from_rfc3339("2021-03-17T20:30:45.111Z").unwrap()
            }]
        )
    }
    #[test]
    fn assert_multiple_date() {
        assert_eq!(
            split_in_fight(vec![
                DateTime::parse_from_rfc3339("2021-03-17T20:30:45.111Z").unwrap(),
                DateTime::parse_from_rfc3339("2021-03-17T20:30:55.111Z").unwrap(),
                DateTime::parse_from_rfc3339("2021-03-17T20:31:28.111Z").unwrap(),
                DateTime::parse_from_rfc3339("2021-03-17T17:18:44.111Z").unwrap(),
                DateTime::parse_from_rfc3339("2021-03-17T17:18:45.111Z").unwrap(),
                DateTime::parse_from_rfc3339("2021-03-17T15:02:12.111Z").unwrap(),
                DateTime::parse_from_rfc3339("2021-03-17T15:01:45.111Z").unwrap(),
                DateTime::parse_from_rfc3339("2021-03-17T15:02:45.111Z").unwrap(),
                DateTime::parse_from_rfc3339("2021-03-17T15:03:45.111Z").unwrap()
            ],120),
            vec![Fight{
                start: DateTime::parse_from_rfc3339("2021-03-17T15:01:45.111Z").unwrap(),
                end: DateTime::parse_from_rfc3339("2021-03-17T15:03:45.111Z").unwrap()
            },Fight{
                start: DateTime::parse_from_rfc3339("2021-03-17T17:18:44.111Z").unwrap(),
                end: DateTime::parse_from_rfc3339("2021-03-17T17:18:45.111Z").unwrap()
            },Fight{
                start: DateTime::parse_from_rfc3339("2021-03-17T20:30:45.111Z").unwrap(),
                end: DateTime::parse_from_rfc3339("2021-03-17T20:31:28.111Z").unwrap()
            },]
        )
    }
}