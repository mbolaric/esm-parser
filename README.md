# Digital tachographs: "\*.DDD" File Parsing Library

[![Build Status][actions-badge]][actions-url]

`esm-parser` is a Rust library and set of example CLIs for parsing digital
tachograph `.DDD` exports. It automatically identifies the tachograph
generation and data type, then returns strongly typed Rust data structures.

## What it supports

- Gen1 and Gen2 tachograph data.
- Driver-card and vehicle-unit (VU) `.DDD` parsing.
- Parsing from a file path or an in-memory byte slice.
- JSON and XML export through the included command-line examples.
- Gen1 and Gen2 digital signature and certificate verification for both cards and vehicle units (VU).
- WebAssembly builds for browser-based parsing.

## Requirements and local build

Use a current stable Rust toolchain. The project does not currently publish an
MSRV policy.

```bash
git clone https://github.com/mbolaric/esm-parser.git
cd esm-parser
cargo build
```

## Parse and verify a DDD file from Rust

Parse directly from a file:

```rust,no_run
use esm_parser::parse_from_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tachograph_data = parse_from_file("card.ddd")?;
    println!("{tachograph_data:#?}");
    Ok(())
}
```

Or verify digital signatures across any parsed card or vehicle unit:

```rust,no_run
use esm_parser::parse_from_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tachograph_data = parse_from_file("vehicle_unit.ddd")?;
    let erca_gen1_pk = std::fs::read("gen1-erca.bin")?;
    let erca_gen2_pk = std::fs::read("gen2-erca.bin")?;

    // Verify using convenience method on parsed data:
    let verify_result = tachograph_data.verify(Some(&erca_gen1_pk), Some(&erca_gen2_pk))?;
    println!("{verify_result:#?}");
    Ok(())
}
```

Both entry points return `TachographData`, which distinguishes Gen1/Gen2 and
card/VU results. See the crate-level API documentation in
[`src/lib.rs`](src/lib.rs) for the available data modules.

## Export with the example CLIs

The examples parse a DDD file and write the selected serialization format.
Passing an explicit output path is optional; when omitted, the exporter derives
the path from the input file.

```bash
# JSON
cargo run --example esm2json -- \
  --ddd-file card.ddd \
  --json-file card.json \
  --pretty

# XML
cargo run --example esm2xml -- \
  --ddd-file card.ddd \
  --xml-file card.xml \
  --pretty
```

Use `cargo run --example esm2json -- --help` or
`cargo run --example esm2xml -- --help` to see all CLI options.

## Digital signature and certificate verification

The example exporters verify cards and vehicle units when the corresponding ERCA public key certificate is
provided:

```bash
# Verify a Driver Card:
cargo run --example esm2json -- \
  --ddd-file card.ddd \
  --json-file card.json \
  --erca-gen1-file gen1-erca.bin \
  --erca-gen2-file gen2-erca.bin \
  --pretty

# Verify a Vehicle Unit (Gen1):
cargo run --example esm2json -- \
  --ddd-file dtco3.ddd \
  --erca-gen1-file gen1-erca.bin \
  --pretty

# Verify a Vehicle Unit (Gen2):
cargo run --example esm2json -- \
  --ddd-file dtco4.ddd \
  --erca-gen2-file gen2-erca.bin \
  --pretty
```

- A Gen1 ERCA value must be exactly **144 bytes** (RSA-1024).
- A Gen2 ERCA CVC must be exactly **205 bytes** (ECDSA brainpoolP256r1, SHA-256).
- **Cards**:
  - The public API `verify_card` verifies one generation/application map at a time.
  - `CardGeneration::Combined` is split into its Gen1 and Gen2 application maps (or verified via `verify_combined_card`).
  - For combined Driver and Workshop cards, the CLI verifies each supplied application separately and writes `*_verify_gen1.*` and `*_verify_gen2.*` results.
  - Company and Control Gen2 applications do not have a signed elementary file verification path.
- **Vehicle Units (VU)**:
  - `verify_vu_full`: Verifies the certificate chain (`ERCA -> MSCA -> VU_Sign`) and cryptographic signatures across every downloaded TREP record (Overview, Activities, Events and Faults, Detailed Speed, and Technical Data).
  - `verify_vu`: Verifies the Vehicle Unit's own Overview certificate chain (`ERCA -> MSCA -> VU_Sign`).
  - Supports ERCA Link Certificate rollover resolution for Gen2 VU and Card verification.

Read the complete contract, certificate-chain behaviour, and fixture-test instructions in
[`docs/gen2-signature-impl-reference.md`](docs/gen2-signature-impl-reference.md).

## WebAssembly

The parser can be compiled for browser use with `wasm-pack`.

```bash
cargo install wasm-pack
./scripts/build-wasm.sh
python3 -m http.server 8090
```

The repository build script regenerates the TypeScript contract before invoking
`wasm-pack build --target web`. Additional `wasm-pack build` options can be
passed through, for example `./scripts/build-wasm.sh --release`.

The generated declaration is checked in at `types/wasm-boundary.d.ts` and is
embedded into the `wasm-bindgen` package. The WebAssembly parser returns a
discriminated `{ kind, data }` envelope while native Rust and CLI exports retain
their existing untagged output. Regenerate the declaration whenever a serialized
parser type changes; the TypeScript-enabled test suite fails if the checked-in
contract is stale.

Open <http://127.0.0.1:8090/examples/web/> after starting the server from the
**repository root**. Do not serve `examples/web/` as the server root: the demo
loads the generated package from `pkg/` at the repository root.

## Privacy and test fixtures

DDD files can contain personal driver and vehicle data. Do not commit real
production DDD files or certificate material. The fixture-backed integration
tests are intentionally ignored by default so private files stay local.

## Development checks

Run the standard local validation suite before submitting a change:

```bash
cargo check --all-targets
cargo test --all-targets
cargo test --all-features
cargo test --doc
cargo fmt -- --check
cargo clippy --all-targets -- -D warnings
```

## Further reading

- [Gen2 signature verification implementation reference](docs/gen2-signature-impl-reference.md)
- [Project license](LICENSE)

[actions-badge]: https://github.com/mbolaric/esm-parser/actions/workflows/rust.yml/badge.svg?branch=master
[actions-url]: https://github.com/mbolaric/esm-parser/actions/workflows/rust.yml?query=branch%3Amaster
