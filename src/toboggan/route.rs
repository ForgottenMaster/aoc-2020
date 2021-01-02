pub struct Route<'a> {
    map: &'a Map,
    x_delta: usize,
    y_delta: usize,
    x: usize,
    y: usize
}

impl Route<'_> {
    pub fn new(map: &Map, x_delta: usize, y_delta: usize) -> Self {
        let x = 0;
        let y = 0;
        Self {
            map,
            x_delta,
            y_delta,
            x,
            y
        }
    }
}
