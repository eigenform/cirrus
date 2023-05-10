#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

// Include all of the bindings generated from build.rs
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/bindings/bindings.rs"));

