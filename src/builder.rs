
use {TopLevel, Device, Variant};
use codegen;
use reader;

use std::env;
use std::io;
use std::fs::{self, File};
use std::path::{PathBuf, Path};

#[derive(Debug)]
pub struct BuildError(i32, String);

impl From<io::Error> for BuildError {
    fn from(other: io::Error) -> Self {
        BuildError(1, format!("IO Error: {:?}", other))
    }
}

impl From<reader::ReadError> for BuildError {
    fn from(other: reader::ReadError) -> Self {
        BuildError(1, format!("ReadError: {:?}", other))
    }
}

pub fn build<S: AsRef<Path>, D: AsRef<Path>>(src_path: S, dst_path: D, setup_link: bool, in_cargo: bool) -> Result<(), BuildError> {
    build_inner(src_path, dst_path, true, true)
}

pub fn build_inner<S: AsRef<Path>, D: AsRef<Path>>(src_path: S, dst_path: D, setup_link: bool, in_cargo: bool) -> Result<(), BuildError> {
    let src_path = src_path.as_ref();
    let dst_path = dst_path.as_ref();

    let device = load_device(&src_path)?;    


    // let rebuild = match fs::metadata(&dst_path) {        
    //     Ok(dst_meta) => compare_modification_times(&fs::metadata(&src_path)?, &dst_meta),
    //     Err(_) => true,
    // };

    //if rebuild {
    let src_dir = src_path.parent().expect("Source file name must be lib.rx or mod.rx");
    {
        let mut f_mod = try!(File::create(dst_path));
        let cfg = codegen::modules::Config { path: PathBuf::from(dst_path), is_root: dst_path.file_name() == Some(::std::ffi::OsStr::new("lib.rs")) };
        codegen::modules::gen_mod(&cfg, &mut f_mod, &device, dst_path.parent().expect("Destination file name must be lib.rs or mod.rs"))?;
        if in_cargo {
            println!("cargo:rerun-if-changed={}", src_path.display());
        }
        for f in fs::read_dir(src_dir.join("periph"))? {
            let f = f?;
            if in_cargo {
                println!("cargo:rerun-if-changed={}", f.path().display());
            }
        }
    }
    if setup_link && device.variants.len() > 0 {        
        if let Some(v) = get_selected_variant(&device) {
            if let Some(ref link_script) = v.link {
                //println!("cargo:warning=link_script {}", link_script);                
                let src = PathBuf::from("link").join(link_script);
                if in_cargo {
                    println!("cargo:rerun-if-changed={}", src.display());
                }
                copy_link_script(&src);
            } else {
                if in_cargo {
                    println!("cargo:warning=Link script {:?} was not found for variant {}", v.link, v.name);
                }
            }
        } else {
            if in_cargo {
                println!("cargo:warning=No link script found, please check that you have selected a known device variant from the following:");        
                for v in device.variants.iter() {
                    println!("cargo:warning=   {}", v.name);
                }
            }
        }
    }
    Ok(())
}

fn get_selected_variant(device: &Device) -> Option<Variant> {
    for v in device.variants.iter() {
        if let Ok(_) = env::var(format!("CARGO_FEATURE_{}", v.name.to_uppercase())) {
            return Some(v.clone())
        }
    }
    None
}

fn copy_link_script(src: &Path) {
    // Pass our linker script to the top crate
    //let script = "link.ld";
    //let src = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    fs::copy(src, out.join("link.ld")).unwrap();
    println!("cargo:rustc-link-search={}", out.display());    
}


fn load_device<P: AsRef<Path>>(p: P) -> Result<Device, BuildError> {
    let mut input = try!(File::open(&p));
    match try!(reader::read(&mut input, p.as_ref())) {
        TopLevel::Device(device) => Ok(device),
        TopLevel::Board(_) => Err(BuildError(1, format!("Expected Device, got Board"))),
        TopLevel::Peripheral(_) => Err(BuildError(1, format!("Expected Device, got Peripheral"))),
    }
}

#[allow(dead_code)]
fn compare_modification_times(src_metadata: &fs::Metadata,
                                dst_metadata: &fs::Metadata)
                                -> bool {
    use std::os::unix::fs::MetadataExt;
    src_metadata.mtime() >= dst_metadata.mtime()
}
