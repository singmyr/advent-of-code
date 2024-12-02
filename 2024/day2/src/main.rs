use std::{fs::File, io::{BufRead, BufReader}};

fn parse_file(file_path: &str) -> Vec<Vec<i64>> {
    let mut reports: Vec<Vec<i64>> = Vec::new();
    let file: File = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Unable to read file: {}", err);
            std::process::exit(1);
        },
    };
    let reader = BufReader::new(file);

    reader.lines().for_each(|line| {
        reports.push(line.unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect());
    });

    reports
}

enum ReportKind {
    Increasing,
    Decreasing,
}

fn check(report: &Vec<i64>) -> bool {
    let mut is_safe = true;
    let kind: ReportKind = if report[1] > report[0] {
        ReportKind::Increasing
    } else if report[1] < report[0] {
        ReportKind::Decreasing
    } else {
        return false;
    };

    for i in 1..report.len() {
        let level = report[i];
        let prev_level = report[i-1];

        let diff = match kind {
            ReportKind::Increasing => level - prev_level,
            ReportKind::Decreasing => prev_level - level,
        };

        if diff < 1 || diff > 3 {
            is_safe = false;
            break;
        }
    }

    is_safe
}

fn part1(input_file: &str) -> i64 {
    let reports = parse_file(input_file);
    let mut safe: i64 = 0;

    for report in reports {
        if check(&report) {
            safe += 1;
        }
    }

    safe
}

fn part2(input_file: &str) -> i64 {
    let reports = parse_file(input_file);
    let mut safe: i64 = 0;

    for report in reports {
        let mut is_safe = check(&report);

        if !is_safe {
            for i in 0..report.len() {
                let mut r = report.clone();
                r.remove(i);
                if check(&r) {
                    is_safe = true;
                    break;
                }
            }
        }
        

        if is_safe {
            safe += 1;
        }
    }

    safe
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1("sample.txt"), 2);
    }

    #[test]
    fn test_part1_real() {
        assert_eq!(part1("input.txt"), 411);
    }

    #[test]
    fn test_part2_sample() {
        assert_eq!(part2("sample.txt"), 4);
    }

    #[test]
    fn test_part2_real() {
        assert_eq!(part2("input.txt"), 465);
    }
}

fn main() {
    use std::time::Instant;

    let start = Instant::now();
    let answer = part1("input.txt");
    println!("Answer to part 1: {answer} ({}µs)", start.elapsed().as_micros());

    let start = Instant::now();
    let answer = part2("input.txt");
    println!("Answer to part 2: {answer} ({}µs)", start.elapsed().as_micros());
}
