use std::cmp::max;
use std::io::{stdin, BufRead, BufReader};

#[derive(Debug)]
struct Game {
    states: Vec<GameState>,
    id: u32,
}

#[derive(Debug)]
struct GameState {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let mut lines = BufReader::new(stdin().lock()).lines();
    let mut total_power: u32 = 0;
    while let Some(Ok(line)) = lines.next() {
        let game = parse_line(&line);
        let mut min_red = 0u32;
        let mut min_green = 0u32;
        let mut min_blue = 0u32;
        for state in game.states {
            min_red = max(min_red, state.red);
            min_green = max(min_green, state.green);
            min_blue = max(min_blue, state.blue);
        }
        total_power += min_red * min_green * min_blue;
    }
    println!("Total power of sets: {total_power}");
}

fn parse_line(line: &str) -> Game {
    let mut segments = line.split(": ");
    let mut game_id_segments = segments.next().expect("Missing Game ID segment").split(" ");
    game_id_segments.next();
    let game_id: u32 = game_id_segments
        .next()
        .expect("Missing Game ID")
        .parse()
        .expect("Invalid Game ID");

    let game_state_segments = segments
        .next()
        .expect("Missing Game States segment")
        .split("; ");
    let mut game_states: Vec<GameState> = Vec::new();
    for segment in game_state_segments {
        let mut game_state = GameState {
            red: 0,
            green: 0,
            blue: 0,
        };
        let color_counts = segment.split(", ");
        for color_count in color_counts {
            let parsed_color_count: Vec<&str> = color_count.split(" ").collect();
            fn parse_count(num_string: &str) -> u32 {
                num_string.parse().expect("Invalid count")
            }
            match parsed_color_count.as_slice() {
                [num, "red"] => game_state.red += parse_count(num),
                [num, "green"] => game_state.green += parse_count(num),
                [num, "blue"] => game_state.blue += parse_count(num),
                _ => panic!("Invalid marble type"),
            }
        }
        game_states.push(game_state);
    }

    Game {
        states: game_states,
        id: game_id,
    }
}
