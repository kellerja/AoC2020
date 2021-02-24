use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::fmt::Debug;

const RADIX: u32 = 10;

trait Evaluate : Debug + ToString {
    fn evaluate(&self) -> i64;

    fn is_binary(&self) -> bool;
}

#[derive(Debug)]
struct Expression {
    expr: Box<dyn Evaluate>
}

impl Evaluate for Expression {
    fn evaluate(&self) -> i64 {
        self.expr.evaluate()
    }

    fn is_binary(&self) -> bool {
        false
    }
}

impl ToString for Expression {
    fn to_string(&self) -> String {
        self.expr.to_string()
    }
}

#[derive(Debug)]
struct Multiplication {
    a: Box<dyn Evaluate>,
    b: Box<dyn Evaluate>
}

impl Evaluate for Multiplication {
    fn evaluate(&self) -> i64 {
        self.a.evaluate() * self.b.evaluate()
    }

    fn is_binary(&self) -> bool {
        true
    }
}

impl ToString for Multiplication {
    fn to_string(&self) -> String {
        let a_string = match self.a.is_binary() {
            true => format!("({})", self.a.to_string()),
            false => self.a.to_string()
        };
        let b_string = match self.b.is_binary() {
            true => format!("({})", self.b.to_string()),
            false => self.b.to_string()
        };
        format!("{} * {}", a_string, b_string)
    }
}

#[derive(Debug)]
struct Addition {
    a: Box<dyn Evaluate>,
    b: Box<dyn Evaluate>
}

impl Evaluate for Addition {
    fn evaluate(&self) -> i64 {
        self.a.evaluate() + self.b.evaluate()
    }

    fn is_binary(&self) -> bool {
        true
    }
}

impl ToString for Addition {
    fn to_string(&self) -> String {
        let a_string = match self.a.is_binary() {
            true => format!("({})", self.a.to_string()),
            false => self.a.to_string()
        };
        let b_string = match self.b.is_binary() {
            true => format!("({})", self.b.to_string()),
            false => self.b.to_string()
        };
        format!("{} + {}", a_string, b_string)
    }
}

impl Evaluate for i64 {
    fn evaluate(&self) -> i64 {
        *self
    }

    fn is_binary(&self) -> bool {
        false
    }
}

pub fn solve(input: &File, is_addition_precedence: bool) -> Option<i64> {
    let expressions = parse_input(input, is_addition_precedence);
    Some(expressions.iter().map(|expr| expr.evaluate()).sum())
}

fn parse_parentheses(remaining: &mut dyn Iterator<Item=char>, is_addition_precedence: bool) -> Box<dyn Evaluate> {
    let mut open_parentheses_count = 1;
    let inner = parse_expression(&mut remaining.take_while(|&c| {
        if c == '(' {
            open_parentheses_count += 1;
        } else if c == ')' {
            open_parentheses_count -= 1;
        }
        open_parentheses_count > 0
    }), is_addition_precedence);
    let _closing_tag = remaining.next();
    return inner
}

fn parse_constant(first: char, remaining: &mut dyn Iterator<Item=char>) -> Box<dyn Evaluate> {
    let mut constant: String = remaining.take_while(|c| c.is_digit(10)).collect();
    constant.insert(0, first);
    Box::from(constant.parse::<i64>().unwrap())
}

fn parse_right_side(remaining: &mut dyn Iterator<Item=char>, is_addition_precedence: bool) -> Box<dyn Evaluate> {
    while let Some(c) = remaining.next() {
        match c {
            _w if c.is_whitespace() => continue,
            '(' =>  return parse_parentheses(remaining, is_addition_precedence),
            c0 if c.is_digit(RADIX) => return parse_constant(c0, remaining),
            _ => panic!("Unexpected right side char {}. Remaining: {}", c, remaining.collect::<String>())
        }
    }
    panic!("Parsing right side failed. Remaining: {}", remaining.collect::<String>())
}

fn parse_expression(remaining: &mut dyn Iterator<Item=char>, is_addition_precedence: bool) -> Box<dyn Evaluate> {
    let mut prev_expression: Option<Box<dyn Evaluate>> = None;
    while let Some(c) = remaining.next() {
        prev_expression = Some(match c {
            _w if c.is_whitespace() => continue,
            '*' => {
                let b = if is_addition_precedence {
                    parse_expression(remaining, is_addition_precedence)
                } else {
                    parse_right_side(remaining, is_addition_precedence)
                };
                Box::from(Multiplication{ a: prev_expression.unwrap(), b: Box::from(b) })
            },
            '+' => {
                let b = parse_right_side(remaining, is_addition_precedence);
                Box::from(Addition{ a: prev_expression.unwrap(), b: Box::from(b) })
            },
            '(' => parse_parentheses(remaining, is_addition_precedence),
            ')' => panic!("Unexpected closing tag. Remaining: {}", remaining.collect::<String>()),
            c0 if c.is_digit(RADIX) => parse_constant(c0, remaining),
            _ => panic!("Unexpected right side char {}. Remaining: {}", c, remaining.collect::<String>())
        });
    }
    prev_expression.unwrap()
}

fn parse_input(input: &File, is_addition_precedence: bool) -> Vec<Expression> {
    BufReader::new(input).lines().map(|line| {
        let line = line.unwrap();
        let mut chars = line.chars();
        Expression { expr: parse_expression(&mut chars, is_addition_precedence) }
    }).collect()
}
