use anyhow::Result;

pub fn run_day_one() -> Result<()> {
    let measures: Vec<i32> = std::fs::read_to_string("inputs/day1.txt")?
        .lines()
        .into_iter()
        .filter_map(|s| s.parse().ok())
        .collect();

    let count = count_increased(measures.clone());
    println!("The measures increased: {} times", count);
    let count = count_increased(clean_measures(measures));
    println!("The measures increased: {} times", count);
    Ok(())
}

fn count_increased(measures: Vec<i32>) -> i32 {
    let mut count = -1;
    measures.into_iter().fold(0, |acc, m| {
        if m > acc {
            count += 1;
        }
        m
    });
    count
}

fn clean_measures(measures: Vec<i32>) -> Vec<i32> {
    let mut new_measurents = Vec::new();
    for (i, m) in measures.iter().enumerate() {
        if i + 2 < measures.len() {
            new_measurents.push(m + measures[i + 1] + measures[i + 2])
        }
    }
    new_measurents
}

#[test]
fn count_increased_test() {
    let measures = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let count = count_increased(measures);
    assert_eq!(7, count);
}

#[test]
fn clean_measures_test() {
    let measures = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let count = clean_measures(measures);
    assert_eq!(8, count.len());
    let mut it = count.into_iter();
    assert_eq!(607, it.next().unwrap());
    assert_eq!(618, it.next().unwrap());
    assert_eq!(792, it.last().unwrap());
}
