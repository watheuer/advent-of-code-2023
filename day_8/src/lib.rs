mod graph {
    use std::collections::HashMap;

    pub type NodeId = [char; 3];

    #[derive(Debug)]
    pub struct Node {
        pub id: NodeId,
        pub left: NodeId,
        pub right: NodeId,
    }

    pub struct Graph {
        node_map: HashMap<NodeId, Node>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                node_map: HashMap::new(),
            }
        }

        pub fn add_node(&mut self, node: Node) {
            self.node_map.insert(node.id, node);
        }

        pub fn get_node(&self, id: &NodeId) -> Option<&Node> {
            self.node_map.get(id)
        }
    }

    pub enum Direction {
        Left,
        Right,
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::{Direction, Graph, Node};
    use std::fs::File;
    use std::io::{BufRead, BufReader, Read};

    pub fn parse_graph(input: impl Read) -> (Vec<Direction>, Graph) {
        let mut lines = BufReader::new(input).lines();
        let direction_line = lines
            .next()
            .expect("Unexpected EOF")
            .expect("No directions line");
        let directions: Vec<Direction> = direction_line
            .chars()
            .map(|c| match c {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("Unknown direction"),
            })
            .collect();
        lines.next();

        let mut graph = Graph::new();
        while let Some(Ok(line)) = lines.next() {
            let chars: Vec<char> = line.chars().collect();
            let parent_id = &chars[0..3];
            let left_id = &chars[7..10];
            let right_id = &chars[12..15];
            let node = Node {
                id: [parent_id[0], parent_id[1], parent_id[2]],
                left: [left_id[0], left_id[1], left_id[2]],
                right: [right_id[0], right_id[1], right_id[2]],
            };
            graph.add_node(node);
        }
        (directions, graph)
    }

    #[test]
    fn part_1() {
        let file = File::open("input").expect("Expected a file called 'input'");
        let (directions, graph) = parse_graph(file);
        let mut current_node_id = ['A', 'A', 'A'];
        let mut num_moves = 0;
        while current_node_id != ['Z', 'Z', 'Z'] {
            let iter = directions.iter();
            for direction in iter {
                let current_node = graph.get_node(&current_node_id).expect("Invalid current node");
                match direction {
                    Direction::Left => {
                        current_node_id = graph.get_node(&current_node.left).expect("No left node").id
                    }
                    Direction::Right => {
                        current_node_id = graph.get_node(&current_node.right).expect("No right node").id
                    }
                }
                num_moves += 1;
            }
        }
        println!("Number of moves: {num_moves}");
    }
}
