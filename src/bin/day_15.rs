use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs;

fn main() {
    let input = fs::read_to_string("Input_Day_15").unwrap();
    let (board_str, moves_str) = input.split_once("\n\n").unwrap();

    let board = Board::new(board_str);
    let wide_board = WideBoard::new(&board);
    let move_list = MoveList::try_new(moves_str).unwrap();

    let finished_board = Simulator::simulate(board, &move_list);
    let gps_sum = GpsTracker::gps_coordinate_sum(&finished_board);
    println!("{}\nGps Sum: {}", &finished_board, gps_sum);

    let finished_wide_board = WideSimulator::simulate(wide_board, &move_list);
    let wide_gps_sum = WideGpsTracker::gps_coordinate_sum(&finished_wide_board);
    println!("{}\nWide Gps Sum: {}", &finished_wide_board, wide_gps_sum);
}

#[derive(Debug, Clone)]
struct Board {
    grid: Box<[Box<[char]>]>,
    robot_pos: (i64, i64),
}

impl Board {
    fn new(input: &str) -> Self {
        let mut rows = vec![];

        for line in input.lines() {
            let row = line.chars().collect::<Vec<_>>().into_boxed_slice();
            rows.push(row);
        }

        let mut robot_pos = (0, 0);
        'search_for_robot: for (y, row) in rows.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == '@' {
                    robot_pos = (x as i64, y as i64);
                    break 'search_for_robot;
                }
            }
        }

        Self {
            grid: rows.into_boxed_slice(),
            robot_pos,
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let out = self
            .grid
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", out)
    }
}

#[derive(Debug, Clone)]
struct WideBoard {
    grid: Box<[Box<[char]>]>,
    robot_pos: (i64, i64),
}

impl WideBoard {
    fn new(board: &Board) -> Self {
        let mut rows = vec![];

        for row in &board.grid {
            let row = row
                .iter()
                .flat_map(|c| match *c {
                    '#' => ['#'; 2],
                    '.' => ['.'; 2],
                    'O' => ['[', ']'],
                    '@' => ['@', '.'],
                    _ => panic!("Invalid board cell!"),
                })
                .collect::<Vec<_>>();
            rows.push(row.into_boxed_slice());
        }

        let robot_pos = (board.robot_pos.0 * 2, board.robot_pos.1);

        Self {
            grid: rows.into_boxed_slice(),
            robot_pos,
        }
    }
}

impl Display for WideBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let out = self
            .grid
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", out)
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn try_new(c: char) -> Option<Self> {
        match c {
            '^' => Some(Self::Up),
            'v' => Some(Self::Down),
            '<' => Some(Self::Left),
            '>' => Some(Self::Right),
            _ => None,
        }
    }

    fn to_vector(&self) -> (i64, i64) {
        match *self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }
}

#[derive(Debug, Clone)]
struct MoveList {
    moves: Box<[Move]>,
}

impl MoveList {
    pub fn try_new(input: &str) -> Option<Self> {
        let move_list_str = input.lines().collect::<Vec<_>>().join("");
        let moves = move_list_str
            .chars()
            .map(|c| Move::try_new(c))
            .collect::<Option<Vec<_>>>()?;

        Some(Self {
            moves: moves.into_boxed_slice(),
        })
    }
}

struct GpsTracker;

impl GpsTracker {
    pub fn gps_coordinate_sum(board: &Board) -> u32 {
        let mut sum = 0;

        for (y, row) in board.grid.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == 'O' {
                    let gps = (100 * y + x) as u32;
                    sum += gps;
                }
            }
        }

        sum
    }
}

struct WideGpsTracker;

impl WideGpsTracker {
    pub fn gps_coordinate_sum(board: &WideBoard) -> u32 {
        let mut sum = 0;

        for (y, row) in board.grid.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == '[' {
                    let gps = (100 * y + x) as u32;
                    sum += gps;
                }
            }
        }

        sum
    }
}

struct Simulator;

impl Simulator {
    pub fn simulate(mut board: Board, move_list: &MoveList) -> Board {
        'sim_move: for robot_move in &move_list.moves {
            let v = robot_move.to_vector();
            let target_pos = (board.robot_pos.0 + v.0, board.robot_pos.1 + v.1);
            let target_obj = board.grid[target_pos.1 as usize][target_pos.0 as usize];

            match target_obj {
                '#' => continue,
                '.' => {
                    board.grid[target_pos.1 as usize][target_pos.0 as usize] = '@';
                    board.grid[board.robot_pos.1 as usize][board.robot_pos.0 as usize] = '.';
                    board.robot_pos = target_pos;
                }
                'O' => {
                    let mut dist_target_pos = target_pos;
                    let distant_space = 'find_dist_space: loop {
                        dist_target_pos.0 += v.0;
                        dist_target_pos.1 += v.1;

                        match board.grid[dist_target_pos.1 as usize][dist_target_pos.0 as usize] {
                            '#' => continue 'sim_move,
                            '.' => {
                                break 'find_dist_space dist_target_pos;
                            }
                            _ => continue 'find_dist_space,
                        };
                    };

                    board.grid[distant_space.1 as usize][distant_space.0 as usize] = 'O';
                    board.grid[target_pos.1 as usize][target_pos.0 as usize] = '@';
                    board.grid[board.robot_pos.1 as usize][board.robot_pos.0 as usize] = '.';
                    board.robot_pos = target_pos;
                }
                _ => eprintln!("Invalid Board Cell!"),
            };
        }

        board
    }
}

struct WideSimulator;

impl WideSimulator {
    pub fn simulate(mut board: WideBoard, move_list: &MoveList) -> WideBoard {
        'sim_move: for robot_move in &move_list.moves {
            let v = robot_move.to_vector();
            let target_pos = (board.robot_pos.0 + v.0, board.robot_pos.1 + v.1);
            let target_obj = board.grid[target_pos.1 as usize][target_pos.0 as usize];

            match target_obj {
                '#' => continue,
                '.' => {
                    board.grid[target_pos.1 as usize][target_pos.0 as usize] = '@';
                    board.grid[board.robot_pos.1 as usize][board.robot_pos.0 as usize] = '.';
                    board.robot_pos = target_pos;
                }
                '[' | ']' => match *robot_move {
                    Move::Left | Move::Right => {
                        let mut dist_target_pos = target_pos;
                        let distant_space = 'find_dist_space: loop {
                            dist_target_pos.0 += v.0;
                            dist_target_pos.1 += v.1;

                            match board.grid[dist_target_pos.1 as usize][dist_target_pos.0 as usize]
                            {
                                '#' => continue 'sim_move,
                                '.' => {
                                    break 'find_dist_space dist_target_pos;
                                }
                                _ => continue 'find_dist_space,
                            };
                        };

                        let distance = distant_space.0.abs_diff(target_pos.0);
                        let direction_modifier = if matches!(robot_move, Move::Right) {
                            1
                        } else {
                            -1
                        };
                        for offset in 0..distance {
                            let x = target_pos.0 + (offset as i64 * direction_modifier);
                            let cell = board.grid[target_pos.1 as usize][x as usize];
                            let new_cell = match cell {
                                '[' => ']',
                                ']' => '[',
                                _ => panic!("Invalid Cell Value!"),
                            };
                            board.grid[target_pos.1 as usize][x as usize] = new_cell;
                        }

                        board.grid[distant_space.1 as usize][distant_space.0 as usize] =
                            if matches!(robot_move, Move::Right) {
                                ']'
                            } else {
                                '['
                            };
                        board.grid[target_pos.1 as usize][target_pos.0 as usize] = '@';
                        board.grid[board.robot_pos.1 as usize][board.robot_pos.0 as usize] = '.';
                        board.robot_pos = target_pos;
                    }
                    Move::Up => {
                        let box_pos =
                            if board.grid[target_pos.1 as usize][target_pos.0 as usize] == '[' {
                                (target_pos.0, target_pos.1)
                            } else {
                                (target_pos.0 - 1, target_pos.1)
                            };

                        let can_move = Self::try_move_box_up(box_pos, &mut board);
                        if can_move {
                            board.grid[target_pos.1 as usize][target_pos.0 as usize] = '@';
                            board.grid[board.robot_pos.1 as usize][board.robot_pos.0 as usize] =
                                '.';
                            board.robot_pos = target_pos;
                        }
                    }
                    Move::Down => {
                        let box_pos =
                            if board.grid[target_pos.1 as usize][target_pos.0 as usize] == '[' {
                                (target_pos.0, target_pos.1)
                            } else {
                                (target_pos.0 - 1, target_pos.1)
                            };

                        let can_move = Self::try_move_box_down(box_pos, &mut board);
                        if can_move {
                            board.grid[target_pos.1 as usize][target_pos.0 as usize] = '@';
                            board.grid[board.robot_pos.1 as usize][board.robot_pos.0 as usize] =
                                '.';
                            board.robot_pos = target_pos;
                        }
                    }
                },
                _ => eprintln!("Invalid Board Cell!"),
            };
        }

        board
    }

    fn try_move_box_up(box_pos: (i64, i64), board: &mut WideBoard) -> bool {
        fn move_box(box_pos: (i64, i64), board: &mut WideBoard) {
            let left_cell = board.grid[(box_pos.1 - 1) as usize][box_pos.0 as usize];
            let right_cell = board.grid[(box_pos.1 - 1) as usize][(box_pos.0 + 1) as usize];

            match (left_cell, right_cell) {
                ('.', '.') => (),
                ('[', _) => move_box((box_pos.0, box_pos.1 - 1), board),
                (']', '[') => {
                    move_box((box_pos.0 - 1, box_pos.1 - 1), board);
                    move_box((box_pos.0 + 1, box_pos.1 - 1), board);
                }
                (']', _) => move_box((box_pos.0 - 1, box_pos.1 - 1), board),
                (_, '[') => move_box((box_pos.0 + 1, box_pos.1 - 1), board),
                _ => eprintln!("Invalid Grid Cell!"),
            };

            board.grid[(box_pos.1 - 1) as usize][box_pos.0 as usize] = '[';
            board.grid[(box_pos.1 - 1) as usize][(box_pos.0 + 1) as usize] = ']';
            board.grid[box_pos.1 as usize][box_pos.0 as usize] = '.';
            board.grid[box_pos.1 as usize][(box_pos.0 + 1) as usize] = '.';
        }

        let mut can_move_cache = HashMap::new();
        if Self::can_move_box_up(box_pos, board, &mut can_move_cache) {
            move_box(box_pos, board);
            true
        } else {
            false
        }
    }

    fn can_move_box_up(
        box_pos: (i64, i64),
        board: &WideBoard,
        cache: &mut HashMap<(i64, i64), bool>,
    ) -> bool {
        fn inner(
            box_pos: (i64, i64),
            board: &WideBoard,
            cache: &mut HashMap<(i64, i64), bool>,
        ) -> bool {
            let left_cell = board.grid[(box_pos.1 - 1) as usize][box_pos.0 as usize];
            let right_cell = board.grid[(box_pos.1 - 1) as usize][(box_pos.0 + 1) as usize];

            match (left_cell, right_cell) {
                ('.', '.') => true,
                ('#', _) | (_, '#') => false,
                ('[', _) => {
                    WideSimulator::can_move_box_up((box_pos.0, box_pos.1 - 1), board, cache)
                }
                (']', '[') => {
                    WideSimulator::can_move_box_up((box_pos.0 - 1, box_pos.1 - 1), board, cache)
                        && WideSimulator::can_move_box_up(
                            (box_pos.0 + 1, box_pos.1 - 1),
                            board,
                            cache,
                        )
                }
                (']', _) => {
                    WideSimulator::can_move_box_up((box_pos.0 - 1, box_pos.1 - 1), board, cache)
                }
                (_, '[') => {
                    WideSimulator::can_move_box_up((box_pos.0 + 1, box_pos.1 - 1), board, cache)
                }
                _ => false,
            }
        }

        if let Some(res) = cache.get(&box_pos) {
            *res
        } else {
            let res = inner(box_pos, board, cache);
            cache.insert(box_pos, res);
            res
        }
    }

    fn try_move_box_down(box_pos: (i64, i64), board: &mut WideBoard) -> bool {
        fn move_box(box_pos: (i64, i64), board: &mut WideBoard) {
            let left_cell = board.grid[(box_pos.1 + 1) as usize][box_pos.0 as usize];
            let right_cell = board.grid[(box_pos.1 + 1) as usize][(box_pos.0 + 1) as usize];

            match (left_cell, right_cell) {
                ('.', '.') => (),
                ('[', _) => move_box((box_pos.0, box_pos.1 + 1), board),
                (']', '[') => {
                    move_box((box_pos.0 - 1, box_pos.1 + 1), board);
                    move_box((box_pos.0 + 1, box_pos.1 + 1), board);
                }
                (']', _) => move_box((box_pos.0 - 1, box_pos.1 + 1), board),
                (_, '[') => move_box((box_pos.0 + 1, box_pos.1 + 1), board),
                _ => eprintln!("Invalid Grid Cell!"),
            };

            board.grid[(box_pos.1 + 1) as usize][box_pos.0 as usize] = '[';
            board.grid[(box_pos.1 + 1) as usize][(box_pos.0 + 1) as usize] = ']';
            board.grid[box_pos.1 as usize][box_pos.0 as usize] = '.';
            board.grid[box_pos.1 as usize][(box_pos.0 + 1) as usize] = '.';
        }

        let mut can_move_cache = HashMap::new();
        if Self::can_move_box_down(box_pos, board, &mut can_move_cache) {
            move_box(box_pos, board);
            true
        } else {
            false
        }
    }

    fn can_move_box_down(
        box_pos: (i64, i64),
        board: &WideBoard,
        cache: &mut HashMap<(i64, i64), bool>,
    ) -> bool {
        fn inner(
            box_pos: (i64, i64),
            board: &WideBoard,
            cache: &mut HashMap<(i64, i64), bool>,
        ) -> bool {
            let left_cell = board.grid[(box_pos.1 + 1) as usize][box_pos.0 as usize];
            let right_cell = board.grid[(box_pos.1 + 1) as usize][(box_pos.0 + 1) as usize];

            match (left_cell, right_cell) {
                ('.', '.') => true,
                ('#', _) | (_, '#') => false,
                ('[', _) => {
                    WideSimulator::can_move_box_down((box_pos.0, box_pos.1 + 1), board, cache)
                }
                (']', '[') => {
                    WideSimulator::can_move_box_down((box_pos.0 - 1, box_pos.1 + 1), board, cache)
                        && WideSimulator::can_move_box_down(
                            (box_pos.0 + 1, box_pos.1 + 1),
                            board,
                            cache,
                        )
                }
                (']', _) => {
                    WideSimulator::can_move_box_down((box_pos.0 - 1, box_pos.1 + 1), board, cache)
                }
                (_, '[') => {
                    WideSimulator::can_move_box_down((box_pos.0 + 1, box_pos.1 + 1), board, cache)
                }
                _ => false,
            }
        }

        if let Some(res) = cache.get(&box_pos) {
            *res
        } else {
            let res = inner(box_pos, board, cache);
            cache.insert(box_pos, res);
            res
        }
    }
}
