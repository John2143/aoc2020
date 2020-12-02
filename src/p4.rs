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
        let m = REGEX.captures(&s).unwrap();

        Self {
            min: m.get(1).unwrap().as_str().parse().unwrap(),
            max: m.get(2).unwrap().as_str().parse().unwrap(),
            letter: m.get(3).unwrap().as_str().chars().nth(0).unwrap(),
            text: m.get(4).unwrap().as_str(),
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
