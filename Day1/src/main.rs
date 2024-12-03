fn part1(left_column:& Vec<i32>, right_column:& Vec<i32>) {
    let total_diff: i32 = left_column.iter()
        .zip(right_column.iter())
        .map(|(left, right)| (left - right).abs())
        .sum();

    println!("Part 1: {}", total_diff);
}

fn part2(left_column:& Vec<i32>, right_column:& Vec<i32>) {
    let left_accum: i32 = left_column.iter()
        .map(|left_value| {
            let nb_occurences = right_column.iter().filter(|&right| *right == *left_value).count() as i32;
            return left_value * nb_occurences;
        })
        .sum();

    println!("Part 2: {}", left_accum);
}

fn main() {
    let file_content = include_str!("../subject/input.txt");

    let (mut left_column, mut right_column): (Vec<i32>, Vec<i32>) = file_content
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace().map(|part| part.parse::<i32>().unwrap());
            return (parts.next().unwrap(), parts.next_back().unwrap())
        })
        .unzip();

    left_column.sort();
    right_column.sort();

    part1(&left_column, &right_column);
    part2(&left_column, &right_column);
}
