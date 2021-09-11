use core::iter;

pub trait Sanitizer {
    type IntoIter<I: Iterator<Item = char>>: IntoIterator<Item = char>;

    #[must_use]
    fn sanitize<I: IntoIterator<Item = char>>(self, iter: I) -> Self::IntoIter<I::IntoIter>;
}

impl Sanitizer for () {
    type IntoIter<I: Iterator<Item = char>> = I;

    fn sanitize<I: IntoIterator<Item = char>>(self, iter: I) -> Self::IntoIter<I::IntoIter> {
        iter.into_iter()
    }
}

impl<F> Sanitizer for F
where
    //
    F: FnMut(char) -> Option<char>,
{
    type IntoIter<I: Iterator<Item = char>> = iter::FlatMap<I, Option<char>, Self>;

    fn sanitize<I: IntoIterator<Item = char>>(self, iter: I) -> Self::IntoIter<I::IntoIter> {
        iter.into_iter().flat_map(self)
    }
}

pub struct BoolClosureAdapter<F>(F);

impl<F> From<F> for BoolClosureAdapter<F>
where
    for<'a> F: FnMut(&'a char) -> bool,
{
    fn from(value: F) -> Self {
        Self(value)
    }
}

impl<F> Sanitizer for BoolClosureAdapter<F>
where
    //
    for<'a> F: FnMut(&'a char) -> bool,
{
    type IntoIter<I: Iterator<Item = char>> = iter::Filter<I, F>;

    fn sanitize<I: IntoIterator<Item = char>>(self, iter: I) -> Self::IntoIter<I::IntoIter> {
        iter.into_iter().filter(self.0)
    }
}

/* // TODO: this does not work, beause of conflicting implementation
impl<F> Sanitizer for F
where
    //
    F: FnMut(char) -> bool,
{
    type IntoIter<I: Iterator<Item = char>> = iter::Filter<I, Self>;

    fn sanitize<I: IntoIterator<Item = char>>(self, iter: I) -> Self::IntoIter<I::IntoIter> {
        iter.into_iter().filter(self)
    }
}*/
