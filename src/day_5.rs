use std::{collections::HashMap, str::FromStr};

use crate::runner::{Parse, Run};
use anyhow::{Error, Result};
use std::cmp::Ordering;

pub struct Day5 {}

impl Parse<Vec<Line>, u16> for Day5 {
    fn parse_input(input: &str) -> Result<Vec<Line>> {
        let r = input
            .lines()
            .filter_map(|l| {
                let (p1, p2) = l.split_once(" -> ")?;
                let start = Point::from_str(p1).ok()?;
                let end = Point::from_str(p2).ok()?;
                let direction = Direction::from_points(&start, &end)?;
                let line = Line {
                    start,
                    end,
                    direction,
                };
                Some(line)
            })
            .collect();
        Ok(r)
    }
}

impl Run<Vec<Line>, u16> for Day5 {
    fn part_one(input: &Vec<Line>) -> Result<u16> {
        let filter = |x: &&Line| x.direction.is_normal();
        let diagram = create_diagram(input, filter);
        let lim: u16 = 2;
        Ok(diagram.into_iter().filter(|(_, v)| *v >= lim).count() as u16)
    }

    fn part_two(input: &Vec<Line>) -> Result<u16> {
        let filter = |_: &&Line| true;
        let diagram = create_diagram(input, filter);
        let lim: u16 = 2;
        Ok(diagram.into_iter().filter(|(_, v)| *v >= lim).count() as u16)
    }
    // add code here
}

#[derive(Debug)]
struct Point {
    x: u16,
    y: u16,
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (x, y) = s.split_once(',').ok_or(anyhow::anyhow!("Invalid input"))?;
        Ok(Point {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

#[derive(Debug)]
pub struct Line {
    start: Point,
    end: Point,
    direction: Direction,
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    RightUp,
    RightDown,
    LeftUp,
    LeftDown,
}

impl Direction {
    fn from_points(p1: &Point, p2: &Point) -> Option<Self> {
        let x_cmp = p1.x.cmp(&p2.x);
        let y_cmp = p1.y.cmp(&p2.y);
        match (x_cmp, y_cmp) {
            (Ordering::Less, Ordering::Less) => Some(Self::RightDown),
            (Ordering::Less, Ordering::Equal) => Some(Self::Right),
            (Ordering::Less, Ordering::Greater) => Some(Self::RightUp),
            (Ordering::Equal, Ordering::Less) => Some(Self::Down),
            (Ordering::Equal, Ordering::Equal) => None,
            (Ordering::Equal, Ordering::Greater) => Some(Self::Up),
            (Ordering::Greater, Ordering::Less) => Some(Self::LeftDown),
            (Ordering::Greater, Ordering::Equal) => Some(Self::Left),
            (Ordering::Greater, Ordering::Greater) => Some(Self::LeftUp),
        }
    }

    fn is_normal(&self) -> bool {
        matches!(
            self,
            Direction::Right | Direction::Down | Direction::Up | Direction::Left
        )
    }
}

// TODO: There gotta be a better way to do this
fn create_diagram(lines: &[Line], filter: fn(&&Line) -> bool) -> HashMap<String, u16> {
    let mut diagram = HashMap::new();
    for l in lines.iter().filter(filter) {
        let start: u16;
        let end: u16;
        let on_x: bool;
        let mut increase: bool = false;
        let mut special_y: u16 = 0;
        match l.direction {
            Direction::Up => {
                start = l.end.y;
                end = l.start.y;
                on_x = false;
            }
            Direction::Down => {
                start = l.start.y;
                end = l.end.y;
                on_x = false;
            }
            Direction::Left => {
                start = l.end.x;
                end = l.start.x;
                on_x = true;
            }
            Direction::Right => {
                start = l.start.x;
                end = l.end.x;
                on_x = true;
            }
            Direction::RightUp => {
                start = l.start.x;
                end = l.end.x;
                special_y = l.start.y;
                on_x = true;
                increase = false;
            }
            Direction::RightDown => {
                start = l.start.x;
                end = l.end.x;
                special_y = l.start.y;
                on_x = true;
                increase = true;
            }
            Direction::LeftUp => {
                start = l.end.x;
                end = l.start.x;
                special_y = l.end.y;
                on_x = true;
                increase = true;
            }
            Direction::LeftDown => {
                start = l.end.x;
                end = l.start.x;
                special_y = l.end.y;
                on_x = true;
                increase = false;
            }
        }
        (start..end + 1).for_each(|i| {
            let key: String;
            match (on_x, l.direction.is_normal()) {
                // If is diagonal both coordinates have to move with different starts
                // and directions
                (true, false) => {
                    key = format!("{},{}", i, special_y);

                    //so this increase or decrease y depending of the direction
                    match (increase, special_y) {
                        (true, _) => special_y += 1,
                        (false, y) if y != 0 => special_y -= 1,
                        _ => {}
                    }
                }
                (true, true) => key = format!("{},{}", i, l.start.y),
                (false, true) => key = format!("{},{}", l.start.x, i),
                _ => panic!("invalid case"),
            }
            match diagram.get_mut(&key) {
                Some(v) => *v += 1,
                None => {
                    diagram.insert(key, 1);
                }
            }
        });
    }
    diagram
}

#[cfg(test)]
mod tests_day5 {
    use crate::runner::Executor;

    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        let input = Day5::parse_input(INPUT)?;
        let result = Day5::part_one(&input)?;
        assert_eq!(result, 5);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = Day5::parse_input(INPUT)?;
        let result = Day5::part_two(&input)?;
        assert_eq!(result, 12);
        Ok(())
    }

    #[test]
    fn test_run() -> Result<()> {
        let (r1, r2) = Day5::run("inputs/day5.test")?;
        assert_eq!(r1, 5);
        assert_eq!(r2, 12);
        Ok(())
    }

    #[test]
    fn test_direction_from_points() {
        let p1 = Point { x: 0, y: 5 };
        let p2 = Point { x: 0, y: 9 };
        let dir = Direction::from_points(&p1, &p2);
        assert!(dir.is_some());
        assert_eq!(Some(Direction::Down), dir);

        let p1 = Point { x: 0, y: 14 };
        let p2 = Point { x: 0, y: 9 };
        let dir = Direction::from_points(&p1, &p2);
        assert!(dir.is_some());
        assert_eq!(Some(Direction::Up), dir);

        let p1 = Point { x: 6, y: 5 };
        let p2 = Point { x: 0, y: 5 };
        let dir = Direction::from_points(&p1, &p2);
        assert!(dir.is_some());
        assert_eq!(Some(Direction::Left), dir);

        let p1 = Point { x: 0, y: 5 };
        let p2 = Point { x: 6, y: 5 };
        let dir = Direction::from_points(&p1, &p2);
        assert!(dir.is_some());
        assert_eq!(Some(Direction::Right), dir);

        let p1 = Point { x: 0, y: 8 };
        let p2 = Point { x: 8, y: 0 };
        let dir = Direction::from_points(&p1, &p2);
        assert!(dir.is_some());
        assert_eq!(Some(Direction::RightUp), dir);

        let p1 = Point { x: 3, y: 2 };
        let p2 = Point { x: 2, y: 3 };
        let dir = Direction::from_points(&p1, &p2);
        assert!(dir.is_some());
        assert_eq!(Some(Direction::LeftDown), dir);

        let p1 = Point { x: 7, y: 6 };
        let p2 = Point { x: 5, y: 4 };
        let dir = Direction::from_points(&p1, &p2);
        assert!(dir.is_some());
        assert_eq!(Some(Direction::LeftUp), dir);

        let p1 = Point { x: 3, y: 7 };
        let p2 = Point { x: 5, y: 9 };
        let dir = Direction::from_points(&p1, &p2);
        assert!(dir.is_some());
        assert_eq!(Some(Direction::RightDown), dir);

        let p1 = Point { x: 0, y: 5 };
        let p2 = Point { x: 0, y: 5 };
        let dir = Direction::from_points(&p1, &p2);
        assert!(dir.is_none());
    }

    const INPUT: &str = include_str!("../inputs/day5.test");

    #[test]
    fn test_create_diagram() -> Result<()> {
        let input = Day5::parse_input(INPUT)?;
        let filter = |x: &&Line| x.direction.is_normal();
        let diagram = create_diagram(&input, filter);
        assert_eq!(diagram.len(), 21);
        assert_eq!(diagram.get("0,9"), Some(&2));
        assert_eq!(diagram.get("7,0"), Some(&1));
        assert_eq!(diagram.get("9,4"), Some(&1));
        assert_eq!(diagram.get("0,0"), None);
        Ok(())
    }

    #[test]
    fn test_create_diagram_diagonal() -> Result<()> {
        let input = Day5::parse_input(INPUT)?;
        let filter = |_: &&Line| true;
        let diagram = create_diagram(&input, filter);
        assert_eq!(diagram.len(), 39);
        assert_eq!(diagram.get("0,9"), Some(&2));
        assert_eq!(diagram.get("7,0"), Some(&1));
        assert_eq!(diagram.get("9,4"), Some(&1));
        assert_eq!(diagram.get("0,0"), Some(&1));
        assert_eq!(diagram.get("4,4"), Some(&3));
        assert_eq!(diagram.get("0,1"), None);
        Ok(())
    }
}
