//! # esm-parser
//!
//! A Rust library for parsing and cryptographically verifying digital tachograph
//! data from `.DDD` (ESM) files.
//!
//! This crate reads, parses, and verifies binary exports from **Driver Cards**,
//! **Workshop Cards**, **Company Cards**, **Control Cards**, and **Vehicle Units (VU)**,
//! supporting both **First Generation (Gen1)** and **Second Generation (Gen2 / Gen2v2)**
//! EU tachograph specifications.
//!
//! ## Features
//!
//! - **Automatic Format Detection**: Detects whether an input `.DDD` file or byte slice
//!   contains Card or Vehicle Unit (VU) data and identifies its generation (Gen1, Gen2, or Combined).
//! - **Comprehensive Data Extraction**: Parses driver activities, places, vehicles used,
//!   events, faults, detailed speed, calibration records, GNSS records, border crossings,
//!   and load/unload operations into strongly typed Rust structures.
//! - **Digital Signature & Certificate Verification**:
//!   - **Gen1**: RSA-1024 signature recovery and SHA-1 digest verification against the Gen1 ERCA root public key (`144` bytes).
//!   - **Gen2**: Card Verifiable Certificate (CVC) chain validation (`ERCA -> MSCA -> Card/VU`),
//!     ERCA Link Certificate rollover resolution, and ECDSA (`brainpoolP256r1` / SHA-256) signature
//!     verification against the Gen2 ERCA root certificate (`205` bytes).
//! - **Serialization & WebAssembly**: All parsed data structures implement [`serde::Serialize`]
//!   for JSON/XML export, with first-class `wasm32` support and TypeScript type generation.
//!
//! ## Usage
//!
//! ### Parse from a File
//!
//! ```rust,no_run
//! use esm_parser::{TachographData, parse_from_file};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let tachograph_data = parse_from_file("path/to/your_file.ddd")?;
//!     match tachograph_data {
//!         TachographData::CardGen1(data) => println!("Parsed Gen1 Card Data: {data:?}"),
//!         TachographData::CardGen2(data) => println!("Parsed Gen2 Card Data: {data:?}"),
//!         TachographData::VUGen1(data) => println!("Parsed Gen1 VU Data: {data:?}"),
//!         TachographData::VUGen2(data) => println!("Parsed Gen2 VU Data: {data:?}"),
//!     }
//!     Ok(())
//! }
//! ```
//!
//! ### Parse from an In-Memory Byte Slice
//!
//! ```rust,no_run
//! use esm_parser::parse_from_memory;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let ddd_bytes: &[u8] = &[0x00, 0x02, 0x01, 0x02, 0x03];
//!     let tachograph_data = parse_from_memory(ddd_bytes)?;
//!     println!("{tachograph_data:#?}");
//!     Ok(())
//! }
//! ```
//!
//! ### Verify Digital Signatures and Certificates
//!
//! Use [`TachographData::verify`] (or [`TachographData::verify_with_erca_paths`]) to verify
//! any parsed Card or Vehicle Unit against the European Root Certification Authority (ERCA) keys:
//!
//! ```rust,no_run
//! use esm_parser::parse_from_file;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let tachograph_data = parse_from_file("card_or_vu.ddd")?;
//!     let erca_gen1_pk = std::fs::read("gen1-erca.bin")?; // 144-byte Gen1 ERCA key
//!     let erca_gen2_pk = std::fs::read("gen2-erca.bin")?; // 205-byte Gen2 ERCA CVC
//!
//!     let verify_result = tachograph_data.verify(Some(&erca_gen1_pk), Some(&erca_gen2_pk))?;
//!     println!("{verify_result:#?}");
//!     Ok(())
//! }
//! ```
//!
//! ## Crate Organization
//!
//! - [`tacho`]: Data structures, enums, and verification types shared between Gen1 and Gen2.
//! - [`gen1`]: First-generation (Gen1) Card and Vehicle Unit data structures and verification logic.
//! - [`gen2`]: Second-generation (Gen2 and Gen2v2) Card and Vehicle Unit data structures, GNSS/Smart Tachograph records, and ECC verification logic.
//!
//! ## Cargo Features
//!
//! - `write`: Enables low-level binary serialization through `esm_parser::writer` (not a complete `.DDD` writer).
//! - `logging`: Enables diagnostic logging during binary parsing via `log` and `binary-data/logging`.
//! - `typescript`: Enables TypeScript declaration generation via `ts-rs` and exposes [`wasm_boundary_declaration`].
//! - `wasm-console-panic-hook`: Installs a browser console panic hook when targeting `wasm32`.

mod common;
mod consts;
mod error;
mod helpers;
mod parser;
mod tachograph;
mod tachograph_data;
mod tachograph_gen1;
mod tachograph_gen2;
#[cfg(feature = "typescript")]
mod typescript;
mod verification;

#[cfg(feature = "write")]
pub mod writer;

pub use common::*;
pub(crate) use consts::*;
pub use error::{Error, Result};

pub mod tacho {
    //! # Common Tachograph Data Structures
    //!
    //! This module defines and re-exports data structures and enums that are common
    //! across both Gen1 and Gen2 versions of digital tachograph data. These types
    //! represent fundamental data points that have a consistent format regardless
    //! of the specific tachograph generation.
    pub use super::tachograph::*;
}

pub mod gen1 {
    //! # Tachograph Gen1 Data Structures
    //!
    //! This module contains the specific data structures and parsing logic
    //! for the first generation (Gen1) of digital tachograph data.
    pub use super::tachograph_gen1::*;
}
pub mod gen2 {
    //! # Tachograph Gen2 Data Structures
    //!
    //! This module contains the specific data structures and parsing logic
    //! for the second generation (Gen2) of digital tachograph data. It includes
    //! support for new features like GNSS data and updated record formats.
    pub use super::tachograph_gen2::*;
}
#[cfg(target_arch = "wasm32")]
pub use helpers::{LogLevel, init_console_logging};
pub use parser::{parse_from_file, parse_from_memory};
pub use tachograph::{
    CardFileData, CardFileID, CardFilesMap, CardGeneration, DataFiles, VUDataFiles, VUFileData, VUFilesList, VerifyItem,
    VerifyResult, VerifyResultStatus, VerifyStatus, VuCertificateKind, VuVerifyItem, VuVerifyResult,
};
pub use tachograph_data::{SerializedTachographData, TachographData};
#[cfg(feature = "typescript")]
pub use typescript::wasm_boundary_declaration;
pub use verification::{
    TachographVerifyResult, VuOverview, verify_card, verify_card_with_erca_path, verify_card_with_time, verify_combined_card,
    verify_tachograph_data, verify_tachograph_data_with_erca_paths, verify_vu, verify_vu_certificate_chain, verify_vu_full,
    verify_vu_full_with_erca_path, verify_vu_full_with_time, verify_vu_with_erca_path,
};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(typescript_custom_section)]
const TS_WASM_BOUNDARY: &'static str = include_str!("../types/wasm-boundary.d.ts");

/// This function is the entry point for the WebAssembly module.
/// It is executed automatically when the WASM module is loaded in the browser.
/// It sets up a custom panic hook to provide more informative error messages in the console.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start_wasm() {
    #[cfg(feature = "wasm-console-panic-hook")]
    helpers::set_panic_hook();
}
