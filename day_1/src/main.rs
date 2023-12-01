use std::io::{stdin, BufRead, BufReader};
use crate::char_buf::CharBuf;

fn main() {
    let lines = BufReader::new(stdin().lock()).lines();
    let mut total: u32 = 0;
    for line_res in lines {
        if let Ok(line) = line_res {
            let first_digit = get_first_digit(line.chars(), false);
            let second_digit = get_first_digit(line.chars().rev(), true);
            let number_string = format!("{}{}", first_digit, second_digit);
            let number: u32 = number_string.parse().unwrap();
            total += number;
        }
    }
    println!("Total: {total}");
}

fn get_first_digit(chars: impl Iterator<Item = char>, rev: bool) -> u32 {
    let mut char_buf = CharBuf::new(rev);
    for char in chars {
        if let Some(digit) = char_buf.add_char(char) {
            return digit;
        }
    }
    panic!("No initial digit found");
}

mod char_buf {
    const RADIX: u32 = 10;
    const CHAR_BUF_LENGTH: usize = 5;

    pub struct CharBuf {
        arr: [char; CHAR_BUF_LENGTH],
        next_index: usize,
        rev: bool
    }

    impl CharBuf {
        pub fn new(rev: bool) -> Self {
            CharBuf {
                arr: [' '; CHAR_BUF_LENGTH],
                next_index: 0,
                rev
            }
        }

        fn get_index(i: usize) -> usize {
            i % CHAR_BUF_LENGTH
        }

        pub fn add_char(&mut self, c: char) -> Option<u32> {
            if let Some(digit) = c.to_digit(RADIX) {
                return Some(digit);
            }

            self.arr[self.next_index] = c;
            self.next_index = CharBuf::get_index(self.next_index + 1);

            let arranged_arr: [char; CHAR_BUF_LENGTH] = [
                self.arr[CharBuf::get_index(self.next_index)],
                self.arr[CharBuf::get_index(self.next_index+1)],
                self.arr[CharBuf::get_index(self.next_index+2)],
                self.arr[CharBuf::get_index(self.next_index+3)],
                self.arr[CharBuf::get_index(self.next_index+4)],
            ];

            if self.rev {
                match arranged_arr {
                    [_, _, 'e', 'n', 'o'] => return Some(1u32),
                    [_, _, 'o', 'w', 't'] => return Some(2u32),
                    ['e', 'e', 'r', 'h', 't'] => return Some(3u32),
                    [_, 'r', 'u', 'o', 'f'] => return Some(4u32),
                    [_, 'e', 'v', 'i', 'f'] => return Some(5u32),
                    [_, _, 'x', 'i', 's'] => return Some(6u32),
                    ['n', 'e', 'v', 'e', 's'] => return Some(7u32),
                    ['t', 'h', 'g', 'i', 'e'] => return Some(8u32),
                    [_, 'e', 'n', 'i', 'n'] => return Some(9u32),
                    _ => ()
                }
            } else {
                match arranged_arr {
                    [_, _, 'o', 'n', 'e'] => return Some(1u32),
                    [_, _, 't', 'w', 'o'] => return Some(2u32),
                    ['t', 'h', 'r', 'e', 'e'] => return Some(3u32),
                    [_, 'f', 'o', 'u', 'r'] => return Some(4u32),
                    [_, 'f', 'i', 'v', 'e'] => return Some(5u32),
                    [_, _, 's', 'i', 'x'] => return Some(6u32),
                    ['s', 'e', 'v', 'e', 'n'] => return Some(7u32),
                    ['e', 'i', 'g', 'h', 't'] => return Some(8u32),
                    [_, 'n', 'i', 'n', 'e'] => return Some(9u32),
                    _ => ()
                }
            }

            // Return None if no matches
            None
        }
    }
}
