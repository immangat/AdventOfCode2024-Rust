use std::fs;

fn main() {
    println!("Hello, world!");
    /*
    Read the file
    Read each line
    Convert each line to vector of numbers
    Store each in a vector

     */

    let contents = fs::read_to_string("./input.txt").expect("Should read a file here");
    let lines = contents.lines();
    let reports: Vec<Vec<i64>>= lines.map(|line|{
        let levels: Vec<_> = line.split_whitespace().collect();
        levels.iter().filter_map(|number|{
            number.parse().ok()
        }).collect()
    }).collect();
}
