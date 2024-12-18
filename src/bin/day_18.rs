use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};
use std::fs;

fn main() {
    let input = fs::read_to_string("Input_Day_18").unwrap();
    let coords = processs_input(&input);

    let mut memory_space = Box::new([[CoordinateState::Available; 71]; 71]);

    // simulate first kilobyte
    coords[0..1024].iter().for_each(|(x, y)| {
        memory_space[*y][*x] = CoordinateState::Corrupted;
    });

    let (part_1_ans, mut best_path) = dijkstra(&memory_space).unwrap();
    println!("Node Count: {}", part_1_ans);

    let mut skipped_count = 0;
    let mut tried_count = 0;

    for (i, (x, y)) in coords[1024..].iter().enumerate() {
        memory_space[*y][*x] = CoordinateState::Corrupted;
        if let Err(_) = best_path.binary_search(&(*x, *y)) {
            println!(
                "[{}/{}] Skipping ({},{})...",
                i + 1024,
                coords.len(),
                *x,
                *y
            );
            skipped_count += 1;
            continue;
        }
        tried_count += 1;

        println!("[{}/{}] Trying ({},{})...", i + 1024, coords.len(), *x, *y);
        if let Some((_, new_best_path)) = dijkstra(&memory_space) {
            best_path = new_best_path;
        } else {
            println!("Killer Node: ({},{})", *x, *y);
            break;
        }
    }
    println!("Skipped: {}", skipped_count);
    println!("Tried:   {}", tried_count);
}

fn processs_input(input: &str) -> Vec<(usize, usize)> {
    let mut coords = vec![];

    input
        .lines()
        .map(|l| {
            let (x_str, y_str) = l.split_once(',').unwrap();
            (x_str.parse().unwrap(), y_str.parse().unwrap())
        })
        .for_each(|pos| coords.push(pos));

    coords
}

fn get_neighbors(pos: (usize, usize)) -> Vec<(usize, usize)> {
    let horizontal = match pos.0 {
        0 => vec![(1, pos.1)],
        1..70 => vec![(pos.0 + 1, pos.1), (pos.0 - 1, pos.1)],
        70 => vec![(69, pos.1)],
        _ => unreachable!(),
    };
    let vertical = match pos.1 {
        0 => vec![(pos.0, 1)],
        1..70 => vec![(pos.0, pos.1 + 1), (pos.0, pos.1 - 1)],
        70 => vec![(pos.0, 69)],
        _ => unreachable!(),
    };

    horizontal.into_iter().chain(vertical.into_iter()).collect()
}

fn dijkstra(memory_space: &[[CoordinateState; 71]; 71]) -> Option<(u64, Vec<(usize, usize)>)> {
    let mut priority_queue: BinaryHeap<Reverse<Node>> = BinaryHeap::new();

    for (y, row) in memory_space.iter().enumerate() {
        for (x, status) in row.iter().enumerate() {
            if let CoordinateState::Available = status {
                let node = if x == 0 && y == 0 {
                    let mut node = Node::new((x, y));
                    node.set_lowest_distance(0);
                    node
                } else {
                    Node::new((x, y))
                };

                priority_queue.push(Reverse(node));
            }
        }
    }

    let mut back_track = HashMap::new();

    while let Some(Reverse(node)) = priority_queue.pop() {
        let adjacents = get_neighbors(node.pos);
        let mut node_vec = priority_queue.into_vec();
        node_vec
            .iter_mut()
            .filter(|Reverse(n)| adjacents.contains(&n.pos))
            .for_each(|Reverse(n)| {
                if let Some(current_lowest_distance) = node.current_lowest_distance {
                    if n.set_lowest_distance(current_lowest_distance + 1) {
                        back_track.insert(n.pos, node.pos);
                    }
                } else {
                    if n.set_lowest_distance(1) {
                        back_track.insert(n.pos, node.pos);
                    }
                }
            });

        node_vec.sort();
        priority_queue = node_vec.into();
    }

    let mut node_count = 0;
    let mut current_node = (70, 70);

    let mut best_path = vec![current_node];

    while let Some(pos) = back_track.get(&current_node) {
        node_count += 1;
        current_node = *pos;
        best_path.push(current_node);
    }

    if current_node != (0, 0) {
        None
    } else {
        best_path.sort();
        Some((node_count, best_path))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum CoordinateState {
    Available,
    Corrupted,
}

struct Node {
    pos: (usize, usize),
    current_lowest_distance: Option<u64>,
}

impl Node {
    pub fn new(pos: (usize, usize)) -> Self {
        Self {
            pos,
            current_lowest_distance: None,
        }
    }

    pub fn set_lowest_distance(&mut self, distance: u64) -> bool {
        match self.current_lowest_distance {
            Some(current) => {
                if current > distance {
                    self.current_lowest_distance = Some(distance);
                    true
                } else {
                    false
                }
            }
            None => {
                self.current_lowest_distance = Some(distance);
                true
            }
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.current_lowest_distance == other.current_lowest_distance
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.current_lowest_distance, other.current_lowest_distance) {
            (Some(lhs), Some(rhs)) => lhs.cmp(&rhs),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => Ordering::Equal,
        }
    }
}
