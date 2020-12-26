use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::ops::Range;
use std::collections::HashSet;
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
    fields: Vec<String>,
    values: Vec<u64>
}

impl Ticket {
    fn parse(input: &str) -> Option<Self> {
        let input = input.split(",");
        let mut values = Vec::new();
        for s in input {
            if let Ok(value) = s.parse() {
                values.push(value);
            } else {
                return None;
            }
        }
        Some(Ticket { fields: Vec::with_capacity(values.len()), values })
    }

    fn find_illegal_fields(&self, rules: &Vec<TicketField>) -> Vec<u64> {
        if rules.is_empty() {
            return Vec::new();
        }
        self.values.iter()
            .filter(|&num|
                !rules.iter().map(|rule| rule.is_valid(*num)).fold(false, |acc, result| acc || result)
            ).map(|num| *num).collect()
    }
}

fn get_possible_fields_map(fields: &[TicketField]) -> Vec<Vec<&str>> {
    let field_names: Vec<&str> = fields.iter().map(|field| field.name.as_str()).collect();
    vec![field_names.clone(); field_names.len()]
}

fn remove_invalid_possible_fields(possible_fields: &mut Vec<Vec<&str>>, valid_tickets: &[&Ticket], fields: &[TicketField]) {
    for ticket in valid_tickets {
        for (i, &value) in ticket.values.iter().enumerate() {
            for field in fields {
                if !field.is_valid(value) {
                    possible_fields[i].retain(|&name| name != field.name);
                }
            }
        }
    }
}

fn remove_cerain_fields_from_uncertain_fields(possible_fields: &mut Vec<Vec<&str>>) {
    let mut handled_fields = HashSet::with_capacity(possible_fields.len());
    loop {
        let certain_fields: HashSet<&str> = possible_fields.iter()
            .filter(|fields| fields.len() == 1)
            .map(|fields| fields.first().unwrap().to_owned()).collect();
        let unhandled_field = certain_fields.difference(&handled_fields).next();
        match unhandled_field {
            None => break,
            Some(unhandled_field) => {
                for fields in possible_fields.iter_mut() {
                    if fields.len() == 1 {
                        continue;
                    }
                    fields.retain(|field| field != unhandled_field);
                }
                let unhandled_field = unhandled_field.to_owned();
                handled_fields.insert(unhandled_field);
            }
        }
    }
}

fn certain_field_map<'a>(possible_fields: &'a Vec<Vec<&str>>) -> Option<Vec<&'a str>> {
    if possible_fields.iter().any(|field| field.len() != 1) {
        return None
    } else {
        Some(possible_fields.iter().map(|field| field.first().unwrap().to_owned()).collect())
    }
}

pub fn solve(input: &File) -> (Option<u64>, Option<u64>) {
    let input = parse_input(input);
    if let Some((fields, my_ticket, tickets)) = input {
        let mut valid_tickets: Vec<&Ticket> = tickets.iter().filter(|ticket| ticket.find_illegal_fields(&fields).is_empty()).collect();
        valid_tickets.push(&my_ticket);
        let mut possible_field_map = get_possible_fields_map(&fields);
        remove_invalid_possible_fields(&mut possible_field_map, &valid_tickets, &fields);
        remove_cerain_fields_from_uncertain_fields(&mut possible_field_map);
        let field_map = certain_field_map(&possible_field_map);

        let departure_fields_product = field_map.and_then(|field_map| {
            let mut result = 1;
            for (i, field) in field_map.iter().enumerate() {
                if field.starts_with("departure") {
                    result *= my_ticket.values[i];
                }
            }
            Some(result)
        });
        let invalid_ticket_fields_sum = tickets.iter().map(|ticket| ticket.find_illegal_fields(&fields).iter().sum::<u64>()).sum();
        (Some(invalid_ticket_fields_sum), departure_fields_product)
    } else {
        (None, None)
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
