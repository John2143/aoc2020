use std::lazy::Lazy;

use regex::Regex;

#[derive(Debug)]
struct PWTest<'a> {
    min: usize,
    max: usize,
    letter: char,
    text: &'a str,
}

impl<'a> PWTest<'a> {
    fn validate(&self) -> usize {
        let a = self.text.chars().nth(self.min - 1).unwrap();
        let b = self.text.chars().nth(self.max - 1).unwrap();

        if a != b && (a == self.letter || b == self.letter) {
            1
        } else {
            0
        }
    }
}

const REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)-(\d+) (\w): (\d+)").unwrap());

impl<'a> PWTest<'a> {
    fn new(s: &'a str) -> Self {
        let m: Vec<_> = REGEX
            .captures(&s)
            .unwrap()
            .iter()
            .map(|x| x.unwrap().as_str())
            .collect();

        Self {
            min: m[1].parse().unwrap(),
            max: m[2].parse().unwrap(),
            letter: m[3].chars().nth(0).unwrap(),
            text: m[4],
        }
    }
}

pub fn main() {
    let valids: usize = crate::input(3)
        .lines()
        .map(PWTest::new)
        .map(|x| x.validate())
        .sum();
    println!("valid {}", valids);
}
