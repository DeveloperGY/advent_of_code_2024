use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input = fs::read_to_string("Input_Day_10").unwrap();
    let board = Board::new(&input);
    println!("Score: {}", board.total_score());
    println!("Rating: {}", board.total_rating());
}

#[derive(Debug, Copy, Clone)]
struct Cell {
    pub level: u8,
}

impl Cell {
    pub fn new(level: u8) -> Self {
        Self { level }
    }
}

struct Board {
    cells: Vec<Vec<Cell>>,
    starts: Vec<(u8, u8)>,
}

impl Board {
    pub fn new(input: &str) -> Self {
        let mut rows = vec![];
        let mut starts = vec![];

        for line in input.lines() {
            let cells: Vec<_> = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .map(Cell::new)
                .collect();

            rows.push(cells);
        }

        for y in 0..rows.len() {
            for x in 0..rows[y].len() {
                if let Cell { level: 0 } = rows[y][x] {
                    starts.push((x as u8, y as u8));
                }
            }
        }

        Self {
            cells: rows,
            starts,
        }
    }

    fn score(&self, x: u8, y: u8) -> u64 {
        let mut visited = HashSet::new();
        let mut to_visit = vec![(x, y)];
        let mut count = 0;

        // helper values
        let width = self.cells[y as usize].len() as u8;
        let height = self.cells.len() as u8;

        while let Some((x, y)) = to_visit.pop() {
            let lvl = self.cells[y as usize][x as usize].level;
            if lvl == 9 {
                count += 1;
                visited.insert((x, y));
                continue;
            }

            let mut adjacent = vec![];

            match x {
                0 => {
                    adjacent.push((x + 1, y));
                }
                x if x == width - 1 => {
                    adjacent.push((x - 1, y));
                }
                x => {
                    adjacent.push((x + 1, y));
                    adjacent.push((x - 1, y));
                }
            };

            match y {
                0 => {
                    adjacent.push((x, y + 1));
                }
                y if y == height - 1 => {
                    adjacent.push((x, y - 1));
                }
                y => {
                    adjacent.push((x, y + 1));
                    adjacent.push((x, y - 1));
                }
            };

            let valid_adjacent = adjacent.into_iter().filter(|(x, y)| {
                self.cells[*y as usize][*x as usize].level == lvl + 1
                    && !visited.contains(&(*x, *y))
            });

            valid_adjacent.for_each(|c| to_visit.push(c));
            visited.insert((x, y));
        }

        count
    }

    pub fn total_score(&self) -> u64 {
        self.starts.iter().map(|(x, y)| self.score(*x, *y)).sum()
    }

    fn rating(&self, x: u8, y: u8) -> u64 {
        let mut to_visit = vec![(x, y)];
        let mut level_nines = HashMap::new();

        // helper values
        let width = self.cells[y as usize].len() as u8;
        let height = self.cells.len() as u8;

        while let Some((x, y)) = to_visit.pop() {
            let lvl = self.cells[y as usize][x as usize].level;
            if lvl == 9 {
                match level_nines.get_mut(&(x, y)) {
                    Some(rating) => {
                        *rating += 1;
                    }
                    None => {
                        level_nines.insert((x, y), 1);
                    }
                };
                continue;
            }

            let mut adjacent = vec![];

            match x {
                0 => {
                    adjacent.push((x + 1, y));
                }
                x if x == width - 1 => {
                    adjacent.push((x - 1, y));
                }
                x => {
                    adjacent.push((x + 1, y));
                    adjacent.push((x - 1, y));
                }
            };

            match y {
                0 => {
                    adjacent.push((x, y + 1));
                }
                y if y == height - 1 => {
                    adjacent.push((x, y - 1));
                }
                y => {
                    adjacent.push((x, y + 1));
                    adjacent.push((x, y - 1));
                }
            };

            let valid_adjacent = adjacent
                .into_iter()
                .filter(|(x, y)| self.cells[*y as usize][*x as usize].level == lvl + 1);

            valid_adjacent.for_each(|c| to_visit.push(c));
        }

        level_nines.values().sum()
    }
    pub fn total_rating(&self) -> u64 {
        self.starts.iter().map(|(x, y)| self.rating(*x, *y)).sum()
    }
}
