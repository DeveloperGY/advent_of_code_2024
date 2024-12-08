use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input = fs::read_to_string("Input_Day_8").unwrap();
    let map = Map::new(&input);

    let antinodes = map.get_antinodes();
    println!("Antinodes Count: {}", antinodes.len());

    let resonant_antinodes = map.get_all_antinodes();
    println!("Resonant Antinodes Count: {}", resonant_antinodes.len());
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Antenna {
    frequency: char,
    x: i64,
    y: i64,
}

impl Antenna {
    pub fn new(frequency: char, (x, y): (i64, i64)) -> Self {
        Self { frequency, x, y }
    }
}

struct Map {
    antennae: HashMap<char, HashSet<Antenna>>,
    width: i64,
    height: i64,
}

impl Map {
    pub fn new(input: &str) -> Self {
        let mut antennae: HashMap<char, HashSet<Antenna>> = HashMap::new();

        let antenna_char_map = input
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        for y in 0..antenna_char_map.len() {
            for x in 0..antenna_char_map[y].len() {
                let c = antenna_char_map[y][x];
                if c != '.' {
                    let antenna = Antenna::new(c, (x as i64, y as i64));

                    match antennae.get_mut(&c) {
                        Some(set) => {
                            set.insert(antenna);
                        }
                        None => {
                            let mut set = HashSet::new();
                            set.insert(antenna);
                            antennae.insert(c, set);
                        }
                    };
                }
            }
        }

        let width = antenna_char_map.len() as i64;
        let height = antenna_char_map[0].len() as i64;

        Self {
            antennae,
            width,
            height,
        }
    }

    pub fn get_all_antinodes(&self) -> HashSet<(i64, i64)> {
        let mut antinodes = HashSet::new();

        for (_freq, set) in &self.antennae {
            for a in set {
                for b in set {
                    if a != b {
                        antinodes.insert((a.x, a.y));
                        antinodes.insert((b.x, b.y));

                        let mut a_side_done = false;
                        let mut b_side_done = false;

                        let dx = b.x - a.x;
                        let dy = b.y - a.y;

                        let mut coefficent: i64 = 1;

                        'gen_resonant_antinodes: loop {
                            if a_side_done && b_side_done {
                                break 'gen_resonant_antinodes;
                            }

                            let dx = coefficent * dx;
                            let dy = coefficent * dy;

                            let x1 = b.x + dx;
                            let y1 = b.y + dy;

                            let x2 = a.x - dx;
                            let y2 = a.y - dy;

                            if (0..self.width).contains(&x1)
                                && (0..self.height).contains(&y1)
                                && !b_side_done
                            {
                                antinodes.insert((x1, y1));
                            } else {
                                b_side_done = true;
                            }

                            if (0..self.width).contains(&x2)
                                && (0..self.height).contains(&y2)
                                && !a_side_done
                            {
                                antinodes.insert((x2, y2));
                            } else {
                                a_side_done = true;
                            }

                            coefficent += 1;
                        }
                    }
                }
            }
        }

        antinodes
    }

    pub fn get_antinodes(&self) -> HashSet<(i64, i64)> {
        let mut antinodes = HashSet::new();

        for (_freq, set) in &self.antennae {
            for a in set {
                for b in set {
                    if a != b {
                        let dx = b.x - a.x;
                        let dy = b.y - a.y;

                        let x1 = b.x + dx;
                        let y1 = b.y + dy;

                        let x2 = a.x - dx;
                        let y2 = a.y - dy;

                        if (0..self.width).contains(&x1) && (0..self.height).contains(&y1) {
                            antinodes.insert((x1, y1));
                        }

                        if (0..self.width).contains(&x2) && (0..self.height).contains(&y2) {
                            antinodes.insert((x2, y2));
                        }
                    }
                }
            }
        }

        antinodes
    }
}
