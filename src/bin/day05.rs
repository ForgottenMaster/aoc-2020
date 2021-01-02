use {
    aoc::{airplane::seating::Seating, io::file::get_file_contents},
    std::io::Result,
};

fn get_seats() -> Result<Box<[Seating]>> {
    let mut seats = Vec::new();
    for line in get_file_contents("data/day05/input.txt")?.trim().lines() {
        seats.push(line.parse()?);
    }
    Ok(seats.into_boxed_slice())
}

fn main() -> Result<()> {
    let seats = get_seats()?;
    let seat_ids = seats
        .into_iter()
        .map(|seat| seat.seat_id())
        .collect::<Vec<_>>();
    let (min, max) = (
        *seat_ids.iter().min().unwrap(),
        *seat_ids.iter().max().unwrap(),
    );

    println!("Part 1 = {}", max);

    for i in min..=max {
        if !seat_ids.contains(&i) {
            println!("Part 2 = {}", i);
            break;
        }
    }

    Ok(())
}
