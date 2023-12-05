use std::array;
use std::collections::HashSet;
use std::hash::{Hash};
use std::io::{stdin, BufRead, BufReader};
use std::rc::Rc;

const GRID_SIZE: usize = 140;

#[derive(Hash, Eq, PartialEq)]
struct Number {
    x_start: usize,
    x_end: usize,
    y: usize,
    val: u32,
}

struct Gear {
    neighbor_indices: Vec<(usize, usize)>,
}

impl Gear {
    fn new(y: usize, x: usize) -> Self {
        let neighbors: [(i32, i32); 8] = [
            (y as i32, x as i32 + 1),
            (y as i32, x as i32 - 1),
            (y as i32 - 1, x as i32 + 1),
            (y as i32 - 1, x as i32),
            (y as i32 - 1, x as i32 - 1),
            (y as i32 + 1, x as i32 + 1),
            (y as i32 + 1, x as i32),
            (y as i32 + 1, x as i32 - 1),
        ];
        Self {
            neighbor_indices: neighbors
                .iter()
                .filter(|(y, x)| *y >= 0 && *y < 140 && *x >= 0 && *x < 140)
                .map(|(y, x)| -> (usize, usize) { (*y as usize, *x as usize) })
                .collect(),
        }
    }
}

fn main() {
    let mut lines = BufReader::new(stdin().lock()).lines().enumerate();
    let mut numbers: [[Option<Rc<Number>>; GRID_SIZE]; GRID_SIZE] =
        array::from_fn(|_| -> [Option<Rc<Number>>; GRID_SIZE] {
            array::from_fn(|_| -> Option<Rc<Number>> { None })
        });
    let mut gears: Vec<Gear> = Vec::new();
    while let Some((line_num, Ok(line))) = lines.next() {
        parse_line(&line, &mut numbers, &mut gears, line_num);
    }

    let mut total: u32 = 0;
    for gear in gears {
        let mut set = HashSet::new();
        let mut gear_ratio = 1u32;
        for (y, x) in gear.neighbor_indices {
            if let Some(num) = &numbers[y][x] {
                set.insert(num);
            }
        }
        if set.len() == 2 {
            for num in set {
                gear_ratio *= num.val;
            }
            total += gear_ratio;
        }
    }
    println!("Total: {total}");
}

fn parse_line(
    line: &str,
    numbers: &mut [[Option<Rc<Number>>; GRID_SIZE]; GRID_SIZE],
    gears: &mut Vec<Gear>,
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
                    let num = Rc::new(Number {
                        x_start,
                        x_end: x,
                        y,
                        val,
                    });

                    for x_val in x_start..x {
                        numbers[y][x_val] = Some(Rc::clone(&num));
                    }

                    num_start = None;
                }

                if c == '*' {
                    gears.push(Gear::new(y, x));
                }
            }
        }
    }
}
