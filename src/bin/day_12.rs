use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("Input_Day_12").unwrap();
    let farm = Farm::new(&input);
    let regions = farm.get_regions();

    let price: u64 = regions.iter().map(|r| r.area * r.perimeter).sum();
    println!("Price: ${}", price);

    let discount_price: u64 = regions.into_iter().map(|r| r.area * r.side_count()).sum();
    println!("Discount Price: ${}", discount_price);
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Pos {
    pub x: u64,
    pub y: u64,
}

impl Pos {
    pub fn new(x: u64, y: u64) -> Self {
        Self { x, y }
    }

    pub fn is_adjacent(&self, rhs: &Self) -> bool {
        match *rhs {
            Self { x, y } if (x, y) == (self.x + 1, self.y) => true,
            Self { x, y } if (x, y) == (self.x - 1, self.y) => true,
            Self { x, y } if (x, y) == (self.x, self.y + 1) => true,
            Self { x, y } if (x, y) == (self.x, self.y - 1) => true,
            _ => false,
        }
    }

    pub fn adjacency_list(&self, width: u64, height: u64) -> Vec<Self> {
        match (self.x, self.y) {
            (0, 0) => vec![Self::new(1, 0), Self::new(0, 1)],
            (x, 0) if x == width - 1 => vec![Self::new(x - 1, 0), Self::new(x, 1)],
            (0, y) if y == height - 1 => vec![Self::new(0, y - 1), Self::new(1, y)],
            (x, y) if x == width - 1 && y == height - 1 => {
                vec![Self::new(x - 1, y), Self::new(x, y - 1)]
            }
            (0, y) => vec![Self::new(0, y - 1), Self::new(0, y + 1), Self::new(1, y)],
            (x, y) if x == width - 1 => vec![
                Self::new(x, y - 1),
                Self::new(x, y + 1),
                Self::new(x - 1, y),
            ],
            (x, 0) => vec![Self::new(x - 1, 0), Self::new(x + 1, 0), Self::new(x, 1)],
            (x, y) if y == height - 1 => vec![
                Self::new(x - 1, y),
                Self::new(x + 1, y),
                Self::new(x, y - 1),
            ],
            (x, y) => vec![
                Self::new(x - 1, y),
                Self::new(x + 1, y),
                Self::new(x, y - 1),
                Self::new(x, y + 1),
            ],
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Plot {
    plant_type: char,
    pos: Pos,
}

impl Plot {
    pub fn new(plant_type: char, pos: Pos) -> Self {
        Self { plant_type, pos }
    }

    pub fn is_adjacent(&self, rhs: &Self) -> bool {
        self.pos.is_adjacent(&rhs.pos) && self.plant_type == rhs.plant_type
    }
}

#[derive(Debug, Clone)]
struct Region {
    pub area: u64,
    pub perimeter: u64,
    plots: HashSet<Plot>,
}

impl Region {
    pub fn new(area: u64, perimeter: u64, plots: HashSet<Plot>) -> Self {
        Self {
            area,
            perimeter,
            plots,
        }
    }

    pub fn side_count(&self) -> u64 {
        let left_most = self
            .plots
            .iter()
            .min_by(|lhs, rhs| lhs.pos.x.cmp(&rhs.pos.x))
            .unwrap();
        let right_most = self
            .plots
            .iter()
            .max_by(|lhs, rhs| lhs.pos.x.cmp(&rhs.pos.x))
            .unwrap();
        let top_most = self
            .plots
            .iter()
            .min_by(|lhs, rhs| lhs.pos.y.cmp(&rhs.pos.y))
            .unwrap();
        let bottom_most = self
            .plots
            .iter()
            .max_by(|lhs, rhs| lhs.pos.y.cmp(&rhs.pos.y))
            .unwrap();

        let width = right_most.pos.x - left_most.pos.x + 1;
        let height = bottom_most.pos.y - top_most.pos.y + 1;
        let x_off = left_most.pos.x;
        let y_off = top_most.pos.y;

        if width >= 128 {
            panic!("Width is greater than 128");
        }
        if height >= 128 {
            panic!("Height is greater than 128");
        }

        let mut horizontal_board = vec![0_u128; height as usize];
        let mut vertical_board = vec![0_u128; width as usize];

        let right_most_bit = 1_u128 << (128 - width - 1);
        let bottom_most_bit = 1_u128 << (128 - height - 1);

        // generate bitboards
        for plot in &self.plots {
            let x = plot.pos.x - x_off;
            let y = plot.pos.y - y_off;
            vertical_board[x as usize] |= bottom_most_bit << (height - y - 1);
            horizontal_board[y as usize] |= right_most_bit << (width - x - 1);
        }

        // generate horizontal (left/right) border bitmasks
        let mut left_borders = vec![];
        let mut right_borders = vec![];

        for bitmask in &horizontal_board {
            let plots = *bitmask;
            let spaces = !plots;

            let right_border = plots & ((spaces << 1) + 1);
            let left_border = plots & ((spaces >> 1) | 2_u128.pow(127));

            left_borders.push(left_border);
            right_borders.push(right_border);
        }

        let mut top_borders = vec![];
        let mut bottom_borders = vec![];

        for bitmask in &vertical_board {
            let plots = *bitmask;
            let spaces = !plots;

            let bottom_border = plots & ((spaces << 1) + 1);
            let top_border = plots & ((spaces >> 1) | 2_u128.pow(127));

            top_borders.push(top_border);
            bottom_borders.push(bottom_border);
        }

        let mut right_sides: u64 = 0;
        let mut left_sides: u64 = 0;
        let mut top_sides: u64 = 0;
        let mut bottom_sides: u64 = 0;

        let mut last_right_mask = 0;
        for bitmask in right_borders {
            let new_sides = bitmask & !last_right_mask;
            right_sides += new_sides.count_ones() as u64;

            last_right_mask = bitmask
        }

        let mut last_left_mask = 0;
        for bitmask in left_borders {
            let new_sides = bitmask & !last_left_mask;
            left_sides += new_sides.count_ones() as u64;

            last_left_mask = bitmask
        }

        let mut last_top_mask = 0;
        for bitmask in top_borders {
            let new_sides = bitmask & !last_top_mask;
            top_sides += new_sides.count_ones() as u64;

            last_top_mask = bitmask
        }

        let mut last_bottom_mask = 0;
        for bitmask in bottom_borders {
            let new_sides = bitmask & !last_bottom_mask;
            bottom_sides += new_sides.count_ones() as u64;

            last_bottom_mask = bitmask
        }

        let side_count = right_sides + left_sides + top_sides + bottom_sides;

        if self.plots.iter().next().unwrap().plant_type == 'Y' {
            println!("Side Count: {}", side_count);
            for bitmask in &horizontal_board {
                println!("{:01$b}", *bitmask >> (127 - width), width as usize);
            }
            println!();
        }

        side_count
    }
}

#[derive(Debug, Clone)]
struct Farm {
    plots: Vec<Vec<Plot>>,
    width: u64,
    height: u64,
}

impl Farm {
    pub fn new(input: &str) -> Self {
        let plots = input
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| Plot::new(c, Pos::new(x as u64, y as u64)))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let width = plots[0].len() as u64;
        let height = plots.len() as u64;
        Self {
            plots,
            width,
            height,
        }
    }

    pub fn get_regions(&self) -> Vec<Region> {
        let mut regions = vec![];

        let mut to_visit: Vec<Plot> = self.plots.iter().flatten().copied().collect();
        to_visit.sort();

        while let Some(plot) = to_visit.pop() {
            let (region, visited) = self.get_region(plot);
            regions.push(region);
            for plot in visited {
                if let Ok(index) = to_visit.binary_search(&plot) {
                    to_visit.remove(index);
                }
            }
        }

        regions
    }

    fn get_region(&self, plot: Plot) -> (Region, HashSet<Plot>) {
        let mut visited = HashSet::new();
        let mut to_visit = vec![plot];

        let mut area = 0;
        let mut perimeter = 0;

        while let Some(plot) = to_visit.pop() {
            let adjacent_positions = plot.pos.adjacency_list(self.width, self.height);
            let adjacent_plots: Vec<Plot> = adjacent_positions
                .iter()
                .map(|pos| self.plots[pos.y as usize][pos.x as usize])
                .filter(|p| plot.is_adjacent(p))
                .collect();

            area += 1;
            perimeter += 4 - adjacent_plots.len() as u64;

            visited.insert(plot);
            let discovered_plots = adjacent_plots
                .into_iter()
                .filter(|p| !visited.contains(p) && !to_visit.contains(p))
                .collect::<Vec<_>>();
            to_visit.extend_from_slice(&discovered_plots);
        }

        println!(
            "Area: {}\nHashArea: {}\nPerimeter: {}\nPlant: {}\n",
            area,
            visited.len(),
            perimeter,
            plot.plant_type
        );

        (
            Region {
                area,
                perimeter,
                plots: visited.clone(),
            },
            visited,
        )
    }
}
