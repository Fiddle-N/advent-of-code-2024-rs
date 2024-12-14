use std::{
    collections::HashMap,
    ops::{Add, Mul},
};

use num_bigint::BigUint;
use regex::Regex;

advent_of_code::solution!(14);

const EXAMPLE_INPUT_LINES: u32 = 12;
const EXAMPLE_WIDTH: u32 = 11;
const EXAMPLE_HEIGHT: u32 = 7;
const REAL_WIDTH: u32 = 101;
const REAL_HEIGHT: u32 = 103;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

struct Vector {
    pos: Point,
    vel: Point,
}

struct Cycle {
    offset: u32,
    period: u32,
}

struct Cycle2D {
    x: Cycle,
    y: Cycle,
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<i32> for Point {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

fn parse(lines: Vec<&str>) -> Vec<Vector> {
    let re = Regex::new(r"-?\d+").expect("Regex pattern should be valid");
    lines
        .into_iter()
        .map(|line| {
            let mut nums = re
                .find_iter(line)
                .map(|m| m.as_str().parse::<i32>().expect("Should fit into i32"));
            let pos = Point {
                x: nums.next().unwrap(),
                y: nums.next().unwrap(),
            };
            let vel = Point {
                x: nums.next().unwrap(),
                y: nums.next().unwrap(),
            };
            Vector { pos, vel }
        })
        .collect()
}

fn advance(vectors: &Vec<Vector>, time: u32, width: u32, height: u32) -> Vec<Point> {
    vectors
        .iter()
        .map(|vector| {
            let unmod_pos =
                vector.pos + vector.vel * time.try_into().expect("Time should fit into i32");
            Point {
                x: unmod_pos
                    .x
                    .rem_euclid(width.try_into().expect("Width should fit into i32")),
                y: unmod_pos
                    .y
                    .rem_euclid(height.try_into().expect("Height should fit into i32")),
            }
        })
        .collect()
}

fn find_cycles(vectors: &Vec<Vector>, width: u32, height: u32) -> Option<Cycle2D> {
    let mut time: u32 = 0;
    let mut x_time_pattern = vec![];
    let mut y_time_pattern = vec![];
    loop {
        time += 1;
        let advanced_points = advance(&vectors, time, width, height);
        if time > width * height {
            return None;
        }

        if x_time_pattern.len() == 3 && y_time_pattern.len() == 3 {
            break;
        }

        let mut x_points = HashMap::new();
        for point in advanced_points.iter() {
            let count = x_points.entry(point.x).or_insert(0);
            *count += 1;
        }

        let mut y_points = HashMap::new();
        for point in advanced_points.iter() {
            let count = y_points.entry(point.y).or_insert(0);
            *count += 1;
        }

        // apply heuristics to find cycles
        if advanced_points
            .iter()
            .any(|point| x_points[&point.x] > width / 4)
        {
            x_time_pattern.push(time);
        }

        if advanced_points
            .iter()
            .any(|point| y_points[&point.y] > height / 4)
        {
            y_time_pattern.push(time);
        }
    }
    assert_eq!(
        x_time_pattern[2] - x_time_pattern[1],
        x_time_pattern[1] - x_time_pattern[0]
    );
    assert_eq!(
        y_time_pattern[2] - y_time_pattern[1],
        y_time_pattern[1] - y_time_pattern[0]
    );

    Some(Cycle2D {
        x: Cycle {
            offset: x_time_pattern[0],
            period: x_time_pattern[1] - x_time_pattern[0],
        },
        y: Cycle {
            offset: y_time_pattern[0],
            period: y_time_pattern[1] - y_time_pattern[0],
        },
    })
}

// Using Chinese Remainder Theorem, we can find the time when the periods are aligned
// Let
// n === x_offset mod x_period
// n === y_offset mod y_period
//
// Then we can rearrange the y cycle to get
// n = y_period * j + y_offset (for some integer j)
//
// Substituting this into the x cycle, we get
// y_period * j + y_offset === x_offset mod x_period
//
// Rearranging this, we get
// y_period * j === (x_offset - y_offset) mod x_period
// (y_period % x_period) j === (x_offset - y_offset) mod x_period

// We can find the modular inverse of y_period % x_period to remove it from the left side
// j === (x_offset - y_offset) * (mod_inv (y_period % x_period) mod=x_period)) mod x_period
//
// Which enables use to find j using modulo operator
// j = (x_offset - y_offset) * (mod_inv (y_period % x_period) mod=x_period)) % x_period
//
// Substituting this back into the y cycle, we get
// n = y_period * ((x_offset - y_offset) * (mod_inv (y_period % x_period) mod=x_period)) % x_period) + y_offset
fn crt(cycle: Cycle2D) -> u32 {
    let base = BigUint::from(cycle.y.period % cycle.x.period);
    let mod_val = BigUint::from(cycle.x.period);
    let mod_inv = base.modinv(&mod_val).expect("Modular inverse should exist");
    let result = cycle.y.period * ((cycle.x.offset - cycle.y.offset) * mod_inv % cycle.x.period)
        + cycle.y.offset;
    result.try_into().expect("Result should fit into u32")
}

pub fn part_one(input: &str) -> Option<u32> {
    let time = 100;
    let lines = input.lines().collect::<Vec<_>>();
    let width: u32;
    let height: u32;
    if lines.len()
        == EXAMPLE_INPUT_LINES
            .try_into()
            .expect("Should fit into usize")
    {
        // example input
        width = EXAMPLE_WIDTH;
        height = EXAMPLE_HEIGHT;
    } else {
        // real input
        width = REAL_WIDTH;
        height = REAL_HEIGHT;
    }
    let vectors = parse(lines);
    let advanced_points = advance(&vectors, time, width, height);

    let quadrants = {
        let width_mid = ((width - 1) / 2)
            .try_into()
            .expect("Width should fit into i32");
        let height_mid = ((height - 1) / 2)
            .try_into()
            .expect("Height should fit into i32");
        let mut quadrants = HashMap::new();
        for point in advanced_points {
            if point.x == width_mid || point.y == height_mid {
                continue;
            }
            let count = quadrants
                .entry((point.x < width_mid, point.y < height_mid))
                .or_insert(0);
            *count += 1;
        }
        quadrants
    };

    Some(quadrants.values().product())
}

pub fn part_two(input: &str) -> Option<u32> {
    // part_two can only be used on the real input
    let lines = input.lines().collect::<Vec<_>>();
    let width = REAL_WIDTH;
    let height = REAL_HEIGHT;
    let vectors = parse(lines);

    let cycle = find_cycles(&vectors, width, height).expect("Cycle should be found");
    Some(crt(cycle))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }
}
