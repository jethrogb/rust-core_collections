#![allow(stable_features,unused_features,deprecated)]
#![feature(collections,alloc,dropck_parametricity,collections_bound,heap_api,core_intrinsics,unique,oom,filling_drop,sip_hash_13,hashmap_hasher,shared,fused,collections_range,pub_restricted,generic_param_attrs,may_dangle,dropck_eyepatch,placement_new_protocol)]
#![cfg_attr(not(no_no_drop_flag),feature(unsafe_no_drop_flag))]
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
