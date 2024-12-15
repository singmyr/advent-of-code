use std::{fs::File, io::{BufRead, BufReader}};

fn parse_file(file_path: &str) -> (Vec<Vec<char>>, Vec<char>) {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut movement: Vec<char> = Vec::new();

    let file: File = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Unable to read file: {}", err);
            std::process::exit(1);
        },
    };
    let reader = BufReader::new(file);

    let mut parsing_movement = false;
    reader.lines().for_each(|line| {
        let line = line.unwrap();
        if line.is_empty() {
            parsing_movement = true;
            return;
        }

        if parsing_movement {
            // Parsing movement
            let mut row: Vec<char> = line.chars().collect();
            movement.append(&mut row);
        } else {
            // Parsing map
            map.push(line.chars().collect());
        }

        // let parts: Vec<u64> = line.split(&['+', ','][..]).filter_map(|s| s.parse().ok()).collect();
    });

    (map, movement)
}

fn get_robot_pos(map: &Vec<Vec<char>>, player_char: char) -> Option<(usize, usize)> {
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == player_char {
                return Some((x, y))
            }
        }
    }

    None
}

fn can_move(map: &Vec<Vec<char>>, dir: char, x: usize, y: usize) -> bool {
    let movable: bool;
    let mut cur_x = x;
    let mut cur_y = y;
    loop {
        match dir {
            '^' => cur_y -= 1,
            'v' => cur_y += 1,
            '<' => cur_x -= 1,
            '>' => cur_x += 1,
            _ => {}
        }
        if map[cur_y][cur_x] == '#' {
            movable = false;
            break;
        } else if map[cur_y][cur_x] == '.' {
            movable = true;
            break;
        }
    }

    movable
}

fn can_move2(map: &Vec<Vec<char>>, dir: char, x: usize, y: usize) -> bool {
    let cur_y = match dir {
        '^' => y - 1,
        'v' => y + 1,
        _ => y,
    };
    let cur_x = match dir {
        '<' => x - 1,
        '>' => x + 1,
        _ => x,
    };

    let tile = map[cur_y][cur_x];
    match dir {
        '^' => {
            if tile == '[' {
                return can_move2(map, dir, cur_x, cur_y) && can_move2(map, dir, cur_x+1, cur_y);
            } else if tile == ']' {
                return can_move2(map, dir, cur_x, cur_y) && can_move2(map, dir, cur_x-1, cur_y);
            }
            return tile == '.';
        },
        'v' => {
            if tile == '[' {
                return can_move2(map, dir, cur_x, cur_y) && can_move2(map, dir, cur_x+1, cur_y);
            } else if tile == ']' {
                return can_move2(map, dir, cur_x, cur_y) && can_move2(map, dir, cur_x-1, cur_y);
            }
            return tile == '.';
        },
        '<' => {
            if tile == '.' {
                return true;
            }
            if tile == '#' {
                return false;
            }
            return can_move2(map, dir, cur_x-1, cur_y);
        },
        '>' => {
            if tile == '.' {
                return true;
            }
            if tile == '#' {
                return false;
            }
            return can_move2(map, dir, cur_x+1, cur_y);
        },
        _ => false,
    }
}

fn find_dot_in_path(map: &Vec<Vec<char>>, dir: char, x: usize, y: usize) -> (bool, usize, usize) {
    if map[y][x] == '.' {
        return (true, x, y);
    }

    match dir {
        '^' => return find_dot_in_path(map, dir, x, y-1),
        'v' => return find_dot_in_path(map, dir, x, y+1),
        '<' => return find_dot_in_path(map, dir, x-1, y),
        '>' => return find_dot_in_path(map, dir, x+1, y),
        _ => return (false, 0, 0)
    }
}

fn move_robot(map: &mut Vec<Vec<char>>, dir: char, x: &mut usize, y: &mut usize) {
    match dir {
        '^' => {
            if map[*y-1][*x] == 'O' {
                let (possible, new_x, new_y) = find_dot_in_path(map, dir, *x, *y-1);
                if possible {
                    map[new_y][new_x] = 'O';
                }
            }
            map[*y][*x] = '.';
            *y -= 1;
        },
        'v' => {
            if map[*y+1][*x] == 'O' {
                let (possible, new_x, new_y) = find_dot_in_path(map, dir, *x, *y+1);
                if possible {
                    map[new_y][new_x] = 'O';
                }
            }
            map[*y][*x] = '.';
            *y += 1;
        },
        '<' => {
            if map[*y][*x-1] == 'O' {
                let (possible, new_x, new_y) = find_dot_in_path(map, dir, *x-1, *y);
                if possible {
                    map[new_y][new_x] = 'O';
                }
            }
            map[*y][*x] = '.';
            *x -= 1;
        },
        '>' => {
            if map[*y][*x+1] == 'O' {
                let (possible, new_x, new_y) = find_dot_in_path(map, dir, *x+1, *y);
                if possible {
                    map[new_y][new_x] = 'O';
                }
            }
            map[*y][*x] = '.';
            *x += 1;
        },
        _ => {}
    }
    map[*y][*x] = '@';
}

fn move_robot2(map: &mut Vec<Vec<char>>, dir: char, x: usize, y: usize) {
    match dir {
        '^' => {
            let tile = map[y][x];
            if tile == '@' {
                if map[y-1][x] != '.' {
                    move_robot2(map, dir, x, y-1);
                }
                map[y-1][x] = '@';
                map[y][x] = '.';
            } else if tile == ']' {
                move_robot2(map, dir, x, y-1);
                move_robot2(map, dir, x-1, y);
            } else if tile == '[' {
                if map[y-1][x] != '.' {
                    move_robot2(map, dir, x, y-1);
                }
                if map[y-1][x+1] != '.' {
                    move_robot2(map, dir, x+1, y-1);
                }
                map[y-1][x] = '[';
                map[y-1][x+1] = ']';
                map[y][x] = '.';
                map[y][x+1] = '.';
            }
        },
        'v' => {
            let tile = map[y][x];
            if tile == '@' {
                if map[y+1][x] != '.' {
                    move_robot2(map, dir, x, y+1);
                }
                map[y+1][x] = '@';
                map[y][x] = '.';
            } else if tile == ']' {
                move_robot2(map, dir, x, y+1);
                move_robot2(map, dir, x-1, y);
            } else if tile == '[' {
                if map[y+1][x] != '.' {
                    move_robot2(map, dir, x, y+1);
                }
                if map[y+1][x+1] != '.' {
                    move_robot2(map, dir, x+1, y+1);
                }
                map[y+1][x] = '[';
                map[y+1][x+1] = ']';
                map[y][x] = '.';
                map[y][x+1] = '.';
            }
        },
        '<' => {
            let pos = map[y][..x].iter().rposition(|&c| c == '.').unwrap();
            map[y].remove(pos);
            map[y].insert(x, '.');
        },
        '>' => {
            let pos = map[y][x..].iter().position(|&c| c == '.').unwrap();
            map[y].remove(x+pos);
            map[y].insert(x, '.');
        },
        _ => {}
    }
}

fn resize_map(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_map: Vec<Vec<char>> = Vec::new();

    for row in map {
        let mut r: Vec<char> = Vec::new();

        for c in row {
            match c {
                '#' => {
                    r.push('#');
                    r.push('#');
                },
                'O' => {
                    r.push('[');
                    r.push(']');
                },
                '.' => {
                    r.push('.');
                    r.push('.');
                },
                '@' => {
                    r.push('@');
                    r.push('.');
                },
                _ => {}
            }
        }

        new_map.push(r);
    }

    new_map
}

fn part1(input_file: &str) -> usize {
    let (mut map, movement) = parse_file(input_file);
    let (mut robot_x, mut robot_y) = get_robot_pos(&map, '@').expect("Unable to parse robot position");
    
    for dir in movement {
        if can_move(&map, dir, robot_x, robot_y) {
            move_robot(&mut map, dir, &mut robot_x, &mut robot_y);
        }
    }

    let mut sum: usize = 0;

    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'O' {
                sum += (y * 100) + x;
            }
        }
    }

    sum
}

fn part2(input_file: &str) -> usize {
    let (map, movement) = parse_file(input_file);
    let mut map = resize_map(map);
    let (mut robot_x, mut robot_y) = get_robot_pos(&map, '@').expect("Unable to parse robot position");

    for dir in movement {
        let can_move = can_move2(&map, dir, robot_x, robot_y);

        if can_move {
            move_robot2(&mut map, dir, robot_x, robot_y);
            match dir {
                '^' => robot_y -= 1,
                'v' => robot_y += 1,
                '<' => robot_x -= 1,
                '>' => robot_x += 1,
                _ => {},
            };
        }
    }

    let mut sum: usize = 0;

    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '[' {
                sum += (y * 100) + x;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sample_file() {
        let map: Vec<Vec<char>> = vec![
            vec!['#','#','#','#','#','#','#','#'],
            vec!['#','.','.','O','.','O','.','#'],
            vec!['#','#','@','.','O','.','.','#'],
            vec!['#','.','.','.','O','.','.','#'],
            vec!['#','.','#','.','O','.','.','#'],
            vec!['#','.','.','.','O','.','.','#'],
            vec!['#','.','.','.','.','.','.','#'],
            vec!['#','#','#','#','#','#','#','#']
        ];
        let movement: Vec<char> = vec!['<','^','^','>','>','>','v','v','<','v','>','>','v','<','<'];
        assert_eq!(parse_file("sample.txt"), (map, movement))
    }

    #[test]
    fn test_get_robot_pos() {
        let map: Vec<Vec<char>> = vec![
            vec!['#','#','#','#','#','#','#','#'],
            vec!['#','.','.','O','.','O','.','#'],
            vec!['#','#','@','.','O','.','.','#'],
            vec!['#','.','.','.','O','.','.','#'],
            vec!['#','.','#','.','O','.','.','#'],
            vec!['#','.','.','.','O','.','.','#'],
            vec!['#','.','.','.','.','.','.','#'],
            vec!['#','#','#','#','#','#','#','#']
        ];
        assert_eq!(get_robot_pos(&map, '@'), Some((2, 2)));
    }

    #[test]
    fn test_map_resize() {
        let map: Vec<Vec<char>> = vec![
            vec!['#','#','#','#','#','#','#'],
            vec!['#','.','.','.','#','.','#'],
            vec!['#','.','.','.','.','.','#'],
            vec!['#','.','.','O','O','@','#'],
            vec!['#','.','.','O','.','.','#'],
            vec!['#','.','.','.','.','.','#'],
            vec!['#','#','#','#','#','#','#'],
        ];
        assert_eq!(resize_map(map), vec![
            vec!['#','#','#','#','#','#','#','#','#','#','#','#','#','#'],
            vec!['#','#','.','.','.','.','.','.','#','#','.','.','#','#'],
            vec!['#','#','.','.','.','.','.','.','.','.','.','.','#','#'],
            vec!['#','#','.','.','.','.','[',']','[',']','@','.','#','#'],
            vec!['#','#','.','.','.','.','[',']','.','.','.','.','#','#'],
            vec!['#','#','.','.','.','.','.','.','.','.','.','.','#','#'],
            vec!['#','#','#','#','#','#','#','#','#','#','#','#','#','#'],
        ]);
    }

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1("sample.txt"), 2028);
    }

    #[test]
    fn test_part1_real() {
        assert_eq!(part1("input.txt"), 1457740);
    }

    #[test]
    fn test_part2_sample() {
        assert_eq!(part2("sample2.txt"), 9021);
    }

    #[test]
    fn test_part2_real() {
        assert_eq!(part2("input.txt"), 1467145);
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
