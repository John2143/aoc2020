use std::str::FromStr;

struct PWTest<'a> {
    min: usize,
    max: usize,
    letter: char,
    text: &'a str,
}

impl<'a> PWTest<'a> {
    fn validate(&self) -> usize {
        let count = self.text.chars().fold(0usize, |acc, x| if x == self.letter { acc + 1 } else { acc });

        if count <= self.max && count >= self.min {
            1
        }else{
            0
        }
    }
}

fn from_str(s: &str) -> PWTest<'_> {
    let parts: Vec<_> = s.split(" ").collect();
    assert!(parts.len() == 3);
    let ranges: Vec<_> = parts[0].split("-").collect();
    assert!(ranges.len() == 2);

    PWTest {
        min: ranges[0].parse().unwrap(),
        max: ranges[1].parse().unwrap(),
        letter: parts[1].chars().nth(0).unwrap(),
        text: parts[2],
    }
}

pub fn main() {
    let valids: usize = crate::input(3).lines().map(from_str).map(|x| x.validate()).sum();
    println!("valid {}", valids);
}
