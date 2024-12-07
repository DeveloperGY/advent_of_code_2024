use std::fs;

fn main() {
    let input = fs::read_to_string("Input_Day_7").unwrap();

    let mut calibration_inputs = input
        .lines()
        .map(|l| l.split_once(':').unwrap())
        .map(|(lhs, rhs)| {
            (
                lhs.to_string(),
                rhs.split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect::<Vec<u64>>(),
            )
        })
        .map(|(lhs, rhs)| {
            let ans = lhs.parse::<u64>().unwrap();
            let nums = rhs;
            CalibrationInput::new(ans, nums)
        })
        .collect::<Vec<_>>();

    calibration_inputs
        .iter_mut()
        .for_each(|input| input.calibrate());

    let total: u64 = calibration_inputs
        .iter()
        .filter(|i| i.is_valid().unwrap())
        .map(|input| input.ans)
        .sum();
    println!("Total: {}", total);
}

#[derive(Copy, Clone)]
pub enum CalibrationOp {
    Add,
    Mul,
    Concat,
}

impl CalibrationOp {
    /// Goes to the next operation in the list, returning true if it looped back to the beginning
    fn next(&mut self) -> bool {
        *self = match *self {
            Self::Add => Self::Mul,
            Self::Mul => Self::Concat,
            Self::Concat => Self::Add,
        };

        matches!(self, Self::Add)
    }
}

impl Default for CalibrationOp {
    fn default() -> Self {
        Self::Add
    }
}

struct InputOps {
    ops: Vec<CalibrationOp>,
}

impl InputOps {
    pub fn new(count: usize) -> Self {
        let ops = vec![CalibrationOp::default(); count];
        Self { ops }
    }

    /// Returns true if it looped back to the beginning
    pub fn next(&mut self) -> bool {
        let mut should_change = true;

        for op in &mut self.ops {
            if should_change {
                should_change = op.next();
            }
        }

        self.ops
            .iter()
            .map(|o| matches!(o, CalibrationOp::Add))
            .reduce(|acc, i| acc && i)
            .unwrap()
    }

    pub fn get_op(&self, index: usize) -> Option<CalibrationOp> {
        self.ops.get(index).cloned()
    }
}

struct CalibrationInput {
    ans: u64,
    inputs: Vec<u64>,
    is_valid: Option<bool>,
}

impl CalibrationInput {
    pub fn new(ans: u64, inputs: Vec<u64>) -> Self {
        Self {
            ans,
            inputs,
            is_valid: None,
        }
    }

    pub fn is_valid(&self) -> Option<bool> {
        self.is_valid
    }

    pub fn calibrate(&mut self) {
        let mut res = false;

        let mut current_op_combo = InputOps::new(self.inputs.len() - 1);

        'find_valid_combo: loop {
            let mut combo_res = self.inputs[0];

            for (i, input) in self.inputs[1..].iter().enumerate() {
                let current_op = current_op_combo.get_op(i).unwrap();

                match current_op {
                    CalibrationOp::Add => combo_res += input,
                    CalibrationOp::Mul => combo_res *= input,
                    CalibrationOp::Concat => {
                        let digit_count = input.ilog10() + 1;
                        combo_res = combo_res * 10_u64.pow(digit_count) + input;
                    }
                }
            }

            if combo_res == self.ans {
                res = true;
                break 'find_valid_combo;
            }

            if current_op_combo.next() {
                break 'find_valid_combo;
            }
        }

        self.is_valid = Some(res);
    }
}
