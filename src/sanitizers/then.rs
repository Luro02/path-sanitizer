use crate::sanitizer::Sanitizer;

/// Can be used to chain two sanitizers, first `A` is applied then `B`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Then<A: Sanitizer, B: Sanitizer> {
    a: A,
    b: B,
}

impl<A: Sanitizer, B: Sanitizer> Then<A, B> {
    #[must_use]
    pub fn new(a: A, b: B) -> Self {
        Self { a, b }
    }
}

impl<A: Sanitizer, B: Sanitizer> Sanitizer for Then<A, B> {
    type Iter<I: Iterator<Item = char>> = <B as Sanitizer>::Iter<<A as Sanitizer>::Iter<I>>;

    fn sanitize<I: Iterator<Item = char>>(self, iter: I) -> Self::Iter<I> {
        self.b.sanitize(self.a.sanitize(iter))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_sanitize() {
        let string = "1aa4aa7aa";
        let sanitizer = Then::new(
            |c: char| {
                if c == 'a' {
                    Some('b')
                } else {
                    Some(c)
                }
            },
            |c: char| {
                if c == 'b' {
                    Some('d')
                } else {
                    Some(c)
                }
            },
        );
        let mut iter = sanitizer.sanitize(string.chars());
        assert_eq!(iter.next(), Some('1'));
        assert_eq!(iter.next(), Some('d'));
        assert_eq!(iter.next(), Some('d'));
        assert_eq!(iter.next(), Some('4'));
        assert_eq!(iter.next(), Some('d'));
        assert_eq!(iter.next(), Some('d'));
        assert_eq!(iter.next(), Some('7'));
        assert_eq!(iter.next(), Some('d'));
        assert_eq!(iter.next(), Some('d'));
        assert_eq!(iter.next(), None);
    }
}
