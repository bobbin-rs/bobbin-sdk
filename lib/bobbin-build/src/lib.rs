//! This crate provides build-time functionality.

use std::env;
use std::io;
use std::fs;
use std::path::{PathBuf, Path};

/// Searches for a linker script in `link/` based on enabled Cargo features and copies it to the top
/// crate's build directory. Matching files with a `.ld` extension are copied to `link.ld`, and files
/// with a `.x` extension are copied to `memory.x`.
pub fn setup_linker() {
    let cfg_target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    if cfg_target_os != "none" {
        return
    }

    if let Some((ld_script, dst)) = find_ld_script("link").unwrap() {
        copy_link_script(ld_script, dst);
    }
}

fn find_ld_script<P: AsRef<Path>>(ld_dir: P) -> io::Result<Option<(PathBuf, &'static str)>> {
    for dir_entry in ld_dir.as_ref().read_dir()? {
        if let Ok(dir_entry) = dir_entry {
            let path = dir_entry.path();
            if let Some(ext) = path.extension() {
                if ext == "ld"{
                    let var = format!("CARGO_FEATURE_{}", path.file_stem().unwrap().to_str().unwrap().to_uppercase());
                    if let Ok(_) = env::var(var) {
                        return Ok(Some((path.clone(), "link.ld")))
                    }
                } else if ext == "x" {
                    let var = format!("CARGO_FEATURE_{}", path.file_stem().unwrap().to_str().unwrap().to_uppercase());
                    if let Ok(_) = env::var(var) {
                        return Ok(Some((path.clone(), "memory.x")))
                    }                    
                }
            }            
        }        
    }
    Ok(None)
}

fn copy_link_script<P: AsRef<Path>>(src: P, dst: &str) {
    // Pass our linker script to the top crate
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::copy(src, out.join(dst)).unwrap();
    println!("cargo:rustc-link-search={}", out.display());    
}