use std::{cmp::Ordering, fs::File, io::{BufRead, BufReader}};

fn parse_file(file_path: &str) -> (Vec<i64>, Vec<i64>) {
    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();
    let file: File = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Unable to read file: {}", err);
            std::process::exit(1);
        },
    };
    let reader = BufReader::new(file);

    reader.lines().for_each(|line| {
        let parts: Vec<i64> = line.unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
        left.push(parts[0]);
        right.push(parts[1]);    
    });

    (left, right)
}

fn part1(input_file: &str) -> i64 {
    let (mut left, mut right) = parse_file(input_file);
    left.sort();
    right.sort();

    let mut diff: i64 = 0;
    for (i, l) in left.iter().enumerate() {
        diff += (l-right[i]).abs()
    }

    diff
}

fn part2(input_file: &str) -> i64 {
    let (mut left, mut right) = parse_file(input_file);
    for (_, l) in left.iter_mut().enumerate() {
        // Find how many times it's present in the right
        let mut count: i64 = 0;
        for (_, r) in right.iter_mut().enumerate() {
            match l.cmp(&r) {
                Ordering::Equal => {
                    count += 1;
                },
                _ => {},
            }
        }
        *l *= count;
    }

    let mut sum: i64 = 0;
    for (_, l) in left.iter().enumerate() {
        sum += l;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1("sample.txt"), 11);
    }

    #[test]
    fn test_part1_real() {
        assert_eq!(part1("input.txt"), 1830467);
    }

    #[test]
    fn test_part2_sample() {
        assert_eq!(part2("sample.txt"), 31);
    }

    #[test]
    fn test_part2_real() {
        assert_eq!(part2("input.txt"), 26674158);
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
