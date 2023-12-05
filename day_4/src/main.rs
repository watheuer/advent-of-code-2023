use std::collections::HashSet;
use std::io::{BufRead, BufReader, stdin};

struct Card {
    numbers: Vec<u32>,
    winners: HashSet<u32>,
    index: usize,
}

fn main() {
    let mut lines = BufReader::new(stdin().lock()).lines();
    let mut cards: Vec<Card> = Vec::new();
    let mut index = 0usize;
    while let Some(Ok(line)) = lines.next() {
        cards.push(parse_line(index, &line));
        index += 1;
    }

    // num of duplicates of each card
    let mut duplicates = [1u32; 219];

    let mut total = 0;
    for Card {
        numbers,
        winners,
        index,
    } in cards
    {
        let mut next_index = 1;
        for number in numbers {
            if winners.contains(&number) {
                duplicates[index + next_index] += duplicates[index];
                next_index += 1;
            }
        }
        total += duplicates[index];
    }
    println!("Total cards: {total}");
}

fn parse_line(i: usize, line: &str) -> Card {
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
        index: i,
    }
}
