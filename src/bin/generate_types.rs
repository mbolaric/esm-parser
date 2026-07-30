use std::error::Error;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::{env, fs};

use esm_parser::wasm_boundary_declaration;

fn main() -> Result<(), Box<dyn Error>> {
    let output_path = env::args_os().nth(1).map(PathBuf::from).unwrap_or_else(|| PathBuf::from("types/wasm-boundary.d.ts"));
    let declaration = wasm_boundary_declaration();

    match fs::read_to_string(&output_path) {
        Ok(existing) if existing == declaration => return Ok(()),
        Ok(_) => {}
        Err(error) if error.kind() == ErrorKind::NotFound => {}
        Err(error) => return Err(error.into()),
    }

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(output_path, declaration)?;

    Ok(())
}
