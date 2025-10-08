use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use esm_parser::TachographData;
use esm_parser::{Export, parse_from_file};
use indicatif::ProgressBar;
use serde::Serialize;
use std::fmt;

#[allow(dead_code)]
pub enum ExportType {
    Json,
    Xml,
}

impl fmt::Display for ExportType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ExportType::*;
        match self {
            Json => write!(f, "JSON"),
            Xml => write!(f, "XML"),
        }
    }
}

fn prepare_out_path(ddd_file: &str, out_file: &str, export_type: &ExportType) -> String {
    if out_file.is_empty() {
        let mut path = PathBuf::from(ddd_file);
        match export_type {
            ExportType::Json => path.set_extension("json"),
            ExportType::Xml => path.set_extension("xml"),
        };
        return path.display().to_string();
    }

    out_file.to_string()
}

fn parse_inner(
    export_type: &ExportType,
    data: &(impl Export + Serialize),
    out_path: &str,
    pb: &ProgressBar,
    pretty: bool,
) -> Result<(), Error> {
    pb.println(format!("[+] Obtain {} data ...", export_type));
    let out_str = match export_type {
        ExportType::Json if pretty => data.to_json_pretty()?,
        ExportType::Json => data.to_json()?,
        ExportType::Xml if pretty => data.to_xml_pretty()?,
        ExportType::Xml => data.to_xml()?,
    };

    let mut file = File::create(out_path)?;
    pb.println(format!("[+] Write {} to file ({:}) ...", export_type, out_path));
    file.write_all(out_str.as_bytes())?;
    Ok(())
}

fn parse(export_type: &ExportType, data: &(impl Export + Serialize), out_path: &str, pb: &ProgressBar, pretty: bool) {
    match parse_inner(export_type, data, out_path, pb, pretty) {
        Ok(_) => pb.println("[+] Parsing Done"),
        Err(err) => pb.println(format!("[-] {:}", err)),
    }
}

fn verify(data: &TachographData, erca_gen1_file: &str, _erca_gen2_file: &str, pb: &ProgressBar) {
    // Verification
    pb.println("[+] Start certificate verification.");
    match data {
        TachographData::CardGen1(_card_gen1) => {
            // FIXME:
            let _erca_cert: Vec<u8> = if !erca_gen1_file.is_empty() {
                let path = Path::new(erca_gen1_file);
                if !path.exists() {
                    pb.println(format!("[-] ERCA File not Exists: {:}", erca_gen1_file));
                    pb.println("[-] Certificate verification disabled.");
                    Vec::new()
                } else {
                    fs::read(erca_gen1_file).unwrap_or_default()
                }
            } else {
                Vec::new()
            };
            pb.println("[-] Certificate verification is not implemented.");
        }
        TachographData::CardGen2(_card_gen1) => {}
        _ => {
            pb.println("[-] Certificate Verification not supported");
            pb.println("[-] Certificate verification disabled.");
        }
    }
}

#[allow(dead_code)]
pub fn export(
    export_type: &ExportType,
    ddd_file: &str,
    out_file: &str,
    erca_gen1_file: &str,
    erca_gen2_file: &str,
    pretty: bool,
) {
    #[cfg(debug_assertions)]
    let pb = ProgressBar::hidden();
    #[cfg(not(debug_assertions))]
    let pb = ProgressBar::new(0);

    pb.println("[+] Start Parsing");

    let path = Path::new(ddd_file);
    if !path.exists() {
        pb.println(format!("[-] File not Exists: {:}", ddd_file));
        pb.println("[+] Parsing Done");
        return;
    }

    let out_path = prepare_out_path(ddd_file, out_file, export_type);
    match parse_from_file(ddd_file) {
        Ok(data) => {
            parse(export_type, &data, &out_path, &pb, pretty);
            verify(&data, &erca_gen1_file, &erca_gen2_file, &pb);
        }
        Err(err) => {
            pb.println(format!("[-] {:}", err));
            pb.println("[+] Parsing Done");
        }
    }
}

#[derive(Debug)]
pub enum Error {
    File(std::io::Error),
    Parsing(esm_parser::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            File(e) => write!(f, "File Read/Write Error ({e})"),
            Parsing(e) => write!(f, "Parsing Error ({e})"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::File(value)
    }
}

impl From<esm_parser::Error> for Error {
    fn from(value: esm_parser::Error) -> Self {
        Error::Parsing(value)
    }
}

impl std::error::Error for Error {}
