use crate::sanitizer::Sanitizer;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PrefixStripper<P> {
    is_prefix: P,
}

impl<P: FnMut(char) -> bool> PrefixStripper<P> {
    #[must_use]
    pub fn new(is_prefix: P) -> Self {
        Self { is_prefix }
    }
}

impl<P: FnMut(char) -> bool> Sanitizer for PrefixStripper<P> {
    type IntoIter<I: Iterator<Item = char>> = PrefixStripperIter<I, P>;

    fn sanitize<I: IntoIterator<Item = char>>(self, iter: I) -> Self::IntoIter<I::IntoIter> {
        PrefixStripperIter::new(iter.into_iter(), self.is_prefix)
    }
}

// TODO: implement traits
pub struct PrefixStripperIter<I: Iterator<Item = char>, P: FnMut(char) -> bool> {
    iter: I,
    prefix_ended: bool,
    is_prefix: P,
}

impl<I: Iterator<Item = char>, P: FnMut(char) -> bool> PrefixStripperIter<I, P> {
    #[must_use]
    fn new(iter: I, is_prefix: P) -> Self {
        Self {
            iter,
            prefix_ended: false,
            is_prefix,
        }
    }
}

impl<I: Iterator<Item = char>, P: FnMut(char) -> bool> Iterator for PrefixStripperIter<I, P> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        for c in self.iter.by_ref() {
            if self.prefix_ended {
                return Some(c);
            }

            if !(self.is_prefix)(c) {
                self.prefix_ended = true;
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
    fn test_strip_prefix() {
        let string = "  \n \r\t   Hel l\to  ";
        let sanitizer = PrefixStripper::new(char::is_whitespace);
        let mut iter = sanitizer.sanitize(string.chars());

        assert_eq!(iter.next(), Some('H'));
        assert_eq!(iter.next(), Some('e'));
        assert_eq!(iter.next(), Some('l'));
        assert_eq!(iter.next(), Some(' '));
        assert_eq!(iter.next(), Some('l'));
        assert_eq!(iter.next(), Some('\t'));
        assert_eq!(iter.next(), Some('o'));
        assert_eq!(iter.next(), Some(' '));
        assert_eq!(iter.next(), Some(' '));
        assert_eq!(iter.next(), None);
    }
}
