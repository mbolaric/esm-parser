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
- Gen1 and Gen2 card-signature verification, subject to the supported
  certificate profiles described below.
- WebAssembly builds for browser-based parsing.

## Requirements and local build

Use a current stable Rust toolchain. The project does not currently publish an
MSRV policy.

```bash
git clone https://github.com/mbolaric/esm-parser.git
cd esm-parser
cargo build
```

## Parse a DDD file from Rust

Parse directly from a file:

```rust,no_run
use esm_parser::parse_from_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tachograph_data = parse_from_file("card.ddd")?;
    println!("{tachograph_data:#?}");
    Ok(())
}
```

Or parse bytes already held in memory:

```rust,no_run
use esm_parser::parse_from_memory;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = std::fs::read("card.ddd")?;
    let tachograph_data = parse_from_memory(&bytes)?;
    println!("{tachograph_data:#?}");
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

## Card-signature verification

The example exporters verify a card when the corresponding ERCA certificate is
provided:

```bash
cargo run --example esm2json -- \
  --ddd-file card.ddd \
  --json-file card.json \
  --erca-gen1-file gen1-erca.bin \
  --erca-gen2-file gen2-erca.cvc \
  --pretty
```

- A Gen1 ERCA value must be exactly **144 bytes**.
- A Gen2 ERCA CVC must be exactly **205 bytes**.
- The public API verifies one generation/application map at a time.
  `CardGeneration::Combined` is deliberately rejected because a combined card
  needs separate Gen1 and Gen2 data maps and may need unrelated roots.
- For combined Driver and Workshop cards, the CLI verifies each supplied
  application separately and writes `*_verify_gen1.*` and `*_verify_gen2.*`
  results. Company and Control Gen2 applications do not have a supported
  `Card_Sign` verification path.

The Gen2 verifier intentionally supports the fixed CS#1 brainpoolP256r1,
SHA-256 profile used by the supported card applications. It fails closed for
other profiles, LinkCertificate rollover resolution, and VU signature-record
verification. Read the complete contract, certificate-chain behaviour, and
fixture-test instructions in
[`docs/gen2-signature-impl-reference.md`](docs/gen2-signature-impl-reference.md).

## WebAssembly

The parser can be compiled for browser use with `wasm-pack`.

```bash
cargo install wasm-pack
wasm-pack build --target web
python3 -m http.server 8090
```

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
cargo test --doc
cargo fmt -- --check
cargo clippy --all-targets -- -D warnings
```

## Further reading

- [Gen2 signature verification implementation reference](docs/gen2-signature-impl-reference.md)
- [Project license](LICENSE)

[actions-badge]: https://github.com/mbolaric/esm-parser/actions/workflows/rust.yml/badge.svg?branch=master
[actions-url]: https://github.com/mbolaric/esm-parser/actions/workflows/rust.yml?query=branch%3Amaster
