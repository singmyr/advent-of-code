use std::{fs::File, io::{BufRead, BufReader}};

struct Robot {
    x_pos: usize,
    y_pos: usize,
    x_vel: i64,
    y_vel: i64,
}

fn parse_file(file_path: &str) -> Vec<Robot> {
    let mut robots: Vec<Robot> = Vec::new();

    let file: File = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Unable to read file: {}", err);
            std::process::exit(1);
        },
    };
    let reader = BufReader::new(file);

    reader.lines().for_each(|line| {
        let parts: Vec<Vec<i64>> = line.unwrap().split_whitespace().map(|r| {
            r[2..].split(',').map(|d| {
                d.trim().parse().unwrap()
            }).collect()
        }).collect();

        robots.push(Robot {
            x_pos: parts[0][0] as usize,
            y_pos: parts[0][1] as usize,
            x_vel: parts[1][0],
            y_vel: parts[1][1],
        });
    });

    robots
}

fn calc_safety_factor(width: usize, height: usize, robots: &Vec<Robot>) -> u64 {
    let mut safety_factors: Vec<u64> = vec![0; 4];

    let quadrant_width = ((width / 2) as f64).floor() as usize;
    let quadrant_height = ((height / 2) as f64).floor() as usize;

    let borderlands_x_start = quadrant_width;
    let borderlands_x_end = width - quadrant_width - 1;
    let borderlands_y_start = quadrant_height;
    let borderlands_y_end = height - quadrant_height - 1;

    for robot in robots.iter() {
        if robot.x_pos >= borderlands_x_start && robot.x_pos <= borderlands_x_end {
            continue;
        }

        if robot.y_pos >= borderlands_y_start && robot.y_pos <= borderlands_y_end {
            continue;
        }

        let index = if robot.x_pos < borderlands_x_start {
            if robot.y_pos < borderlands_y_start {
                0
            } else {
                1
            }
        } else {
            if robot.y_pos < borderlands_y_start {
                2
            } else {
                3
            }
        };

        safety_factors[index] += 1;
    }

    safety_factors[0] * safety_factors[1] * safety_factors[2] * safety_factors[3]
}

fn part1(input_file: &str, width: usize, height: usize) -> u64 {
    let mut robots = parse_file(input_file);

    for _ in 0..100 {
        for (_, robot) in robots.iter_mut().enumerate() {
            let mut x_pos_new: i64 = robot.x_pos as i64 + robot.x_vel;
            let mut y_pos_new: i64 = robot.y_pos as i64 + robot.y_vel;

            if x_pos_new < 0 {
                x_pos_new = width as i64 - x_pos_new.abs();
            } else if x_pos_new > width as i64 - 1 {
                x_pos_new = x_pos_new - width as i64;
            }

            if y_pos_new < 0 {
                y_pos_new = height as i64 - y_pos_new.abs();
            } else if y_pos_new > height as i64 - 1 {
                y_pos_new = y_pos_new - height as i64;
            }

            robot.x_pos = x_pos_new.abs() as usize;
            robot.y_pos = y_pos_new.abs() as usize;
        }
    }

    calc_safety_factor(width, height, &robots)
}

fn get_maxed_tile_count(robots: &Vec<Robot>, width: usize, height: usize) -> usize {
    // let mut tiles: Vec<bool> = vec![false; width * height];
    // let mut connections_map: Vec<usize> = vec![0; width * height];

    let mut tiles: Vec<Vec<bool>> = Vec::with_capacity(height);
    for _ in 0..height {
        tiles.push(vec![false; width]);
    }

    for robot in robots {
        tiles[robot.y_pos][robot.x_pos] = true;
    }

    let mut maxed_tiles = 0;
    for (y, row) in tiles.iter().enumerate() {
        for (x, have_robot) in row.iter().enumerate() {
            if !have_robot {
                continue;
            }
            let mut count = 0;

            if x > 0 && row[x-1] {
                count += 1;
            }
            if x < width - 1 && row[x+1] {
                count += 1;
            }

            if y > 0 && tiles[y-1][x] {
                count += 1;
            }
            if y < height - 1 && tiles[y+1][x] {
                count += 1;
            }

            // println!("{count}");
            if count >= 4 {
                maxed_tiles += 1;
            }
        }
    }

    maxed_tiles
}

fn part2(input_file: &str, width: usize, height: usize) -> u64 {
    let mut robots = parse_file(input_file);

    let mut i = 0;
    loop {
        i += 1;
        for (_, robot) in robots.iter_mut().enumerate() {
            let mut x_pos_new: i64 = robot.x_pos as i64 + robot.x_vel;
            let mut y_pos_new: i64 = robot.y_pos as i64 + robot.y_vel;

            if x_pos_new < 0 {
                x_pos_new = width as i64 - x_pos_new.abs();
            } else if x_pos_new > width as i64 - 1 {
                x_pos_new = x_pos_new - width as i64;
            }

            if y_pos_new < 0 {
                y_pos_new = height as i64 - y_pos_new.abs();
            } else if y_pos_new > height as i64 - 1 {
                y_pos_new = y_pos_new - height as i64;
            }

            robot.x_pos = x_pos_new.abs() as usize;
            robot.y_pos = y_pos_new.abs() as usize;
        }

        let maxed = get_maxed_tile_count(&robots, width, height);
        if maxed > 160 {
            return i;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1("sample.txt", 11, 7), 12);
    }

    #[test]
    fn test_part1_real() {
        assert_eq!(part1("input.txt", 101, 103), 219150360);
    }

    #[test]
    fn test_part2_real() {
        assert_eq!(part2("input.txt", 101, 103), 8053);
    }
}

fn main() {
    use std::time::Instant;

    let start = Instant::now();
    let answer = part1("input.txt", 101, 103);
    println!("Answer to part 1: {answer} ({}µs)", start.elapsed().as_micros());

    let start = Instant::now();
    let answer = part2("input.txt", 101, 103);
    println!("Answer to part 2: {answer} ({}µs)", start.elapsed().as_micros());
}
