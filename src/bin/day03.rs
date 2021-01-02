use {
    aoc::{
        io::file::get_file_contents,
        toboggan::map::{Map, Space},
    },
    std::io::Result,
};

fn main() -> Result<()> {
    let map = get_file_contents("data/day03/input.txt")?.parse::<Map>()?;

    println!(
        "Part 1 = {}",
        map.follow_route(3, 1)
            .filter(|space| { **space == Space::Tree })
            .count()
    );

    let routes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    println!(
        "Part 2 = {}",
        routes
            .iter()
            .map(|(x, y)| {
                map.follow_route(*x, *y)
                    .filter(|space| **space == Space::Tree)
                    .count()
            })
            .product::<usize>()
    );

    Ok(())
}
