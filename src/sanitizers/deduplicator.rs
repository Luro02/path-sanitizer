use crate::sanitizer::Sanitizer;

/// Removes duplicate characters, by only keeping the first character
/// in a chain of multiple characters where the closure returns true.
pub struct Deduplicator<F>(F);

impl<F: FnMut(char) -> bool> Deduplicator<F> {
    #[must_use]
    pub fn new(f: F) -> Self {
        Self(f)
    }
}

impl<F: FnMut(char) -> bool> Sanitizer for Deduplicator<F> {
    type Iter<I: Iterator<Item = char>> = DeduplicatorIter<F, I>;

    fn sanitize<I: Iterator<Item = char>>(self, iter: I) -> Self::Iter<I> {
        DeduplicatorIter::new(self.0, iter)
    }
}

pub struct DeduplicatorIter<F: FnMut(char) -> bool, I: Iterator<Item = char>> {
    f: F,
    iter: I,
    flag: bool,
}

impl<F: FnMut(char) -> bool, I: Iterator<Item = char>> DeduplicatorIter<F, I> {
    #[must_use]
    fn new(f: F, iter: I) -> Self {
        Self {
            f,
            iter,
            flag: false,
        }
    }
}

impl<F: FnMut(char) -> bool, I: Iterator<Item = char>> Iterator for DeduplicatorIter<F, I> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        for c in self.iter.by_ref() {
            if !(self.f)(c) {
                self.flag = false;
                return Some(c);
            }

            if !self.flag {
                self.flag = true;
                return Some(c);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_sanitize() {
        let string = "1aa\u{00A0}\u{2008}\u{00A0}\t 47 \n\r";
        let sanitizer = Deduplicator::new(char::is_whitespace);

        let mut iter = sanitizer.sanitize(string.chars());
        assert_eq!(iter.next(), Some('1'));
        assert_eq!(iter.next(), Some('a'));
        assert_eq!(iter.next(), Some('a'));
        assert_eq!(iter.next(), Some('\u{00A0}'));
        assert_eq!(iter.next(), Some('4'));
        assert_eq!(iter.next(), Some('7'));
        assert_eq!(iter.next(), Some(' '));
        assert_eq!(iter.next(), None);
    }
}
