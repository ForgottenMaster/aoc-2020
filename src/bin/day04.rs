use {
    aoc::{io::file::get_file_contents, passport::PassportValidation},
    std::io::Result,
};

fn get_passport_validations() -> Result<Box<[PassportValidation]>> {
    let input = get_file_contents("data/day04/input.txt")?;
    let mut current_passport_validation = PassportValidation::default();
    let mut passport_validations = Vec::new();
    for line in input.trim().lines() {
        if line.is_empty() {
            passport_validations.push(current_passport_validation);
            current_passport_validation = PassportValidation::default();
        } else {
            current_passport_validation.merge(line.parse()?);
        }
    }
    Ok(passport_validations.into_boxed_slice())
}

fn main() -> Result<()> {
    let passport_validations = get_passport_validations()?;

    println!(
        "Part 1 = {}",
        passport_validations
            .iter()
            .filter(|passport_validation| passport_validation.valid())
            .count()
    );

    println!(
        "Part 2 = {}",
        passport_validations
            .iter()
            .filter(|passport_validation| passport_validation.valid_full())
            .count()
    );

    Ok(())
}
