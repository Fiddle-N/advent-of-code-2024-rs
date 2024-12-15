use std::{collections::HashMap, hash::Hash, vec};

advent_of_code::solution!(15);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Space {
    Empty,
    Box,
    BoxLeft,
    BoxRight,
    Wall,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Move {
    Up,
    Right,
    Down,
    Left,
}

fn move_point(point: Point, direction: Move) -> Point {
    match direction {
        Move::Up => Point {
            x: point.x,
            y: point.y - 1,
        },
        Move::Right => Point {
            x: point.x + 1,
            y: point.y,
        },
        Move::Down => Point {
            x: point.x,
            y: point.y + 1,
        },
        Move::Left => Point {
            x: point.x - 1,
            y: point.y,
        },
    }
}
struct Warehouse {
    map_: HashMap<Point, Space>,
}

impl Warehouse {
    fn get(&self, point: &Point) -> Option<&Space> {
        self.map_.get(point)
    }

    fn insert(&mut self, point: Point, space: Space) {
        self.map_.insert(point, space);
    }

    fn iter(&self) -> impl Iterator<Item = (&Point, &Space)> {
        self.map_.iter()
    }

    fn shift_boxes(&mut self, mut boxes: Vec<Point>, dir: Move) {
        // Boxes are assumed to contain consecutive boxes in the axis of the move direction
        boxes.reverse();
        for box_pos in boxes {
            let box_type = self.get(&box_pos).expect("Box should be in map");
            let empty_pos = move_point(box_pos, dir);
            self.insert(empty_pos, *box_type);
            self.insert(box_pos, Space::Empty);
        }
    }
}

struct WarehouseSimulator {
    warehouse: Warehouse,
    moves: Vec<Move>,
    start: Point,
}

impl WarehouseSimulator {
    fn parse_row_and_find_start(
        map_: &mut HashMap<Point, Space>,
        row: &str,
        y: u32,
        expand: bool,
    ) -> Option<Point> {
        let mut start = None;
        let mut x = 0;
        let mut row = row.to_owned();
        let mut row_iter = row.drain(..);

        loop {
            let first;
            let second;
            match row_iter.next() {
                Some('#') => {
                    first = Space::Wall;
                    second = Space::Wall;
                }
                Some('.') => {
                    first = Space::Empty;
                    second = Space::Empty;
                }
                Some('@') => {
                    start = Some(Point { x, y });
                    first = Space::Empty;
                    second = Space::Empty;
                }
                Some('O') => {
                    first = if expand { Space::BoxLeft } else { Space::Box };
                    second = Space::BoxRight;
                }
                Some(_) => panic!("Should not expect other characters in input"),
                None => break,
            }
            map_.insert(Point { x, y }, first);
            if expand {
                x += 1;
                map_.insert(Point { x, y }, second);
            }
            x += 1;
        }

        start
    }

    fn parse(input: &str, expand: bool) -> Self {
        let mut split_input = input.trim_end().split("\n\n");

        let map_input = split_input.next().expect("Map should be in input");
        let moves_input = split_input.next().expect("Moves should be in input");

        let mut map_ = HashMap::new();
        let mut start: Option<Point> = None;
        for (y, row) in map_input.lines().enumerate() {
            let y = y.try_into().expect("Should fit in u32");
            let row_start = WarehouseSimulator::parse_row_and_find_start(&mut map_, row, y, expand);
            if row_start.is_some() {
                start = row_start;
            }
        }

        let mut moves = vec![];
        for chunk in moves_input.lines() {
            for move_ in chunk.chars() {
                moves.push(match move_ {
                    '^' => Move::Up,
                    '>' => Move::Right,
                    'v' => Move::Down,
                    '<' => Move::Left,
                    _ => panic!("Should not be present in input"),
                });
            }
        }

        Self {
            warehouse: Warehouse { map_ },
            moves,
            start: start.expect("Start should be in input"),
        }
    }

    fn find(&self, mut pos: Point, dir: Move) -> Option<Vec<Point>> {
        let mut boxes = vec![];
        loop {
            pos = move_point(pos, dir);
            match self.warehouse.get(&pos) {
                Some(Space::Empty) => return Some(boxes),
                Some(Space::Wall) => return None,
                Some(Space::Box | Space::BoxLeft | Space::BoxRight) => {
                    boxes.push(pos);
                }
                _ => panic!(),
            }
        }
    }

    fn find_recursive(&self, mut pos: Point, dir: Move) -> Option<Vec<Point>> {
        let mut q = vec![pos];
        let mut boxes = vec![];
        while !q.is_empty() {
            pos = q.remove(0);
            let box_side_pos = move_point(pos, dir);
            match self.warehouse.get(&box_side_pos) {
                Some(Space::Wall) => return None,
                Some(space @ (Space::BoxLeft | Space::BoxRight)) => {
                    let box_move = if *space == Space::BoxLeft {
                        Move::Right
                    } else {
                        Move::Left
                    };

                    if !q.contains(&box_side_pos) {
                        q.push(box_side_pos);
                    }
                    if !boxes.contains(&box_side_pos) {
                        boxes.push(box_side_pos);
                    }

                    let box_other_side_pos = move_point(box_side_pos, box_move);
                    if !q.contains(&box_other_side_pos) {
                        q.push(box_other_side_pos);
                    }
                    if !boxes.contains(&box_other_side_pos) {
                        boxes.push(box_other_side_pos);
                    }
                }
                _ => (),
            };
        }
        Some(boxes)
    }

    fn simulate(&mut self) {
        let mut pos = self.start;
        for move_ in self.moves.iter() {
            let new_pos = move_point(pos, *move_);
            let new_space = self
                .warehouse
                .get(&new_pos)
                .expect("Will always be in map as map is bounded");
            match (new_space, move_) {
                (Space::Wall, _) => continue,
                (Space::Empty, _) => pos = new_pos,
                (Space::BoxLeft | Space::BoxRight, Move::Up | Move::Down) => {
                    let boxes = self.find_recursive(pos, *move_);
                    match boxes {
                        Some(boxes) => self.warehouse.shift_boxes(boxes, *move_),
                        None => continue,
                    }
                    pos = new_pos;
                }
                (Space::BoxLeft | Space::BoxRight | Space::Box, _) => {
                    let boxes = self.find(pos, *move_);
                    match boxes {
                        Some(boxes) => self.warehouse.shift_boxes(boxes, *move_),
                        None => continue,
                    }
                    pos = new_pos;
                }
            }
        }
    }

    fn calculate(&self) -> u32 {
        self.warehouse
            .iter()
            .filter_map(|(point, space)| match space {
                Space::Box | Space::BoxLeft => Some(100 * point.y + point.x),
                _ => None,
            })
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut simulator = WarehouseSimulator::parse(input, false);
    simulator.simulate();
    Some(simulator.calculate())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut simulator = WarehouseSimulator::parse(input, true);
    simulator.simulate();
    Some(simulator.calculate())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_large_example() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_one_small_example() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(9021));
    }

}
