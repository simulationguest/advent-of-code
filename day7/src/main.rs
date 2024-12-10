fn main() {
    let input = include_str!("./input.txt");
    let mut q1_sum = 0;
    let mut q2_sum = 0;

    for line in input.trim().lines() {
        let equation = parse_line(line);

        if q1(&equation) {
            q1_sum += equation.result;
        }

        if q2(&equation) {
            q2_sum += equation.result;
        }
    }

    println!("Q1: {q1_sum}");
    println!("Q2: {q2_sum}");
}

struct Equation {
    result: u64,
    params: Vec<u64>,
}

fn q1(equation: &Equation) -> bool {
    let params = &equation.params;
    let no_permutations = 2u64.pow((params.len() as u32) - 1);

    for i in 0..no_permutations {
        let mut sum = params[0];
        let mut i = i;
        for j in 1..params.len() {
            if i & 1 == 0 {
                sum += params[j];
            } else {
                sum *= params[j];
            }
            i >>= 1;
        }
        if sum == equation.result {
            return true;
        }
    }
    false
}

fn q2(equation: &Equation) -> bool {
    let params = &equation.params;
    let no_permutations = 3u64.pow((params.len() as u32) - 1);

    for i in 0..no_permutations {
        let mut sum = params[0];
        let mut i = i;
        for j in 1..params.len() {
            let n = i % 3;
            match n {
                0 => sum += params[j],
                1 => sum *= params[j],
                _ => sum = concat(sum, params[j]),
            }
            i /= 3;
        }
        if sum == equation.result {
            return true;
        }
    }
    false
}

fn concat(a: u64, b: u64) -> u64 {
    fn number_of_digits(n: u64) -> u32 {
        n.checked_ilog10().unwrap_or(0) + 1
    }

    let n = number_of_digits(b);
    a * 10u64.pow(n) + b
}

fn parse_line(line: &str) -> Equation {
    let (result, params) = line.split_once(':').unwrap();

    let result = result.parse().unwrap();

    let params: Vec<_> = params
        .split(' ')
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    Equation { result, params }
}
