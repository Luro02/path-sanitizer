use crate::sanitizer_ext::SanitizerExt;
use crate::sanitizers::Replacer;
use crate::Platform;
use crate::{constant_arrays, Sanitizer};

pub struct OneDrive<const RP: char, const P: char> {}

impl<const RP: char, const P: char> OneDrive<RP, P> {
    constant_arrays! {
        /// https://support.microsoft.com/en-us/office/restrictions-and-limitations-in-onedrive-and-sharepoint-64883a5d-228e-48f5-b3d2-eb39e07630fa
        const FORBIDDEN_CHARACTERS_FOLDER: [char; _] = ['"', '*', ':', '<', '>', '?', '/', '\\', '|'];
        /// https://support.microsoft.com/en-us/office/restrictions-and-limitations-in-onedrive-and-sharepoint-64883a5d-228e-48f5-b3d2-eb39e07630fa
        const FORBIDDEN_CHARACTERS_FILE: [char; _] = [
            '~', '"', '#', '%', '&', '*', ':', '<', '>', '?', '/', '\\', '{', '|', '}',
        ];
        const RESERVED_FILENAMES: [&'static str; _] = [
            ".lock", "CON", "PRN", "AUX", "NUL",
            "COM0", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9",
            "LPT0", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
            // TODO: _vti_ cannot appear anywhere in the filename!
            "_vti_",
            "desktop.ini"
        ];
    }
}

impl<const RP: char, const P: char> Default for OneDrive<RP, P> {
    fn default() -> Self {
        Self {}
    }
}

impl<const RP: char, const P: char> Platform for OneDrive<RP, P> {
    type FilenameSanitizer<'a> = impl Sanitizer + 'a;
    type FolderSanitizer<'a> = impl Sanitizer + 'a;

    fn filename_sanitizer(&self) -> Self::FilenameSanitizer<'_> {
        Replacer::from(Self::FORBIDDEN_CHARACTERS_FILE.map(|c| (c, RP)))
            .strip_prefix(|c| c == '~')
            .strip_prefix(char::is_whitespace)
            .padding::<P, 27>(Self::RESERVED_FILENAMES)
    }

    fn folder_sanitizer(&self) -> Self::FolderSanitizer<'_> {
        Replacer::from(Self::FORBIDDEN_CHARACTERS_FOLDER.map(|c| (c, RP)))
            .strip_prefix(|c| c == '~')
            .strip_prefix(char::is_whitespace)
            // those are forbidden for folders as well:
            .padding::<P, 27>(Self::RESERVED_FILENAMES)
    }
}
