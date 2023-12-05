use std::collections::HashSet;
use std::io::{stdin, BufRead, BufReader};

struct Card {
    numbers: Vec<u32>,
    winners: HashSet<u32>
}

fn main() {
    let mut lines = BufReader::new(stdin().lock()).lines();
    let mut cards: Vec<Card> = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        cards.push(parse_line(&line));
    }

    let mut total = 0;
    for Card { winners, numbers } in cards {
        let mut points = 0;
        for number in numbers {
            if winners.contains(&number) {
                points = if points == 0 { 1 } else { points * 2 };
            }
        }
        total += points;
    }
    println!("Total points: {total}");
}

fn parse_line(line: &str) -> Card {
    let winner_strings: &Vec<&str> = &line[10..39].split_ascii_whitespace().collect();
    let number_strings: &Vec<&str> = &line[42..116].split_ascii_whitespace().collect();

    Card {
        numbers: number_strings
            .iter()
            .map(|s| s.parse().expect("Invalid number on card"))
            .collect(),
        winners: winner_strings
            .iter()
            .map(|s| s.parse().expect("Invalid number on card"))
            .collect(),
    }
}
