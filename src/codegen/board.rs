use Board;
use super::modules;

use std::io::{Write, Read, Result};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::collections::HashSet;

pub struct Config {
    pub out_path: PathBuf,
    pub cargo_template: PathBuf,
}

pub fn gen_board<W: Write>(cfg: Config, _out: &mut W, b: &Board) -> Result<()> {
    Ok(())
}