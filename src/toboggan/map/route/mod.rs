use super::{Map, Space};

pub struct Route<'a> {
    map: &'a Map,
    x_delta: usize,
    y_delta: usize,
    x: usize,
    y: usize,
}

impl<'a> Route<'a> {
    pub fn new(map: &'a Map, x_delta: usize, y_delta: usize) -> Self {
        let x = 0;
        let y = 0;
        Self {
            map,
            x_delta,
            y_delta,
            x,
            y,
        }
    }
}

impl<'a> Iterator for Route<'a> {
    type Item = &'a Space;

    fn next(&mut self) -> Option<Self::Item> {
        self.x = (self.x + self.x_delta) % self.map.cols; // x wraps around after reaching last column.
        self.y += self.y_delta;
        let index = self.y * self.map.cols + self.x;
        if self.y >= self.map.rows {
            None
        } else {
            Some(self.map.repr.iter().nth(index).unwrap())
        }
    }
}
