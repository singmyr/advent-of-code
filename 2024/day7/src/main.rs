use std::{fs::File, io::{BufRead, BufReader}};

struct EquationNode {
    target_sum: u64,
    current_sum: u64,
    numbers: Vec<u64>,
}

impl EquationNode {
    fn new(target_sum: u64, current_sum: u64, numbers: &Vec<u64>) -> EquationNode {
        let node = EquationNode {
            target_sum,
            current_sum,
            numbers: numbers.clone(),
        };

        node
    }

    fn get_next_number(&mut self) -> u64 {
        let number = *self.numbers.get(0).unwrap();
        self.numbers.remove(0);

        number
    }

    fn clone(&self) -> EquationNode {
        Self::new(self.target_sum, self.current_sum, &self.numbers.clone())
    }
}

fn possibly_true1(node: &mut EquationNode, ) -> bool {
    if node.numbers.is_empty() {
        return false;
    }

    let current_number = node.get_next_number();
    let add_sum = node.current_sum + current_number;
    let mul_sum = node.current_sum * current_number;

    if add_sum == node.target_sum || mul_sum == node.target_sum {
        return true;
    }

    node.current_sum = add_sum;
    if possibly_true1(&mut node.clone()) {
        return true;
    }

    node.current_sum = mul_sum;
    if possibly_true1(&mut node.clone()) {
        return true;
    }

    false
}

fn possibly_true2(node: &mut EquationNode, ) -> bool {
    if node.numbers.is_empty() {
        return false;
    }

    let current_number = node.get_next_number();
    let add_sum = node.current_sum + current_number;
    let mul_sum = node.current_sum * current_number;
    let mut con_sum = node.current_sum.to_string();
    con_sum.push_str(current_number.to_string().as_str());
    let con_sum: u64 = con_sum.parse().unwrap();

    if add_sum == node.target_sum || mul_sum == node.target_sum || con_sum == node.target_sum {
        return true;
    }

    node.current_sum = add_sum;
    if possibly_true2(&mut node.clone()) {
        return true;
    }

    node.current_sum = mul_sum;
    if possibly_true2(&mut node.clone()) {
        return true;
    }

    node.current_sum = con_sum;
    if possibly_true2(&mut node.clone()) {
        return true;
    }

    false
}

fn parse_file(file_path: &str) -> Vec<EquationNode> {
    let mut nodes: Vec<EquationNode> = Vec::new();
    let file: File = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Unable to read file: {}", err);
            std::process::exit(1);
        },
    };
    let reader = BufReader::new(file);

    reader.lines().for_each(|line| {
        let line = line.unwrap();
        let [sum, numbers]: [&str; 2] = line.split(':').collect::<Vec<&str>>().try_into().unwrap();
        let sum: u64 = sum.parse().unwrap();
        let numbers: Vec<u64> = numbers.trim().split_whitespace().map(|n| n.parse().unwrap()).collect();

        nodes.push(EquationNode::new(sum, 0, &numbers));
    });

    nodes
}


fn part1(input_file: &str) -> u64 {
    let mut nodes = parse_file(input_file);

    let mut sum: u64 = 0;

    for (_, node) in nodes.iter_mut().enumerate() {
        node.current_sum = node.get_next_number();
        if possibly_true1(node) {
            sum += node.target_sum;
        }
    }

    sum
}

fn part2(input_file: &str) -> u64 {
    let mut nodes = parse_file(input_file);

    let mut sum: u64 = 0;

    for (_, node) in nodes.iter_mut().enumerate() {
        node.current_sum = node.get_next_number();
        if possibly_true2(node) {
            sum += node.target_sum;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1("sample.txt"), 3749);
    }

    #[test]
    fn test_part1_real() {
        assert_eq!(part1("input.txt"), 5512534574980);
    }

    #[test]
    fn test_part2_sample() {
        assert_eq!(part2("sample.txt"), 11387);
    }

    #[test]
    fn test_part2_real() {
        assert_eq!(part2("input.txt"), 328790210468594);
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
