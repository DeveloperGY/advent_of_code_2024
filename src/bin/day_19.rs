use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("Input_Day_19").unwrap();
    let (towels, patterns) = process_input(&input);

    let mut pattern_cache = HashMap::new();

    let valid_combination_count = patterns
        .into_iter()
        .map(|p| valid_pattern_count(&towels, &p, &mut pattern_cache))
        .filter(|count| *count > 0)
        .collect::<Vec<_>>();

    let valid_pattern_count = valid_combination_count.iter().count();
    let total_combo_count: u64 = valid_combination_count.iter().sum();

    println!(
        "Advent of Code 2024 - Day 19\n[Part 1] Valid Pattern Count: {}\n[Part 2] Total Combinations: {}",
        valid_pattern_count,
        total_combo_count
    );
}

fn process_input(input: &str) -> (Vec<String>, Vec<String>) {
    // Returns a list of towels and a list of patterns

    // First line is the list of towels
    // All other lines are patterns

    let (towels, patterns) = input.split_once("\n\n").unwrap();

    let towels = towels
        .split(',')
        .map(|towel_str| towel_str.trim().to_string())
        .collect();

    let patterns = patterns
        .lines()
        .map(|pattern_str| pattern_str.trim().to_string())
        .collect();

    (towels, patterns)
}

fn valid_pattern_count(
    towels: &[String],
    pattern: &str,
    pattern_cache: &mut HashMap<String, u64>,
) -> u64 {
    if let Some(count) = pattern_cache.get(pattern) {
        return *count;
    }

    if pattern.is_empty() {
        return 1;
    }

    let possible_towels = towels
        .iter()
        .filter(|towel| pattern.starts_with(towel.as_str()))
        .collect::<Vec<_>>();

    if possible_towels.is_empty() {
        pattern_cache.insert(pattern.to_string(), 0);
        return 0;
    }

    let working_combination_count = possible_towels
        .into_iter()
        .map(|towel| {
            valid_pattern_count(towels, pattern.strip_prefix(towel).unwrap(), pattern_cache)
        })
        .filter(|count| *count > 0)
        .reduce(|lhs, rhs| lhs + rhs);

    if let Some(working_combination_count) = working_combination_count {
        pattern_cache.insert(pattern.to_string(), working_combination_count);
        working_combination_count
    } else {
        pattern_cache.insert(pattern.to_string(), 0);
        0
    }
}
