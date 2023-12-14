mod hands {
    use std::cmp::Ordering;
    use std::collections::HashMap;

    #[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
    enum HandType {
        FiveOfAKind = 6,
        FourOfAKind = 5,
        FullHouse = 4,
        ThreeOfAKind = 3,
        TwoPair = 2,
        OnePair = 1,
        HighCard = 0,
    }

    #[derive(Debug)]
    pub struct Hand {
        cards: [u8; 5],
        hand_type: HandType,
        pub bid: usize,
    }

    impl Hand {
        pub fn from_str(hand_val: &str, bid: usize) -> Self {
            let card_strengths: [u8; 5] = hand_val
                .chars()
                .map(Hand::char_to_strength)
                .collect::<Vec<u8>>()
                .try_into()
                .expect("Invalid hand");
            Hand {
                cards: card_strengths,
                hand_type: Hand::get_hand_type(card_strengths),
                bid,
            }
        }

        fn get_hand_type(hand: [u8; 5]) -> HandType {
            let mut map: HashMap<u8, u8> = HashMap::new();
            for card in hand {
                match map.get(&card) {
                    Some(count) => map.insert(card, count + 1),
                    None => map.insert(card, 1),
                };
            }

            match map.len() {
                // 5 of a kind
                1 => HandType::FiveOfAKind,
                // 4 of a kind or full house
                2 => {
                    for val in map.values() {
                        if *val == 4u8 {
                            return HandType::FourOfAKind;
                        }
                    }
                    HandType::FullHouse
                }
                // 3 of a kind or two pair
                3 => {
                    for val in map.values() {
                        if *val == 3u8 {
                            return HandType::ThreeOfAKind;
                        }
                    }
                    HandType::TwoPair
                }
                // One pair
                4 => HandType::OnePair,
                // High card
                5 => HandType::HighCard,
                _ => panic!("Invalid hand type"),
            }
        }

        fn char_to_strength(c: char) -> u8 {
            match c {
                '2' => 0,
                '3' => 1,
                '4' => 2,
                '5' => 3,
                '6' => 4,
                '7' => 5,
                '8' => 6,
                '9' => 7,
                'T' => 8,
                'J' => 9,
                'Q' => 10,
                'K' => 11,
                'A' => 12,
                _ => panic!("Invalid card"),
            }
        }
    }

    impl Eq for Hand {}
    impl PartialEq<Self> for Hand {
        fn eq(&self, other: &Self) -> bool {
            self.cards == other.cards && self.hand_type == other.hand_type
        }
    }
    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> Ordering {
            match self.hand_type.cmp(&other.hand_type) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => {
                    // If equal hand types, iterate over values
                    for i in 0..self.cards.len() {
                        match self.cards[i].cmp(&other.cards[i]) {
                            Ordering::Less => return Ordering::Less,
                            Ordering::Greater => return Ordering::Greater,
                            Ordering::Equal => {}
                        }
                    }
                    Ordering::Equal
                }
            }
        }
    }
    impl PartialOrd<Self> for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::hands::Hand;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[test]
    fn part_1() {
        let mut lines = BufReader::new(File::open("input").expect("Missing input")).lines();
        let mut hands: Vec<Hand> = Vec::new();
        while let Some(Ok(line)) = lines.next() {
            let mut segments = line.split_whitespace();
            let hand_val = segments.next().expect("Missing hand value");
            let bid: usize = segments
                .next()
                .expect("Missing bid")
                .parse()
                .expect("Invalid bid");
            let hand = Hand::from_str(&hand_val, bid);
            hands.push(hand);
        }
        hands.sort();
        let mut total_winnings = 0;
        for (i, hand) in hands.iter().enumerate() {
            let rank = i + 1;
            total_winnings += rank * hand.bid;
        }
        println!("Total winnings: {total_winnings}");
    }
}
