use crate::constant_arrays;
use crate::sanitizer::Sanitizer;
use crate::sanitizers::Replacer;
use crate::Platform;

// TODO: make default const RP = '\u{FFFD}'
// NOTE: currently rustfmt destroys any defaults
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Linux<const RP: char> {}

impl<const RP: char> Default for Linux<RP> {
    fn default() -> Self {
        Self {}
    }
}

// TODO: Do something with SHOULD_BE_FORBIDDEN?
impl<const RP: char> Linux<RP> {
    constant_arrays! {
        /// Linux only forbids those two characters:
        const FORBIDDEN_CHARACTERS: [char; _] = ['/', '\x00'];
        /// Characters that are allowed, but make it very difficult to work with in shells
        const SHOULD_BE_FORBIDDEN: [char; _] = ['~', '\\', '"'];
    }
}

impl<const RP: char> Platform for Linux<RP> {
    type FilenameSanitizer<'a> = impl Sanitizer + 'a;
    type FolderSanitizer<'a> = Replacer<[(char, char); 2]>;

    fn filename_sanitizer(&self) -> Self::FilenameSanitizer<'_> {
        Replacer::from(Self::FORBIDDEN_CHARACTERS.map(|c| (c, RP)))
    }

    fn folder_sanitizer(&self) -> Self::FolderSanitizer<'_> {
        Replacer::from(Self::FORBIDDEN_CHARACTERS.map(|c| (c, RP)))
    }
}
