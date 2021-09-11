#![cfg_attr(not(feature = "std"), no_std)]
#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]
#![feature(const_generics_defaults)]

mod platform;
mod platforms;
mod sanitizer;
mod sanitizer_ext;
mod sanitizers;
mod utils;

pub use platform::Platform;
pub use sanitizer::Sanitizer;

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

/// A convenience function for sanitizing a filename.
#[cfg(feature = "alloc")]
#[must_use]
pub fn sanitize_filename<S: Sanitizer>(filename: &str, sanitizer: S) -> String {
    sanitizer.sanitize(filename.chars()).into_iter().collect()
}
