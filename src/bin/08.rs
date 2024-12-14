use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
};

advent_of_code::solution!(8);

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
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

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

struct CityScanner {
    grid: Vec<Vec<char>>,
    height: i32,
    width: i32,
    resonant_harmonics: bool,
}

impl CityScanner {
    fn new(input: &str, resonant_harmonics: bool) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let height = grid.len().try_into().expect("Height fits into i32");
        let width = grid[0].len().try_into().expect("Width fits into i32");

        Self {
            grid,
            height,
            width,
            resonant_harmonics,
        }
    }

    fn boundary_check(&self, point: Point) -> bool {
        (point.x >= 0 && point.x < self.width) && (point.y >= 0 && point.y < self.height)
    }

    fn find_antinodes(&self, antennas: HashMap<char, Vec<Point>>) -> HashSet<Point> {
        let mut antinodes = HashSet::new();
        for points in antennas.into_values() {
            for (point_1, point_2) in points
                .into_iter()
                .combinations(2)
                .map(|pair| (pair[0], pair[1]))
            {
                if self.resonant_harmonics {
                    // antinodes occur at the points of antenna pairs
                    antinodes.insert(point_1);
                    antinodes.insert(point_2);
                }
                let delta = point_1 - point_2;

                let mut antinode_1 = point_1;
                loop {
                    antinode_1 = antinode_1 + delta;
                    if !self.boundary_check(antinode_1) {
                        break;
                    }
                    antinodes.insert(antinode_1);
                    if !self.resonant_harmonics {
                        break;
                    };
                }

                let mut antinode_2 = point_2;
                loop {
                    antinode_2 = antinode_2 - delta;
                    if !self.boundary_check(antinode_2) {
                        break;
                    }
                    antinodes.insert(antinode_2);
                    if !self.resonant_harmonics {
                        break;
                    };
                }
            }
        }
        antinodes
    }

    fn find_antennas(&self) -> HashMap<char, Vec<Point>> {
        let mut antennas = HashMap::new();
        for (y, row) in self.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == '.' {
                    continue;
                }
                let point = Point {
                    x: x.try_into().expect("x fits into i32"),
                    y: y.try_into().expect("y fits into i32"),
                };
                let antenna_points = antennas.entry(*cell).or_insert(vec![]);
                antenna_points.push(point);
            }
        }
        antennas
    }

    fn solve(&self) -> u32 {
        let antennas = self.find_antennas();
        let antinodes = self.find_antinodes(antennas);
        antinodes
            .len()
            .try_into()
            .expect("Antinodes count fits into u32")
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(CityScanner::new(input, false).solve())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(CityScanner::new(input, true).solve())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
