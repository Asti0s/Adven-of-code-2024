use regex::Regex;

fn part1(ordering_values : &Vec<(i32, i32)>, values_to_order : &Vec<Vec<i32>>) {
    let total_accum: i32 = values_to_order
        .iter()
        .map(|values| {
            for i in 0..values.len() {
                for (x, y) in ordering_values {
                    if values[i] == *x {
                        for j in (0..i).rev() {
                            if values[j] == *y {
                                return 0;
                            }
                        }
                    }
                }
            }

            return values[values.len() / 2];
        }).sum();

    println!("Total accum: {}", total_accum);
}

fn part2(ordering_values : &Vec<(i32, i32)>, values_to_order : Vec<Vec<i32>>) {
    let total_accum: i32 = values_to_order
        .iter()
        .map(|values| {
            let mut values = values.clone();
            let mut is_modified = false;

            let mut i = 0;
            let mut tmp;

            while i < values.len() {
                for (x, y) in ordering_values {
                    if values[i] == *x {
                        for j in (0..i).rev() {
                            if values[j] == *y {
                                tmp = values[i];
                                values[i] = values[j];
                                values[j] = tmp;
                                is_modified = true;
                                i = 0;
                            }
                        }
                    }
                }

                i += 1;
            }

            if is_modified {
                return values[values.len() / 2];
            }

            return 0;
        }).sum();

    println!("Total accum: {}", total_accum);
}

fn main() {
    let file_content = include_str!("../subject/input.txt");

    let ordering_regex = Regex::new(r"(\d+)\|(\d+)").unwrap();
    let ordering_values: Vec<(i32, i32)> = ordering_regex
        .captures_iter(file_content)
        .map(|cap| {
            let x = cap[1].parse::<i32>().unwrap();
            let y = cap[2].parse::<i32>().unwrap();
            return (x, y);
        })
        .collect();

    let values_regex = Regex::new(r"^\d+(?:,\d+)*$").unwrap();
    let values_to_order: Vec<Vec<i32>> = file_content
        .lines()
        .filter(|line| values_regex.is_match(line))
        .map(|line| {
            return line.split(",")
                .map(|part| part.parse::<i32>().unwrap())
                .collect();
        })
        .collect();

    part1(&ordering_values, &values_to_order);
    part2(&ordering_values, values_to_order);
}
