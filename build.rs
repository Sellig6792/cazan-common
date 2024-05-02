use rustc_version::{version_meta, Channel};

fn main() {
    match version_meta().unwrap().channel {
        Channel::Nightly => {
            println!("cargo:rustc-cfg=TOOLCHAIN_IS_NIGHTLY");
        }
        _ => {}
    }
}