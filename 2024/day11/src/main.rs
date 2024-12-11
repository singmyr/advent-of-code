use std::{collections::HashMap, fs};

fn parse_file(file_path: &str) -> Vec<u64> {
    fs::read_to_string(file_path).expect("Unable to read file").split(' ').map(|n| n.parse().unwrap()).collect()
}

fn blink(stones: Vec<u64>, blinks: u64) -> u64 {
    let mut stones_counter: HashMap<u64, u64> = HashMap::new();

    stones.iter().for_each(|s| {
        *stones_counter.entry(*s).or_insert(0) += 1;
    });

    let mut stones = stones_counter;

    for _ in 0..blinks {
        let mut fresh: HashMap<u64, u64> = HashMap::new();

        for (num, count) in &stones {
            if *num == 0 {
                *fresh.entry(1).or_insert(0) += count;
            } else if num.to_string().len() % 2 == 0 {
                let stone = num.to_string();
                let (first, second) = stone.split_at(stone.len()/2);
                let first: u64 = first.parse().unwrap();
                let second: u64 = second.parse().unwrap();
                *fresh.entry(first).or_insert(0) += count;
                *fresh.entry(second).or_insert(0) += count
            } else {
                *fresh.entry(*num*2024).or_insert(0) += count;
            }
        }

        stones = fresh;
    }

    stones.into_iter().fold(0, |acc, value| acc+value.1)
}

fn part1(input_file: &str) -> u64 {
    let stones = parse_file(input_file);

    blink(stones, 25)
}

fn part2(input_file: &str) -> u64 {
    let stones = parse_file(input_file);

    blink(stones, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1("sample.txt"), 55312);
    }

    #[test]
    fn test_part1_real() {
        assert_eq!(part1("input.txt"), 207683);
    }

    #[test]
    fn test_part2_sample() {
        assert_eq!(part2("sample.txt"), 65601038650482);
    }

    #[test]
    fn test_part2_real() {
        assert_eq!(part2("input.txt"), 244782991106220);
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
