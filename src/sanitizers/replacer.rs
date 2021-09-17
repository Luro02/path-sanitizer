use crate::sanitizer::Sanitizer;
use crate::utils::Map;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Replacer<M: Map<char, char>>(M);

impl<M: Map<char, char>> From<M> for Replacer<M> {
    fn from(map: M) -> Self {
        Self(map)
    }
}

impl<M: Map<char, char>> Sanitizer for Replacer<M> {
    type Iter<I: Iterator<Item = char>> = ReplacerIter<I, M>;

    fn sanitize<I: Iterator<Item = char>>(self, iter: I) -> Self::Iter<I> {
        ReplacerIter { iter, map: self.0 }
    }
}

pub struct ReplacerIter<I: Iterator<Item = char>, M: Map<char, char>> {
    iter: I,
    map: M,
}

impl<I: Iterator<Item = char>, M: Map<char, char>> Iterator for ReplacerIter<I, M> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.iter.next()?;

        Some(self.map.get(&c).map_or(c, |c| *c))
    }
}
