use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("Input_Day_6").unwrap();
    let sim = GuardSim::new(&input);

    if let Some(position_set) = sim.simulate() {
        println!("Pos Count: {}", position_set.len());

        let loop_count = sim.simulate_with_walls(position_set);
        println!("Loop Count: {}", loop_count);
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match *self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Guard {
    x: i32,
    y: i32,
    dir: Direction,
}

impl Guard {
    pub fn new(x: i32, y: i32, dir: Direction) -> Self {
        Self { x, y, dir }
    }

    pub fn move_forward(&mut self) {
        match self.dir {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        };
    }

    pub fn turn(&mut self) {
        self.dir = self.dir.turn_right();
    }

    /// Returns true if the guard is out of bounds
    pub fn is_oob(&self, width: i32, height: i32) -> bool {
        let is_x_oob = (..0).contains(&self.x) || (width..).contains(&self.x);
        let is_y_oob = (..0).contains(&self.y) || (height..).contains(&self.y);

        is_x_oob || is_y_oob
    }

    pub fn get_index(&self, width: i32) -> usize {
        (self.y * width + self.x) as usize
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct PositionGuard {
    x: i32,
    y: i32,
}

impl PositionGuard {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn get_index(&self, width: i32) -> usize {
        (self.y * width + self.x) as usize
    }
}

impl From<Guard> for PositionGuard {
    fn from(value: Guard) -> Self {
        Self::new(value.x, value.y)
    }
}

struct GuardSim {
    guard: Guard,
    board: Vec<char>,
    width: i32,
    height: i32,
}

impl GuardSim {
    pub fn new(input: &str) -> Self {
        let mut guard = Guard::new(0, 0, Direction::Up);
        let mut board = vec![];

        let mut last_x: i32 = 0;
        let mut last_y: i32 = 0;
        input.lines().enumerate().for_each(|(y, l)| {
            l.chars().enumerate().for_each(|(x, c)| {
                match c {
                    '.' | '#' => board.push(c),
                    '^' => {
                        guard = Guard::new(x as i32, y as i32, Direction::Up);
                        board.push('.');
                    }
                    _ => panic!("Invalid Character"),
                };
                last_x = x as i32;
            });
            last_y = y as i32;
        });

        let width: i32 = last_x + 1;
        let height: i32 = last_y + 1;

        Self {
            guard,
            board,
            width,
            height,
        }
    }

    pub fn simulate(&self) -> Option<HashSet<PositionGuard>> {
        let mut elapsed_guard = HashSet::new();

        let mut current_guard = self.guard;

        'sim: loop {
            if !elapsed_guard.insert(current_guard) {
                // we are looping
                return None;
            }

            current_guard = self.step(current_guard);

            if current_guard.is_oob(self.width, self.height) {
                break 'sim;
            }
        }

        let position_guard: HashSet<_> =
            elapsed_guard.into_iter().map(PositionGuard::from).collect();
        Some(position_guard)
    }

    pub fn simulate_with_walls(mut self, mut wall_positions: HashSet<PositionGuard>) -> usize {
        let mut count = 0;
        wall_positions.remove(&self.guard.into());

        for pos in wall_positions {
            let wall_index = pos.get_index(self.width);
            self.board[wall_index] = '#';

            if self.simulate().is_none() {
                count += 1;
            }

            self.board[wall_index] = '.';
        }

        count
    }

    fn step(&self, guard: Guard) -> Guard {
        let mut shrodingers_guard = guard;

        shrodingers_guard.move_forward();
        if shrodingers_guard.is_oob(self.width, self.height) {
            shrodingers_guard
        } else {
            let pos_index = shrodingers_guard.get_index(self.width);

            let c = self.board[pos_index];
            if c == '#' {
                let mut turned_guard = guard;
                turned_guard.turn();
                turned_guard
            } else {
                shrodingers_guard
            }
        }
    }
}
