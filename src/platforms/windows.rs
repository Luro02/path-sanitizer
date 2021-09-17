use crate::sanitizer::Sanitizer;
use crate::sanitizer_ext::SanitizerExt;
use crate::sanitizers::Replacer;
use crate::{constant_arrays, Platform};

pub struct Windows<const RP: char, const P: char> {}

impl<const RP: char, const P: char> Default for Windows<RP, P> {
    fn default() -> Self {
        Self {}
    }
}

// https://docs.microsoft.com/en-us/windows/win32/fileio/naming-a-file

impl<const RP: char, const P: char> Windows<RP, P> {
    constant_arrays! {
        /// Reserved Characters on Windows
        const RESERVED_CHARACTERS: [char; _] = ['<', '>', ':', '"', '/', '\\', '|', '?', '*', '\0'];
        /// These filenames are forbidden (including filename extension)
        ///
        /// For example the following filenames are invalid:
        /// - `NUL`
        /// - `NUL.txt`
        /// - `NUL.txt.txt`
        const RESERVED_FILENAMES: [&'static str; _] = [
            "CON", "PRN", "AUX", "NUL",
            "COM0", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9",
            "LPT0", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9"
        ];
    }
}

impl<const RP: char, const P: char> Platform for Windows<RP, P> {
    type FilenameSanitizer<'a> = impl Sanitizer + 'a;
    type FolderSanitizer<'a> = Replacer<[(char, char); 2]>;

    fn filename_sanitizer(&self) -> Self::FilenameSanitizer<'_> {
        // TODO: this should also padd something like: NUL.txt to NUL_.txt
        // TODO: remove trailing dots and trailing spaces (forbidden)
        // TODO: trailing spaces can be removed by deduplicating them first and then only removing the last character?
        // replace the explicitly forbidden characters:
        Replacer::from(Self::RESERVED_CHARACTERS.map(|c| (c, RP)))
            // replace control characters in the filename
            .replace_control::<RP>()
            // remove leading whitespace from the filename
            .strip_prefix(char::is_whitespace)
            // padd forbidden filenames
            .padding::<P, 24>(Self::RESERVED_FILENAMES)
    }

    fn folder_sanitizer(&self) -> Self::FolderSanitizer<'_> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_iter_eq_str {
        ( $iter:expr, $string:expr ) => {{
            let mut chars = $string.chars();

            for c in $iter {
                assert_eq!(Some(c), chars.next());
            }

            assert_eq!(None, chars.next());
        }};
    }

    #[test]
    fn test_filename_reserved() {
        let platform: Windows<'\u{FFFD}', '_'> = Windows::default();
        let iter = platform.filename_sanitizer().sanitize("NUL".chars());

        assert_iter_eq_str!(iter, "NUL_");
    }
}
