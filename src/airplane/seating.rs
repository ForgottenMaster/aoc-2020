use std::{
    io::{Error, ErrorKind, Result},
    str::FromStr,
};

const ROW_COUNT: u8 = 128;
const COL_COUNT: u8 = 8;

pub struct Seating {
    row_min: u8,
    row_max: u8,
    col_min: u8,
    col_max: u8,
}

pub enum SeatLocator {
    RowLower,
    RowUpper,
    ColLower,
    ColUpper,
}

fn extract_single_number(min: u8, max: u8) -> Option<u8> {
    if min == max {
        Some(min)
    } else {
        None
    }
}

impl Seating {
    pub fn new() -> Self {
        Self {
            row_min: 0,
            row_max: ROW_COUNT - 1,
            col_min: 0,
            col_max: COL_COUNT - 1,
        }
    }

    pub fn row_and_col(&self) -> (Option<u8>, Option<u8>) {
        (
            extract_single_number(self.row_min, self.row_max),
            extract_single_number(self.col_min, self.col_max),
        )
    }

    pub fn state(&self) -> ((u8, u8), (u8, u8)) {
        ((self.row_min, self.row_max), (self.col_min, self.col_max))
    }

    pub fn apply_locator(&mut self, locator: SeatLocator) {
        let half_size_row = (self.row_max - self.row_min) / 2;
        let half_size_col = (self.col_max - self.col_min) / 2;
        match locator {
            SeatLocator::RowLower => {
                self.row_max = self.row_min + half_size_row;
            }
            SeatLocator::RowUpper => {
                self.row_min = self.row_max - half_size_row;
            }
            SeatLocator::ColLower => {
                self.col_max = self.col_min + half_size_col;
            }
            SeatLocator::ColUpper => {
                self.col_min = self.col_max - half_size_col;
            }
        }
    }

    pub fn seat_id(&self) -> u16 {
        if let (Some(row), Some(col)) = self.row_and_col() {
            row as u16 * 8 + col as u16
        } else {
            0
        }
    }
}

impl FromStr for Seating {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self> {
        let mut seating = Self::new();
        for c in string.trim().chars() {
            seating.apply_locator(match c {
                'F' => Ok(SeatLocator::RowLower),
                'B' => Ok(SeatLocator::RowUpper),
                'L' => Ok(SeatLocator::ColLower),
                'R' => Ok(SeatLocator::ColUpper),
                _ => Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Invalid input data: {}", string),
                )),
            }?);
        }
        Ok(seating)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_initial_state() {
        assert_eq!(
            Seating::new().state(),
            ((0, ROW_COUNT - 1), (0, COL_COUNT - 1))
        );
    }

    #[test]
    fn select_row_lower() {
        let mut seating = Seating::new();
        seating.apply_locator(SeatLocator::RowLower);
        assert_eq!(
            seating.state(),
            ((0, (ROW_COUNT - 1) / 2), (0, COL_COUNT - 1))
        );
    }

    #[test]
    fn select_row_upper() {
        let mut seating = Seating::new();
        seating.apply_locator(SeatLocator::RowUpper);
        assert_eq!(
            seating.state(),
            (((ROW_COUNT - 1) / 2 + 1, ROW_COUNT - 1), (0, COL_COUNT - 1))
        );
    }

    #[test]
    fn select_col_lower() {
        let mut seating = Seating::new();
        seating.apply_locator(SeatLocator::ColLower);
        assert_eq!(
            seating.state(),
            ((0, ROW_COUNT - 1), (0, (COL_COUNT - 1) / 2))
        );
    }

    #[test]
    fn select_col_upper() {
        let mut seating = Seating::new();
        seating.apply_locator(SeatLocator::ColUpper);
        assert_eq!(
            seating.state(),
            ((0, ROW_COUNT - 1), ((COL_COUNT - 1) / 2 + 1, COL_COUNT - 1))
        );
    }

    #[test]
    fn no_row_selected() {
        let mut seating = Seating::new();
        seating.apply_locator(SeatLocator::RowLower);
        seating.apply_locator(SeatLocator::RowUpper);
        seating.apply_locator(SeatLocator::RowLower);
        seating.apply_locator(SeatLocator::RowUpper);
        seating.apply_locator(SeatLocator::RowUpper);
        seating.apply_locator(SeatLocator::RowLower);
        assert_eq!(seating.state(), ((44, 45), (0, COL_COUNT - 1)));
    }

    #[test]
    fn row_selected() {
        let mut seating = Seating::new();
        seating.apply_locator(SeatLocator::RowLower);
        seating.apply_locator(SeatLocator::RowUpper);
        seating.apply_locator(SeatLocator::RowLower);
        seating.apply_locator(SeatLocator::RowUpper);
        seating.apply_locator(SeatLocator::RowUpper);
        seating.apply_locator(SeatLocator::RowLower);
        seating.apply_locator(SeatLocator::RowLower);
        assert_eq!(seating.row_and_col(), (Some(44), None));
    }

    #[test]
    fn no_col_selected() {
        let mut seating = Seating::new();
        seating.apply_locator(SeatLocator::ColUpper);
        seating.apply_locator(SeatLocator::ColLower);
        assert_eq!(seating.state(), ((0, ROW_COUNT - 1), (4, 5)));
    }

    #[test]
    fn col_selected() {
        let mut seating = Seating::new();
        seating.apply_locator(SeatLocator::ColUpper);
        seating.apply_locator(SeatLocator::ColLower);
        seating.apply_locator(SeatLocator::ColUpper);
        assert_eq!(seating.row_and_col(), (None, Some(5)));
    }

    #[test]
    fn full_decode() -> Result<()> {
        assert_eq!(
            "FBFBBFFRLR".parse::<Seating>()?.row_and_col(),
            (Some(44), Some(5))
        );
        Ok(())
    }

    #[test]
    fn seat_id() -> Result<()> {
        assert_eq!("FBFBBFFRLR".parse::<Seating>()?.seat_id(), 357);
        Ok(())
    }
}
