use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Race {
    time: u32,
    distance: u32,
}

fn read_input() -> Result<Vec<Race>, Box<dyn Error>> {
    let mut lines = BufReader::new(File::open("input")?).lines();
    let times: Vec<u32> = lines
        .next()
        .expect("Missing times")?
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().expect("Bad time value"))
        .collect();
    let distances: Vec<u32> = lines
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let races = read_input();
        let mut winning_combos: Vec<u32> = vec![];
        for race in races.expect("Unable to read races") {
            let mut winning_ways = 0u32;
            for time_held in 1..race.time {
                let seconds_to_race = race.time - time_held;
                if seconds_to_race * time_held > race.distance {
                    winning_ways += 1;
                }
            }
            winning_combos.push(winning_ways);
        }
        let total = winning_combos
            .iter()
            .copied()
            .reduce(|acc, e| acc * e)
            .expect("");
        println!("Total: {total}");
    }
}
