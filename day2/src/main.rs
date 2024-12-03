use std::fs;
use std::time::Instant;

fn part_1(contents: String) {
    let start = Instant::now();
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
    println!("{count}");
    let duration = start.elapsed();

    // Print the execution time
    println!("Program executed in: {:?} for part_1", duration);
}

fn part_2(contents: String) {
    let start = Instant::now();
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
        } else {
            
            for j in 0..report.len() {
                let mut failed = false;
                let mut first = true;
                let mut up = true;
                let mut new_report = report.clone();
                new_report.remove(j);
                for i in 0..new_report.len() - 1 {
                    let difference = new_report[i] - new_report[i + 1];
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
                    count += 1;
                    break
                }
            }
        }
    });
    println!("{count}");
    let duration = start.elapsed();

    // Print the execution time
    println!("Program executed in: {:?} for part_2", duration);
}
fn main() {
    /*
    Read the file
    Read each line
    Convert each line to vector of numbers
    Store each in a vector
    Go over each vector in the vector
    check if it is safe
    if yes add to count
    return count
     */

    let contents = fs::read_to_string("./input.txt").expect("Should read a file here");
    part_1(contents.clone());
    part_2(contents.clone());
}
