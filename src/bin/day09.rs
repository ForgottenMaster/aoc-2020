use {
    aoc::{
        io::file::get_file_contents,
        xmas::{Message, Preamble, XMAS},
    },
    std::io::Result,
};

const PREAMBLE_LENGTH: usize = 25;

fn main() -> Result<()> {
    let input = get_file_contents("data/day09.txt")?;
    let preamble_end_index = input
        .char_indices()
        .filter(|(_, c)| *c == '\n')
        .skip(PREAMBLE_LENGTH - 1)
        .next()
        .unwrap()
        .0;

    let (number, message) = {
        let preamble = input[0..preamble_end_index].parse::<Preamble>().unwrap();
        let message = input[preamble_end_index + 1..].parse::<Message>().unwrap();
        let cloned = message.clone();
        let xmas = XMAS::new(preamble, message);
        let number = xmas.filter(|(_, valid)| !valid).next().unwrap().0;
        println!("Part 1 = {}", number);
        (number, cloned)
    };

    let found_sequence = message.find_sequence_totalling(number);
    let min = found_sequence.iter().min().unwrap();
    let max = found_sequence.iter().max().unwrap();
    println!("Part 2 = {}", min + max);

    Ok(())
}
