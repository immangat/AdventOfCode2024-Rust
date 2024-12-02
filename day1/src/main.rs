use day1::{abs};
use std::collections::HashMap;
use std::fs;

fn main() {
    println!("Day 1\n-----------");

    const INPUT_FILE: &str = "./input.txt";
    let file_contents = fs::read_to_string(INPUT_FILE).expect("Failed to read input file.");
    let lines = file_contents.lines();

    let mut first_column_numbers: Vec<i64> = Vec::new();
    let mut second_column_numbers: Vec<i64> = Vec::new();

    lines.filter_map(|line| {
        let numbers: Vec<_> = line.split_whitespace().collect();
        if numbers.len() < 2 {
            None
        } else {
            Some((numbers[0].parse().ok()?, numbers[1].parse().ok()?))
        }
    }).for_each(|(number_one, number_two)| {
        first_column_numbers.push(number_one);
        second_column_numbers.push(number_two);
    });

    first_column_numbers.sort();
    second_column_numbers.sort();

    let count: i64 = first_column_numbers.iter()
        .zip(second_column_numbers.iter())
        .map(|(a, b)| abs(a - b))
        .sum();
    println!("{count}");

    let mut scores: HashMap<i64, i64> = HashMap::new();
    for number in second_column_numbers {
        *scores.entry(number).or_insert(0) += 1;
    }

    let total_score: i64 = scores.into_iter().map(|(key, value)| key * value).sum();
    println!("{total_score}");
}
