fn check_line(line: &Vec<i32>) -> bool {
    let is_decreasing = line[0] > line[1];

    for i in 0..line.len() - 1 {
        if (is_decreasing && line[i] < line[i + 1])
            || (!is_decreasing && line[i] > line[i + 1])
            || ((line[i] - line[i + 1]).abs() > 3)
            || (line[i] == line[i + 1])
        {
            return false;
        }
    }

    return true;
}

fn part1(input: &Vec<Vec<i32>>) {
    let count = input.iter().filter(|line| check_line(line)).count();
    println!("Part 1: {}", count);
}

fn part2(input: &Vec<Vec<i32>>) {
    let foo = input
        .iter()
        .map(|line| {
            check_line(line)
                || (0..line.len()).any(|i| {
                    let mut new_line = line.clone();
                    new_line.remove(i);
                    check_line(&new_line)
                })
        })
        .filter(|e| *e)
        .count();

    println!("Part 2: {}", foo);
}

fn main() {
    let file_content = include_str!("../subject/input.txt");

    let input: Vec<Vec<i32>> = file_content
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|part| part.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    part1(&input);
    part2(&input);
}
