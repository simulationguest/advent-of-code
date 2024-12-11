fn main() {
    let mut numbers = parse_input("28 4 3179 96938 0 6617406 490 816207");
    let mut blank = Vec::new();
    for _ in 0..25 {
        (numbers, blank) = rules(numbers, blank);
    }
    println!("Q1: {}", numbers.len());
}

fn rules(mut input: Vec<usize>, mut output: Vec<usize>) -> (Vec<usize>, Vec<usize>) {
    for &n in &input {
        if n == 0 {
            output.push(1);
            continue;
        }
        let digits = count_digits(n);
        if digits % 2 == 0 {
            let upper = n / 10usize.pow(digits as u32 / 2);
            let lower = n % 10usize.pow(digits as u32 / 2);
            output.push(upper);
            output.push(lower);
            continue;
        }
        output.push(n * 2024)
    }
    input.clear();
    (output, input)
}

fn count_digits(n: usize) -> usize {
    1 + n.checked_ilog10().unwrap_or_default() as usize
}

fn parse_input(s: &str) -> Vec<usize> {
    s.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
