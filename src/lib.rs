#![allow(stable_features,unused_features)]
#![feature(collections,alloc,unsafe_no_drop_flag,dropck_parametricity,collections_bound,heap_api,core_intrinsics,unique,oom,filling_drop,sip_hash_13,hashmap_hasher)]
#![no_std]

#[macro_use]
extern crate collections as core_collections;
extern crate alloc;
#[cfg(feature="rand")]
extern crate rand;

// Needed for older compilers, to ignore thread_local!/println! macros in tests
macro_rules! thread_local {
    (static $name:ident: $t:ty = $init:expr) => { };
    (pub static $name:ident: $t:ty = $init:expr) => { };
}
macro_rules! println {
    ($fmt:expr) => { () };
    ($fmt:expr, $($arg:tt)*) => { () };
}

include!(concat!(env!("OUT_DIR"), "/collections.rs"));
pub use collections::*;
