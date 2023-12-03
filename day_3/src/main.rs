use std::io::{stdin, BufRead, BufReader};

const GRID_SIZE: usize = 140;

#[derive(Debug)]
struct Number {
    x_start: usize,
    x_end: usize,
    y: usize,
    val: u32,
}

fn main() {
    let mut lines = BufReader::new(stdin().lock()).lines().enumerate();
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbol_zones = [[false; GRID_SIZE]; GRID_SIZE];
    while let Some((line_num, Ok(line))) = lines.next() {
        parse_line(&line, &mut numbers, &mut symbol_zones, line_num);
    }

    let mut total: u32 = 0;
    for Number {
        x_start,
        x_end,
        y,
        val,
    } in numbers
    {
        for x in x_start..x_end {
            if symbol_zones[y][x] {
                total += val;
                break;
            }
        }
    }
    println!("Total: {total}");
}

fn parse_line(
    line: &str,
    numbers: &mut Vec<Number>,
    symbol_zones: &mut [[bool; GRID_SIZE]; GRID_SIZE],
    y: usize,
) {
    let mut num_start: Option<usize> = None;
    // Iterate over line with an extra '.' to handle numbers ending at the last index
    for (x, c) in line
        .chars()
        .enumerate()
        .chain([(140usize, '.')].into_iter())
    {
        match c {
            // Match digits. Start tracking a new number if not already
            '0'..='9' => {
                if let None = num_start {
                    num_start = Some(x);
                }
            }
            _ => {
                if let Some(x_start) = num_start {
                    let val_slice = &line[x_start..x];
                    let val: u32 = val_slice.parse().expect("Invalid number");
                    let num = Number {
                        x_start,
                        x_end: x,
                        y,
                        val,
                    };
                    numbers.push(num);
                    num_start = None;
                }

                // Also add symbol zones if not a period
                if c != '.' {
                    if y > 0 {
                        symbol_zones[y - 1][x] = true;
                        if x > 0 {
                            symbol_zones[y - 1][x - 1] = true;
                        }
                        if x < line.len() - 1 {
                            symbol_zones[y - 1][x + 1] = true;
                        }
                    }
                    if y < line.len() {
                        symbol_zones[y + 1][x] = true;
                        if x > 0 {
                            symbol_zones[y + 1][x - 1] = true;
                        }
                        if x < line.len() - 1 {
                            symbol_zones[y + 1][x + 1] = true;
                        }
                    }
                    if x > 0 {
                        symbol_zones[y][x - 1] = true;
                    }
                    if x < line.len() - 1 {
                        symbol_zones[y][x + 1] = true;
                    }
                }
            }
        }
    }
}
