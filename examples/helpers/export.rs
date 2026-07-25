use std::fmt;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use esm_parser::gen1::CardResponseParameterData as CardResponseParameterDataGen1;
use esm_parser::gen2::{CardResponseParameterData as CardResponseParameterDataGen2, ParsedCard};
use esm_parser::tacho::{CardGeneration, DataFiles};
use esm_parser::{Export, TachographData, parse_from_file, verify_card_with_erca_path};
use indicatif::ProgressBar;
use serde::Serialize;

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
    pb.println(format!("[+] Obtain {} certificate verification result ...", export_type));
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

fn verification_path_for_generation(out_verify_path: &str, generation: CardGeneration) -> String {
    let mut path = PathBuf::from(out_verify_path);
    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    let ext = path.extension().and_then(|extension| extension.to_str()).unwrap_or("");
    let generation = match generation {
        CardGeneration::Gen1 => "gen1",
        CardGeneration::Gen2 => "gen2",
        CardGeneration::Combined => "combined",
    };
    let file_name = if ext.is_empty() { format!("{stem}_{generation}") } else { format!("{stem}_{generation}.{ext}") };
    path.set_file_name(file_name);
    path.display().to_string()
}

struct VerificationContext<'a> {
    export_type: &'a ExportType,
    erca_gen1_file: &'a str,
    erca_gen2_file: &'a str,
    out_verify_path: &'a str,
    pb: &'a ProgressBar,
    pretty: bool,
}

impl VerificationContext<'_> {
    fn verify_card_data(&self, card: &dyn DataFiles, generation: CardGeneration, erca_file: &str, out_verify_path: &str) {
        if erca_file.is_empty() {
            self.pb.println(format!("[-] {generation} ERCA certificate was not provided; verification is skipped."));
            return;
        }

        match verify_card_with_erca_path(generation, card.get_data_files(), erca_file) {
            Ok(result) => match verify_inner(self.export_type, &result, out_verify_path, self.pb, self.pretty) {
                Ok(_) => self.pb.println("[+] Certificate verification Done."),
                Err(err) => self.pb.println(format!("[-] {:}", err)),
            },
            Err(err) => {
                self.pb.println(format!("[-] Certificate verification error: {}.", err));
                self.pb.println("[+] Certificate verification Done.")
            }
        }
    }

    fn verify_combined_card_data(&self, gen1_card: &dyn DataFiles, gen2_card: &dyn DataFiles, gen2_signature_supported: bool) {
        self.verify_card_data(
            gen1_card,
            CardGeneration::Gen1,
            self.erca_gen1_file,
            &verification_path_for_generation(self.out_verify_path, CardGeneration::Gen1),
        );

        if gen2_signature_supported {
            self.verify_card_data(
                gen2_card,
                CardGeneration::Gen2,
                self.erca_gen2_file,
                &verification_path_for_generation(self.out_verify_path, CardGeneration::Gen2),
            );
        } else {
            self.pb.println("[i] Gen2 signature verification is not applicable to Company and Control cards.");
        }
    }
}

fn verify(data: &TachographData, context: &VerificationContext<'_>) {
    // Verification
    context.pb.println("[+] Start certificate verification.");
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
                context.verify_card_data(card, CardGeneration::Gen1, context.erca_gen1_file, context.out_verify_path);
                return;
            }
            context.pb.println("[-] Unsupported Card Type verification is not possible.");
        }
        TachographData::CardGen2(card_gen2) => match &card_gen2.card_data_responses {
            CardResponseParameterDataGen2::DriverCard(ParsedCard::Gen2(card)) => {
                context.verify_card_data(card.as_ref(), CardGeneration::Gen2, context.erca_gen2_file, context.out_verify_path);
            }
            CardResponseParameterDataGen2::WorkshopCard(ParsedCard::Gen2(card)) => {
                context.verify_card_data(card.as_ref(), CardGeneration::Gen2, context.erca_gen2_file, context.out_verify_path);
            }
            CardResponseParameterDataGen2::DriverCard(ParsedCard::Combined(gen1_card, gen2_card)) => {
                context.verify_combined_card_data(gen1_card.as_ref(), gen2_card.as_ref(), true);
            }
            CardResponseParameterDataGen2::WorkshopCard(ParsedCard::Combined(gen1_card, gen2_card)) => {
                context.verify_combined_card_data(gen1_card.as_ref(), gen2_card.as_ref(), true);
            }
            CardResponseParameterDataGen2::CompanyCard(ParsedCard::Gen2(_))
            | CardResponseParameterDataGen2::ControlCard(ParsedCard::Gen2(_)) => {
                context.pb.println("[i] Gen2 signature verification is not applicable to Company and Control cards.");
            }
            CardResponseParameterDataGen2::CompanyCard(ParsedCard::Combined(gen1_card, gen2_card)) => {
                context.verify_combined_card_data(gen1_card.as_ref(), gen2_card.as_ref(), false);
            }
            CardResponseParameterDataGen2::ControlCard(ParsedCard::Combined(gen1_card, gen2_card)) => {
                context.verify_combined_card_data(gen1_card.as_ref(), gen2_card.as_ref(), false);
            }
            _ => context.pb.println("[-] Unsupported Gen2 Card Type verification is not possible."),
        },
        _ => {
            context.pb.println("[-] Certificate Verification not supported");
            context.pb.println("[-] Certificate verification disabled.");
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
            if !erca_gen1_file.is_empty() || !erca_gen2_file.is_empty() {
                let verification = VerificationContext {
                    export_type,
                    erca_gen1_file,
                    erca_gen2_file,
                    out_verify_path: &out_verify_path,
                    pb: &pb,
                    pretty,
                };
                verify(&data, &verification);
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gives_combined_applications_distinct_verification_result_paths() {
        let gen1_path = verification_path_for_generation("output/card_verify.json", CardGeneration::Gen1);
        let gen2_path = verification_path_for_generation("output/card_verify.json", CardGeneration::Gen2);

        assert_eq!(Path::new(&gen1_path).file_name().unwrap(), "card_verify_gen1.json");
        assert_eq!(Path::new(&gen2_path).file_name().unwrap(), "card_verify_gen2.json");
        assert_ne!(gen1_path, gen2_path);
    }
}
