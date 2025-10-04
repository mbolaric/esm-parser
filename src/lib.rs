//! # esm-parser
//!
//! A Rust library for parsing digital tachograph data from DDD files.
//!
//! This crate provides functionality to read and parse the binary data from
//! driver cards and vehicle units (VU), supporting both Gen1 and Gen2
//! specifications of the tachograph system.
//!
//! ## Features
//!
//! - Parses Gen1 and Gen2 DDD files.
//! - Automatically detects data type (Card or VU) and generation (Gen1 or Gen2).
//! - Extracts a wide range of data including driver activities, events, faults,
//!   vehicle usage, and calibration data.
//! - Provides structured Rust types for all parsed data for easy access.
//!
//! ## Usage
//!
//! To parse a DDD file from the filesystem:
//!
//! ```rust,no_run
//! use esm_parser::{parse_from_file, TachographData};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let path_to_ddd = "path/to/your_file.ddd";
//!     match parse_from_file(path_to_ddd) {
//!         Ok(tachograph_data) => {
//!             println!("Successfully parsed DDD file.");
//!             // Now you can work with the parsed data
//!             match tachograph_data {
//!                 TachographData::CardGen1(data) => println!("Parsed Gen1 Card Data."),
//!                 TachographData::CardGen2(data) => println!("Parsed Gen2 Card Data."),
//!                 TachographData::VUGen1(data) => println!("Parsed Gen1 VU Data."),
//!                 TachographData::VUGen2(data) => println!("Parsed Gen2 VU Data."),
//!             }
//!         }
//!         Err(e) => {
//!             eprintln!("Failed to parse DDD file: {:?}", e);
//!         }
//!     }
//!     Ok(())
//! }
//! ```
//!
//! To parse from an in-memory byte slice:
//!
//! ```rust,no_run
//! use esm_parser::parse_from_memory;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let ddd_byte_data: &[u8] = &[0x00, 0x02, 0x01, 0x02, 0x03]; // Example data
//!     match parse_from_memory(ddd_byte_data) {
//!         Ok(tachograph_data) => {
//!             println!("Successfully parsed in-memory DDD data.");
//!         }
//!         Err(e) => {
//!             eprintln!("Failed to parse in-memory data: {:?}", e);
//!         }
//!     }
//!     Ok(())
//! }
//! ```

mod common;
mod consts;
mod error;
mod helpers;
mod parser;
mod tachograph;
mod tachograph_data;
mod tachograph_gen1;
mod tachograph_gen2;
mod verification;

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
pub use parser::{parse_from_file, parse_from_memory};
pub use tachograph_data::TachographData;
pub use verification::verify;
