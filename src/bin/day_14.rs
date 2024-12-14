use std::fs;
use std::ops::{Add, Mul};

fn main() {
    let input = fs::read_to_string("Input_Day_14").unwrap();
    let mut robots = parse_input(&input);

    let future_robots = robots
        .iter()
        .map(|r| r.fast_forward(100))
        .map(|mut r| {
            r.correct_position();
            println!("{:?}", r);
            r
        })
        .collect::<Vec<_>>();

    let quadrants = future_robots
        .into_iter()
        .filter(Robot::is_in_quadrant)
        .map(|r| r.get_quadrant().unwrap())
        .collect::<Vec<_>>();

    let quad_1 = quadrants.iter().filter(|v| **v == 1).count();
    let quad_2 = quadrants.iter().filter(|v| **v == 2).count();
    let quad_3 = quadrants.iter().filter(|v| **v == 3).count();
    let quad_4 = quadrants.iter().filter(|v| **v == 4).count();
    let score = quad_1 * quad_2 * quad_3 * quad_4;
    println!("Score: {}", score);

    let mut bots = robots.clone();
    // 6493
    let mut seconds = 0;
    // let mut total_score = score;
    // let mut total_count = 1;
    loop {
        seconds += 1;
        println!("Trying ({}) seconds", seconds);

        bots = robots
            .iter()
            .map(|b| {
                let mut b = b.fast_forward(seconds);
                b.correct_position();
                b
            })
            .collect();

        let board = Board::new(&bots);
        board.print();
        println!("- {} seconds", seconds);
        if board.is_tree() {
            println!("Tree in {} seconds!", seconds);
            break;
        }

        // if score < (total_score / total_count) {
        // } else {
        //     total_score += score;
        //     total_count += 1;
        // }

        // std::thread::sleep(std::time::Duration::from_millis(20));
    }
}

#[derive(Debug, Copy, Clone)]
struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add<Vel> for Pos {
    type Output = Pos;
    fn add(self, rhs: Vel) -> Self::Output {
        Pos::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[derive(Debug, Copy, Clone)]
struct Vel {
    pub x: i32,
    pub y: i32,
}

impl Vel {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Mul<i32> for Vel {
    type Output = Vel;

    fn mul(self, rhs: i32) -> Self::Output {
        Vel::new(self.x * rhs, self.y * rhs)
    }
}

#[derive(Debug, Copy, Clone)]
struct Robot {
    pos: Pos,
    vel: Vel,
}

impl Robot {
    pub fn new(pos: Pos, vel: Vel) -> Self {
        Self { pos, vel }
    }

    pub fn fast_forward(&self, time_in_seconds: i32) -> Self {
        let vel = self.vel * time_in_seconds;
        let pos = self.pos + vel;
        Self::new(pos, vel)
    }

    pub fn correct_position(&mut self) {
        let mut true_x = self.pos.x % 101;
        let mut true_y = self.pos.y % 103;

        if true_x < 0 {
            true_x += 101
        };
        if true_y < 0 {
            true_y += 103
        };

        self.pos = Pos::new(true_x, true_y);
    }

    pub fn is_in_quadrant(&self) -> bool {
        self.pos.x != 50 && self.pos.y != 51
    }

    pub fn get_quadrant(&self) -> Option<i32> {
        match (self.pos.x, self.pos.y) {
            (0..50, 0..51) => Some(2),
            (51..101, 0..51) => Some(1),
            (0..50, 52..103) => Some(3),
            (51..101, 52..103) => Some(4),
            _ => None,
        }
    }
}

fn parse_input(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|l| {
            l.split_once(' ')
                .map(|(pos, vel)| (pos.to_string(), vel.to_string()))
                .unwrap()
        })
        .map(|(pos_str, vel_str)| {
            let pos_coord = pos_str.trim_start_matches("p=").trim().to_string();
            let vel_coord = vel_str.trim_start_matches("v=").trim().to_string();
            (pos_coord, vel_coord)
        })
        .map(|(pos, vel)| {
            let pos = pos
                .split_once(',')
                .map(|(lhs, rhs)| (lhs.parse::<i32>().unwrap(), rhs.parse::<i32>().unwrap()))
                .unwrap();
            let vel = vel
                .split_once(',')
                .map(|(lhs, rhs)| (lhs.parse::<i32>().unwrap(), rhs.parse::<i32>().unwrap()))
                .unwrap();
            let pos = Pos::new(pos.0, pos.1);
            let vel = Vel::new(vel.0, vel.1);
            Robot::new(pos, vel)
        })
        .collect::<Vec<_>>()
}

struct Board {
    output: [[char; 101]; 103],
}

impl Board {
    pub fn new(robots: &[Robot]) -> Self {
        let mut board = [[' '; 101]; 103];

        for bot in robots {
            board[bot.pos.y as usize][bot.pos.x as usize] = '#';
        }

        Self { output: board }
    }

    pub fn print(&self) {
        let mut chars = self.output;

        // chars[51][50] = '^';

        for line in chars {
            let str = line.iter().collect::<String>();
            println!("{}", str);
        }
        println!();
    }

    pub fn is_tree(&self) -> bool {
        let center = self.output[51][51];

        if center != '#' {
            println!("No Center");
            return false;
        }

        let og_x = {
            let mut found_space = false;
            let mut x = 50;
            loop {
                x += 1;

                let c = self.output[51][x];

                if c == '#' && found_space {
                    break;
                } else if c == ' ' && !found_space {
                    found_space = true;
                } else if x == 100 {
                    println!("No outline!");
                    return false;
                }
            }
            x
        };

        let (mut x, mut y) = (og_x, 51_usize);
        let mut current_side = "right";

        loop {
            match current_side {
                "right" => {
                    let down = self.output[y + 1][x];
                    let left = self.output[y][x - 1];

                    if left == '#' {
                        current_side = "bottom";
                        x -= 1;
                    } else if down == '#' {
                        y += 1;
                    } else {
                        println!("No Bottom");
                        return false;
                    }
                }
                "bottom" => {
                    let up = self.output[y - 1][x];
                    let left = self.output[y][x - 1];

                    if up == '#' {
                        current_side = "left";
                        y -= 1;
                    } else if left == '#' {
                        x -= 1;
                    } else {
                        println!("No left");
                        return false;
                    }
                }
                "left" => {
                    let right = self.output[y][x + 1];
                    let up = self.output[y - 1][x];

                    if right == '#' {
                        current_side = "top";
                        x += 1;
                    } else if up == '#' {
                        y -= 1;
                    } else {
                        println!("No top");
                        return false;
                    }
                }
                "top" => {
                    let down = self.output[y + 1][x];
                    let right = self.output[y][x + 1];

                    if down == '#' {
                        current_side = "right";
                        y += 1;
                    } else if right == '#' {
                        x += 1;
                    } else {
                        println!("No right");
                        return false;
                    }
                }
                _ => {
                    println!("WTFFFFFF!!!!!!");
                    return false;
                }
            };

            if (x, y) == (og_x, 51) {
                return true;
            }
        }

        false
    }
}
