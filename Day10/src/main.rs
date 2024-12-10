static DIRECTIONS: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

#[derive(Clone)]
struct Pos {
    x: i32,
    y: i32,
}

fn get_value_at(map: &[Vec<char>], x: i32, y: i32) -> Option<i32> {
    if x >= 0 && x < map[0].len() as i32 && y >= 0 && y < map.len() as i32 {
        let char = map[y as usize][x as usize];
        if char.is_ascii_digit() {
            return Some(char.to_digit(10).unwrap() as i32);
        }
    }
    None
}

fn find_paths(
    map: &mut Vec<Vec<char>>,
    path_pos: Pos,
    target_value: i32,
    lock_completed_pathes: bool,
) -> i32 {
    let mut count = 0;

    for &(dx, dy) in &DIRECTIONS {
        let new_x = path_pos.x + dx;
        let new_y = path_pos.y + dy;

        if let Some(value) = get_value_at(map, new_x, new_y) {
            if target_value == 8 && value == 9 {
                if lock_completed_pathes {
                    map[new_y as usize][new_x as usize] = '.';
                }
                count += 1;
            } else if value == target_value + 1 {
                count += find_paths(
                    map,
                    Pos { x: new_x, y: new_y },
                    value,
                    lock_completed_pathes,
                );
            }
        }
    }

    count
}

fn main() {
    let file_content = include_str!("../subject/input.txt");
    let map: Vec<Vec<char>> = file_content
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut part1_count = 0;
    let mut part2_count = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == '0' {
                let mut clone = map.clone();
                let pos = Pos {
                    x: j as i32,
                    y: i as i32,
                };

                part1_count += find_paths(&mut clone, pos.clone(), 0, true);

                clone = map.clone();
                part2_count += find_paths(&mut clone, pos.clone(), 0, false);
            }
        }
    }

    println!("Part 1: {}", part1_count);
    println!("Part 2: {}", part2_count);
}
