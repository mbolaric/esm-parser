//! Example demonstrating Vehicle Unit (VU) data signature verification
//! using the consolidated `VuVerifyResult` and `VuVerifyItem` types.
//!
//! Usage:
//! ```bash
//! cargo run --example verify_vu -- -d <path/to/vu.DDD> -e <path/to/erca.bin>
//! ```

mod helpers;

use std::path::Path;

use clap::Parser;
use esm_parser::tacho::{VuCertificateKind, VuVerifyItem, VuVerifyResult};
use esm_parser::{TachographData, parse_from_file, verify_vu_full_with_erca_path};
use helpers::init_logging;

#[derive(Parser, Debug)]
#[clap(
    version,
    author = "Milan Bolaric",
    about = "Verify Vehicle Unit (VU) digital signatures and certificate chains",
    name = "verify_vu"
)]
pub struct Args {
    /// Path to the VU download file (*.DDD / *.V1B / *.TGD)
    #[clap(short, long)]
    pub ddd_file: String,

    /// Path to the European Root Certification Authority (ERCA) public key file (144 bytes for Gen1, 205 bytes for Gen2)
    #[clap(short, long)]
    pub erca_file: String,
}

fn main() {
    init_logging();
    let args = Args::parse();

    if !Path::new(&args.ddd_file).exists() {
        eprintln!("Error: VU DDD file does not exist: {}", args.ddd_file);
        std::process::exit(1);
    }
    if !Path::new(&args.erca_file).exists() {
        eprintln!("Error: ERCA public key file does not exist: {}", args.erca_file);
        std::process::exit(1);
    }

    println!("Parsing VU file: {}...", args.ddd_file);
    let data = match parse_from_file(&args.ddd_file) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Failed to parse VU file: {err}");
            std::process::exit(1);
        }
    };

    let (data_files, gen_label) = match &data {
        TachographData::VUGen1(vu) => (vu.get_data_files(), "Gen1"),
        TachographData::VUGen2(vu) => (vu.get_data_files(), "Gen2"),
        _ => {
            eprintln!("Error: The specified file is a tachograph card file, not a Vehicle Unit (VU) file.");
            std::process::exit(1);
        }
    };

    println!("Parsed {gen_label} VU data.");
    println!("Verifying signatures against ERCA key: {}...", args.erca_file);

    let verify_result: VuVerifyResult = match verify_vu_full_with_erca_path(data_files, &args.erca_file) {
        Ok(res) => res,
        Err(err) => {
            eprintln!("Verification failed: {err}");
            std::process::exit(1);
        }
    };

    println!("Overall Status: {:?}", verify_result.status);
    println!("Total items verified: {}", verify_result.result.len());

    for (i, item) in verify_result.result.iter().enumerate() {
        match item {
            VuVerifyItem::Certificate { certificate, status, end_of_validity } => {
                let name = match certificate {
                    VuCertificateKind::MemberStateCertificate => "MSCA Certificate",
                    VuCertificateKind::VuCertificate => "VU Sign Certificate",
                };
                println!("  [{i:02}] Certificate: {name} => status={status:?}, validity={end_of_validity:?}");
            }
            VuVerifyItem::Record { trep_id, position, status, end_of_validity } => {
                println!(
                    "  [{i:02}] Record: TREP={trep_id:?} (pos {position}) => status={status:?}, validity={end_of_validity:?}"
                );
            }
        }
    }
}
