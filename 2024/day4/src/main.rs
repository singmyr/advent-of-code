use std::{fs::File, io::{BufRead, BufReader}};

fn parse_file(file_path: &str) -> Vec<Vec<char>> {
    let file: File = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Unable to read file: {}", err);
            std::process::exit(1);
        },
    };
    let reader = BufReader::new(file);

    reader.lines().map(|line| {
        line.unwrap().chars().collect()
    }).collect()
}

enum SearchDirection {
    North,
    NorthEast,
    NorthWest,
    East,
    South,
    SouthEast,
    SouthWest,
    West,
}

fn find_word(matrix: &Vec<Vec<char>>, start_x: usize, start_y: usize, direction: SearchDirection) -> bool {
    match direction {
        SearchDirection::North => {
            if start_y >= 3 {
                return matrix[start_y][start_x] == 'X' &&
                    matrix[start_y-1][start_x] == 'M' &&
                    matrix[start_y-2][start_x] == 'A' &&
                    matrix[start_y-3][start_x] == 'S';
            }

            false
        },
        SearchDirection::NorthEast => {
            if start_y >= 3 && start_x < matrix[start_y].len() - 3 {
                return matrix[start_y][start_x] == 'X' &&
                    matrix[start_y-1][start_x+1] == 'M' &&
                    matrix[start_y-2][start_x+2] == 'A' &&
                    matrix[start_y-3][start_x+3] == 'S';
            }

            false
        },
        SearchDirection::NorthWest => {
            if start_y >= 3 && start_x >= 3 {
                return matrix[start_y][start_x] == 'X' &&
                    matrix[start_y-1][start_x-1] == 'M' &&
                    matrix[start_y-2][start_x-2] == 'A' &&
                    matrix[start_y-3][start_x-3] == 'S';
            }

            false
        },
        SearchDirection::East => {
            if start_x < matrix[start_y].len() - 3 {
                return matrix[start_y][start_x] == 'X' &&
                    matrix[start_y][start_x+1] == 'M' &&
                    matrix[start_y][start_x+2] == 'A' &&
                    matrix[start_y][start_x+3] == 'S';
            }

            false
        },
        SearchDirection::South => {
            if start_y < matrix[start_y].len() - 3 {
                return matrix[start_y][start_x] == 'X' &&
                    matrix[start_y+1][start_x] == 'M' &&
                    matrix[start_y+2][start_x] == 'A' &&
                    matrix[start_y+3][start_x] == 'S';
            }

            false
        },
        SearchDirection::SouthEast => {
            if start_y < matrix[start_y].len() - 3 && start_x < matrix[start_y].len() - 3 {
                return matrix[start_y][start_x] == 'X' &&
                    matrix[start_y+1][start_x+1] == 'M' &&
                    matrix[start_y+2][start_x+2] == 'A' &&
                    matrix[start_y+3][start_x+3] == 'S';
            }

            false
        },
        SearchDirection::SouthWest => {
            if start_y < matrix.len() - 3 && start_x >= 3 {
                return matrix[start_y][start_x] == 'X' &&
                    matrix[start_y+1][start_x-1] == 'M' &&
                    matrix[start_y+2][start_x-2] == 'A' &&
                    matrix[start_y+3][start_x-3] == 'S';
            }

            false
        },
        SearchDirection::West => {
            if start_x >= 3 {
                return matrix[start_y][start_x] == 'X' &&
                    matrix[start_y][start_x-1] == 'M' &&
                    matrix[start_y][start_x-2] == 'A' &&
                    matrix[start_y][start_x-3] == 'S';
            }

            false
        },
    }
}

fn part1(input_file: &str) -> i64 {
    let matrix = parse_file(input_file);

    let mut occurrences = 0;

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            if matrix[y][x] == 'X' {
                if find_word(&matrix, x, y, SearchDirection::North) {
                    occurrences += 1;
                }
                if find_word(&matrix, x, y, SearchDirection::NorthEast) {
                    occurrences += 1;
                }
                if find_word(&matrix, x, y, SearchDirection::NorthWest) {
                    occurrences += 1;
                }
                if find_word(&matrix, x, y, SearchDirection::East) {
                    occurrences += 1;
                }
                if find_word(&matrix, x, y, SearchDirection::South) {
                    occurrences += 1;
                }
                if find_word(&matrix, x, y, SearchDirection::SouthEast) {
                    occurrences += 1;
                }
                if find_word(&matrix, x, y, SearchDirection::SouthWest) {
                    occurrences += 1;
                }
                if find_word(&matrix, x, y, SearchDirection::West) {
                    occurrences += 1;
                }
            }
        }
    }

    occurrences
}

fn find_x(matrix: &Vec<Vec<char>>, start_x: usize, start_y: usize) -> bool {
    if start_x > 0 && start_y > 0 && start_x < matrix[start_y].len() - 1 && start_y < matrix.len() - 1 {
        return
            matrix[start_y][start_x] == 'A' &&
            (
                (
                    matrix[start_y-1][start_x-1] == 'M' &&
                    matrix[start_y-1][start_x+1] == 'S' &&
                    matrix[start_y+1][start_x-1] == 'M' &&
                    matrix[start_y+1][start_x+1] == 'S'
                )
                ||
                (
                    matrix[start_y-1][start_x-1] == 'S' &&
                    matrix[start_y-1][start_x+1] == 'M' &&
                    matrix[start_y+1][start_x-1] == 'S' &&
                    matrix[start_y+1][start_x+1] == 'M'
                )
                ||
                (
                    matrix[start_y-1][start_x-1] == 'M' &&
                    matrix[start_y-1][start_x+1] == 'M' &&
                    matrix[start_y+1][start_x-1] == 'S' &&
                    matrix[start_y+1][start_x+1] == 'S'
                )
                ||
                (
                    matrix[start_y-1][start_x-1] == 'S' &&
                    matrix[start_y-1][start_x+1] == 'S' &&
                    matrix[start_y+1][start_x-1] == 'M' &&
                    matrix[start_y+1][start_x+1] == 'M'
                )
            );
    }

    false
}

fn part2(input_file: &str) -> i64 {
    let matrix = parse_file(input_file);

    let mut occurrences = 0;

    for y in 1..matrix.len()-1 {
        for x in 1..matrix[y].len()-1 {
            if matrix[y][x] == 'A' {
                if find_x(&matrix, x, y) {
                    occurrences += 1;
                }
            }
        }
    }

    occurrences
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1("sample.txt"), 18);
    }

    #[test]
    fn test_part1_real() {
        assert_eq!(part1("input.txt"), 2560);
    }

    #[test]
    fn test_part2_sample() {
        assert_eq!(part2("sample.txt"), 9);
    }

    #[test]
    fn test_part2_real() {
        assert_eq!(part2("input.txt"), 1910);
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
