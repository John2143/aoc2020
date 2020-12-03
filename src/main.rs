#![feature(never_type, once_cell)]
mod p5;

fn input(day: i64) -> String {
    std::fs::read_to_string(format!("./data/d{}.txt", day)).unwrap()
}

fn main() {
    p5::main();
}
