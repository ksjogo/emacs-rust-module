// Copyright (c) 2016  Sebastian Wiesner <swiesner@lunaryorn.com>

// Licensed under the Apache License, Version 2.0 (the "License"); you may not
// use this file except in compliance with the License.  You may obtain a copy
// of the License at

// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.  See the
// License for the specific language governing permissions and limitations under
// the License.

extern crate bindgen;

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn generate_emacs_bindings(header: &str, out_dir: &str) -> io::Result<()> {
    let dest_path = Path::new(out_dir).join("emacs.rs");

    let mut bindings = bindgen::Builder::new(header);
    let generated_bindings = bindings.builtins()
        .forbid_unknown_types()
        .generate()
        .expect("Failed to generate bindings");

    let mut file = try!(File::create(dest_path));
    try!(file.write(b"pub mod emacs {\n"));
    try!(file.write(generated_bindings.to_string().as_bytes()));
    try!(file.write(b"\n}"));
    Ok(())
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    generate_emacs_bindings("emacs-module.h", &out_dir).unwrap();
}
