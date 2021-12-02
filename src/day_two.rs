pub fn run_day_two() {
    let list = crate::lines_from_file("inputs/day2.txt").expect("Could not load lines");
    let movements = list
        .into_iter()
        .map(|s| {
            let mut parts = s.split(' ');
            (
                parts.next().unwrap().to_string(),
                parts.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<(String, u32)>>();
    let pos = positions(movements.clone());
    println!("The position after movements is: {}", pos);
    let pos = complex_positions(movements);
    println!("The position after movements is: {}", pos);
}

fn positions(movements: Vec<(String, u32)>) -> u32 {
    let mut x_pos = 0;
    let mut y_pos = 0;
    for (movement, x) in movements {
        match movement.as_str() {
            "forward" => x_pos += x,
            "down" => y_pos += x,
            "up" => y_pos -= x,
            _ => println!("invalid movement"),
        }
    }
    x_pos * y_pos
}

fn complex_positions(movements: Vec<(String, u32)>) -> u32 {
    let mut x_pos = 0;
    let mut y_pos = 0;
    let mut aim = 0;
    for (movement, x) in movements {
        match movement.as_str() {
            "forward" => {
                x_pos += x;
                y_pos += aim * x;
            }
            "down" => aim += x,
            "up" => aim -= x,
            _ => println!("invalid movement"),
        }
    }
    x_pos * y_pos
}

#[test]
fn test_positions() {
    let movements = vec![
        ("forward".to_string(), 5),
        ("down".to_string(), 5),
        ("forward".to_string(), 8),
        ("up".to_string(), 3),
        ("down".to_string(), 8),
        ("forward".to_string(), 2),
    ];
    let pos = positions(movements);
    assert_eq!(pos, 150)
}

#[test]
fn test_complex_positions() {
    let movements = vec![
        ("forward".to_string(), 5),
        ("down".to_string(), 5),
        ("forward".to_string(), 8),
        ("up".to_string(), 3),
        ("down".to_string(), 8),
        ("forward".to_string(), 2),
    ];
    let pos = complex_positions(movements);
    assert_eq!(pos, 900)
}
