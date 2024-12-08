fn create_antinodes_part1(
    antenna_map: &Vec<&str>,
    antinode_map: &mut Vec<String>,
    x: i32,
    y: i32,
    antenna_char: char,
) {
    for (i, line) in antenna_map.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if i as i32 == y && j as i32 == x || c != antenna_char {
                continue;
            }

            let offset: (i32, i32) = (j as i32 - x, i as i32 - y);
            let new_pos: (i32, i32) = (j as i32 + offset.0, i as i32 + offset.1);
            if new_pos.0 < 0
                || new_pos.0 as usize >= line.len()
                || new_pos.1 < 0
                || new_pos.1 as usize >= antenna_map.len()
            {
                continue;
            }

            antinode_map[new_pos.1 as usize]
                .replace_range(new_pos.0 as usize..(new_pos.0 + 1) as usize, "#");
        }
    }
}

fn create_antinodes_part2(
    antenna_map: &Vec<&str>,
    antinode_map: &mut Vec<String>,
    x: i32,
    y: i32,
    antenna_char: char,
) {
    for (i, line) in antenna_map.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if i as i32 == y && j as i32 == x || c != antenna_char {
                if i as i32 == y && j as i32 == x {
                    antinode_map[i].replace_range(j..(j + 1), "#");
                }
                continue;
            }

            let offset: (i32, i32) = (j as i32 - x, i as i32 - y);
            let mut new_pos: (i32, i32) = (j as i32 + offset.0, i as i32 + offset.1);

            while new_pos.0 >= 0
                && line.len() > new_pos.0 as usize
                && new_pos.1 >= 0
                && antenna_map.len() > new_pos.1 as usize
            {
                antinode_map[new_pos.1 as usize]
                    .replace_range(new_pos.0 as usize..(new_pos.0 + 1) as usize, "#");

                new_pos = (new_pos.0 + offset.0, new_pos.1 + offset.1);
            }
        }
    }
}

fn main() {
    let file_content = include_str!("../subject/input.txt");
    let antenna_map: Vec<&str> = file_content.lines().collect();

    let mut antinode_map: Vec<String> = Vec::new();

    // Part 1
    for line in antenna_map.iter() {
        let mut new_line = String::new();
        for _ in 0..line.len() {
            new_line.push('.');
        }
        antinode_map.push(new_line);
    }

    for (i, antenna_line) in antenna_map.iter().enumerate() {
        for (j, antenna_char) in antenna_line.chars().enumerate() {
            if antenna_char == '.' {
                continue;
            }

            create_antinodes_part1(
                &antenna_map,
                &mut antinode_map,
                j as i32,
                i as i32,
                antenna_char,
            );
        }
    }

    let antinode_count = antinode_map.iter().fold(0, |acc, line| {
        acc + line.chars().filter(|c| *c == '#').count()
    });

    println!("Part 1: {}", antinode_count);

    // Part 2
    antinode_map.clear();
    for line in antenna_map.iter() {
        let mut new_line = String::new();
        for _ in 0..line.len() {
            new_line.push('.');
        }
        antinode_map.push(new_line);
    }

    for (i, antenna_line) in antenna_map.iter().enumerate() {
        for (j, antenna_char) in antenna_line.chars().enumerate() {
            if antenna_char == '.' {
                continue;
            }

            create_antinodes_part2(
                &antenna_map,
                &mut antinode_map,
                j as i32,
                i as i32,
                antenna_char,
            );
        }
    }

    let antinode_count = antinode_map.iter().fold(0, |acc, line| {
        acc + line.chars().filter(|c| *c == '#').count()
    });

    for line in antinode_map.iter() {
        println!("{}", line);
    }

    println!("Part 2: {}", antinode_count);
}
