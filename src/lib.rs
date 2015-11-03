extern crate rustc_serialize;

#[cfg(feature = "embedded")]
extern crate libc;

mod actiondispatch;
pub mod xfstruct;
pub mod validation;
pub mod errors;

#[cfg(not(feature = "embedded"))]
pub use self::xfstruct::*;

#[cfg(not(feature = "embedded"))]
pub use self::validation::*;

mod embed;

#[cfg(feature = "embedded")]
pub use self::embed::embed::{char_count};

