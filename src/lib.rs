extern crate rustc_serialize;

mod actiondispatch;
pub mod xfstruct;
// pub mod xfstate;
pub mod validation;
pub mod errors;

#[cfg(not(feature = "embedded"))]
pub use self::xfstruct::*;

#[cfg(not(feature = "embedded"))]
pub use self::validation::*;

// Embedded config
//

#[cfg(feature = "embedded")]
extern crate libc;

#[cfg(feature = "embedded")]
mod embed;

#[cfg(feature = "embedded")]
pub use self::embed::embed::*;
