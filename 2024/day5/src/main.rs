use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};

fn parse_file(file_path: &str) -> (HashMap<u64, Vec<u64>>, Vec<Vec<u64>>) {
    let mut ordering_rules: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut updates: Vec<Vec<u64>> = Vec::new();
    let file: File = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Unable to read file: {}", err);
            std::process::exit(1);
        },
    };
    let reader = BufReader::new(file);

    let mut parsing_updates = false;
    reader.lines().for_each(|line| {
        let line = line.unwrap();
        if line.len() == 0 {
            parsing_updates = true;
            return;
        }

        if parsing_updates {
            updates.push(line.split(',').map(|c| c.parse().unwrap()).collect());
        } else {
            let data: Vec<u64> = line.split('|').map(|c| c.parse().unwrap()).collect();
            if !ordering_rules.contains_key(&data[0]) {
                ordering_rules.insert(data[0], Vec::new());
            }

            ordering_rules.get_mut(&data[0]).unwrap().push(data[1]);
        }
    });

    (ordering_rules, updates)
}

fn part1(input_file: &str) -> u64 {
    let (ordering_rules, updates) = parse_file(&input_file);
    
    let mut sum: u64 = 0;

    for update in updates {
        let mut correct = true;
        for (i, page) in update.iter().enumerate() {
            if ordering_rules.contains_key(&page) {
                let rules = ordering_rules.get(&page).unwrap();
                if i > 0 {
                    for n in 0..i {
                        if rules.contains(&update[n]) {
                            correct = false;
                        }
                    }
                }
            }
        }
        if correct {
            if update.len() % 2 > 0 {
                sum += update[update.len()/2];
            }
        }
    }

    sum
}

fn part2(input_file: &str) -> u64 {
    let (ordering_rules, mut updates) = parse_file(&input_file);
    
    let mut sum: u64 = 0;

    for (_, update) in updates.iter_mut().enumerate() {
        let mut corrected = false;
        loop {
            let mut correct = true;
            'update: for (i, page) in update.iter().enumerate() {
                if ordering_rules.contains_key(&page) {
                    let rules = ordering_rules.get(&page).unwrap();
                    if i > 0 {
                        for n in 0..i {
                            if rules.contains(&update[n]) {
                                correct = false;
                                corrected = true;
                                let val = update[n];
                                update.remove(n);
                                update.insert(i, val);
                                break 'update;
                            }
                        }
                    }
                }
            }

            if correct {
                break;
            }
        }
        if corrected {
            if update.len() % 2 > 0 {
                sum += update[update.len()/2];
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1("sample.txt"), 143);
    }

    #[test]
    fn test_part1_real() {
        assert_eq!(part1("input.txt"), 6949);
    }

    #[test]
    fn test_part2_sample() {
        assert_eq!(part2("sample.txt"), 123);
    }

    #[test]
    fn test_part2_real() {
        assert_eq!(part2("input.txt"), 4145);
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
