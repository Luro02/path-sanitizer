mod control;
mod deduplicator;
mod padder;
mod prefix_stripper;
mod replacer;
mod then;
mod whitespace;

pub use control::Control;
pub use deduplicator::{Deduplicator, DeduplicatorIter};
pub use padder::{Padder, PadderIter};
pub use prefix_stripper::PrefixStripper;
pub use replacer::{Replacer, ReplacerIter};
pub use then::Then;
pub use whitespace::Whitespace;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sanitizer::Sanitizer;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_chaining() {
        let control: Control<'ü'> = Control::default();
        let whitespace: Whitespace<' '> = Whitespace::default();

        let input = "ha\nl\r\t\0lo";
        let mut sanitizer = control.sanitize(whitespace.sanitize(input.chars()));
        assert_eq!(sanitizer.next(), Some('h'));
        assert_eq!(sanitizer.next(), Some('a'));
        assert_eq!(sanitizer.next(), Some(' '));
        assert_eq!(sanitizer.next(), Some('l'));
        assert_eq!(sanitizer.next(), Some(' '));
        assert_eq!(sanitizer.next(), Some(' '));
        assert_eq!(sanitizer.next(), Some('ü'));
        assert_eq!(sanitizer.next(), Some('l'));
        assert_eq!(sanitizer.next(), Some('o'));
        assert_eq!(sanitizer.next(), None);
    }
}
