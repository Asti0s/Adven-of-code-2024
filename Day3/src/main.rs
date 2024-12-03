use regex::Regex;

fn part2(mul_regex: &Regex, file_content: &str) {
    let total_accum: i32 = mul_regex
        .captures_iter(file_content)
        .map(|cap| {
            let string_position = cap.get(0).unwrap().start();

            for i in (0..string_position).rev() {
                if &file_content[i..i + 7] == "don't()" {
                    return 0;
                } else if &file_content[i..i + 4] == "do()" {
                    break;
                }
            }

            return cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap();
        })
        .sum();

    println!("Part 2: {}", total_accum);
}

fn part1(mul_regex: &Regex, file_content: &str) {
    let total_accum: i32 = mul_regex
        .captures_iter(file_content)
        .map(|cap| cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap())
        .sum();

    println!("Part 1: {}", total_accum);
}

fn main() {
    let file_content = include_str!("../subject/input.txt");
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    part1(&mul_regex, file_content);
    part2(&mul_regex, file_content);
}
