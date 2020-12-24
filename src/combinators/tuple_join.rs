#[derive(Clone)]
pub struct TupleJoin<Iter, Item>
where
    Iter: Iterator<Item = Item> + Clone,
{
    _iter: Iter,
}

impl<Iter, Item> TupleJoin<Iter, Item>
where
    Iter: Iterator<Item = Item> + Clone,
{
    pub fn new(iter: Iter) -> Self {
        Self { _iter: iter }
    }
}

impl<Iter, A, B, C> Iterator for TupleJoin<Iter, (A, (B, C))>
where
    Iter: Iterator<Item = (A, (B, C))> + Clone,
{
    type Item = (A, B, C);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((a, (b, c))) = self._iter.next() {
            Some((a, b, c))
        } else {
            None
        }
    }
}

impl<Iter, A, B, C, D> Iterator for TupleJoin<Iter, (A, (B, C, D))>
where
    Iter: Iterator<Item = (A, (B, C, D))> + Clone,
{
    type Item = (A, B, C, D);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((a, (b, c, d))) = self._iter.next() {
            Some((a, b, c, d))
        } else {
            None
        }
    }
}

impl<Iter, A, B, C, D, E> Iterator for TupleJoin<Iter, (A, (B, C, D, E))>
where
    Iter: Iterator<Item = (A, (B, C, D, E))> + Clone,
{
    type Item = (A, B, C, D, E);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((a, (b, c, d, e))) = self._iter.next() {
            Some((a, b, c, d, e))
        } else {
            None
        }
    }
}
