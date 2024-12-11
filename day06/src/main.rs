use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let input = input.trim();

    let mut obstacles: Vec<Position> = Vec::new();

    let mut guard_position = None;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = (x as _, y as _);
            match c {
                '#' => obstacles.push(pos),
                '^' => guard_position = Some(pos),
                '.' => (),
                _ => unimplemented!(),
            }
        }
    }

    let bounds: Position = (
        input.lines().count() as _,
        input.lines().next().unwrap().chars().count() as _,
    );

    let mut guard_position = guard_position.unwrap();

    let mut direction = Direction::Up;

    let mut visited = HashSet::new();

    while check_bounds(guard_position, bounds) {
        visited.insert(guard_position);

        let front = in_front(guard_position, direction);
        if obstacles.contains(&front) {
            direction = direction.turn_right();
        }
        guard_position = in_front(guard_position, direction);
    }

    println!("{}", visited.len());
}

type Position = (i32, i32);

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

fn check_bounds(pos: Position, bounds: Position) -> bool {
    pos.0 >= 0 && pos.0 < bounds.0 && pos.1 >= 0 && pos.1 < bounds.1
}

fn in_front(pos: Position, dir: Direction) -> Position {
    match dir {
        Direction::Up => (pos.0, pos.1 - 1),
        Direction::Down => (pos.0, pos.1 + 1),
        Direction::Left => (pos.0 - 1, pos.1),
        Direction::Right => (pos.0 + 1, pos.1),
    }
}
