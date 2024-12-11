use std::{collections::HashMap, ops::Add};

advent_of_code::solution!(10);

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Path {
    start: Point,
    end: Point,
}

const DIRS: [Point; 4] = [
    Point { x: 0, y: 1 },
    Point { x: 0, y: -1 },
    Point { x: 1, y: 0 },
    Point { x: -1, y: 0 },
];

struct MapData {
    map: HashMap<Point, u32>,
    starting_points: Vec<Point>,
}

fn parse(input: &str) -> MapData {
    let mut map = HashMap::new();
    let mut starting_points = vec![];
    for (y, row) in input.lines().enumerate() {
        for (x, char) in row.chars().enumerate() {
            let point = Point {
                x: x.try_into().expect("Fits into i32"),
                y: y.try_into().expect("Fits into i32"),
            };
            let height = char.to_digit(10).expect("Fits into u32");
            map.insert(point, height);
            if height == 0 {
                starting_points.push(point);
            }
        }
    }
    MapData {
        map,
        starting_points,
    }
}

struct MapSolver<'a> {
    map_data: &'a MapData,
    paths: HashMap<Path, u32>,
}

impl<'a> MapSolver<'a> {
    fn new(map_: &'a MapData) -> Self {
        Self {
            map_data: map_,
            paths: HashMap::new(),
        }
    }

    fn solve_path(&mut self, pos: &Point, height: u32, start: &Point) {
        if height == 9 {
            let count = self
                .paths
                .entry(Path {
                    start: *start,
                    end: *pos,
                })
                .or_insert(0);
            *count += 1;
            return;
        }
        for dir in DIRS.iter() {
            let new_pos = *pos + *dir;
            let new_height = match self.map_data.map.get(&new_pos) {
                Some(new_height) if new_height == &(height + 1) => *new_height,
                _ => continue,
            };
            self.solve_path(&new_pos, new_height, start);
        }
    }

    fn solve(&mut self) {
        for start in self.map_data.starting_points.iter() {
            self.solve_path(start, 0, start);
        }
    }
}

fn solve(input: &str) -> HashMap<Path, u32> {
    let map_data = parse(input);
    let mut solver = MapSolver::new(&map_data);
    solver.solve();
    solver.paths
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = solve(input);
    Some(result.len().try_into().expect("Fits into u32"))
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = solve(input);
    Some(result.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
