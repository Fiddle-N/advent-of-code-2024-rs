use std::collections::{HashMap, HashSet};
use std::ops::Add;

advent_of_code::solution!(6);

#[derive(PartialEq)]
enum Space {
    Empty,
    Obstruction,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn point_offset(&self) -> Point {
        match self {
            Direction::Up => Point { x: 0, y: -1 },
            Direction::Right => Point { x: 1, y: 0 },
            Direction::Down => Point { x: 0, y: 1 },
            Direction::Left => Point { x: -1, y: 0 },
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct ParsedMap {
    start: Point,
    map: HashMap<Point, Space>,
}

#[derive(PartialEq)]
enum PathEnd {
    Exit,
    Cycle,
}

struct PathOutput {
    end_type: PathEnd,
    visited: HashSet<Point>,
}

fn parsed_map(input: &str) -> ParsedMap {
    let mut start = None;
    let mut map = HashMap::new();
    for (y, row) in input.lines().enumerate() {
        for (x, space) in row.chars().enumerate() {
            let coord = Point {
                x: x.try_into().unwrap(),
                y: y.try_into().unwrap(),
            };
            match space {
                '.' => map.insert(coord, Space::Empty),
                '#' => map.insert(coord, Space::Obstruction),
                '^' => {
                    start = Some(coord);
                    map.insert(coord, Space::Empty)
                }
                _ => panic!("Unexpected value"),
            };
        }
    }
    let start = start.expect("^ must be present in input");
    ParsedMap { start, map }
}

fn trace_path(
    mut pos: Point,
    map: &HashMap<Point, Space>,
    added_obstruction: Option<Point>,
) -> PathOutput {
    let mut next_pos;
    let mut dir = Direction::Up;
    let mut visited = HashSet::new();
    let mut visited_orientation = HashSet::new();

    loop {
        if visited_orientation.contains(&(pos, dir)) {
            return PathOutput {
                end_type: PathEnd::Cycle,
                visited,
            };
        }
        visited.insert(pos);
        visited_orientation.insert((pos, dir));

        next_pos = pos + dir.point_offset();

        match map.get(&next_pos) {
            None => {
                return PathOutput {
                    end_type: PathEnd::Exit,
                    visited,
                }
            }
            Some(Space::Empty) if Some(next_pos) != added_obstruction => {
                pos = next_pos;
            }
            _ => {
                // Space::Obstruction or Space::Empty but not added obstruction
                dir = dir.turn_right();
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let ParsedMap { start, map } = parsed_map(input);
    let path = trace_path(start, &map, None);
    Some(
        path.visited
            .len()
            .try_into()
            .expect("Path length fits into u32"),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let ParsedMap { start, map } = parsed_map(input);
    let mut looped_obstructions = 0;
    for (point, space) in map.iter() {
        if *point == start || *space == Space::Obstruction {
            continue;
        }
        let path = trace_path(start, &map, Some(*point));
        if path.end_type == PathEnd::Cycle {
            looped_obstructions += 1;
        }
    }
    Some(looped_obstructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
