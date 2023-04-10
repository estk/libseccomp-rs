// SPDX-License-Identifier: Apache-2.0 or MIT
//
// Copyright 2021 Sony Group Corporation
//

use std::{env, fmt::Display, str::FromStr};

const LIBSECCOMP_LIB_PATH: &str = "LIBSECCOMP_LIB_PATH";
const LIBSECCOMP_LINK_TYPE: &str = "LIBSECCOMP_LINK_TYPE";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-env-changed={}", LIBSECCOMP_LIB_PATH);
    println!("cargo:rerun-if-env-changed={}", LIBSECCOMP_LINK_TYPE);

    let custom_path = env::var(LIBSECCOMP_LIB_PATH);

    if let Ok(path) = custom_path {
        println!("cargo:rustc-link-search=native={}", path);
    }

    let custom_type = env::var(LIBSECCOMP_LINK_TYPE)
        .ok()
        .map(|s| LinkType::from_str(&s))
        .transpose()?;

    if let Some(lt) = custom_type {
        println!("cargo:rustc-link-lib={}=seccomp", lt);
    } else {
        println!("cargo:rustc-link-lib=seccomp");
    }
    Ok(())
}

#[derive(Debug)]
enum LinkType {
    Static,
    Dylib,
}
impl Display for LinkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Static => write!(f, "static"),
            Self::Dylib => write!(f, "dylib"),
        }
    }
}
impl FromStr for LinkType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "framework" => Err("Seccomp is a Linux specific technology".into()),
            "static" => Ok(Self::Static),
            "dylib" => {
                println!(
                    "cargo:warning=dylib link type specified in env var '{}', dylib is the default",
                    LIBSECCOMP_LINK_TYPE
                );
                Ok(Self::Dylib)
            }
            _ => Err("Unknown link type".into()),
        }
    }
}
