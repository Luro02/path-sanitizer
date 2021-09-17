use core::iter::{FusedIterator, Peekable};

use crate::sanitizer::Sanitizer;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Padder<I: Iterator<Item = char>, const P: char, const N: usize> {
    strings: [I; N],
    insert_before: Option<char>,
}

impl<I: Iterator<Item = char>, const P: char, const N: usize> Padder<I, P, N> {
    #[must_use]
    pub fn new(strings: [I; N], insert_before: Option<char>) -> Self {
        Self {
            strings,
            insert_before,
        }
    }
}

impl<C: Iterator<Item = char>, const P: char, const N: usize> Sanitizer for Padder<C, P, N> {
    type Iter<I: Iterator<Item = char>> = PadderIter<I, C, P, N>;

    fn sanitize<I: Iterator<Item = char>>(self, iter: I) -> Self::Iter<I> {
        PadderIter::new(iter, self.strings, self.insert_before)
    }
}

// TODO: implement traits
#[derive(Debug, Clone)]
pub struct PadderIter<
    I: Iterator<Item = char>,
    C: Iterator<Item = char>,
    const P: char,
    const N: usize,
> {
    iter: Peekable<I>,
    strings: [Peekable<C>; N],
    matched: bool,
    insert_before: Option<char>,
}

impl<I, C, const P: char, const N: usize> PadderIter<I, C, P, N>
where
    I: Iterator<Item = char>,
    C: Iterator<Item = char>,
{
    #[must_use]
    fn new(iter: I, strings: [C; N], insert_before: Option<char>) -> Self {
        Self {
            iter: iter.peekable(),
            strings: strings.map(Iterator::peekable),
            matched: false,
            insert_before,
        }
    }
}

impl<I, C, const P: char, const N: usize> Iterator for PadderIter<I, C, P, N>
where
    I: Iterator<Item = char>,
    C: Iterator<Item = char>,
{
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let next_c = {
            let peeked = self.iter.peek();
            if self.matched
                && peeked
                    .map(|c| Some(*c) == self.insert_before)
                    .unwrap_or(true)
            {
                self.matched = false;
                return Some(P);
            } else if let Some(c) = self.iter.next() {
                self.matched = false;
                c
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

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower_bound, upper_bound) = self.iter.size_hint();

        if let Some(upper_bound) = upper_bound {
            // at most one padding char will be added
            (lower_bound, Some(upper_bound + 1))
        } else {
            (lower_bound, None)
        }
    }
}

impl<I, C, const P: char, const N: usize> FusedIterator for PadderIter<I, C, P, N>
//
where
    I: FusedIterator<Item = char>,
    C: FusedIterator<Item = char>,
{
}

#[cfg(test)]
mod tests {
    use super::*;

    use core::str::Chars;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_simple() {
        let padder: Padder<Chars<'_>, '\u{FFFD}', 3> =
            Padder::new(["foo", "barin", "bazing"].map(str::chars), None);
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
        let padder: Padder<Chars<'_>, '\u{FFFD}', 3> =
            Padder::new(["foob", "foo", "fo"].map(str::chars), None);
        let string = "foob";
        let mut iter = padder.sanitize(string.chars());

        assert_eq!(iter.next(), Some('f'));
        assert_eq!(iter.next(), Some('o'));
        assert_eq!(iter.next(), Some('o'));
        assert_eq!(iter.next(), Some('b'));
        assert_eq!(iter.next(), Some('\u{FFFD}'));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_with_insert_before() {
        let padder: Padder<Chars<'_>, '\u{FFFD}', 3> =
            Padder::new(["foo", "barin", "bazing"].map(str::chars), Some('.'));

        let string = "foo.txt.txt";
        let mut iter = padder.sanitize(string.chars());

        assert_eq!(iter.next(), Some('f'));
        assert_eq!(iter.next(), Some('o'));
        assert_eq!(iter.next(), Some('o'));
        assert_eq!(iter.next(), Some('\u{FFFD}'));
        assert_eq!(iter.next(), Some('.'));
        assert_eq!(iter.next(), Some('t'));
        assert_eq!(iter.next(), Some('x'));
        assert_eq!(iter.next(), Some('t'));
        assert_eq!(iter.next(), Some('.'));
        assert_eq!(iter.next(), Some('t'));
        assert_eq!(iter.next(), Some('x'));
        assert_eq!(iter.next(), Some('t'));
        assert_eq!(iter.next(), None);
    }
}
