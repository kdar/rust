// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Shim which is passed to Cargo as "rustdoc" when running the bootstrap.
//!
//! See comments in `src/bootstrap/rustc.rs` for more information.

extern crate bootstrap;

use std::env;
use std::process::Command;
use std::path::PathBuf;

fn main() {
    let args = env::args_os().skip(1).collect::<Vec<_>>();
    let rustdoc = env::var_os("RUSTDOC_REAL").unwrap();
    let libdir = env::var_os("RUSTC_LIBDIR").unwrap();

    let mut dylib_path = bootstrap::util::dylib_path();
    dylib_path.insert(0, PathBuf::from(libdir));

    let mut cmd = Command::new(rustdoc);
    cmd.args(&args)
       .arg("--cfg").arg(format!("stage{}", env::var("RUSTC_STAGE").unwrap()))
       .arg("--cfg").arg("dox")
       .env(bootstrap::util::dylib_path_var(),
            env::join_paths(&dylib_path).unwrap());
    std::process::exit(match cmd.status() {
        Ok(s) => s.code().unwrap_or(1),
        Err(e) => panic!("\n\nfailed to run {:?}: {}\n\n", cmd, e),
    })
}

