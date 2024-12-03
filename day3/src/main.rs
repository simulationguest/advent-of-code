use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");

    let do_regex = Regex::new(r"(?s)(?:(?:do\(\))|^).*?(?:(?:don't\(\))|$)").unwrap();
    let mul_regex = Regex::new(r"(?m)mul\((\d+),(\d+)\)").unwrap();

    let total_sum = sum_muls(&mul_regex, input);
    println!("Total sum: {total_sum}");

    let partial_sum: i32 = do_regex
        .find_iter(input)
        .map(|m| {
            let m = m.as_str();
            sum_muls(&mul_regex, m)
        })
        .sum();
    println!("Partial sum: {partial_sum}");
}


// pass the regex to the function to avoid reinitializing it on every call
fn sum_muls(re: &Regex, m: &str) -> i32 {
    re 
                .captures_iter(m)
                .map(|cap| {
                    let a: i32 = cap[1].parse().unwrap();
                    let b: i32 = cap[2].parse().unwrap();
                    a * b
                })
                .sum()
}