enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_distinct_positions(mut map: Vec<String>, mut x: usize, mut y: usize, mut dir: Direction) -> Option<i32> {
    let mut max_counter = 0;

    while y < map.len() && x < map[0].len() {
        match dir {
            Direction::Up => {
                if y == 0 {
                    break;
                }

                if map[y - 1].chars().nth(x).unwrap() != '#' {
                    map[y].replace_range(x..x + 1, "X");
                    y -= 1;
                } else {
                    dir = Direction::Right;
                }
            }
            Direction::Down => {
                if y + 1 >= map.len() {
                    break;
                }

                if map[y + 1].chars().nth(x).unwrap() != '#' {
                    map[y].replace_range(x..x + 1, "X");
                    y += 1;
                } else {
                    dir = Direction::Left;
                }
            }
            Direction::Left => {
                if x == 0 {
                    break;
                }

                if map[y].chars().nth(x - 1).unwrap() != '#' {
                    map[y].replace_range(x..x + 1, "X");
                    x -= 1;
                } else {
                    dir = Direction::Up;
                }
            }
            Direction::Right => {
                if x + 1 >= map[0].len() {
                    break;
                }

                if map[y].chars().nth(x + 1).unwrap() != '#' {
                    map[y].replace_range(x..x + 1, "X");
                    x += 1;
                } else {
                    dir = Direction::Down;
                }
            }
        }

        max_counter += 1;
        if max_counter > 7000 {
            return None;
        }
    }

    let mut count = 1;
    for line in map.iter() {
        count += line.chars().filter(|&c| c == 'X').count();
    }

    return Some(count as i32);
}

fn main() {
    let file_content = include_str!("../subject/input.txt");
    let mut map: Vec<String> = file_content.lines().map(|s| s.to_string()).collect();

    let mut x = 0;
    let mut y = 0;
    let mut dir = Direction::Up;
    for (i, line) in map.iter().enumerate() {
        if line.contains("^") {
            y = i;
            x = line.find("^").unwrap();
            dir = Direction::Up;
            break;
        } else if line.contains("v") {
            y = i;
            x = line.find("v").unwrap();
            dir = Direction::Down;
            break;
        } else if line.contains("<") {
            y = i;
            x = line.find("<").unwrap();
            dir = Direction::Left;
            break;
        } else if line.contains(">") {
            y = i;
            x = line.find(">").unwrap();
            dir = Direction::Right;
            break;
        }
    }

    let part1 = get_distinct_positions(map.clone(), x.clone(), y.clone(), dir).unwrap();
    println!("Part1: {}", part1);

    let mut part2 = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let mut remove = false;
            if map[i].chars().nth(j).unwrap() != '#' {
                remove = true;
            }

            map[i].replace_range(j..j + 1, "#");
            let count = get_distinct_positions(map.clone(), x, y, Direction::Up);
            if count.is_none() {
                part2 += 1;
            }

            if remove {
                map[i].replace_range(j..j + 1, ".");
            }
        }
    }

    println!("Part2: {}", part2);
}
