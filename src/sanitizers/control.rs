use core::iter;

use crate::sanitizer::Sanitizer;

/// This sanitizer replaces control characters like `\0` or `\a` with the specified replacement character.
pub struct Control<const RP: char> {}

impl<const RP: char> Default for Control<RP> {
    fn default() -> Self {
        Self {}
    }
}

impl<const RP: char> Sanitizer for Control<RP> {
    type Iter<I: Iterator<Item = char>> = iter::Map<I, fn(char) -> char>;

    fn sanitize<I: Iterator<Item = char>>(self, iter: I) -> Self::Iter<I> {
        iter.map(|c| if c.is_control() { RP } else { c })
    }
}
