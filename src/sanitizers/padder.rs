use core::iter::Peekable;
use core::str::Chars;

use crate::sanitizer::Sanitizer;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Padder<const P: char, const N: usize> {
    strings: [&'static str; N],
    insert_before: Option<char>,
}

impl<const P: char, const N: usize> Padder<P, N> {
    #[must_use]
    pub fn new(strings: [&'static str; N], insert_before: Option<char>) -> Self {
        Self {
            strings,
            insert_before,
        }
    }
}

impl<const P: char, const N: usize> Sanitizer for Padder<P, N> {
    type IntoIter<I: Iterator<Item = char>> = PadderIter<I, P, N>;

    fn sanitize<I: IntoIterator<Item = char>>(self, iter: I) -> Self::IntoIter<I::IntoIter> {
        PadderIter::new(iter.into_iter(), self.strings, self.insert_before)
    }
}

// TODO: implement traits
pub struct PadderIter<I: Iterator<Item = char>, const P: char, const N: usize> {
    iter: I,
    strings: [Peekable<Chars<'static>>; N],
    matched: bool,
    insert_before: Option<char>,
}

impl<I: Iterator<Item = char>, const P: char, const N: usize> PadderIter<I, P, N> {
    #[must_use]
    pub fn new(iter: I, strings: [&'static str; N], insert_before: Option<char>) -> Self {
        Self {
            iter,
            strings: strings.map(|s| s.chars().peekable()),
            matched: false,
            insert_before,
        }
    }
}

// TODO: use insert_before?
impl<I: Iterator<Item = char>, const P: char, const N: usize> Iterator for PadderIter<I, P, N> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let next_c = {
            if let Some(c) = self.iter.next() {
                self.matched = false;
                c
            } else if self.matched {
                self.matched = false;
                return Some(P);
            } else {
                return None;
            }
        };

        for chars in self.strings.iter_mut() {
            if let Some(c) = chars.next() {
                if c != next_c {
                    // exhaust the iterator, because it does not match the input
                    chars.for_each(drop);
                } else if chars.peek().is_none() {
                    // the input does match the string => apply padding in the next iteration
                    self.matched = true;
                }
            }
        }

        Some(next_c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_simple() {
        let padder: Padder<'\u{FFFD}', 3> = Padder::new(["foo", "barin", "bazing"], None);
        let string = "foo";
        let mut iter = padder.sanitize(string.chars());

        assert_eq!(iter.next(), Some('f'));
        assert_eq!(iter.next(), Some('o'));
        assert_eq!(iter.next(), Some('o'));
        assert_eq!(iter.next(), Some('\u{FFFD}'));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_with_multiple_match() {
        let padder: Padder<'\u{FFFD}', 3> = Padder::new(["foob", "foo", "fo"], None);
        let string = "foob";
        let mut iter = padder.sanitize(string.chars());

        assert_eq!(iter.next(), Some('f'));
        assert_eq!(iter.next(), Some('o'));
        assert_eq!(iter.next(), Some('o'));
        assert_eq!(iter.next(), Some('b'));
        assert_eq!(iter.next(), Some('\u{FFFD}'));
        assert_eq!(iter.next(), None);
    }
}
