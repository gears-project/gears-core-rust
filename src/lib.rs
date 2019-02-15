#![deny(trivial_casts,
        trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces,
        unused_qualifications
       )]

#![doc(html_root_url = "https://docs.rs/gears/")]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_yaml;
#[macro_use]
extern crate jsonapi;

#[macro_use]
extern crate log;
extern crate glob;
extern crate uuid;

extern crate ratel;

pub mod structure;
pub mod runtime;
pub mod generation;
pub mod validation;
pub mod parser;
pub mod util;
