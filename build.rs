/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::env;
use std::process::{Command, Stdio};

fn main() {
  let out_dir = env::var("OUT_DIR").unwrap();

  let cc = env::var("CC").unwrap_or(format!("gcc"));
  let cxx = env::var("CXX").unwrap_or(format!("g++"));

  let result = Command::new("make")
      .args(&["-R", "-f", "makefile.cargo"])
      .stdout(Stdio::inherit())
      .stderr(Stdio::inherit())
      .env("CC",  &cc)
      .env("CXX", &cxx)
      .status()
      .unwrap();
  assert!(result.success());
  println!("cargo:rustc-link-search=native={}", out_dir);
  println!("cargo:rustc-link-lib=static=stb-image");
  println!("cargo:rustc-link-lib=static=stb-image-resize");
  println!("cargo:rustc-link-lib=static=stb-image-write");
}
