#[derive(Clone)]
pub struct Cartesian<Iter1, Iter2>
where
    Iter1: Iterator + Clone,
    Iter2: Iterator + Clone,
    <Iter1 as Iterator>::Item: Clone,
{
    _iter_1: Iter1,
    _iter_2: Iter2,
    _item_1: Option<<Iter1 as Iterator>::Item>,
    _iter_2_template: Iter2,
}

impl<Iter1, Iter2> Cartesian<Iter1, Iter2>
where
    Iter1: Iterator + Clone,
    Iter2: Iterator + Clone,
    <Iter1 as Iterator>::Item: Clone,
{
    pub fn new(iter_1: Iter1, iter_2: Iter2) -> Self {
        let mut obj = Self {
            _iter_1: iter_1,
            _iter_2: iter_2.clone(),
            _item_1: None,
            _iter_2_template: iter_2,
        };
        obj._item_1 = obj._iter_1.next();
        obj
    }
}

impl<Iter1, Iter2> Iterator for Cartesian<Iter1, Iter2>
where
    Iter1: Iterator + Clone,
    Iter2: Iterator + Clone,
    <Iter1 as Iterator>::Item: Clone,
{
    type Item = (<Iter1 as Iterator>::Item, <Iter2 as Iterator>::Item);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if let Some(ref item_1) = self._item_1 {
            if let Some(item_2) = self._iter_2.next() {
                Some((item_1.clone(), item_2))
            } else {
                self._item_1 = self._iter_1.next();
                self._iter_2 = self._iter_2_template.clone(); // resets the second iterator to beginning
                self.next()
            }
        } else {
            None
        }
    }
}
