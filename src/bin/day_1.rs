use std::{fs, io};

fn main() {
    let (left_list, right_list) = get_sorted_lists().unwrap();

    let total_distance: u32 = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(lhs, rhs)| lhs.abs_diff(*rhs))
        .sum();

    println!("Distance: {}", total_distance);

    let similarity_score: usize = left_list
        .iter()
        .map(|val| *val as usize * right_list.iter().filter(|v| **v == *val).count())
        .sum();

    println!("Similarity Score: {}", similarity_score);
}

fn get_sorted_lists() -> io::Result<(Vec<i32>, Vec<i32>)> {
    let (mut left_list, mut right_list) = get_lists()?;
    left_list.sort();
    right_list.sort();

    Ok((left_list, right_list))
}

fn get_lists() -> io::Result<(Vec<i32>, Vec<i32>)> {
    Ok(fs::read_to_string("Input_Day_1")?
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(lhs, rhs)| (lhs.parse::<i32>().unwrap(), rhs.parse::<i32>().unwrap()))
        .unzip())
}
