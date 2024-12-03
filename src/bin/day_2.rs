use std::{fs, io};

fn main() {
    // let report = Report::new(&[0, 1, 2, 3, 4, 5]);
    // let report = Report::new(&[0, 1, 7, 3, 4, 5]);
    // let report = Report::new(&[0, 1, 2, 3, 2, 5]);
    // let report = Report::new(&[0, 0, 2, 3, 2, 5]);
    // let report = Report::new(&[1, 0, 2, 3, 2, 5]); // killer edge case

    let reports = get_reports().unwrap();

    let safe_report_count = reports.iter().filter(|report| report.is_safe()).count();
    println!("Safe Report Count: {}", safe_report_count);

    let kinda_safe_count = reports
        .iter()
        .filter(|report| report.is_safe_with_dampener())
        .count();

    println!("Kinda Safe Count: {}", kinda_safe_count);
}

fn get_reports() -> io::Result<Vec<Report>> {
    Ok(get_level_lists()?
        .iter()
        .map(|level_list| Report::new(level_list))
        .collect())
}

fn get_level_lists() -> io::Result<Vec<Vec<i64>>> {
    let buf = fs::read_to_string("Input_Day_2")?;

    Ok(buf
        .lines()
        .filter(|line| !line.is_empty())
        .map(str::split_whitespace)
        .map(|split| {
            split
                .map(|level| level.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect())
}

fn generate_report_safety(levels: &[i64]) -> ReportSafety {
    let mut diffs = vec![0; levels.len() - 1];

    for i in 1..levels.len() {
        diffs[i - 1] = levels[i] - levels[i - 1];
    }

    if are_diffs_valid(&diffs) {
        ReportSafety::Safe
    } else {
        for i in 0..levels.len() {
            let levels = [&levels[..i], &levels[(i + 1)..]].concat();

            let mut diffs = vec![0; levels.len() - 1];

            for i in 1..levels.len() {
                diffs[i - 1] = levels[i] - levels[i - 1];
            }

            if are_diffs_valid(&diffs) {
                return ReportSafety::SafeWithDampen;
            }
        }

        ReportSafety::Unsafe
    }
}

fn are_diffs_valid(diffs: &[i64]) -> bool {
    let mut unchanged_count = 0;
    let mut pos_count = 0;
    let mut neg_count = 0;
    let mut out_of_bounds_count = 0;

    for diff in diffs {
        match *diff {
            0 => {
                unchanged_count += 1;
            }
            1.. => {
                pos_count += 1;
            }
            ..0 => {
                neg_count += 1;
            }
        };

        match diff.abs() {
            1..4 => (),
            _ => out_of_bounds_count += 1,
        };
    }

    let different_trends = pos_count > 0 && neg_count > 0;

    !(out_of_bounds_count + unchanged_count > 0 || different_trends)
}

#[derive(Eq, PartialEq)]
enum ReportSafety {
    Safe,
    SafeWithDampen,
    Unsafe,
}

struct Report {
    levels: Vec<i64>,
    safety: ReportSafety,
}

impl Report {
    pub fn new(levels: &[i64]) -> Self {
        let safety = generate_report_safety(levels);

        Self {
            levels: levels.to_vec(),
            safety,
        }
    }

    fn is_safe(&self) -> bool {
        matches!(self.safety, ReportSafety::Safe)
    }

    fn is_safe_with_dampener(&self) -> bool {
        !matches!(self.safety, ReportSafety::Unsafe)
    }
}
