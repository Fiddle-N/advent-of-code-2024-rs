use std::{collections::{HashMap, HashSet}, ops::Add};

advent_of_code::solution!(12);


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

const DIRECTIONS: [[Point; 2]; 4] = [
    [Point { x: -1, y: 0 }, Point { x: 0, y: 1 }],
    [Point { x: 0, y: 1 }, Point { x: 1, y: 0 }],
    [Point { x: 1, y: 0 }, Point { x: 0, y: -1 }],
    [Point { x: 0, y: -1 }, Point { x: -1, y: 0 }],
];

struct GardenRegion {
    region: HashSet<Point>,
    corners: u32
}

struct Garden {
    regions: Vec<GardenRegion>,
    perimeters: HashMap<Point, u32>,
}

struct GardenPrice {
    perimeter: u32,
    corners: u32,
}

fn parse(input: &str) -> HashMap<Point, char> {
    let mut map = HashMap::new();
    for (y, row) in input.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            map.insert(Point{x: x.try_into().expect("Fit in u32"), y: y.try_into().expect("Fit in u32")}, c);
        }
    }
    map
}

fn solve_garden(map: HashMap<Point, char>) -> Garden {
    let mut perimeters = HashMap::new();
    let mut regions = vec![];
    let mut points: HashSet<_> = map.keys().clone().map(|&point| point).collect();

    while !points.is_empty() {
        let region_point = *points.iter().next().expect("Not empty");
        let region_plant = *map.get(&region_point).expect("Exists");

        let mut region_points = HashSet::new();
        region_points.insert(region_point);
        let mut region_points_to_check = vec![region_point];
        let mut corners = 0;

        loop {
            if region_points_to_check.is_empty() {
                break;
            }
            let next_point = region_points_to_check.pop().expect("Not empty");
            points.take(&next_point);


            let mut perimeter = 0;
            for dirs in DIRECTIONS.iter() {
                {
                    let neighbours = dirs.iter().map(|dir| next_point + *dir);
                    let neighbour_plants: Vec<_> = neighbours.map(|neighbour| map.get(&neighbour)).collect();
                    if !neighbour_plants.contains(&Some(&region_plant)) {
                        corners += 1;
                    }
                    else if neighbour_plants.iter().all(|&plant| plant == Some(&region_plant)) {
                        let diagonal_dir = dirs[0] + dirs[1];
                        let diagonal_neighbour = next_point + diagonal_dir;
                        let diagonal_neighbour_plant = map.get(&diagonal_neighbour);
                        match diagonal_neighbour_plant {
                            Some(&plant) if plant == region_plant => (),
                            _ => {
                                corners += 1;
                            }
                        }
                    }
                }

                {
                    let neighbour = next_point + dirs[0];
                    let neighbour_plant = map.get(&neighbour);
                    match neighbour_plant {
                        Some(&plant) if plant == region_plant => {
                            if !region_points.contains(&neighbour) {
                                region_points.insert(neighbour);
                                region_points_to_check.push(neighbour);
                            }
                        }
                        _ => {
                            perimeter += 1;
                        }
                    }
                }
            }

            perimeters.insert(next_point, perimeter);
        }

        regions.push(GardenRegion {
            region: region_points,
            corners,
        });
    }

    Garden {
        regions,
        perimeters,
    }
}

fn calculate_price(garden: Garden) -> GardenPrice {
    let mut perimeter_price = 0;
    let mut corner_price = 0;

    for garden_region in garden.regions.iter() {
        let area: u32 = garden_region.region.len().try_into().expect("Fits into u32");

        let perimeter = garden_region
            .region
            .iter()
            .map(|point| garden.perimeters.get(point).expect("Exists"))
            .sum::<u32>();

        perimeter_price += area * perimeter;
        corner_price += area * garden_region.corners;

    }

    GardenPrice {
        perimeter: perimeter_price,
        corners: corner_price,
    }
}

fn solve(input: &str) -> GardenPrice {
    let map = parse(input);
    let garden = solve_garden(map);
    calculate_price(garden)
}


pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input).perimeter)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(input).corners)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
