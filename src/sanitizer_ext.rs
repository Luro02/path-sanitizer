use core::str::Chars;

use crate::sanitizer::Sanitizer;
use crate::sanitizers::{
    Control, Deduplicator, Padder, PrefixStripper, Replacer, Then, Whitespace,
};
use crate::utils::Map;

pub trait SanitizerExt: Sanitizer + Sized {
    #[must_use]
    fn then<S: Sanitizer>(self, sanitizer: S) -> Then<Self, S> {
        Then::new(self, sanitizer)
    }

    #[must_use]
    fn deduplicate<F: FnMut(char) -> bool>(self, f: F) -> Then<Self, Deduplicator<F>> {
        self.then(Deduplicator::new(f))
    }

    #[must_use]
    fn replace_control<const RP: char>(self) -> Then<Self, Control<RP>> {
        self.then(Control::default())
    }

    #[must_use]
    fn replace_whitespace<const RP: char>(self) -> Then<Self, Whitespace<RP>> {
        self.then(Whitespace::default())
    }

    #[must_use]
    fn replace<M: Map<char, char>>(self, map: M) -> Then<Self, Replacer<M>> {
        self.then(Replacer::from(map))
    }

    #[must_use]
    fn padding<'a, const P: char, const N: usize>(
        self,
        strings: [&'a str; N],
    ) -> Then<Self, Padder<Chars<'a>, P, N>> {
        self.then(Padder::new(strings.map(str::chars), None))
    }

    #[must_use]
    fn strip_prefix<P: FnMut(char) -> bool>(self, is_prefix: P) -> Then<Self, PrefixStripper<P>> {
        self.then(PrefixStripper::new(is_prefix))
    }
}

impl<S: Sanitizer> SanitizerExt for S {}
