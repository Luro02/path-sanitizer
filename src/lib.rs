#![cfg_attr(not(feature = "std"), no_std)]
#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]
#![feature(const_generics_defaults)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::module_name_repetitions,
    clippy::module_inception,
    clippy::redundant_pub_crate
)]

mod platform;
pub mod platforms;
mod sanitizer;
mod sanitizer_ext;
pub mod sanitizers;
mod utils;

pub use platform::Platform;
pub use sanitizer::Sanitizer;
pub use sanitizer_ext::SanitizerExt;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::string::String;

// TODO: https://github.com/hkalexling/Mango/issues/212
// TODO: https://github.com/szTheory/zaru_crystal
// TODO: add macos support
// TODO: support for filesystems?

#[macro_export]
macro_rules! count {
    ( $($x:expr),* ) => {
        <[()]>::len(&[$( $crate::count!(@replace $x ()) ),*])
    };
    (@replace $_t:tt $e:expr) => {
        $e
    };
}

/// this macro is used, so one does not have to specify the number of elements in the array manually
#[macro_export]
macro_rules! constant_arrays {
    ( $( $( #[doc = $doc:expr] )* const $name:ident : [$type:ty; _] = [$( $x:expr ),+ $(,)? ]);+ $(;)? ) => {
        $(
            $( #[doc = $doc] )*
            const $name : [$type; $crate::count!( $($x),+ )] = [ $($x),+];
        )+
    };
}

/// A convenience function for sanitizing a string.
#[cfg(feature = "alloc")]
#[must_use]
pub fn sanitize(string: &str, sanitizer: impl Sanitizer) -> String {
    sanitizer.sanitize(string.chars()).into_iter().collect()
}

/// A convenience function for sanitizing a filename.
#[cfg(feature = "alloc")]
#[must_use]
pub fn sanitize_filename(string: &str, platform: impl Platform) -> String {
    sanitize(string, platform.filename_sanitizer())
}

/// A convenience function for sanitizing a folder.
#[cfg(feature = "alloc")]
#[must_use]
pub fn sanitize_folder(string: &str, platform: impl Platform) -> String {
    sanitize(string, platform.folder_sanitizer())
}
