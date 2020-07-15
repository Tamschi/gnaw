use crate::{Pop as _, Unshift as _};

pub struct SliceDrain<'a, T>(pub(crate) &'a mut &'a [T]);

impl<'a, T> Iterator for SliceDrain<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.unshift()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.iter().size_hint()
    }
}

impl<'a, T> DoubleEndedIterator for SliceDrain<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct StrDrain<'a>(pub(crate) &'a mut &'a str);

impl<'a> Iterator for StrDrain<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.unshift()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.chars().size_hint()
    }
}

impl<'a> DoubleEndedIterator for StrDrain<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}
