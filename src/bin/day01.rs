use aoc::{cartesian, io::file::get_file_contents};
use std::io::Result;

const SOLUTION: u64 = 2020;

fn calculate_solution<I, F>(iter: I, sum_and_product_func: F) -> u64
where
    I: Iterator,
    F: Fn(<I as Iterator>::Item) -> (u64, u64),
{
    iter.map(sum_and_product_func)
        .filter(|(sum, _)| *sum == SOLUTION)
        .next()
        .unwrap()
        .1
}

fn main() -> Result<()> {
    let numbers = get_file_contents("data/day01/input.txt")?
        .trim()
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    println!(
        "Part 1 = {}",
        calculate_solution(
            cartesian!(numbers.iter(), numbers.iter()),
            |(first, second)| { (first + second, first * second) }
        )
    );
    println!(
        "Part 2 = {}",
        calculate_solution(
            cartesian!(numbers.iter(), numbers.iter(), numbers.iter()),
            |(first, second, third)| { (first + second + third, first * second * third) }
        )
    );

    Ok(())
}
