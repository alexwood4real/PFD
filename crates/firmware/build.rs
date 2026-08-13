//! SPDX-License-Identifier: MIT OR Apache-2.0
//!
//! Copyright (c) 2021–2024 The rp-rs Developers
//! Copyright (c) 2021 rp-rs organization
//! Copyright (c) 2025 Raspberry Pi Ltd.
//!
//! Set up linker scripts

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {

    // Put the linker script somewhere the linker can find it
    let out = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out.display());

    let memory_x = include_bytes!("rp2350.x");
    let mut f = File::create(out.join("memory.x")).unwrap();
    f.write_all(memory_x).unwrap();
    println!("cargo::rustc-cfg=rp2350");
    println!("cargo:rerun-if-changed=rp2350.x");
}
