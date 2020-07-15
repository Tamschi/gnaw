//! Treat [`mut &[]`] and [`mut &str`] (almost) like a [`mut Vec`]!
//!
//! With this crate, you can conveniently chip pieces off a [slice] or [`str`] to use elsewhere.
//!
//! # Example
//!
//! ```rust
//! use gnaw::{Drain as _, Pop as _, Unshift as _};
//!
//! let mut text = "abcdefg";
//!
//! assert_eq!(text.unshift(), Some('a'));
//! assert_eq!(text.pop(), Some('g'));
//!
//! let mut drain = text.drain();
//! assert_eq!(drain.next(), Some('b'));
//! assert_eq!(drain.next(), Some('c'));
//! assert_eq!(drain.next_back(), Some('f'));
//! assert_eq!(drain.next_back(), Some('e'));
//! drop(drain);
//!
//! //TODO: Why isn't the mutable borrow released here?
//! //assert_eq!(text, "d");
//! ```

#![warn(clippy::pedantic)]

mod drains;
use drains::{SliceDrain, StrDrain};

pub trait Drain<'a, T> {
    type Target: DoubleEndedIterator<Item = T> + 'a;
    fn drain(&'a mut self) -> Self::Target;
}

pub trait Pop<T> {
    fn pop(&mut self) -> Option<T>;
}

pub trait Unshift<T> {
    fn unshift(&mut self) -> Option<T>;
}

impl<'a, T: 'a> Drain<'a, &'a T> for &'a [T] {
    type Target = SliceDrain<'a, T>;
    fn drain(&'a mut self) -> Self::Target {
        SliceDrain(self)
    }
}

// TODO
// impl<'a, T: 'a> Drain<'a, &'a mut T> for &'a mut [T] {
//     type Target = MutSliceDrain<'a, T>;
//     fn drain(&'a mut self) -> Self::Target {
//         MutSliceDrain(self)
//     }
// }

impl<'a> Drain<'a, char> for &'a str {
    type Target = StrDrain<'a>;
    fn drain(&'a mut self) -> Self::Target {
        StrDrain(self)
    }
}

// TODO
// impl<'a, T> Pop<&'a mut T> for &'a mut [T] {
//     fn pop(&mut self) -> Option<&'a mut T> {
//         self.split_last_mut().map(|(t, rest)| -> &'a mut T {
//             *self = rest;
//             t
//         })
//     }
// }

impl<'a, T> Pop<&'a T> for &'a [T] {
    fn pop(&mut self) -> Option<&'a T> {
        self.split_last().map(|(t, rest)| {
            *self = rest;
            t
        })
    }
}

impl Pop<char> for &str {
    fn pop(&mut self) -> Option<char> {
        let c = self.chars().last();
        if let Some(c) = c {
            *self = &self[..self.len() - c.len_utf8()];
        }
        c
    }
}

// TODO
// impl<'a, T> Unshift<&'a mut T> for &'a mut [T] {
//     fn unshift(&mut self) -> Option<&'a mut T> {
//         self.split_first_mut().map(|(t, rest)| -> &'a mut T {
//             *self = rest;
//             t
//         })
//     }
// }

impl<'a, T> Unshift<&'a T> for &'a [T] {
    fn unshift(&mut self) -> Option<&'a T> {
        self.split_first().map(|(t, rest)| {
            *self = rest;
            t
        })
    }
}

impl Unshift<char> for &str {
    fn unshift(&mut self) -> Option<char> {
        let c = self.chars().next();
        if let Some(c) = c {
            *self = &self[c.len_utf8()..];
        }
        c
    }
}
