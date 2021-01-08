use {
    aoc::{
        luggage::bag_registry::BagRegistry,
        io::file::get_file_contents
    },
    std::{
        convert::TryFrom,
        io::Result
    }
};

fn main() -> Result<()> {
    let file_contents = get_file_contents("data/day07.txt")?;
    let bag_registry = BagRegistry::try_from(&file_contents as &str).unwrap();
    
    println!("Part 1 = {}", bag_registry.find_containers("shiny gold").count());
    println!("Part 2 = {}", bag_registry.count_nested("shiny gold"));
    
    Ok(())
}
