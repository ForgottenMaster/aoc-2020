use std::{
    convert::TryFrom,
    io::{Error, ErrorKind},
};

#[derive(Debug)]
pub struct Bag<'a> {
    name: &'a str,
    contents: Box<[(u32, &'a str)]>,
}

impl<'a> Bag<'a> {
    pub fn name(&self) -> &'a str {
        &self.name
    }

    pub fn contents(&self) -> &[(u32, &'a str)] {
        &*self.contents
    }
}

impl<'a> TryFrom<&'a str> for Bag<'a> {
    type Error = Error;

    fn try_from(from: &'a str) -> Result<Self, Self::Error> {
        const SEARCH_STRING: &str = " bags contain ";
        if let Some(index) = from.find(SEARCH_STRING) {
            let name = &from[..index];
            let rest = &from[index + SEARCH_STRING.len()..];
            let mut contents = Vec::with_capacity(10);

            for substr in rest.split(", ") {
                if let Some(index) = substr.find(" bag") {
                    let substr = &substr[..index];
                    if substr != "no other" {
                        if let Some(index) = substr.find(" ") {
                            if let Ok(count) = substr[..index].parse() {
                                contents.push((count, &substr[index + 1..]));
                            } else {
                                return Err(Error::new(
                                    ErrorKind::InvalidData,
                                    format!("String invalid for conversion to Bag: {}", from),
                                ));
                            }
                        } else {
                            return Err(Error::new(
                                ErrorKind::InvalidData,
                                format!("String invalid for conversion to Bag: {}", from),
                            ));
                        }
                    }
                } else {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!("String invalid for conversion to Bag: {}", from),
                    ));
                }
            }

            Ok(Self {
                name,
                contents: contents.into_boxed_slice(),
            })
        } else {
            Err(Error::new(
                ErrorKind::InvalidData,
                format!("String invalid for conversion to Bag: {}", from),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_bag_contents() {
        let bag = Bag::try_from("light red bags contain 1 bright white bag.");
        assert!(bag.is_ok());
        let bag = bag.unwrap();
        assert_eq!(bag.name, "light red");
        assert_eq!(*bag.contents, [(1, "bright white")]);
    }

    #[test]
    fn test_no_other_bags() {
        let bag = Bag::try_from("dotted black bags contain no other bags.");
        assert!(bag.is_ok());
        let bag = bag.unwrap();
        assert_eq!(bag.name, "dotted black");
        assert_eq!(*bag.contents, []);
    }

    #[test]
    fn test_multiple_bags() {
        let bag = Bag::try_from("muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.");
        assert!(bag.is_ok());
        let bag = bag.unwrap();
        assert_eq!(bag.name, "muted yellow");
        assert_eq!(*bag.contents, [(2, "shiny gold"), (9, "faded blue")]);
    }
}
