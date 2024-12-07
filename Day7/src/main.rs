struct Equation {
    number: u64,
    test_values: Vec<u64>,
}

fn main() {
    let file_content = include_str!("../subject/input.txt");
    let equations = file_content
        .lines()
        .map(|line| {
            let mut parts = line.split(":");
            let number = parts.next().unwrap().parse().unwrap();
            let test_values = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            return Equation { number, test_values };
        })
        .collect::<Vec<Equation>>();

    let mut total_accum = 0;
    let mut progression = 0;
    for equation in &equations {
        for i in 0..1000000 {
            let mut local_accum = equation.test_values[0];
            for j in 1..equation.test_values.len() {
                let rand = rand::random::<u8>() % 3;

                if rand == 0 {          // Addition
                    local_accum += equation.test_values[j];
                } else if rand == 1 {   // Multiplication
                    local_accum *= equation.test_values[j];
                } else {                // Concatenation
                    local_accum = format!("{}{}", local_accum, equation.test_values[j]).parse().unwrap();
                }
            }

            if local_accum == equation.number {
                total_accum += equation.number;
                break;
            }
        }

        progression += 1;
        println!("Progression: {}%", progression * 100 / equations.len());
    }

    println!("Total: {}", total_accum);
}
