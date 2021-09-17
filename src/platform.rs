use crate::Sanitizer;

pub trait Platform: Default {
    type FilenameSanitizer<'a>: Sanitizer + 'a;
    type FolderSanitizer<'a>: Sanitizer + 'a;

    #[must_use]
    fn filename_sanitizer(&self) -> Self::FilenameSanitizer<'_>;

    #[must_use]
    fn folder_sanitizer(&self) -> Self::FolderSanitizer<'_>;
}
