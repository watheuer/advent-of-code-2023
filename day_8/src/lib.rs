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

        pub fn nodes(&self) -> impl Iterator<Item = &NodeId> {
            self.node_map.keys()
        }

        pub fn num_steps<F>(
            &self,
            from: NodeId,
            exit_condition: F,
            directions: &Vec<Direction>,
        ) -> usize
        where
            F: Fn(NodeId) -> bool,
        {
            let mut current_node_id = from;
            let mut num_moves = 0;
            let mut direction_iter = directions.iter().cycle();
            while !exit_condition(current_node_id) {
                let current_node = self
                    .get_node(&current_node_id)
                    .expect("Invalid current node");
                let direction = direction_iter
                    .next()
                    .expect("Ran out of directions somehow");
                match direction {
                    Direction::Left => {
                        current_node_id =
                            self.get_node(&current_node.left).expect("No left node").id
                    }
                    Direction::Right => {
                        current_node_id = self
                            .get_node(&current_node.right)
                            .expect("No right node")
                            .id
                    }
                }
                num_moves += 1;
            }
            num_moves
        }
    }

    pub enum Direction {
        Left,
        Right,
    }

    fn gcd(mut a: usize, mut b: usize) -> usize {
        if a == b {
            return a;
        }
        if b > a {
            let temp = a;
            a = b;
            b = temp;
        }
        while b > 0 {
            let temp = a;
            a = b;
            b = temp % b;
        }
        return a;
    }

    pub fn lcm(a: usize, b: usize) -> usize {
        return a * (b / gcd(a, b));
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::{lcm, Direction, Graph, Node, NodeId};
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
        let num_moves = graph.num_steps(
            ['A', 'A', 'A'],
            |node_id| node_id == ['Z', 'Z', 'Z'],
            &directions,
        );
        println!("Number of moves: {num_moves}");
    }

    #[test]
    fn part_2() {
        let file = File::open("input").expect("Expected a file called 'input'");
        let (directions, graph) = parse_graph(file);
        let mut start_node_ids: Vec<&NodeId> = graph.nodes().filter(|id| id[2] == 'A').collect();
        let min_num_steps: Vec<usize> = start_node_ids
            .iter()
            .map(|node_id| graph.num_steps(**node_id, |id| id[2] == 'Z', &directions))
            .collect();
        // Get LCM of the shortest paths for each
        let num_steps: usize = min_num_steps
            .into_iter()
            .reduce(|acc, e| lcm(acc, e))
            .expect("Unable to find LCM");
        println!("Number of moves: {num_steps}");
    }
}
