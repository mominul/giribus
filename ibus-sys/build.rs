// This file was generated by gir (https://github.com/gtk-rs/gir @ 2f86b9a)
// from gir-files (https://github.com/gtk-rs/gir-files @ 8de1aa1)
// DO NOT EDIT

#[cfg(not(feature = "dox"))]
extern crate system_deps;

#[cfg(not(feature = "dox"))]
use std::process;

#[cfg(feature = "dox")]
fn main() {} // prevent linking libraries to avoid documentation failure

#[cfg(not(feature = "dox"))]
fn main() {
    if let Err(s) = system_deps::Config::new().probe() {
        let _ = eprintln!("{}", s);
        process::exit(1);
    }
}
