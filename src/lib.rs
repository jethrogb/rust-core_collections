#![allow(stable_features,unused_features)]
#![feature(collections,alloc,unsafe_no_drop_flag,dropck_parametricity,collections_bound,heap_api,core_intrinsics,unique,oom,filling_drop,sip_hash_13,hashmap_hasher)]
#![no_std]

#[macro_use]
extern crate collections as core_collections;
extern crate alloc;
extern crate rand;

include!(concat!(env!("OUT_DIR"), "/collections.rs"));
pub use collections::*;
