use {
    aoc::{
        io::file::get_file_contents,
        iter::chunks::ProvideChunks, // extends the iterator with the chunks method
    },
    std::{
        collections::{HashMap, HashSet},
        io::Result,
    },
};

fn main() -> Result<()> {
    let input = get_file_contents("data/day06.txt")?;
    let iter = input.lines().chunks(|line: &&str| !line.trim().is_empty());

    // For part 1:
    // For each group, we can do a flat map of the lines to get the chars.
    // Flatmap will inline all the char iterators together so when we collect them into a
    // HashSet it will dump all characters in the group into the set, removing dups.
    // Then we just take the length of the hash set to get the count for that group.
    // The final answer is the sum of the group counts.
    println!(
        "Part 1 = {}",
        iter.clone()
            .map(|iter| {
                iter.flat_map(|line| line.chars())
                    .collect::<HashSet<_>>()
                    .len()
            })
            .sum::<usize>()
    );

    // For part 2:
    // The subtotal for each group is the number of characters where the count matches the
    // number of lines.
    // As we iterate over the lines, track the line count, and also add the characters of each line
    // into a hashmap, incrementing a count each time.
    // After that, we can simply do a filter on the hashmap to count only those entries where occurrence
    // count matches line count.
    println!(
        "Part 2 = {}",
        iter.map(|iter| {
            let mut map = HashMap::with_capacity(26);
            let mut lines = 0;
            for line in iter {
                lines += 1;
                for c in line.chars() {
                    *map.entry(c).or_insert(0) += 1;
                }
            }
            map.into_iter().filter(|(_, value)| *value == lines).count()
        })
        .sum::<usize>()
    );

    Ok(())
}
