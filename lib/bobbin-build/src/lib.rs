#![allow(unused_imports)]

use std::env;
use std::io;
use std::fs::{self, File};
use std::path::{PathBuf, Path};

pub fn setup_linker() {
    let cfg_target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    if cfg_target_os != "none" {
        // println!("cargo:warning=CFG_TARGET_OS is {}, skipping Linker Script", cfg_target_os);
        return
    }

    if let Some((ld_script, dst)) = find_ld_script("link").unwrap() {
        copy_link_script(ld_script, dst);
    } else {
        // panic!("No linker script found for variant");
    }
}


pub fn find_ld_script<P: AsRef<Path>>(ld_dir: P) -> io::Result<Option<(PathBuf, &'static str)>> {
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

pub fn copy_link_script<P: AsRef<Path>>(src: P, dst: &str) {
    // Pass our linker script to the top crate
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::copy(src, out.join(dst)).unwrap();
    println!("cargo:rustc-link-search={}", out.display());    
}