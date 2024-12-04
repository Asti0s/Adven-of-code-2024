fn part1(parsed_file: &Vec<&str>) {
    let mut total_count = 0;

    // Horizontal search
    for i in 0..parsed_file.len() {
        let line = &parsed_file[i];

        for j in 0..line.len() - 3 {
            let expression = line[j..j + 4].to_string();

            if expression == "XMAS" || expression == "SAMX" {
                total_count += 1;
            }
        }
    }

    // Vertical search
    for i in 0..parsed_file.len() - 3 {
        let line = &parsed_file[i];

        for j in 0..line.len() {
            if parsed_file[i].chars().nth(j).unwrap() == 'X'
                && parsed_file[i + 1].chars().nth(j).unwrap() == 'M'
                && parsed_file[i + 2].chars().nth(j).unwrap() == 'A'
                && parsed_file[i + 3].chars().nth(j).unwrap() == 'S'
                || parsed_file[i].chars().nth(j).unwrap() == 'S'
                    && parsed_file[i + 1].chars().nth(j).unwrap() == 'A'
                    && parsed_file[i + 2].chars().nth(j).unwrap() == 'M'
                    && parsed_file[i + 3].chars().nth(j).unwrap() == 'X'
            {
                total_count += 1;
            }
        }
    }

    // Diagonal search (left to right)
    for i in 0..parsed_file.len() - 3 {
        let line = &parsed_file[i];

        for j in 0..line.len() - 3 {
            if parsed_file[i].chars().nth(j).unwrap() == 'X'
                && parsed_file[i + 1].chars().nth(j + 1).unwrap() == 'M'
                && parsed_file[i + 2].chars().nth(j + 2).unwrap() == 'A'
                && parsed_file[i + 3].chars().nth(j + 3).unwrap() == 'S'
                || parsed_file[i].chars().nth(j).unwrap() == 'S'
                    && parsed_file[i + 1].chars().nth(j + 1).unwrap() == 'A'
                    && parsed_file[i + 2].chars().nth(j + 2).unwrap() == 'M'
                    && parsed_file[i + 3].chars().nth(j + 3).unwrap() == 'X'
            {
                total_count += 1;
            }
        }
    }

    // Diagonal search (right to left)
    for i in 0..parsed_file.len() - 3 {
        let line = &parsed_file[i];

        for j in 3..line.len() {
            if parsed_file[i].chars().nth(j).unwrap() == 'X'
                && parsed_file[i + 1].chars().nth(j - 1).unwrap() == 'M'
                && parsed_file[i + 2].chars().nth(j - 2).unwrap() == 'A'
                && parsed_file[i + 3].chars().nth(j - 3).unwrap() == 'S'
                || parsed_file[i].chars().nth(j).unwrap() == 'S'
                    && parsed_file[i + 1].chars().nth(j - 1).unwrap() == 'A'
                    && parsed_file[i + 2].chars().nth(j - 2).unwrap() == 'M'
                    && parsed_file[i + 3].chars().nth(j - 3).unwrap() == 'X'
            {
                total_count += 1;
            }
        }
    }

    println!("Part 1: {}", total_count);
}

fn part2(parsed_file: &Vec<&str>) {
    let mut total_count = 0;
    for i in 1..parsed_file.len() - 1 {
        let line = &parsed_file[i];

        for j in 1..line.len() - 1 {
            if line.as_bytes()[j] != b'A' {
                continue;
            }

            let top_left = parsed_file[i - 1].as_bytes()[j - 1] as char;
            let top_right = parsed_file[i - 1].as_bytes()[j + 1] as char;
            let bottom_left = parsed_file[i + 1].as_bytes()[j - 1] as char;
            let bottom_right = parsed_file[i + 1].as_bytes()[j + 1] as char;

            if ((top_left == 'M' && bottom_right == 'S')
                || (top_left == 'S' && bottom_right == 'M'))
                && ((top_right == 'M' && bottom_left == 'S')
                    || (top_right == 'S' && bottom_left == 'M'))
            {
                total_count += 1;
            }
        }
    }

    println!("Part 2: {}", total_count);
}

fn main() {
    let file_content = include_str!("../subject/input.txt");
    let parsed_file: Vec<&str> = file_content.split("\n").collect();

    part1(&parsed_file);
    part2(&parsed_file);
}
