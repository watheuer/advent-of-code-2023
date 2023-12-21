use std::collections::HashSet;

const GRID_SIZE: usize = 139;

#[derive(Eq, PartialEq)]
enum Tile {
    Vertical,
    Horizontal,
    NorthAndEast,
    NorthAndWest,
    SouthAndWest,
    SouthAndEast,
    Ground,
    AnimalStart,
}

#[derive(Hash, Eq, PartialOrd, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn directions(tile: &Tile) -> Vec<Direction> {
    match tile {
        Tile::Vertical => vec![Direction::North, Direction::South],
        Tile::Horizontal => vec![Direction::East, Direction::West],
        Tile::NorthAndEast => vec![Direction::North, Direction::East],
        Tile::NorthAndWest => vec![Direction::North, Direction::West],
        Tile::SouthAndWest => vec![Direction::South, Direction::West],
        Tile::SouthAndEast => vec![Direction::South, Direction::East],
        Tile::AnimalStart => vec![
            Direction::South,
            Direction::East,
            Direction::North,
            Direction::West,
        ],
        Tile::Ground => vec![],
    }
}

fn opposite(direction: &Direction) -> Direction {
    match direction {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::East => Direction::West,
        Direction::West => Direction::East,
    }
}

fn parse_tile(c: char) -> Option<Tile> {
    match c {
        '|' => Some(Tile::Vertical),
        '-' => Some(Tile::Horizontal),
        'L' => Some(Tile::NorthAndEast),
        'J' => Some(Tile::NorthAndWest),
        '7' => Some(Tile::SouthAndWest),
        'F' => Some(Tile::SouthAndEast),
        '.' => Some(Tile::Ground),
        'S' => Some(Tile::AnimalStart),
        _ => None,
    }
}

#[derive(Debug)]
struct Pair(usize, usize);
struct Node {
    visited: bool,
    tile: Tile,
}
struct Graph {
    nodes: Vec<Vec<Node>>,
}

impl Graph {
    fn new(tiles: Vec<Vec<Tile>>) -> Self {
        Self {
            nodes: tiles
                .into_iter()
                .map(|row| {
                    row.into_iter()
                        .map(|tile| Node {
                            visited: false,
                            tile,
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn get_node(&self, pair: &Pair) -> &Node {
        let Pair(x, y) = pair;
        self.nodes.get(*y).unwrap().get(*x).unwrap()
    }

    fn visit(&mut self, pair: &Pair) {
        let Pair(x, y) = pair;
        let node = self.nodes.get_mut(*y).unwrap().get_mut(*x).unwrap();
        node.visited = true;
    }

    fn get_neighbors(&self, pair: &Pair) -> Vec<Pair> {
        let Pair(x, y) = pair;
        let node = self.get_node(pair);
        let direction_options = directions(&node.tile);

        direction_options
            .into_iter()
            .filter(|direction| -> bool {
                match direction {
                    Direction::North => *y > 0,
                    Direction::South => *y < GRID_SIZE,
                    Direction::East => *x < GRID_SIZE,
                    Direction::West => *x > 0,
                }
            })
            .map(|direction| -> (Pair, Direction) {
                match direction {
                    Direction::North => (Pair(*x, y - 1), Direction::North),
                    Direction::South => (Pair(*x, y + 1), Direction::South),
                    Direction::East => (Pair(x + 1, *y), Direction::East),
                    Direction::West => (Pair(x - 1, *y), Direction::West),
                }
            })
            .filter_map(|(pair, direction)| -> Option<Pair> {
                let pair_node = self.get_node(&pair);
                let pair_directions = directions(&pair_node.tile);
                let pair_opposites: HashSet<Direction> = pair_directions
                    .into_iter()
                    .map(|dir| opposite(&dir))
                    .collect();

                match pair_opposites.contains(&direction) {
                    true => Some(pair),
                    false => None,
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_tile, Graph, Pair, Tile};
    use std::collections::VecDeque;
    use std::error::Error;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[test]
    fn part_1() -> Result<(), Box<dyn Error>> {
        let mut starting_pair: Option<Pair> = None;
        let tiles: Vec<Vec<Tile>> = BufReader::new(File::open("input")?)
            .lines()
            .enumerate()
            .map(|(i, line_res)| -> Vec<Tile> {
                let line = line_res.expect("Unable to read line");
                line.chars()
                    .enumerate()
                    .map(|(j, c)| {
                        let tile = parse_tile(c).expect("Invalid char");
                        if tile == Tile::AnimalStart {
                            starting_pair = Some(Pair(j, i));
                        }
                        tile
                    })
                    .collect()
            })
            .collect();

        // BFS until we find the max
        let mut graph = Graph::new(tiles);
        let mut deq: VecDeque<Pair> = VecDeque::new();
        deq.push_back(starting_pair.expect("Unable to find animal starting point"));
        let mut distance = 0usize;

        while !deq.is_empty() {
            let mut next_deq: VecDeque<Pair> = VecDeque::new();
            for pair in deq {
                // visit node
                let node = graph.get_node(&pair);
                graph.visit(&pair);
                let neighbors = graph.get_neighbors(&pair);
                for neighbor in neighbors {
                    let neighbor_node = graph.get_node(&neighbor);
                    if !neighbor_node.visited {
                        next_deq.push_back(neighbor);
                    }
                }
            }

            distance += 1;
            deq = next_deq;
        }

        println!("max distance: {}", distance - 1);

        Ok(())
    }
}
