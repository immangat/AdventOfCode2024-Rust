
use regex::Regex;
use std::fs;

fn read_file(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path).expect("Reading a file");
    contents
}
fn part_one(file_content: &str) {
    // Need to read the string and get all mul(?.?) in the string
    // You can use regex for an easy way to do things
    let mut count = 0;
    let re = Regex::new(r"mul\((\d+),(\d+)\)|").ok();
    match re {
        None => {
            println!("Error while making the regex object")
        }
        Some(value) => {
            let captures = value.captures_iter(file_content);
            captures.for_each(|val| {
                let first_number: i32 = val.get(1).map_or("0", |m| m.as_str()).parse().unwrap();
                let second_number: i32 = val.get(2).map_or("0", |m| m.as_str()).parse().unwrap();
                count += first_number * second_number;
            })
        }
    }
    println!("{count}");
}

fn part_two(file_content: &str) {
    let mut count = 0;
    let re = Regex::new(r"mul\((\d+),(\d+)\)|don't\(\)|do\(\)").ok();
    match re {
        None => {
            println!("Error while making the regex object")
        }
        Some(value) => {
            let mut enable = true;
            let captures = value.captures_iter(file_content);
            for val in captures {
                let matched_string = val.get(0).map_or("", |m| m.as_str());
                if matched_string.starts_with("mul") && enable {
                    let first_number: i32 = val.get(1).map_or("0", |m| m.as_str()).parse().unwrap();
                    let second_number: i32 =
                        val.get(2).map_or("0", |m| m.as_str()).parse().unwrap();
                    count += first_number * second_number;
                    continue
                }
                if matched_string.starts_with("don") {
                    enable = false;
                    continue;
                }
                if matched_string.starts_with("do") {
                    enable = true;
                    continue
                }
            }
        }
    }
    println!("{count}");
}
fn main() {
    let file_content = read_file("input.txt");
    // part_one(&file_content);
    part_two(&file_content);
}
