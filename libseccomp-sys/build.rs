// SPDX-License-Identifier: Apache-2.0 or MIT
//
// Copyright 2021 Sony Group Corporation
//

use libloading::{Library, Symbol};
use std::{
    env,
    fmt::Display,
    io::{self, ErrorKind},
    process::{Command, Output},
    str::FromStr,
};

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

fn get_version() -> Result<(), Box<dyn std::error::Error>> {
    let (link_type, path) = if let Ok(path) = env::var(LIBSECCOMP_LIB_PATH) {
        let link_type = env::var(LIBSECCOMP_LINK_TYPE)
            .map_or(Ok(LinkType::Static), |s| LinkType::from_str(&s))?;
        (link_type, path)
    } else if let Ok(path) = dpkg_query() {
        (LinkType::Dylib, path)
    } else {
        println!("cargo:warning=WARNING: unable to find libseccomp");
        return Ok(());
    };

    println!("cargo:rustc-link-search=native={}", path);

    if matches!(link_type, LinkType::Dylib) {
        unsafe {
            let lib = Library::new(path)?;
            let scver_fn: Symbol<unsafe extern "C" fn() -> *const scmp_version> =
                lib.get(b"seccomp_version")?;
            let res = scver_fn();
            eprintln!(
                "cargo:warning=SCVER: {}.{}.{}",
                (*res).major,
                (*res).minor,
                (*res).micro
            );
        }
    }
    Ok(())
}
#[repr(C)]
struct scmp_version {
    major: u32,
    minor: u32,
    micro: u32,
}
fn dpkg_query() -> io::Result<String> {
    let Output {
        status,
        stderr,
        stdout,
    } = Command::new("dpkg-query")
        .args(["-L", "libseccomp2"])
        .output()?;
    if status.success() {
        let strout = String::from_utf8(stdout)
            .map_err(|e| io::Error::new(ErrorKind::Other, format!("Utf error: {e}")))?;
        let lsc_so = strout.lines().find(|l| l.ends_with("libseccomp.so.2"));
        if let Some(so) = lsc_so {
            Ok(so.into())
        } else {
            Err(io::Error::new(
                ErrorKind::Other,
                "Unable to find libseccomp.so.2",
            ))
        }
    } else {
        Err(io::Error::new(
            ErrorKind::Other,
            format!(
                "error running run dpkg-query: {}",
                String::from_utf8_lossy(&stderr)
            ),
        ))
    }
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
