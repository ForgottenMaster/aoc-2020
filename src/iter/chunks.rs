use std::iter::{Peekable, TakeWhile};

pub struct Chunks<I: Iterator, P> {
    iter: Peekable<I>,
    predicate: P,
}

impl<I: Iterator + Clone, P: Clone> Clone for Chunks<I, P>
where
    <I as Iterator>::Item: Clone,
{
    fn clone(&self) -> Self {
        Self {
            iter: self.iter.clone(),
            predicate: self.predicate.clone(),
        }
    }
}

impl<I: Iterator + Clone, P: FnMut(&<I as Iterator>::Item) -> bool + Clone> Iterator
    for Chunks<I, P>
where
    <I as Iterator>::Item: Clone,
{
    type Item = TakeWhile<Peekable<I>, P>;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.iter.peek() {
            if !(self.predicate)(item) {
                self.iter.next(); // advance the iterator properly to the next location.
            } else {
                let chunk_iterator = self.iter.clone().take_while(self.predicate.clone());
                // we've taken a snapshot of the iterator from the correct position for the purposes of
                // returning the chunk, but we will need to advance the main iterator to the end of the chunk
                // for next time.
                while let Some(item) = self.iter.peek() {
                    if (self.predicate)(item) {
                        self.iter.next();
                    } else {
                        break;
                    }
                }
                return Some(chunk_iterator);
            }
        }
        None // no more peekable items
    }
}

pub trait ProvideChunks: Iterator + Sized {
    fn chunks<P>(self, predicate: P) -> Chunks<Self, P>;
}

impl<T: Iterator> ProvideChunks for T {
    fn chunks<P>(self, predicate: P) -> Chunks<Self, P> {
        Chunks {
            iter: self.peekable(),
            predicate,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_chunk() {
        const INPUT: &str = "abc\nblahblah\npfft";
        let mut chunks = INPUT.lines().chunks(|line: &&str| !line.trim().is_empty());
        let mut chunk = chunks.next().unwrap();
        assert_eq!(chunk.next().unwrap(), "abc");
        assert_eq!(chunk.next().unwrap(), "blahblah");
        assert_eq!(chunk.next().unwrap(), "pfft");
        assert!(chunk.next().is_none());
        assert!(chunks.next().is_none());
    }

    #[test]
    fn test_empty_preceding() {
        const INPUT: &str = "        \nabc\ndef";
        let mut chunks = INPUT.lines().chunks(|line: &&str| !line.trim().is_empty());
        let mut chunk = chunks.next().unwrap();
        assert_eq!(chunk.next().unwrap(), "abc");
        assert_eq!(chunk.next().unwrap(), "def");
        assert!(chunk.next().is_none());
        assert!(chunks.next().is_none());
    }

    #[test]
    fn test_empty_following() {
        const INPUT: &str = "abc\ndef\n    ";
        let mut chunks = INPUT.lines().chunks(|line: &&str| !line.trim().is_empty());
        let mut chunk = chunks.next().unwrap();
        assert_eq!(chunk.next().unwrap(), "abc");
        assert_eq!(chunk.next().unwrap(), "def");
        assert!(chunk.next().is_none());
        assert!(chunks.next().is_none());
    }

    #[test]
    fn test_two_chunks() {
        const INPUT: &str = "abc\ndef\n\nhi";
        let mut chunks = INPUT.lines().chunks(|line: &&str| !line.trim().is_empty());
        let mut chunk = chunks.next().unwrap();
        assert_eq!(chunk.next().unwrap(), "abc");
        assert_eq!(chunk.next().unwrap(), "def");
        assert!(chunk.next().is_none());
        let mut chunk = chunks.next().unwrap();
        assert_eq!(chunk.next().unwrap(), "hi");
        assert!(chunk.next().is_none());
        assert!(chunks.next().is_none());
    }
}
