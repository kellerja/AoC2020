use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;

pub struct FieldPresenceValidator;
pub struct FieldValueValidator;

pub trait PassportValidator {
    fn is_valid(&self, passport: &Passport) -> bool;
}

impl PassportValidator for FieldPresenceValidator {
    fn is_valid(&self, passport: &Passport) -> bool {
        passport.birth_year.is_some() && passport.issue_year.is_some() && passport.expiration_year.is_some() &&
        passport.height.is_some() && passport.hair_color.is_some() && passport.eye_color.is_some() && passport.passport_id.is_some()
    }
}

impl FieldValueValidator {

    fn get_year(&self, value: &Option<String>) -> Option<u16> {
        value.as_ref().and_then(|value| {
            match value.parse::<u16>() {
                Ok(year) => Some(year),
                _ => None
            }
        })
    }

    fn is_birth_year_valid(&self, value: &Option<String>) -> bool {
        match self.get_year(value) {
            Some(year) => 1920 <= year && year <= 2020,
            _ => false
        }
    }

    fn is_issue_year_valid(&self, value: &Option<String>) -> bool {
        match self.get_year(value) {
            Some(year) => 2010 <= year && year <= 2020,
            _ => false
        }
    }

    fn is_expiration_year_valid(&self, value: &Option<String>) -> bool {
        match self.get_year(value) {
            Some(year) => 2020 <= year && year <= 2030,
            _ => false
        }
    }

    fn is_height_valid(&self, value: &Option<String>) -> bool {
        if let Some(raw_value) = value {
            let height_pattern = Regex::new(r"^(?P<amount>[[:digit:]]+)(?P<unit>cm|in)$").unwrap();
            if let Some(value) = height_pattern.captures(raw_value) {
                if let Ok(amount) = value["amount"].parse::<u8>() {
                    match value["unit"].as_ref() {
                        "cm" => return 150 <= amount && amount <= 193,
                        "in" => return 59 <= amount && amount <= 76,
                        _ => eprintln!("Unexpected height: {}", raw_value)
                    }
                }
            }
        }
        false
    }

    fn is_hair_color_valid(&self, value: &Option<String>) -> bool {
        if let Some(value) = value {
            let height_pattern = Regex::new(r"^#[[:xdigit:]]{6}$").unwrap();
            height_pattern.is_match(value)
        } else {
            false
        }
    }

    fn is_eye_color_valid(&self, value: &Option<String>) -> bool {
        if let Some(value) = value {
            let height_pattern = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
            height_pattern.is_match(value)
        } else {
            false
        }
    }

    fn is_passport_id_valid(&self, value: &Option<String>) -> bool {
        if let Some(value) = value {
            let height_pattern = Regex::new(r"^[[:digit:]]{9}$").unwrap();
            height_pattern.is_match(value)
        } else {
            false
        }
    }
}


impl PassportValidator for FieldValueValidator {
    fn is_valid(&self, passport: &Passport) -> bool {
        self.is_birth_year_valid(&passport.birth_year) && self.is_issue_year_valid(&passport.issue_year) && self.is_expiration_year_valid(&passport.expiration_year)
        && self.is_height_valid(&passport.height) && self.is_passport_id_valid(&passport.passport_id)
        && self.is_hair_color_valid(&passport.hair_color) && self.is_eye_color_valid(&passport.eye_color)
    }
}

const BIRTH_YEAR_TOKEN: &str = "byr";
const ISSUE_YEAR_TOKEN: &str = "iyr";
const EXP_YEAR_TOKEN: &str = "eyr";
const HEIGHT_TOKEN: &str = "hgt";
const HAIR_COLOR_TOKEN: &str = "hcl";
const EYE_COLOR_TOKEN: &str = "ecl";
const PASSPORT_ID_TOKEN: &str = "pid";
const COUNTRY_ID_TOKEN: &str = "cid";

#[derive(Debug)]
pub struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>
}

impl Passport {
    fn parse(input: &str) -> Passport {
        let mut passport = Passport::empty_passport();
        for key_value in input.split_whitespace() {
            let mut entry = key_value.split(":");
            let key = entry.next().unwrap();
            let value = entry.next().unwrap();
            match key {
                BIRTH_YEAR_TOKEN => passport.birth_year = Some(value.to_owned()),
                ISSUE_YEAR_TOKEN => passport.issue_year = Some(value.to_owned()),
                EXP_YEAR_TOKEN => passport.expiration_year = Some(value.to_owned()),
                HEIGHT_TOKEN => passport.height = Some(value.to_owned()),
                HAIR_COLOR_TOKEN => passport.hair_color = Some(value.to_owned()),
                EYE_COLOR_TOKEN => passport.eye_color = Some(value.to_owned()),
                PASSPORT_ID_TOKEN => passport.passport_id = Some(value.to_owned()),
                COUNTRY_ID_TOKEN => passport.country_id = Some(value.to_owned()),
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

pub fn solve(input: &File, validator: impl PassportValidator) -> Option<usize> {
    Some(parse_input(input).iter().filter(|passport| validator.is_valid(passport)).count())
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
