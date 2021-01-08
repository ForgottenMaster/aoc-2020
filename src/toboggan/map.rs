pub mod route;

use {
    route::Route,
    std::{
        io::{Error, ErrorKind, Result},
        str::FromStr,
    },
};

#[derive(Debug, PartialEq)]
pub enum Space {
    Empty,
    Tree,
}

#[derive(Debug)]
pub struct Map {
    rows: usize,
    cols: usize,
    repr: Box<[Space]>, // Using boxed slice here gives us the guarantee the map doesn't change after initial allocation.
}

const EMPTY_CELL: char = '.';
const TREE_CELL: char = '#';

impl Map {
    pub fn follow_route(&self, x_delta: usize, y_delta: usize) -> Route {
        Route::new(&self, x_delta, y_delta)
    }
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        // validate input shape and character set which avoids allocating
        let mut rows = 0;
        let mut cols = 0;
        for line in input.lines() {
            cols = {
                let line_length = line.len();
                if rows > 0 && cols != line_length {
                    Err(Error::new(
                        ErrorKind::InvalidData,
                        "Every line in input data must have the same number of characters.",
                    ))
                } else {
                    let mut result = Ok(line_length);
                    for c in line.chars() {
                        if c != EMPTY_CELL && c != TREE_CELL {
                            result = Err(Error::new(
                                ErrorKind::InvalidData,
                                format!(
                                    "The only valid characters are '{}' and '{}'",
                                    EMPTY_CELL, TREE_CELL
                                ),
                            ));
                            break;
                        }
                    }
                    result
                }
            }?; // question mark causes early return of errors from function
            rows += 1;
        }

        // if we're here we didn't hit an error with the validation, so can easily construct the map and allocate
        // the storage space now.
        let repr = input
            .lines()
            .flat_map(|line| {
                line.chars().map(|c| {
                    if c == EMPTY_CELL {
                        Space::Empty
                    } else {
                        Space::Tree
                    }
                })
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();

        // All data is prepared for us to construct the validated map.
        Ok(Self { rows, cols, repr })
    }
}
