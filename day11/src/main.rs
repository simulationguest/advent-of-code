use std::collections::HashMap;

fn main() {
    let mut numbers = parse_input("28 4 3179 96938 0 6617406 490 816207");
    let mut blank = HashMap::new();

    for _ in 0..75 {
        (numbers, blank) = rules(numbers, blank);
    }
    let sum = numbers.values().sum::<u64>();
    println!("Answer: {sum}");
}

type Map = HashMap<u64, u64>;

#[inline]
fn add(map: &mut Map, n: u64, count: u64) {
    map.entry(n).and_modify(|c| *c += count).or_insert(count);
}

fn rules(mut input: Map, mut output: Map) -> (Map, Map) {
    for (n, count) in input.drain() {
        if n == 0 {
            add(&mut output, 1, count);
            continue;
        }

        let digits = count_digits(n);
        if digits % 2 == 0 {
            let upper = n / 10u64.pow(digits / 2);
            let lower = n % 10u64.pow(digits / 2);
            add(&mut output, upper, count);
            add(&mut output, lower, count);
            continue;
        }

        add(&mut output, n * 2024, count);
    }

    (output, input)
}

#[inline]
fn count_digits(n: u64) -> u32 {
    1 + n.checked_ilog10().unwrap_or_default()
}

fn parse_input(s: &str) -> Map {
    s.split_whitespace()
        .map(|s| s.parse().unwrap())
        .fold(Map::new(), |mut map, n| {
            add(&mut map, n, 1);
            map
        })
}
