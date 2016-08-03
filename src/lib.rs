extern crate rustc_serialize;

pub mod actiondispatch;
pub mod xfstruct;
pub mod xfstate;
pub mod validation;
pub mod errors;
pub mod xfrunner;
pub mod dispatcher;

#[cfg(not(feature = "embedded"))]
pub use self::xfstate::*;

#[cfg(not(feature = "embedded"))]
pub use self::xfstruct::*;

#[cfg(not(feature = "embedded"))]
pub use self::validation::*;

#[cfg(not(feature = "embedded"))]
pub use self::xfrunner::*;

#[cfg(not(feature = "embedded"))]
pub use self::dispatcher::*;

#[cfg(not(feature = "embedded"))]
pub use self::actiondispatch::*;

// Embedded config
//

#[cfg(feature = "embedded")]
extern crate libc;

#[cfg(feature = "embedded")]
mod embed;

#[cfg(feature = "embedded")]
pub use self::embed::embed::*;
