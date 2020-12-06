#![feature(never_type, once_cell)]
mod p7;

fn input(day: i64) -> String {
    std::fs::read_to_string(format!("./data/d{}.txt", day)).unwrap()
}

fn main() {
    p7::main();
}
