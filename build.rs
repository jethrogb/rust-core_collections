extern crate rustc_version;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::ops::{Neg,Sub};

/*
 * Let me explain this hack. For the sync shell script it's easiest if every 
 * line in mapping.rs looks exactly the same. This means that specifying an 
 * array literal is not possible. include!() can only expand to expressions, so 
 * just specifying the contents of an array is also not possible.
 *
 * This leaves us with trying to find an expression in which every line looks 
 * the same. This can be done using the `-` operator. This can be a unary 
 * operator (first thing on the first line), or a binary operator (later 
 * lines). That is exactly what's going on here, and Neg and Sub simply build a 
 * vector of the operangs.
 */
struct Mapping(&'static str,&'static str);

impl Neg for Mapping {
	type Output = Vec<Mapping>;
    fn neg(self) -> Vec<Mapping> {
		vec![self.into()]
	}
}

impl Sub<Mapping> for Vec<Mapping> {
    type Output=Vec<Mapping>;
    fn sub(mut self, rhs: Mapping) -> Vec<Mapping> {
		self.push(rhs.into());
		self
	}
}

fn main() {
	let coll_commit=match env::var("CORE_COLLECTIONS_COMMIT") {
		Ok(c) => c,
		Err(env::VarError::NotUnicode(_)) => panic!("Invalid commit specified in CORE_COLLECTIONS_COMMIT"),
		Err(env::VarError::NotPresent) => {
			let mappings=include!("mapping.rs");
			
			let compiler=rustc_version::version_meta().commit_hash.expect("Couldn't determine compiler version");
			mappings.iter().find(|&&Mapping(elem,_)|elem==compiler).expect("Unknown compiler version, upgrade core_collections?").1.to_owned()
		}
	};
	
	if let Some(date)=rustc_version::version_meta().commit_date {
		if &date[..]>"2016-08-23" {
			println!("cargo:rustc-cfg=no_no_drop_flag");
		}
	}
	
	let mut dest_path=PathBuf::from(env::var_os("OUT_DIR").unwrap());
	dest_path.push("collections.rs");
	let mut f=File::create(&dest_path).unwrap();
	
	let mut target_path=PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
	target_path.push("src");
	target_path.push(coll_commit);
	target_path.push("mod.rs");

	f.write_all(br#"#[path=""#).unwrap();
	f.write_all(target_path.into_os_string().into_string().unwrap().as_bytes()).unwrap();
	f.write_all(br#""] mod collections;"#).unwrap();
}
