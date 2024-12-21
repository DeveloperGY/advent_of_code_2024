use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;

fn main() {
    let input = fs::read_to_string("Input_Day_20").unwrap();
    let mut maze = Maze::new(&input);

    let cheat_score_part_1 = maze.cheat(2, 100).len();
    println!("Valid 2 picosecond cheats: {}", cheat_score_part_1);
    let cheat_score_part_2 = maze.cheat(20, 100).len();
    println!("Valid 20 picosecond cheats: {}", cheat_score_part_2);
}

struct Maze {
    grid: Box<[[char; 141]]>,
    start: (usize, usize),
    end: (usize, usize),
    width: usize,
    height: usize,
}

impl Maze {
    pub fn new(input: &str) -> Self {
        let mut rows = vec![];
        let mut start = (0, 0);
        let mut end = (0, 0);

        for (y, line) in input.lines().enumerate() {
            let mut row = ['?'; 141];
            for (x, c) in line.chars().enumerate() {
                if c == 'S' {
                    start = (x, y);
                    row[x] = '.';
                } else if c == 'E' {
                    end = (x, y);
                    row[x] = '.';
                } else {
                    row[x] = c;
                }
            }
            rows.push(row);
        }

        let grid = rows.into_boxed_slice();
        let width = grid[0].len();
        let height = grid.len();

        Self {
            grid,
            start,
            end,
            width,
            height,
        }
    }

    fn dijkstra(&self) -> (HashMap<(usize, usize), u64>, Vec<(usize, usize)>) {
        let mut distances = HashMap::new();
        let mut backtrace = HashMap::new();
        let mut priority_queue = BinaryHeap::new();

        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[y][x] == '.' {
                    let distance = if (x, y) == self.end { 1 } else { u64::MAX };
                    priority_queue.push(Reverse((distance, (x, y))));
                }
            }
        }

        while let Some(Reverse((distance, (x, y)))) = priority_queue.pop() {
            let mut priority_vec = priority_queue.into_sorted_vec();
            distances.insert((x, y), distance);

            let neighbour_cells: Vec<_> = [
                ((x + 1).clamp(0, self.width - 1), y),
                (x.saturating_sub(1).clamp(0, self.width - 1), y),
                (x, (y + 1).clamp(0, self.height - 1)),
                (x, y.saturating_sub(1).clamp(0, self.height - 1)),
            ]
            .into_iter()
            .filter(|pos| !distances.contains_key(pos))
            .collect();

            priority_vec
                .iter_mut()
                .filter(|Reverse((dist, pos))| {
                    neighbour_cells.contains(pos) && *dist > distance + 1
                })
                .for_each(|Reverse((dist, pos))| {
                    backtrace.insert(*pos, (x, y));
                    *dist = distance + 1;
                });

            priority_queue = BinaryHeap::from(priority_vec);
        }

        let mut path = vec![];

        let mut current_pos = self.start;
        path.push(current_pos);
        while let Some(pos) = backtrace.get(&current_pos) {
            path.push(*pos);
            current_pos = *pos;
        }

        (distances, path)
    }

    fn cheat(&mut self, time: usize, threshold: u64) -> HashSet<((usize, usize), (usize, usize))> {
        // Returns a set of all valid cheat positions, indicated by start and end position
        let (distances, mut path) = self.dijkstra();

        // Need to pop from start to end
        path.reverse();

        let mut cheats = HashSet::new();

        let mut current_distance = 0;
        let normal_best = distances[&self.start];
        while let Some(pos) = path.pop() {
            current_distance += 1;
            path.iter()
                .filter(|cell| get_distance(pos, **cell) <= time as u64)
                .filter(|cell| {
                    current_distance + distances[cell] + get_distance(pos, **cell) - 1
                        <= normal_best - threshold
                })
                .for_each(|end| {
                    cheats.insert((pos, *end));
                });
        }

        cheats
    }
}

fn get_distance(start: (usize, usize), end: (usize, usize)) -> u64 {
    let width = start.0.abs_diff(end.0);
    let height = start.1.abs_diff(end.1);
    (width + height) as u64
}
