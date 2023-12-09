use std::cmp::min;
use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut lines = BufReader::new(stdin().lock()).lines();
    let seed_string: String = lines
        .next()
        .expect("Missing seed line")
        .expect("Unable to read seeds");
    let seed_strings: Vec<&str> = seed_string.split(" ").collect();
    let mut seeds: Vec<(u64, bool)> = (&seed_strings[1..])
        .iter()
        .map(|s| (s.parse().expect("Invalid seed"), false))
        .collect();
    lines.next(); // Advance past empty line

    // Traverse each layer until we make it to location
    while let Some(Ok(line)) = lines.next() {
        let segments: Vec<&str> = line.split(" ").collect();
        match segments.as_slice() {
            [dest_start_string, source_start_string, length_string] => {
                let dest_start: u64 = dest_start_string.parse().expect("Invalid dest start");
                let source_start: u64 = source_start_string.parse().expect("Invalid source start");
                let length: u64 = length_string.parse().expect("Invalid length");
                seeds = seeds
                    .iter()
                    .map(|(num, seen)| -> (u64, bool) {
                        return if !seen && *num >= source_start && *num < source_start + length {
                            let diff = num - source_start;
                            (dest_start + diff, true)
                        } else {
                            (*num, *seen)
                        }
                    })
                    .collect();
            }
            [_, "map:"] => {}
            [""] => {
                // Reset for next map
                seeds = seeds.iter().map(|seed| (seed.0, false)).collect();
            }
            _ => panic!("Bad line :("),
        }
    }

    let min_location = seeds.iter().min_by(|x, y| x.0.cmp(&y.0));
    println!("Min location: {}", min_location.expect("No min location!").0);
}
