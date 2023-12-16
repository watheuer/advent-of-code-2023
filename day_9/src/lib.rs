use std::error::Error;
use std::io::Read;
use std::rc::Rc;

type Sequence = Vec<i64>;

fn get_delta_sequence(seq: &Sequence) -> (Sequence, bool) {
    let mut is_all_zero = true;
    let mut next_seq = Vec::with_capacity(seq.len() - 1);
    for window in seq.windows(2) {
        let diff = window[1] - window[0];
        if diff != 0 {
            is_all_zero = false;
        }
        next_seq.push(diff);
    }
    (next_seq, is_all_zero)
}

fn extrapolate_value(sequences: Vec<Rc<Sequence>>) -> Result<i64, Box<dyn Error>> {
    let mut last_delta = *sequences
        .last()
        .ok_or("Missing last value")?
        .last()
        .ok_or("Missing last value")?;
    for seq in sequences.iter().rev() {
        last_delta = seq.last().ok_or("Missing last value")? + last_delta;
    }
    Ok(last_delta)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::rc::Rc;

    #[test]
    fn part_1() -> Result<(), Box<dyn Error>> {
        let mut lines = BufReader::new(File::open("input")?).lines();
        let mut total = 0;
        while let Some(Ok(line)) = lines.next() {
            let sequence: Sequence = line
                .split_whitespace()
                .map(|s| s.parse().expect("Invalid number value"))
                .collect();

            let mut delta_sequences: Vec<Rc<Sequence>> = Vec::new();
            let mut sequence_rc = Rc::new(sequence);
            delta_sequences.push(Rc::clone(&sequence_rc));
            loop {
                let (next_sequence, end) = get_delta_sequence(sequence_rc.as_ref());
                let next_sequence_rc = Rc::new(next_sequence);
                delta_sequences.push(Rc::clone(&next_sequence_rc));
                sequence_rc = next_sequence_rc;
                if end {
                    break;
                }
            }
            total += extrapolate_value(delta_sequences)?;
        }
        println!("Total: {}", total);
        Ok(())
    }
}
