use std::fs;

fn main() {
    let input = fs::read_to_string("Input_Day_13").unwrap();

    let machines: Vec<Machine> = input
        .split("\n\n")
        .map(|m_str| {
            let lines: Vec<_> = m_str.lines().collect();
            let a_str = lines[0].trim_start_matches("Button A: ");
            let b_str = lines[1].trim_start_matches("Button B: ");
            let prize_str = lines[2].trim_start_matches("Prize: ");

            let a_coord = coordinate_parser(a_str);
            let b_coord = coordinate_parser(b_str);
            let prize_coord = coordinate_parser(prize_str);

            Machine::new(a_coord, b_coord, prize_coord)
        })
        .collect();

    let total_minimum_tokens: f64 = machines.iter().flat_map(|m| m.minimum_tokens()).sum();
    println!("Total Minimum Tokens: {}", total_minimum_tokens);

    let total_minimum_tokens_part_2: f64 = machines
        .iter()
        .flat_map(|m| m.minimum_tokens_2_electric_boogaloo())
        .sum();
    println!("Total Minimum Tokens 2: {}", total_minimum_tokens_part_2);
}

fn coordinate_parser(str: &str) -> (f64, f64) {
    let (x_str, y_str) = str.split_once(", ").unwrap();
    let x = x_str
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<f64>()
        .unwrap();
    let y = y_str
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<f64>()
        .unwrap();

    // println!("{} => ({}, {})", str, x, y);

    (x, y)
}

struct Machine {
    a: (f64, f64),
    b: (f64, f64),

    prize: (f64, f64),
}

impl Machine {
    pub fn new(a: (f64, f64), b: (f64, f64), prize: (f64, f64)) -> Self {
        Self { a, b, prize }
    }

    pub fn minimum_tokens(&self) -> Option<f64> {
        let det = self.a.0 * self.b.1 - self.b.0 * self.a.1;

        // adj
        let matrix = [self.b.1, -self.b.0, -self.a.1, self.a.0];

        let a = (matrix[0] * self.prize.0 + matrix[1] * self.prize.1) / det;
        let b = (matrix[2] * self.prize.0 + matrix[3] * self.prize.1) / det;

        if a != a.floor() || b != b.floor() {
            return None;
        }

        let x_mod = a * self.a.0 + b * self.b.0;
        let y_mod = a * self.a.1 + b * self.b.1;

        if x_mod != self.prize.0 || y_mod != self.prize.1 {
            return None;
        }

        if a < 0.0 || b < 0.0 || a > 100.0 || b > 100.0 {
            None
        } else {
            Some(a * 3.0 + b)
        }

        // println!("Det: {}", det);
        // println!("{:?}", matrix);
        // println!("A: {}, B: {}", a, b);
    }

    pub fn minimum_tokens_2_electric_boogaloo(&self) -> Option<f64> {
        let det = self.a.0 * self.b.1 - self.b.0 * self.a.1;
        let prize = (
            self.prize.0 + 10000000000000.0,
            self.prize.1 + 10000000000000.0,
        );

        // adj
        let matrix = [self.b.1, -self.b.0, -self.a.1, self.a.0];

        let a = (matrix[0] * prize.0 + matrix[1] * prize.1) / det;
        let b = (matrix[2] * prize.0 + matrix[3] * prize.1) / det;

        if a != a.floor() || b != b.floor() {
            return None;
        }

        let x_mod = a * self.a.0 + b * self.b.0;
        let y_mod = a * self.a.1 + b * self.b.1;

        if x_mod != prize.0 || y_mod != prize.1 {
            return None;
        }

        if a < 100.0 || b < 100.0 {
            None
        } else {
            Some(a * 3.0 + b)
        }
    }
}
