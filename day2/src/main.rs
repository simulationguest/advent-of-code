fn main() {
    let input = include_str!("./input.txt");

    let mut safe = 0;
    'outer: for line in input.lines() {
        let mut parts: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        if parts[0] > parts[1] {
            parts.reverse();
        }
        if !parts.is_sorted() {
            continue;
        }
        let mut prev = parts[0];
        for i in 1..parts.len() {
            let diff = prev.abs_diff(parts[i]);
            if diff < 1 || diff > 3 {
                continue 'outer;
            }
            prev = parts[i];
        }
        safe += 1;
    }

    println!("{safe}");
}
