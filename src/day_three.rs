use anyhow::Result;

pub fn run_day_three() -> Result<()> {
    let input: Vec<Vec<char>> = std::fs::read_to_string("inputs/day3.txt")?
        .lines()
        .map(|s| s.chars().collect())
        .collect();

    let power = power_consumption(input.clone())?;
    println!("The power consumption is: {}", power);
    let ls = life_supply(input)?;
    println!("The life supply is: {}", ls);
    Ok(())
}

fn binaries_for_power(input: Vec<Vec<char>>) -> Result<(String, String)> {
    let mut gama = String::new();
    let mut epsilon = String::new();
    let len = input[0].len();
    for i in 0..len {
        let mut ones: u32 = 0;
        let mut zeros: u32 = 0;
        for line in &input {
            match line[i] {
                '0' => zeros += 1,
                '1' => ones += 1,
                _ => panic!("invalid input"),
            }
        }
        if ones > zeros {
            gama.push('1');
            epsilon.push('0');
        } else {
            gama.push('0');
            epsilon.push('1');
        }
    }
    Ok((gama, epsilon))
}

fn power_consumption(input: Vec<Vec<char>>) -> Result<isize> {
    let (gama, epsilon) = binaries_for_power(input)?;
    let gama = isize::from_str_radix(gama.as_str(), 2)?;
    let epsilon = isize::from_str_radix(epsilon.as_str(), 2)?;
    Ok(gama * epsilon)
}

fn binaries_for_life(mut input: Vec<Vec<char>>, desition: fn(u32, u32) -> char) -> Result<String> {
    let mut i = 0;
    while input.len() > 1 {
        let mut ones: u32 = 0;
        let mut zeros: u32 = 0;
        for line in &input {
            match line[i] {
                '0' => zeros += 1,
                '1' => ones += 1,
                _ => panic!("invalid input"),
            }
        }
        let ch = desition(ones, zeros);
        input = input.into_iter().filter(|v| v[i] == ch).collect();
        i += 1;
    }
    let result: String = input[0].clone().into_iter().collect();
    Ok(result)
}

fn life_supply(input: Vec<Vec<char>>) -> Result<isize> {
    let des = |ones: u32, zeros: u32| -> char {
        let ch: char;
        if ones >= zeros {
            ch = '0';
        } else {
            ch = '1';
        }
        ch
    };
    let oxigen = binaries_for_life(input.clone(), des)?;
    let des = |ones: u32, zeros: u32| -> char {
        let ch: char;
        if ones >= zeros {
            ch = '1';
        } else {
            ch = '0';
        }
        ch
    };
    let co_scrubber = binaries_for_life(input, des)?;
    let oxigen = isize::from_str_radix(oxigen.as_str(), 2)?;
    let co_scrubber = isize::from_str_radix(co_scrubber.as_str(), 2)?;
    Ok(oxigen * co_scrubber)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binaries_for_power() -> Result<()> {
        let input: Vec<Vec<char>> = vec![
            vec!['0', '0', '1', '0', '0'],
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '0', '1', '1', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '0', '1', '0', '1'],
            vec!['0', '1', '1', '1', '1'],
            vec!['0', '0', '1', '1', '1'],
            vec!['1', '1', '1', '0', '0'],
            vec!['1', '0', '0', '0', '0'],
            vec!['1', '1', '0', '0', '1'],
            vec!['0', '0', '0', '1', '0'],
            vec!['0', '1', '0', '1', '0'],
        ];
        let (gama, epsilon) = binaries_for_power(input)?;
        assert_eq!(gama, "10110");
        assert_eq!(epsilon, "01001");
        Ok(())
    }

    #[test]
    fn test_power_consumption() -> Result<()> {
        let input: Vec<Vec<char>> = vec![
            vec!['0', '0', '1', '0', '0'],
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '0', '1', '1', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '0', '1', '0', '1'],
            vec!['0', '1', '1', '1', '1'],
            vec!['0', '0', '1', '1', '1'],
            vec!['1', '1', '1', '0', '0'],
            vec!['1', '0', '0', '0', '0'],
            vec!['1', '1', '0', '0', '1'],
            vec!['0', '0', '0', '1', '0'],
            vec!['0', '1', '0', '1', '0'],
        ];
        let power = power_consumption(input)?;
        assert_eq!(power, 198);
        Ok(())
    }

    #[test]
    fn test_binaries_for_life_oxygen() -> Result<()> {
        let input: Vec<Vec<char>> = vec![
            vec!['0', '0', '1', '0', '0'],
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '0', '1', '1', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '0', '1', '0', '1'],
            vec!['0', '1', '1', '1', '1'],
            vec!['0', '0', '1', '1', '1'],
            vec!['1', '1', '1', '0', '0'],
            vec!['1', '0', '0', '0', '0'],
            vec!['1', '1', '0', '0', '1'],
            vec!['0', '0', '0', '1', '0'],
            vec!['0', '1', '0', '1', '0'],
        ];
        let des = |ones: u32, zeros: u32| -> char {
            let ch: char;
            if ones >= zeros {
                ch = '1';
            } else {
                ch = '0';
            }
            ch
        };
        let ox = binaries_for_life(input, des)?;
        assert_eq!(ox, "10111");
        Ok(())
    }

    #[test]
    fn test_binaries_for_life_co() -> Result<()> {
        let input: Vec<Vec<char>> = vec![
            vec!['0', '0', '1', '0', '0'],
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '0', '1', '1', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '0', '1', '0', '1'],
            vec!['0', '1', '1', '1', '1'],
            vec!['0', '0', '1', '1', '1'],
            vec!['1', '1', '1', '0', '0'],
            vec!['1', '0', '0', '0', '0'],
            vec!['1', '1', '0', '0', '1'],
            vec!['0', '0', '0', '1', '0'],
            vec!['0', '1', '0', '1', '0'],
        ];
        let des = |ones: u32, zeros: u32| -> char {
            let ch: char;
            if ones >= zeros {
                ch = '0';
            } else {
                ch = '1';
            }
            ch
        };
        let co = binaries_for_life(input, des)?;
        assert_eq!(co, "01010");
        Ok(())
    }

    #[test]
    fn test_life_supply() -> Result<()> {
        let input: Vec<Vec<char>> = vec![
            vec!['0', '0', '1', '0', '0'],
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '0', '1', '1', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '0', '1', '0', '1'],
            vec!['0', '1', '1', '1', '1'],
            vec!['0', '0', '1', '1', '1'],
            vec!['1', '1', '1', '0', '0'],
            vec!['1', '0', '0', '0', '0'],
            vec!['1', '1', '0', '0', '1'],
            vec!['0', '0', '0', '1', '0'],
            vec!['0', '1', '0', '1', '0'],
        ];
        let life_supply = life_supply(input)?;
        assert_eq!(life_supply, 230);
        Ok(())
    }
}
