use {
    aoc::{
        game_console::program::{Program, ProgramResult},
        io::file::get_file_contents,
    },
    std::io::Result,
};

fn main() -> Result<()> {
    // prepare input
    let mut program = get_file_contents("data/day08.txt")?.parse::<Program>()?;

    // part 1 - detect second visit to a given program counter
    println!(
        "Part 1 = {}",
        if let ProgramResult::InfiniteLoop(acc) = program.execute_and_reset() {
            acc
        } else {
            0
        }
    );

    // part 2 - loop through each instruction, execute program while trying to flip the instruction.
    // only pay attention to the Termination result.
    println!("Part 2 = {}", {
        let mut result = 0;
        for i in 0..program.len() {
            if let ProgramResult::Termination(acc) =
                program.execute_with_flipped_operation_and_reset(i)
            {
                result = acc;
                break;
            }
        }
        result
    });
    Ok(())
}
