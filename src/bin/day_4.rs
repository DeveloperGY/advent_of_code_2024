use std::fs;

fn main() {
    let input = fs::read_to_string("Input_Day_4").unwrap();

    let wordsearch = Wordsearch::new(&input);

    let forward_horizontal_count = wordsearch.count_horizontal();
    println!("FW Horizontal Count: {}", forward_horizontal_count);
    let backward_horizontal_count = wordsearch.count_horizontal_rev();
    println!("BW Horizontal Count: {}", backward_horizontal_count);

    let forward_vertical_count = wordsearch.count_vertical();
    println!("FW Vertical Count: {}", forward_vertical_count);
    let backward_vertical_count = wordsearch.count_vertical_rev();
    println!("BW Vertical Count: {}", backward_vertical_count);

    let forward_northwest_count = wordsearch.count_diagonal_northwest();
    println!("FW Northwest Count: {}", forward_northwest_count);
    let backward_northwest_count = wordsearch.count_diagonal_northwest_rev();
    println!("BW Northwest Count: {}", backward_northwest_count);

    let forward_southeast_count = wordsearch.count_diagonal_southeast();
    println!("FW Southeast Count: {}", forward_southeast_count);
    let backward_southeast_count = wordsearch.count_diagonal_southeast_rev();
    println!("BW Southeast Count: {}", backward_southeast_count);

    let total = forward_horizontal_count
        + backward_horizontal_count
        + forward_vertical_count
        + backward_vertical_count
        + forward_northwest_count
        + backward_northwest_count
        + forward_southeast_count
        + backward_southeast_count;

    println!("Total: {}", total);

    let xmas_count = wordsearch.count_x_mas();
    println!("X-MAS Total: {}", xmas_count);
}

struct Wordsearch {
    board: Vec<Vec<char>>,
}

impl Wordsearch {
    const WORD: &'static str = "XMAS";
    pub fn new(board_data: &str) -> Self {
        let mut board = vec![];

        // Data not validated, no need since its prevalidated by AoC
        board_data
            .lines()
            .for_each(|l| board.push(l.trim().chars().collect()));

        Self { board }
    }

    fn count_horizontal(&self) -> u64 {
        let mut count = 0;

        for y in 0..self.board.len() {
            for x in 0..self.board[y].len() - 3 {
                let group = &[
                    self.board[y][x],
                    self.board[y][x + 1],
                    self.board[y][x + 2],
                    self.board[y][x + 3],
                ];

                if let &['X', 'M', 'A', 'S'] = group {
                    count += 1;
                }
            }
        }

        count
    }

    fn count_horizontal_rev(&self) -> u64 {
        let mut count = 0;

        for y in 0..self.board.len() {
            for x in 0..self.board[y].len() - 3 {
                let group = &[
                    self.board[y][x],
                    self.board[y][x + 1],
                    self.board[y][x + 2],
                    self.board[y][x + 3],
                ];

                if let &['S', 'A', 'M', 'X'] = group {
                    count += 1;
                }
            }
        }

        count
    }

    fn count_vertical(&self) -> u64 {
        let mut count = 0;

        for y in 0..self.board.len() - 3 {
            for x in 0..self.board[y].len() {
                let group = &[
                    self.board[y][x],
                    self.board[y + 1][x],
                    self.board[y + 2][x],
                    self.board[y + 3][x],
                ];

                if let &['X', 'M', 'A', 'S'] = group {
                    count += 1;
                }
            }
        }

        count
    }

    fn count_vertical_rev(&self) -> u64 {
        let mut count = 0;

        for y in 0..self.board.len() - 3 {
            for x in 0..self.board[y].len() {
                let group = &[
                    self.board[y][x],
                    self.board[y + 1][x],
                    self.board[y + 2][x],
                    self.board[y + 3][x],
                ];

                if let &['S', 'A', 'M', 'X'] = &group {
                    count += 1;
                }
            }
        }

        count
    }

    fn count_diagonal_northwest(&self) -> u64 {
        let mut count = 0;

        for y in 0..self.board.len() - 3 {
            for x in 0..self.board[y].len() - 3 {
                let group = &[
                    self.board[y][x],
                    self.board[y + 1][x + 1],
                    self.board[y + 2][x + 2],
                    self.board[y + 3][x + 3],
                ];

                if let &['X', 'M', 'A', 'S'] = group {
                    count += 1;
                }
            }
        }

        count
    }

    fn count_diagonal_northwest_rev(&self) -> u64 {
        let mut count = 0;

        for y in 0..self.board.len() - 3 {
            for x in 0..self.board[y].len() - 3 {
                let group = &[
                    self.board[y][x],
                    self.board[y + 1][x + 1],
                    self.board[y + 2][x + 2],
                    self.board[y + 3][x + 3],
                ];

                if let &['S', 'A', 'M', 'X'] = &group {
                    count += 1;
                }
            }
        }

        count
    }

    fn count_diagonal_southeast(&self) -> u64 {
        let mut count = 0;

        for y in 3..self.board.len() {
            for x in 0..self.board[y].len() - 3 {
                let group = &[
                    self.board[y][x],
                    self.board[y - 1][x + 1],
                    self.board[y - 2][x + 2],
                    self.board[y - 3][x + 3],
                ];

                if let &['X', 'M', 'A', 'S'] = group {
                    count += 1;
                }
            }
        }

        count
    }

    fn count_diagonal_southeast_rev(&self) -> u64 {
        let mut count = 0;

        for y in 3..self.board.len() {
            for x in 0..self.board[y].len() - 3 {
                let group = &[
                    self.board[y][x],
                    self.board[y - 1][x + 1],
                    self.board[y - 2][x + 2],
                    self.board[y - 3][x + 3],
                ];

                if let &['S', 'A', 'M', 'X'] = &group {
                    count += 1;
                }
            }
        }

        count
    }

    fn count_x_mas(&self) -> u64 {
        let mut count = 0;

        for y in 1..self.board.len() - 1 {
            for x in 1..self.board[y].len() - 1 {
                let middle = self.board[y][x];
                let northwest = (self.board[y - 1][x - 1], self.board[y + 1][x + 1]);
                let southeast = (self.board[y + 1][x - 1], self.board[y - 1][x + 1]);

                match (middle, northwest, southeast) {
                    ('A', ('M', 'S'), ('M', 'S'))
                    | ('A', ('S', 'M'), ('M', 'S'))
                    | ('A', ('M', 'S'), ('S', 'M'))
                    | ('A', ('S', 'M'), ('S', 'M')) => count += 1,
                    _ => (),
                };
            }
        }

        count
    }
}
