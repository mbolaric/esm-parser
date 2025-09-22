# Digital tachographs: "\*.DDD" File Parsing Library

Library for parsing ESM files (.DDD)
\*.DDD files are digital exports of tachograph data and contain all mandatory information required by EU regulations for:

- Driver monitoring
- Enforcement
- Fleet management
- Security/audit

[![Build Status][actions-badge]][actions-url]

[actions-badge]: https://github.com/mbolaric/esm-parser/actions/workflows/rust.yml/badge.svg?branch=master
[actions-url]: https://github.com/mbolaric/esm-parser/actions/workflows/rust.yml?query=branch%3Amaster

---

## WebAssembly (Wasm) Support

This library can be compiled to WebAssembly, allowing you to use the parser directly in a web browser.

```bash
// Install wasm-pack
cargo install wasm-pack
```

---

## Running the Web Example

1.  Build the wasm package:
    ```bash
    wasm-pack build --target web
    ```
2.  Start a simple web server from the project's root directory. If you have Python installed:
    ```bash
    python3 -m http.server
    ```
3.  Open your browser and navigate to `http://localhost:8000/examples/web/`. Check the developer console for the output.
