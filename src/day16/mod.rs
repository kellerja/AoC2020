use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::ops::Range;
use regex::Regex;

lazy_static! {
    static ref FIELD_RULE_PATTERN: Regex = Regex::new(r"(?P<name>.+): (?P<low_start>\d+)-(?P<low_end>\d+) or (?P<high_start>\d+)-(?P<high_end>\d+)").unwrap();
}

struct TicketField {
    name: String,
    rules: Vec<Range<u64>>
}

impl TicketField {
    fn parse(input: &str) -> Option<Self> {
        FIELD_RULE_PATTERN.captures(input).and_then(|cap| {
            Some(Self {
                name: cap["name"].to_owned(),
                rules: vec![
                    Range {start: cap["low_start"].parse().unwrap(), end: cap["low_end"].parse::<u64>().unwrap() + 1},
                    Range {start: cap["high_start"].parse().unwrap(), end: cap["high_end"].parse::<u64>().unwrap() + 1}
                    ]
            })
        })
    }

    fn is_valid(&self, number: u64) -> bool {
        for rule in &self.rules {
            if rule.contains(&number) {
                return true;
            }
        }
        false
    }
}

#[derive(Clone)]
struct Ticket {
    numbers: Vec<u64>
}

impl Ticket {
    fn parse(input: &str) -> Option<Self> {
        let input = input.split(",");
        let mut numbers = Vec::new();
        for s in input {
            if let Ok(value) = s.parse() {
                numbers.push(value);
            } else {
                return None;
            }
        }
        Some(Ticket { numbers })
    }

    fn find_illegal_fields(&self, rules: &Vec<TicketField>) -> Vec<u64> {
        if rules.is_empty() {
            return Vec::new();
        }
        self.numbers.iter()
            .filter(|&num|
                !rules.iter().map(|rule| rule.is_valid(*num)).fold(false, |acc, result| acc || result)
            ).map(|num| *num).collect()
    }
}

pub fn solve(input: &File) -> Option<u64> {
    let input = parse_input(input);
    if let Some((fields, _my_ticket, tickets)) = input {
        Some(tickets.iter().map(|ticket| ticket.find_illegal_fields(&fields).iter().sum::<u64>()).sum())
    } else {
        None
    }
}

fn parse_input(input: &File) -> Option<(Vec<TicketField>, Ticket, Vec<Ticket>)> {
    let mut lines = BufReader::new(input).lines();
    let mut fields = Vec::new();
    loop {
        let line = lines.next();
        let field = line.and_then(|line| TicketField::parse(&line.unwrap()));
        match field {
            Some(field) => fields.push(field),
            None => break
        }
    };
    lines.next();
    let my_ticket = lines.next().and_then(|line| Ticket::parse(&line.unwrap()));
    lines.next();
    lines.next();
    let tickets: Vec<Ticket> = lines.map(|line| Ticket::parse(&line.unwrap()).unwrap()).collect();
    if fields.is_empty() || my_ticket.is_none() || tickets.is_empty() {
        None
    } else {
        Some((fields, my_ticket.unwrap(), tickets))
    }
}
