use core::iter;

use crate::sanitizer::Sanitizer;

/// This sanitizer replaces whitespace characters like `\t` or `\n` with `RP`
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
