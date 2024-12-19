use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let (patterns, designs) = parse_input(&input);

    let mut total = 0;
    for design in designs {
        if test_design(&patterns, design) {
            total += 1;
        }
    }
    println!("Total: {total}");
}

fn test_design(patterns: &[Regex], design: &str) -> bool {
    if design == "" {
        return true;
    }

    for pattern in patterns {
        if let Some(result) = pattern.find(design) {
            let design = &design[result.end()..design.len()];
            if test_design(patterns, design) {
                return true;
            }
        }
    }
    false
}

fn parse_input(input: &str) -> (Vec<Regex>, impl Iterator<Item = &str>) {
    let (patterns, designs) = input.trim().split_once("\n\n").unwrap();

    let mut patterns: Vec<_> = patterns.split(',').map(|s| s.trim()).collect();

    patterns.sort_by(|a, b| b.len().cmp(&a.len()));

    let patterns = patterns
        .into_iter()
        .map(|s| {
            let re = format!("^{s}");
            Regex::new(&re).unwrap()
        })
        .collect();

    let designs = designs.lines();

    (patterns, designs)
}
