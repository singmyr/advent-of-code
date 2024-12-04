use std::fs;
use regex::Regex;

fn parse_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Unable to read file")
}

fn calculate(instructions: &String) -> i64 {
    use regex::Regex;

    let mut sum: i64 = 0;

    let rgx = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    for rgx_match in rgx.captures_iter(instructions) {
        let part_one: i64 = rgx_match[1].parse().unwrap();
        let part_two: i64 = rgx_match[2].parse().unwrap();

        sum += part_one * part_two;
    }

    sum
}

fn part1(input_file: &str) -> i64 {
    let memory = parse_file(input_file);

    calculate(&memory)
}

fn part2(input_file: &str) -> i64 {
    let memory = parse_file(input_file);

    let rgx = Regex::new(r"don\'t\(\)(?:.|\n)*?do\(\)").unwrap();

    let clean = rgx.replace_all(&memory, "").to_string();
    
    calculate(&clean)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1("sample.txt"), 161);
    }

    #[test]
    fn test_part1_real() {
        assert_eq!(part1("input.txt"), 174960292);
    }

    #[test]
    fn test_part2_sample() {
        assert_eq!(part2("sample2.txt"), 48);
    }

    #[test]
    fn test_part2_real() {
        assert_eq!(part2("input.txt"), 56275602);
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
