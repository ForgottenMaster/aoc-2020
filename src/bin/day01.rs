use aoc::cartesian;
use std::{
    fs::File,
    io::{Read, Result},
};

fn get_input() -> Result<String> {
    let mut file = File::open("data/day01/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> Result<()> {
    let numbers = get_input()?
        .trim()
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    println!("Part 1 = {}", {
        let (_, product) = cartesian!(numbers.iter(), numbers.iter())
            .map(|(value1, value2)| (value1 + value2, value1 * value2))
            .filter(|(sum, _)| *sum == 2020)
            .next()
            .unwrap();
        product
    });
    
    println!("Part 2 = {}", {
        let (_, product) = cartesian!(numbers.iter(), numbers.iter(), numbers.iter())
            .map(|(value1, value2, value3)| (value1 + value2 + value3, value1 * value2 * value3))
            .filter(|(sum, _)| *sum == 2020)
            .next()
            .unwrap();
        product
    });

    Ok(())
}
