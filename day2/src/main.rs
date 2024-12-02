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
    let reports: Vec<Vec<i64>> = lines
        .map(|line| {
            let levels: Vec<_> = line.split_whitespace().collect();
            levels
                .iter()
                .filter_map(|number| number.parse().ok())
                .collect()
        })
        .collect();
    let mut count = 0;
    reports.iter().for_each(|report| {
        let mut failed = false;
        let mut first = true;
        let mut up = true;
        for i in 0..report.len() - 1 {
            let difference = report[i] - report[i + 1];
            if first {  
                first = false;
                up = difference < 0;
            }
            match difference {
                0 => {
                    failed = true;
                    break;
                }
                ..0 => {
                    if !(difference.abs() <= 3) || !up { 
                        failed = true;
                        break;
                    }
                    
                }
                0.. => {
                    if !(difference.abs() <= 3) || up {
                        failed = true;
                        break;
                    }
                }
            }
        }
        if !failed {
            count += 1
        }
    });
    println!("{count}")
}
