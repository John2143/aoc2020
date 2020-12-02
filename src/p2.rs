pub fn main() {
    let numbers = std::fs::read_to_string("./data/d1.txt").unwrap();

    let numbers: Vec<i64> = numbers
        .lines()
        .into_iter()
        .map(|x| x.parse().expect("invalid number"))
        .collect();

    //ez
    for a in &numbers {
        for b in &numbers {
            for c in &numbers {
                if a + b + c == 2020 {
                    println!("a {}, b {}, c{}, a * b * c {}", a, b, c, a * b * c);
                }
            }
        }
    }
}
