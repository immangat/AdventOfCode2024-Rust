use std::fs;

#[derive(Debug)]
enum Direction {
    Nine,
    Eleven,
    Twelve,
    One,
    Three,
    Five,
    Six,
    Seven,
}

const PATTERN: [char; 4] = ['X', 'M', 'A', 'S'];
const PATTERN_TWO: [char; 2] = ['M', 'S'];
impl Direction {
    // Create an iterator over all variants
    fn all() -> Vec<Direction> {
        vec![
            Direction::Nine,
            Direction::Eleven,
            Direction::Twelve,
            Direction::One,
            Direction::Three,
            Direction::Five,
            Direction::Six,
            Direction::Seven,
        ]
    }
    fn subset() -> Vec<Direction> {
        vec![
            Direction::Eleven,
            Direction::One,
            Direction::Five,
            Direction::Seven,
        ]
    }
}

fn part_one(file: &str) {
    let lines = file.lines();
    let grid: Vec<Vec<char>> = lines.map(|x| x.chars().collect()).collect();
    let mut count = 0;
    let y_length = grid.len();
    for y in 0..y_length {
        let x_length = grid[y].len();
        for x in 0..x_length {
            let letter = grid[y][x];
            if letter != 'X' {
                continue;
            }
            for direction in Direction::all() {
                let mut add = true;

                match direction {
                    Direction::Nine => {
                        for k in 1..4 {
                            let (next_x, next_y) = (x as i64 - k, y as i64);
                            if next_x < 0
                                || next_y < 0
                                || next_x >= x_length as i64
                                || next_y >= y_length as i64
                                || grid[next_y as usize][next_x as usize] != PATTERN[k as usize]
                            {
                                add = false;
                                break;
                            }
                        }
                        if add {
                            count += 1;
                        }
                    }
                    Direction::Eleven => {
                        for k in 1..4 {
                            let (next_x, next_y) = (x as i64 - k, y as i64 - k);
                            if next_x < 0
                                || next_y < 0
                                || next_x >= x_length as i64
                                || next_y >= y_length as i64
                                || grid[next_y as usize][next_x as usize] != PATTERN[k as usize]
                            {
                                add = false;
                                break;
                            }
                        }
                        if add {
                            count += 1;
                        }
                    }
                    Direction::One => {
                        for k in 1..4 {
                            let (next_x, next_y) = (x as i64 + k, y as i64 - k);
                            if next_x < 0
                                || next_y < 0
                                || next_x >= x_length as i64
                                || next_y >= y_length as i64
                                || grid[next_y as usize][next_x as usize] != PATTERN[k as usize]
                            {
                                add = false;
                                break;
                            }
                        }
                        if add {
                            count += 1;
                        }
                    }
                    Direction::Three => {
                        for k in 1..4 {
                            let (next_x, next_y) = (x as i64 + k, y as i64);
                            if next_x < 0
                                || next_y < 0
                                || next_x >= x_length as i64
                                || next_y >= y_length as i64
                                || grid[next_y as usize][next_x as usize] != PATTERN[k as usize]
                            {
                                add = false;
                                break;
                            }
                        }
                        if add {
                            count += 1;
                        }
                    }
                    Direction::Five => {
                        for k in 1..4 {
                            let (next_x, next_y) = (x as i64 + k, y as i64 + k);
                            if next_x < 0
                                || next_y < 0
                                || next_x >= x_length as i64
                                || next_y >= y_length as i64
                                || grid[next_y as usize][next_x as usize] != PATTERN[k as usize]
                            {
                                break;
                            }
                        }
                        if add {
                            count += 1;
                        }
                    }
                    Direction::Seven => {
                        for k in 1..4 {
                            let (next_x, next_y) = (x as i64 - k, y as i64 + k);
                            if next_x < 0
                                || next_y < 0
                                || next_x >= x_length as i64
                                || next_y >= y_length as i64
                                || grid[next_y as usize][next_x as usize] != PATTERN[k as usize]
                            {
                                add = false;
                                break;
                            }
                        }
                        if add {
                            count += 1;
                        }
                    }
                    Direction::Twelve => {
                        for k in 1..4 {
                            let (next_x, next_y) = (x as i64, y as i64 - k);
                            if next_x < 0
                                || next_y < 0
                                || next_x >= x_length as i64
                                || next_y >= y_length as i64
                                || grid[next_y as usize][next_x as usize] != PATTERN[k as usize]
                            {
                                add = false;
                                break;
                            }
                        }
                        if add {
                            count += 1
                        }
                    }
                    Direction::Six => {
                        for k in 1..4 {
                            let (next_x, next_y) = (x as i64, y as i64 + k);
                            if next_x < 0
                                || next_y < 0
                                || next_x >= x_length as i64
                                || next_y >= y_length as i64
                                || grid[next_y as usize][next_x as usize] != PATTERN[k as usize]
                            {
                                add = false;
                                break;
                            }
                        }
                        if add {
                            count += 1
                        }
                    }
                }
            }
        }
    }
    println!("{count}")
}

fn part_two(file: &str) {
    let lines = file.lines();
    let grid: Vec<Vec<char>> = lines.map(|x| x.chars().collect()).collect();
    let mut count = 0;
    let y_length = grid.len();
    for y in 0..y_length {
        let x_length = grid[y].len();
        for x in 0..x_length {
            let letter = grid[y][x];
            if letter != 'A' {
                continue;
            }
            let mut add = true;
            let mut letters: Vec<char> = Vec::new();
            for direction in Direction::subset() {
                match direction {
                    Direction::Eleven => {
                        let (next_x, next_y) = (x as i64 - 1, y as i64 - 1);
                        if next_x < 0
                            || next_y < 0
                            || next_x >= x_length as i64
                            || next_y >= y_length as i64
                            || !PATTERN_TWO.contains(&grid[next_y as usize][next_x as usize])
                        {
                            add = false;
                            break;
                        }
                        letters.push(grid[next_y as usize][next_x as usize])
                    }
                    Direction::One => {
                        let (next_x, next_y) = (x as i64 + 1, y as i64 - 1);
                        if next_x < 0
                            || next_y < 0
                            || next_x >= x_length as i64
                            || next_y >= y_length as i64
                            || !PATTERN_TWO.contains(&grid[next_y as usize][next_x as usize])
                        {
                            add = false;
                            break;
                        }
                        letters.push(grid[next_y as usize][next_x as usize]) 
                    }
                    Direction::Five => {
                        let (next_x, next_y) = (x as i64 + 1, y as i64 + 1);
                        if next_x < 0
                            || next_y < 0
                            || next_x >= x_length as i64
                            || next_y >= y_length as i64
                            || !PATTERN_TWO.contains(&grid[next_y as usize][next_x as usize])
                        {
                            add = false;
                            break;
                        }
                        if letters[0] == 'M' && grid[next_y as usize][next_x as usize] != 'S' {
                            add = false;
                            break;
                        } else if letters[0] == 'S' && grid[next_y as usize][next_x as usize] != 'M' {
                            add = false;
                            break;
                        }
                    }
                    Direction::Seven => {
                        let (next_x, next_y) = (x as i64 - 1, y as i64 + 1);
                        if next_x < 0
                            || next_y < 0
                            || next_x >= x_length as i64
                            || next_y >= y_length as i64
                            || !PATTERN_TWO.contains(&grid[next_y as usize][next_x as usize])
                        {
                            add = false;
                            break;
                        }
                        if letters[1] == 'M' && grid[next_y as usize][next_x as usize] != 'S' {
                            add = false;
                            break;
                        } else if letters[1] == 'S' && grid[next_y as usize][next_x as usize] != 'M' {
                            add = false;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            if add {
                count += 1;
            }
        }
    }
    println!("{count}");
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("File should be open");
    part_one(&contents);
    part_two(&contents);
}
