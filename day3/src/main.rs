use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");

    let regex = Regex::new(r"(?m)mul\((\d+),(\d+)\)").unwrap();
    let sum: i32 = regex
        .captures_iter(input)
        .map(|cap| {
            let a: i32 = cap[1].parse().unwrap();
            let b: i32 = cap[2].parse().unwrap();
            a * b
        })
        .sum();

    println!("Sum: {sum}");
}
