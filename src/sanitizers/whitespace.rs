use core::iter;

use crate::sanitizer::Sanitizer;

/// This sanitizer replaces whitespace characters like `\t` or `\n` with `RP`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Whitespace<const RP: char> {}

impl<const RP: char> Default for Whitespace<RP> {
    fn default() -> Self {
        Self {}
    }
}

impl<const RP: char> Sanitizer for Whitespace<RP> {
    type Iter<I: Iterator<Item = char>> = iter::Map<I, fn(char) -> char>;

    fn sanitize<I: Iterator<Item = char>>(self, iter: I) -> Self::Iter<I> {
        iter.map(|c| if c.is_whitespace() { RP } else { c })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_simple() {
        let sanitizer: Whitespace<'\u{FFFD}'> = Whitespace::default();
        let mut iter = sanitizer.sanitize("a\t\n1 \r\t\nr0".chars());

        assert_eq!(iter.next(), Some('a'));
        assert_eq!(iter.next(), Some('\u{FFFD}'));
        assert_eq!(iter.next(), Some('\u{FFFD}'));
        assert_eq!(iter.next(), Some('1'));
        assert_eq!(iter.next(), Some('\u{FFFD}'));
        assert_eq!(iter.next(), Some('\u{FFFD}'));
        assert_eq!(iter.next(), Some('\u{FFFD}'));
        assert_eq!(iter.next(), Some('\u{FFFD}'));
        assert_eq!(iter.next(), Some('r'));
        assert_eq!(iter.next(), Some('0'));
        assert_eq!(iter.next(), None);
    }
}
