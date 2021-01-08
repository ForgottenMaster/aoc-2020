use {
    crate::luggage::bag::Bag,
    std::{collections::HashMap, convert::TryFrom, io::{Error, ErrorKind}},
};

#[derive(Debug)]
pub struct BagRegistry<'a> {
    mapping: HashMap<&'a str, Bag<'a>>,
}

impl<'a> BagRegistry<'a> {
    pub fn len(&self) -> usize {
        self.mapping.len()
    }
    
    pub fn find_containers(&'a self, needle: &'a str) -> impl Iterator<Item = &'a str> {
        self.mapping.keys().filter(move |key| {
            self.bag_contains(*key, needle)
        }).map(|key| *key)
    }
    
    fn bag_contains(&self, name: &'a str, needle: &'a str) -> bool {
        for (_, contains) in self.mapping[name].contents() {
            if (*contains == needle) || self.bag_contains(*contains, needle) {
                return true;
            }
        }
        false
    }
    
    pub fn count_nested(&self, outermost_name: &str) -> u32 {
        let contents = self.mapping[outermost_name].contents();
        if contents.len() == 0 {
            0
        } else {
            let mut count = 0;
            for (number, name) in contents {
                count += (1 + self.count_nested(name)) * number; // add the bag itself plus the number of containing bags, multiplied by the count.
            }
            count
        }
    }
}

impl<'a> TryFrom<&'a str> for BagRegistry<'a> {
    type Error = Error;

    fn try_from(string: &'a str) -> Result<Self, Self::Error> {
        let mapping = string
            .trim()
            .lines()
            .map(|line| Bag::try_from(line))
            .filter(|result| result.is_ok())
            .map(|result| {
                let result = result.unwrap();
                (result.name(), result)
            })
            .collect::<HashMap<&'a str, Bag<'a>>>();
        
        for (_, bag) in &mapping {
            for (_, contains) in bag.contents() {
                if !mapping.contains_key(contains) {
                    return Err(Error::new(ErrorKind::InvalidData, format!("Incomplete bag registry data, missing contents of bag: {}", contains)));
                }
            }
        }
        
        Ok(Self { mapping })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_bags() {
        let registry = BagRegistry::try_from("");
        assert!(registry.is_ok());
        let registry = registry.unwrap();
        assert_eq!(registry.len(), 0);
    }
    
    #[test]
    fn test_incomplete_bag_registry() {
        let registry = BagRegistry::try_from("light red bags contain 1 bright white bag, 2 muted yellow bags.");
        assert!(registry.is_err());
        assert_eq!(format!("{}", registry.unwrap_err()), "Incomplete bag registry data, missing contents of bag: bright white");
    }
    
    #[test]
    fn test_complete_single_bag() {
        let registry = BagRegistry::try_from("faded blue bags contain no other bags.");
        assert!(registry.is_ok());
        assert_eq!(registry.unwrap().len(), 1);
    }
    
    #[test]
    fn test_complete_three_bags() {
        let registry = BagRegistry::try_from("vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\nfaded blue bags contain no other bags.\ndotted black bags contain no other bags.");
        assert!(registry.is_ok());
        assert_eq!(registry.unwrap().len(), 3);
    }
    
    #[test]
    fn test_find_shiny_gold_bag() {
        use std::collections::HashSet;
        let registry = BagRegistry::try_from("light red bags contain 1 bright white bag, 2 muted yellow bags.\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\nbright white bags contain 1 shiny gold bag.\nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\nfaded blue bags contain no other bags.\ndotted black bags contain no other bags.");
        assert!(registry.is_ok());
        let registry = registry.unwrap();
        assert_eq!(registry.len(), 9);
        let results = registry.find_containers("shiny gold").collect::<HashSet<_>>();
        let expected = {
            let mut expected = HashSet::with_capacity(4);
            expected.insert("bright white");
            expected.insert("muted yellow");
            expected.insert("dark orange");
            expected.insert("light red");
            expected
        };
        assert_eq!(results, expected);
    }
}
