use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("Input_Day_11").unwrap();
    let stones = input
        .split_whitespace()
        .flat_map(str::parse::<u64>)
        .collect::<Vec<_>>();

    let mut cache = HashMap::new();

    let iter_25: u64 = stones
        .iter()
        .map(|stone| blink(*stone, 25, &mut cache))
        .sum();
    println!("25: {}", iter_25);

    let iter_75: u64 = stones
        .iter()
        .map(|stone| blink(*stone, 75, &mut cache))
        .sum();
    println!("75: {}", iter_75);
}

fn blink(stone: u64, iter_count: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    fn inner(stone: u64, iter_count: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
        if iter_count == 0 {
            return 1;
        }

        match stone {
            0 => blink(1, iter_count - 1, cache),
            stone if (stone.ilog10() + 1) % 2 == 0 => {
                let divisor = 10_u64.pow((stone.ilog10() + 1) / 2);
                let first = stone / divisor;
                let second = stone % divisor;

                blink(first, iter_count - 1, cache) + blink(second, iter_count - 1, cache)
            }
            stone => blink(stone * 2024, iter_count - 1, cache),
        }
    }

    let key = &(stone, iter_count);
    if let Some(cached) = cache.get(key) {
        *cached
    } else {
        let res = inner(stone, iter_count, cache);
        cache.insert(*key, res);
        res
    }
}
