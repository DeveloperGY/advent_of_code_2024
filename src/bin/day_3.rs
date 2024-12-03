use std::fs;

fn main() {
    let memory = fs::read_to_string("Input_Day_3").unwrap();
    let sum = execute_muls(&memory);

    println!("Sum: {}", sum);

    let conditional_sum: u64 = memory
        .split("do()")
        .map(|s| {
            if let Some(index) = s.find("don't()") {
                s[..index].to_string()
            } else {
                s.to_string()
            }
        })
        .map(|s| execute_muls(&s))
        .sum();

    println!("Conditional Sum: {}", conditional_sum);
}

fn execute_muls(input: &str) -> u64 {
    input
        .split("mul")
        .filter(|s| s.starts_with('(') && s.contains(')'))
        .map(|s| {
            let first_end_paren_index = s.find(')').unwrap();
            s[..=first_end_paren_index].to_string()
        })
        .filter(|s| s.len() <= 9 && s.contains(','))
        .map(|s| s.trim_start_matches('(').trim_end_matches(')').to_string())
        .map(|s| {
            let comma_index = s.find(',').unwrap();
            (
                s[..comma_index].to_string(),
                s[comma_index + 1..].to_string(),
            )
        })
        .flat_map(|(lhs, rhs)| {
            let lhs = lhs.parse::<u16>().ok();
            let rhs = rhs.parse::<u16>().ok();

            match (lhs, rhs) {
                (Some(lhs), Some(rhs)) => Some((lhs, rhs)),
                _ => None,
            }
        })
        .map(|(lhs, rhs)| lhs as u64 * rhs as u64)
        .sum()
}
