fn main() {
    let input = include_str!("../subject/input.txt");
    let mut disk_map: Vec<u64> = Vec::new();

    for (i, c) in input.chars().enumerate() {
        for _ in 0..c.to_digit(10).unwrap() {
            if i % 2 == 0 {
                disk_map.push(i as u64 / 2);
            } else {
                disk_map.push('.' as u64);
            }
        }
    }

    for i in (0..disk_map.len()).rev() {
        if disk_map[i] == '.' as u64 {
            continue;
        }

        for j in 0..disk_map.len() {
            if disk_map[j] != '.' as u64 {
                continue;
            }

            if i <= j {
                break;
            }

            disk_map.swap(i, j);
            break;
        }
    }

    let mut last_free = 0;
    for i in (0..disk_map.len()).rev() {
        if disk_map[i] != '.' as u64 {
            last_free = i;
            break;
        }
    }

    let mut count: u64 = 0;
    for (i, &c) in disk_map.iter().enumerate() {
        if i > last_free {
            break;
        }

        count += c * (i as u64);
    }

    println!("Part 1: {}", count);
}
