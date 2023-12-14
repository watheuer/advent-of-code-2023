use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Race {
    time: u64,
    distance: u64,
}

fn read_input() -> Result<Vec<Race>, Box<dyn Error>> {
    let mut lines = BufReader::new(File::open("input")?).lines();
    let times: Vec<u64> = lines
        .next()
        .expect("Missing times")?
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().expect("Bad time value"))
        .collect();
    let distances: Vec<u64> = lines
        .next()
        .expect("Missing distances")?
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().expect("Bad distances value"))
        .collect();
    let races = times
        .iter()
        .zip(distances.iter())
        .map(|pair| Race {
            time: *pair.0,
            distance: *pair.1,
        })
        .collect();
    Ok(races)
}

fn read_actual_input() -> Result<Race, Box<dyn Error>> {
    let mut lines = BufReader::new(File::open("input")?).lines();
    let time: u64 = lines
        .next()
        .expect("Missing time line")?
        .split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .join("")
        .parse()
        .expect("Invalid time as u64");
    let distance: u64 = lines
        .next()
        .expect("Missing distance line")?
        .split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .join("")
        .parse()
        .expect("Invalid distance as u64");
    Ok(Race { time, distance })
}

fn get_num_winners(race: &Race) -> u64 {
    let mut winning_ways = 0u64;
    for time_held in 1..race.time {
        let seconds_to_race = race.time - time_held;
        if seconds_to_race * time_held > race.distance {
            winning_ways += 1;
        }
    }
    winning_ways
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let races = read_input();
        let mut winning_combos: Vec<u64> = vec![];
        for race in races.expect("Unable to read races") {
            winning_combos.push(get_num_winners(&race));
        }
        let total = winning_combos
            .iter()
            .copied()
            .reduce(|acc, e| acc * e)
            .expect("");
        println!("Total: {total}");
    }

    #[test]
    fn part_two() {
        let race = read_actual_input().expect("Unable to read race");
        let winning_ways = get_num_winners(&race);
        println!("Total: {winning_ways}");
    }
}
