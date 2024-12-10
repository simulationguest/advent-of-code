fn main() {
    let input = include_str!("./input.txt");
    let mut sum_of_true_equations = 0;

    for line in input.trim().lines() {
        let (result, equation) = parse_line(line);

        let no_permutations = 2u64.pow((equation.len() as u32) - 1);

        for i in 0..no_permutations {
            let mut sum = equation[0];
            let mut i = i;
            for j in 1..equation.len() {
                if i & 1 == 0 {
                    sum += equation[j];
                } else {
                    sum *= equation[j];
                }
                i >>= 1;
            }
            if sum == result {
                sum_of_true_equations += result;
                break;
            }
        }
    }

    println!("Sum of true equations: {sum_of_true_equations}");
}

fn parse_line(line: &str) -> (u64, Vec<u64>) {
    let (result, equation) = line.split_once(':').unwrap();

    let result = result.parse().unwrap();

    let equation: Vec<_> = equation
        .split(' ')
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    (result, equation)
}
