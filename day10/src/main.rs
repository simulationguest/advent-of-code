use std::collections::{HashSet, VecDeque};

type Point = (usize, usize);

struct Grid {
    values: Vec<Vec<u8>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        Self {
            values: input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap() as u8)
                        .collect()
                })
                .collect(),
        }
    }
    fn get(&self, (x, y): Point) -> u8 {
        self.values[y][x]
    }
    fn neighbors(&self, (x, y): Point) -> Vec<Point> {
        let mut points = Vec::new();

        if x > 0 {
            points.push((x - 1, y));
        }

        if y > 0 {
            points.push((x, y - 1));
        }

        if x < self.values[0].len() - 1 {
            points.push((x + 1, y));
        }

        if y < self.values.len() - 1 {
            points.push((x, y + 1));
        }

        points
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let grid = Grid::new(input);

    let mut q1 = 0;
    let mut q2 = 0;
    for (y, row) in grid.values.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == 0 {
                let (ends, routes) = find_trails(&grid, (x, y));
                q1 += ends;
                q2 += routes;
            }
        }
    }
    println!("Q1: {q1}\nQ2: {q2}");
}

fn find_trails(grid: &Grid, start: Point) -> (usize, usize) {
    let mut count = 0;
    let mut queue = VecDeque::from([start]);
    let mut ends = HashSet::new();

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        let current_value = grid.get(current);

        if current_value == 9 {
            count += 1;
            ends.insert(current);
            continue;
        }

        let neighbors = grid.neighbors(current);

        for neighbor in neighbors {
            if grid.get(neighbor) != current_value + 1 {
                continue;
            }
            queue.push_back(neighbor);
        }
    }
    (ends.len(), count)
}
