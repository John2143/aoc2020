const PREAMBLE_LENGTH: usize = 25;

fn is_valid(preamble: &[i64], current: i64) -> bool {
    for a in preamble {
        for b in preamble {
            if a + b == current {
                return true;
            }
        }
    }

    false
}

fn find_first_bad_number(input: &[i64]) -> i64 {
    let mut preamble = vec![0; PREAMBLE_LENGTH];

    for (i, &current) in input.iter().enumerate() {
        if i < PREAMBLE_LENGTH {
            preamble[i] = current;
            continue;
        }

        let ok = is_valid(&preamble, current);

        if !ok {
            return current;
        }

        preamble[i % PREAMBLE_LENGTH] = current;
    }

    panic!();
}

fn find_sum_range(input: &[i64], target: i64) -> i64 {
    for start in 0 .. input.len() {
        for end in start .. input.len() {
            let nums = input.iter().skip(start).take(end - start);
            let sum = nums.sum::<i64>();

            if sum == target {
                let min = input.iter().skip(start).take(end - start).min();
                let max = input.iter().skip(start).take(end - start).max();

                return min.unwrap() + max.unwrap();
            } else if sum > target {
                break;
            }
        }
    }

    panic!();
}


pub fn main() {
    let input: Vec<i64> = crate::input(11).lines().map(|x| x.parse().unwrap()).collect();

    let first_bad = find_first_bad_number(&input);

    println!("first bad: {}", first_bad);

    let range = find_sum_range(&input, first_bad);

    dbg!(&range);
}
