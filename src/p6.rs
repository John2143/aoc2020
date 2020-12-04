use std::collections::HashMap;

#[derive(Default, Debug)]
struct Passport<'a> {
    fields: HashMap<&'a str, &'a str>,
}

impl<'a> Passport<'a> {
    fn from_lines(lines: Vec<&'a str>) -> Self {
        let mut s = Self::default();
        for field in lines.into_iter().map(|l| l.split(" ")).flatten() {
            let lr = field.split(":").collect::<Vec<_>>();
            s.fields.insert(lr[0], lr[1]);
        }
        s
    }

    fn validate(&self) -> bool {
        let needed = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

        for need in &needed {
            if let None = self.fields.get(need) {
                println!("Missing {}", need);
                return false;
            }
        }

        true
    }

    fn validate2(&self) -> bool {
        fn validate(key: &str, value: &str) -> bool {
            match key {
                "byr" => value
                    .parse::<i64>()
                    .ok()
                    .map(|n| n >= 1920 && n <= 2002)
                    .unwrap_or(false),
                "iyr" => value
                    .parse::<i64>()
                    .ok()
                    .map(|n| n >= 2010 && n <= 2020)
                    .unwrap_or(false),
                "eyr" => value
                    .parse::<i64>()
                    .ok()
                    .map(|n| n >= 2020 && n <= 2030)
                    .unwrap_or(false),
                "hgt" => {
                    let (h, typ) = value.split_at(value.len() - 2);
                    let num: i64 = match h.parse() {
                        Ok(n) => n,
                        Err(_) => return false,
                    };

                    match typ {
                        "cm" => num >= 150 && num <= 193,
                        "in" => num >= 59 && num <= 76,
                        _ => false,
                    }
                }
                "hcl" => {
                    value.len() == 7
                        && value.chars().nth(0).unwrap() == '#'
                        && i64::from_str_radix(&value.chars().skip(1).collect::<String>(), 16).is_ok()
                }
                "ecl" => {
                    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().find(|&x| x == &value).is_some()
                },
                "pid" => {
                    value.len() == 9 && value.parse::<i64>().is_ok()
                },
                "cid" => {
                    true
                },
                k => panic!(k.to_owned()),
            }
        }

        for (key, value) in &self.fields {
            let is_ok = validate(key, value);
            if !is_ok {
                return false;
            }
        }

        let needed = [
            "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid",
        ];

        for need in &needed {
            if let None = self.fields.get(need) {
                return false;
            }
        }

        true
    }
}

pub fn main() {
    let passlines = crate::input(6);
    let mut current = Vec::new();
    let mut valid = 0;
    for (i, line) in passlines.lines().enumerate() {
        match line {
            "" => {
                let p = Passport::from_lines(std::mem::replace(&mut current, Vec::new())).validate2();
                println!("{} {}", i, &p);
                if p {
                    valid += 1;
                }
            }
            s => current.push(s),
        }
    }

    let p = Passport::from_lines(std::mem::replace(&mut current, Vec::new()));
    if p.validate2() {
        valid += 1;
    }

    dbg!(&valid);
}
