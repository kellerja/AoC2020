use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;

const BIRTH_YEAR_TOKEN: &str = "byr";
const ISSUE_YEAR_TOKEN: &str = "iyr";
const EXP_YEAR_TOKEN: &str = "eyr";
const HEIGHT_TOKEN: &str = "hgt";
const HAIR_COLOR_TOKEN: &str = "hcl";
const EYE_COLOR_TOKEN: &str = "ecl";
const PASSPORT_ID_TOKEN: &str = "pid";
const COUNTRY_ID_TOKEN: &str = "cid";

#[derive(Debug)]
struct Passport {
    birth_year: Option<isize>,
    issue_year: Option<isize>,
    expiration_year: Option<isize>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<usize>
}

impl Passport {
    fn is_valid(&self) -> bool {
        self.birth_year.is_some() && self.issue_year.is_some() && self.expiration_year.is_some() &&
        self.height.is_some() && self.hair_color.is_some() && self.eye_color.is_some() && self.passport_id.is_some()
    }

    fn parse(input: &str) -> Passport {
        let mut passport = Passport::empty_passport();
        for key_value in input.split_whitespace() {
            let mut entry = key_value.split(":");
            let key = entry.next().unwrap();
            let value = entry.next().unwrap();
            let exception = format!("Could not parse value {} for key {}", value, key);
            match key {
                BIRTH_YEAR_TOKEN => passport.birth_year = Some(value.parse().expect(exception.as_str())),
                ISSUE_YEAR_TOKEN => passport.issue_year = Some(value.parse().expect(exception.as_str())),
                EXP_YEAR_TOKEN => passport.expiration_year = Some(value.parse().expect(exception.as_str())),
                HEIGHT_TOKEN => passport.height = Some(value.to_owned()),
                HAIR_COLOR_TOKEN => passport.hair_color = Some(value.to_owned()),
                EYE_COLOR_TOKEN => passport.eye_color = Some(value.to_owned()),
                PASSPORT_ID_TOKEN => passport.passport_id = Some(value.to_owned()),
                COUNTRY_ID_TOKEN => passport.country_id = Some(value.parse().expect(exception.as_str())),
                _ => println!("Unhandled key {} value {}", key, value)
            }
        }
        passport
    }

    fn empty_passport() -> Passport {
        Passport {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_color: None,
            eye_color: None,
            passport_id: None,
            country_id: None
        }
    }
}

pub fn solve(input: &File) -> Option<usize> {
    Some(parse_input(input).iter().filter(|passport| passport.is_valid()).count())
}

fn parse_input(input: &File) -> Vec<Passport> {
    let mut reader = BufReader::new(input);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();

    let whitespace_pattern = Regex::new(r"(?m)[[:space:]]+").unwrap();
    let new_entry_pattern = Regex::new(r"(?m)(\r\n|\n){2}").unwrap();
    new_entry_pattern.split(contents.as_str())
        .map(|item| whitespace_pattern.replace_all(item, " "))
        .map(|item| Passport::parse(&item)).collect()
}
