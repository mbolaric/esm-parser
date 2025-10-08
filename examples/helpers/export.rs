use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use esm_parser::TachographData;
use esm_parser::gen1::CardResponseParameterData as CardResponseParameterDataGen1;
use esm_parser::tacho::DataFiles;
use esm_parser::verify_card_with_erca_path;
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

fn prepare_out_path(ddd_file: &str, out_file: &str, export_type: &ExportType) -> (String, String) {
    let mut out_path: String = out_file.to_string();
    if out_file.is_empty() {
        let mut path = PathBuf::from(ddd_file);
        match export_type {
            ExportType::Json => path.set_extension("json"),
            ExportType::Xml => path.set_extension("xml"),
        };
        out_path = path.display().to_string();
    }
    let mut path = PathBuf::from(&out_path);
    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let file_name = if ext.is_empty() { format!("{}_{}", stem, "verify") } else { format!("{}_{}.{}", stem, "verify", ext) };
    path.set_file_name(file_name);
    let out_verify_path = path.display().to_string();

    (out_path, out_verify_path)
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
    pb.println(format!("[+] Save the {} parsed data to file ({:}) ...", export_type, out_path));
    file.write_all(out_str.as_bytes())?;
    Ok(())
}

fn parse(export_type: &ExportType, data: &(impl Export + Serialize), out_path: &str, pb: &ProgressBar, pretty: bool) {
    match parse_inner(export_type, data, out_path, pb, pretty) {
        Ok(_) => pb.println("[+] Parsing Done"),
        Err(err) => pb.println(format!("[-] {:}", err)),
    }
}

fn verify_inner(
    export_type: &ExportType,
    data: &(impl Export + Serialize),
    out_path: &str,
    pb: &ProgressBar,
    pretty: bool,
) -> Result<(), Error> {
    pb.println(format!("[+] Obtain {} ertificate verification result ...", export_type));
    let out_str = match export_type {
        ExportType::Json if pretty => data.to_json_pretty()?,
        ExportType::Json => data.to_json()?,
        ExportType::Xml if pretty => data.to_xml_pretty()?,
        ExportType::Xml => data.to_xml()?,
    };

    let mut file = File::create(out_path)?;
    pb.println(format!("[+] Save the {} certificate verification result to a file ({:}) ...", export_type, out_path));
    file.write_all(out_str.as_bytes())?;
    Ok(())
}

fn verify(
    export_type: &ExportType,
    data: &TachographData,
    erca_gen1_file: &str,
    _erca_gen2_file: &str,
    out_verify_path: &str,
    pb: &ProgressBar,
    pretty: bool,
) {
    // Verification
    pb.println("[+] Start certificate verification.");
    match data {
        TachographData::CardGen1(card_gen1) => {
            let card_type: Option<&dyn DataFiles> = match &card_gen1.card_data_responses {
                CardResponseParameterDataGen1::DriverCard(b) => Some(b.as_ref()),
                CardResponseParameterDataGen1::WorkshopCard(b) => Some(b.as_ref()),
                CardResponseParameterDataGen1::ControlCard(b) => Some(b.as_ref()),
                CardResponseParameterDataGen1::CompanyCard(b) => Some(b.as_ref()),
                _ => None,
            };

            if let Some(card) = card_type {
                match verify_card_with_erca_path(esm_parser::tacho::CardGeneration::Gen1, card.get_data_files(), erca_gen1_file) {
                    Ok(result) => match verify_inner(export_type, &result, out_verify_path, pb, pretty) {
                        Ok(_) => pb.println("[+] Certificate verification Done."),
                        Err(err) => pb.println(format!("[-] {:}", err)),
                    },
                    Err(err) => {
                        pb.println(format!("[-] Certificate verification error: {}.", err));
                    }
                }
                return;
            }
            pb.println("[-] Unsupported Card Type verification is not possible.");
        }
        TachographData::CardGen2(_card_gen1) => {
            pb.println("[-] Certificate verification is not implemented.");
        }
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
    if !path.exists() || !path.is_file() {
        pb.println(format!("[-] File not Exists: {:}", ddd_file));
        pb.println("[+] Parsing Done");
        return;
    }

    let (out_path, out_verify_path) = prepare_out_path(ddd_file, out_file, export_type);
    pb.println(format!("[+] Prepared output path: {}, {}.", out_path, out_verify_path));
    match parse_from_file(ddd_file) {
        Ok(data) => {
            parse(export_type, &data, &out_path, &pb, pretty);
            verify(export_type, &data, &erca_gen1_file, &erca_gen2_file, &out_verify_path, &pb, pretty);
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
