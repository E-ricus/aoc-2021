use anyhow::Result;

pub fn run_day_two() -> Result<()> {
    let movements: Vec<Movements> = std::fs::read_to_string("inputs/day2.txt")?
        .lines()
        .filter_map(|s| match s.split_once(' ') {
            Some(("forward", x)) => Some(Movements::Forward(x.parse().ok()?)),
            Some(("down", x)) => Some(Movements::Down(x.parse().ok()?)),
            Some(("up", x)) => Some(Movements::Up(x.parse().ok()?)),
            _ => None,
        })
        .collect();

    let pos = positions(movements.clone());
    println!("The position after movements is: {}", pos);
    let pos = complex_positions(movements);
    println!("The position after movements is: {}", pos);
    Ok(())
}

#[derive(Debug, Clone)]
enum Movements {
    Forward(u32),
    Down(u32),
    Up(u32),
}

fn positions(movements: Vec<Movements>) -> u32 {
    let mut x_pos = 0;
    let mut y_pos = 0;
    for movement in movements {
        match movement {
            Movements::Forward(x) => x_pos += x,
            Movements::Down(x) => y_pos += x,
            Movements::Up(x) => y_pos -= x,
        }
    }
    x_pos * y_pos
}

fn complex_positions(movements: Vec<Movements>) -> u32 {
    let mut x_pos = 0;
    let mut y_pos = 0;
    let mut aim = 0;
    for movement in movements {
        match movement {
            Movements::Forward(x) => {
                x_pos += x;
                y_pos += aim * x;
            }
            Movements::Down(x) => aim += x,
            Movements::Up(x) => aim -= x,
        }
    }
    x_pos * y_pos
}

#[test]
fn test_positions() {
    let movements = vec![
        Movements::Forward(5),
        Movements::Down(5),
        Movements::Forward(8),
        Movements::Up(3),
        Movements::Down(8),
        Movements::Forward(2),
    ];
    let pos = positions(movements);
    assert_eq!(pos, 150)
}

#[test]
fn test_complex_positions() {
    let movements = vec![
        Movements::Forward(5),
        Movements::Down(5),
        Movements::Forward(8),
        Movements::Up(3),
        Movements::Down(8),
        Movements::Forward(2),
    ];
    let pos = complex_positions(movements);
    assert_eq!(pos, 900)
}
