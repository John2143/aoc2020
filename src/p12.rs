use std::collections::HashMap;

pub fn main() {
    let mut input: Vec<usize> = crate::input(12).lines().map(|x| x.parse().unwrap()).collect();
    input.sort();

    let (_, ones, threes) = input.iter().fold((0, 0, 1), |(last, ones, threes), &current| {
        let diff = current - last;
        match diff {
            3 => (current, ones, threes + 1),
            1 => (current, ones + 1, threes),
            _ => (current, ones, threes),
        }
    });

    dbg!(ones, threes);

    dbg!(ones * threes);

    let mut k = vec![0];
    k.append(&mut input);

    let input = k;

    dbg!(&input);

    let mut path_cache = HashMap::new();

    path_cache.insert(input.len() - 1, 1);

    dbg!(&find_path(&input, 0, &mut path_cache));
}

fn find_path(input: &[usize], from: usize, cache: &mut HashMap<usize, usize>) -> usize {
    if let Some(v) = cache.get(&from) {
        return *v;
    }

    let res = [from + 1, from + 2, from + 3].iter().map(|&i| {
        if i < input.len() && input[i] - input[from] <= 3 {
            find_path(&input, i, cache)
        } else {
            0
        }
    }).sum();

    cache.insert(from, res);

    res
}
