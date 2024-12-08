fn main() {
    let input = include_str!("./input.txt");

    let (ordering_rules, updates) = parse_input(input);

    let mut sum = 0;
    'outer: for update in updates {
        for i in 0..update.len() - 1 {
            let current = update[i];
            let next = update[i + 1];
            let mut found = false;
            for (lhs, rhs) in &ordering_rules {
                if *lhs == current && *rhs == next {
                    found = true;
                    break;
                }
            }
            if !found {
                continue 'outer
            }
        }
        sum += update[update.len() / 2];
    }
    println!("Sum: {sum}");
}

fn parse_input(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let (ordering_rules, updates) = input.split_once("\n\n").unwrap();

    let ordering_rules: Vec<(i32, i32)> = ordering_rules.lines().map(|l| {
        let (lhs, rhs) = l.split_once('|').unwrap();
        (lhs.parse().unwrap(), rhs.parse().unwrap())
    }).collect();

    let updates: Vec<Vec<i32>> = updates
        .lines()
        .map(|l| l.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    (ordering_rules, updates)
}
