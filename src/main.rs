#![feature(never_type, once_cell, array_chunks)]
mod p10;

fn input(day: i64) -> String {
    std::fs::read_to_string(format!("./data/d{}.txt", day)).unwrap()
}

fn main() {
    p10::main();
}
