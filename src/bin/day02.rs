use {
    aoc::{
        io::file::get_file_contents, 
        password::{
            password::Password,
            policy::Scheme
        }
    },
    std::io::Result,
};

fn main() -> Result<()> {
    let passwords = get_file_contents("data/day02/input.txt")?
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<Password>>();

    println!(
        "Part 1 = {}",
        passwords
            .iter()
            .filter(|password| password.is_valid(Scheme::OccurrenceCount))
            .count()
    );
    
    println!("Part 2 = {}", passwords.iter().filter(|password| password.is_valid(Scheme::PositionCheck)).count());

    Ok(())
}
