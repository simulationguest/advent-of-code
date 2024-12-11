fn main() {
    let input = include_str!("./input.txt");
    let (mut left, mut right) = parse_input(input);

    left.sort();
    right.sort();

    let total: u32 = left
        .iter()
        .zip(right.iter())
        .map(|(lhs, rhs)| (lhs.abs_diff(*rhs)))
        .sum();
    println!("Total: {total}");

    // part two
    let mut similarity_score = 0;
    for i in left.iter() {
        let count = right.iter().filter(|&&n| n == *i).count() as i32;
        similarity_score += i * count;
    }
    println!("Similarity score: {similarity_score}");
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let line_count = input.lines().count();
    let mut left = Vec::with_capacity(line_count);
    let mut right = Vec::with_capacity(line_count);
    for line in input.lines() {
        let mut parts = line.split_whitespace().map(|s| s.parse().unwrap());
        let lhs = parts.next().unwrap();
        let rhs = parts.next().unwrap();
        left.push(lhs);
        right.push(rhs);
    }
    (left, right)
}
