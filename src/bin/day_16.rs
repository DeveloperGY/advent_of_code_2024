use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::fs;

fn main() {
    let input = fs::read_to_string("Input_Day_16").unwrap();
    let mut maze = Maze::new(&input);

    maze.fill_deadends();

    // let lowest_score = maze.get_lowest_score((135, 1), Direction::East);
    let (lowest_score, best_cells) = maze.get_lowest_maze_score();
    println!("{}\n", Maze::new(&input));
    println!("{}\n", maze);
    println!("Lowest Score: {}", lowest_score);
    println!("Best Cell Count: {}", best_cells);
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn score_mod(&self, dir: Self) -> u64 {
        match (*self, dir) {
            (Self::North, Self::North)
            | (Self::South, Self::South)
            | (Self::East, Self::East)
            | (Self::West, Self::West) => 1,
            (Self::North, Self::South)
            | (Self::South, Self::North)
            | (Self::East, Self::West)
            | (Self::West, Self::East) => 2001,
            _ => 1001,
        }
    }
}

struct Reindeer {
    visited: HashSet<(usize, usize)>,
    current_score: u64,
    current_direction: Direction,
    pos: (usize, usize),
}

impl Reindeer {
    pub fn new(pos: (usize, usize), direction: Direction) -> Self {
        Self {
            visited: HashSet::new(),
            current_score: 0,
            current_direction: direction,
            pos,
        }
    }

    pub fn from_prev(prev: &Reindeer, direction: Direction) -> Self {
        let mut visited = prev.visited.clone();
        visited.insert(prev.pos);
        let score_mod = prev.current_direction.score_mod(direction);
        let score = prev.current_score + score_mod;

        let pos = match (direction, prev.pos) {
            (Direction::North, (x, y)) => (x, y - 1),
            (Direction::South, (x, y)) => (x, y + 1),
            (Direction::East, (x, y)) => (x + 1, y),
            (Direction::West, (x, y)) => (x - 1, y),
        };

        Self {
            visited,
            current_score: score,
            current_direction: direction,
            pos,
        }
    }
}

struct Maze {
    grid: Box<[Box<[char]>]>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Maze {
    pub fn new(input: &str) -> Self {
        let mut rows = vec![];
        let mut start = (0, 0);
        let mut end = (0, 0);

        for (y, line) in input.lines().enumerate() {
            let mut row = vec![];
            for (x, c) in line.chars().enumerate() {
                match c {
                    'S' => {
                        start = (x, y);
                        row.push('S');
                    }
                    'E' => {
                        end = (x, y);
                        row.push('E');
                    }
                    c => row.push(c),
                };
            }
            rows.push(row.into_boxed_slice())
        }

        Self {
            grid: rows.into_boxed_slice(),
            start,
            end,
        }
    }

    pub fn fill_deadends(&mut self) {
        fn inner(maze: &mut Maze) -> u64 {
            let mut deadend_list = vec![];
            for y in 1..maze.grid.len() - 1 {
                for x in 1..maze.grid[0].len() - 1 {
                    if maze.grid[y][x] == '.' {
                        let wall_count = if maze.grid[y][x + 1] == '#' { 1 } else { 0 }
                            + if maze.grid[y][x - 1] == '#' { 1 } else { 0 }
                            + if maze.grid[y + 1][x] == '#' { 1 } else { 0 }
                            + if maze.grid[y - 1][x] == '#' { 1 } else { 0 };

                        if wall_count > 2 {
                            deadend_list.push((x, y));
                        }
                    }
                }
            }

            for (x, y) in &deadend_list {
                maze.grid[*y][*x] = '#';
            }

            deadend_list.len() as u64
        }

        while inner(self) != 0 {}
    }

    fn get_lowest_score(
        &self,
        mut reindeer: Reindeer,
        cache: &mut HashMap<((usize, usize), Direction), u64>,
    ) -> Vec<Reindeer> {
        if reindeer.visited.contains(&reindeer.pos) {
            return vec![];
        }

        if let Some(score) = cache.get(&(reindeer.pos, reindeer.current_direction)) {
            if *score < reindeer.current_score {
                return vec![];
            }
        }

        cache.insert(
            (reindeer.pos, reindeer.current_direction),
            reindeer.current_score,
        );

        let cell = self.grid[reindeer.pos.1][reindeer.pos.0];
        match cell {
            '#' => vec![],
            'E' => {
                println!("Reindeer Finished With {}", reindeer.current_score + 1);
                vec![reindeer]
            }
            '.' | 'S' => {
                let north = Reindeer::from_prev(&reindeer, Direction::North);
                let south = Reindeer::from_prev(&reindeer, Direction::South);
                let east = Reindeer::from_prev(&reindeer, Direction::East);
                let west = Reindeer::from_prev(&reindeer, Direction::West);

                let reindeer_vec = vec![north, south, east, west];
                reindeer_vec
                    .into_iter()
                    .flat_map(|r| self.get_lowest_score(r, cache))
                    .collect()
            }
            _ => unreachable!(),
        }
    }

    pub fn get_lowest_maze_score(&mut self) -> (u64, usize) {
        let mut score_cache = HashMap::new();

        let reindeer = Reindeer::new(self.start, Direction::East);
        let finished_reindeer = self.get_lowest_score(reindeer, &mut score_cache);
        let r = finished_reindeer
            .iter()
            .min_by(|lhs, rhs| lhs.current_score.cmp(&rhs.current_score))
            .unwrap();

        let lowest_score = r.current_score + 1; // Last cell wasnt counted by asexually reproducing reindeer
        let set = finished_reindeer
            .iter()
            .filter(|r| r.current_score == lowest_score - 1)
            .map(|r| r.visited.clone())
            .reduce(|a, b| a.union(&b).map(|e| *e).collect::<HashSet<_>>())
            .unwrap();
        (lowest_score, set.len() + 1)
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let out = self
            .grid
            .iter()
            .map(|row| {
                row.iter()
                    .map(|c| if *c == '.' { ' ' } else { *c })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", out)
    }
}
