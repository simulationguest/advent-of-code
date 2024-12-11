fn main() {
    let input = include_str!("./input.txt");

    let mut safe = 0;
    let mut safe_modified = 0;

    for line in input.lines() {
        let parts = parse_line(line);

        if is_safe(parts.clone()) {
            safe += 1;
            safe_modified += 1;
            continue;
        }

        for i in 0..parts.len() {
            let mut modified = parts.clone();
            modified.remove(i);
            if is_safe(modified) {
                safe_modified += 1;
                break;
            }
        }
    }

    println!("Safe lines: {safe}");
    println!("Safe lines (modified): {safe_modified}");
}

fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn is_safe(mut parts: Vec<i32>) -> bool {
    if parts[0] > parts[1] {
        parts.reverse();
    }
    if !parts.is_sorted() {
        return false;
    }
    let mut prev = parts[0];
    for i in 1..parts.len() {
        let diff = prev.abs_diff(parts[i]);
        if diff < 1 || diff > 3 {
            return false;
        }
        prev = parts[i];
    }
    true
}
