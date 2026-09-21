# Gen2 Signature Verification — Implementation Reference

This document describes the Gen2 signature verification implementation in the `esm-parser` crate, covering both **Smart Card** applications and **Vehicle Unit (VU)** data downloads.

---

## 1. Scope and Architecture

The implementation verifies digital signatures and certificate chains for:

1. **Gen2 Driver and Workshop Cards**: Elementary files in the Gen2 application.
2. **Gen2 Vehicle Units (VU)**: Transfer Response Parameter (TREP) data records extracted from VU downloads.

### Certificate Trust Chains

```text
Card Verification:
  ERCA (self-signed root CVC)
    -> [optional LinkCertificate (ERCA rollover)]
      -> MSCA Card Certificate
        -> DriverCardSign or WorkshopCardSign Certificate
          -> Signed Elementary Files (EF)

Vehicle Unit (VU) Verification:
  ERCA (self-signed root CVC)
    -> [optional LinkCertificate (ERCA rollover)]
      -> MSCA VU Certificate (embedded in Overview TREP 01)
        -> VehicleUnitSign Certificate (embedded in Overview TREP 01)
          -> Signed TREP Data Blocks (Overview, Activities, Events/Faults, Speed, Technical Data)
```

### Cryptographic Profile (CS#1)

The implementation strictly follows Cipher Suite CS#1 as specified in Annex IC, Appendix 11 of [Commission Implementing Regulation (EU) 2016/799](https://eur-lex.europa.eu/eli/reg_impl/2016/799/2016-05-26/eng) and [Commission Implementing Regulation (EU) 2021/1228](https://eur-lex.europa.eu/eli/reg_impl/2021/1228/oj/eng):

| Property                     | Value / Specification                                                |
| ---------------------------- | -------------------------------------------------------------------- |
| **Curve**                    | `brainpoolP256r1` (`1.3.36.3.3.2.8.1.1.7`)                           |
| **Hash Algorithm**           | SHA-256                                                              |
| **Public Key Encoding**      | 65-byte uncompressed SEC1 point (`0x04                               |     | X   |     | Y`) |
| **ECDSA Signature Encoding** | 64-byte plain `r                                                     |     | s`  |
| **Certificate Format**       | Strict DER-TLV Card Verifiable Certificate (CVC), rooted at `0x7F21` |
| **CVC Length**               | Fixed 205 bytes                                                      |

Other profiles fail closed. Larger brainpool/NIST curves (CS#2 / CS#3) are rejected as unsupported key sizes.

---

## 2. Code Map

| Area                         | Implementation File                                                             | Description                                                                                      |
| ---------------------------- | ------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| **Public Verification API**  | [`src/verification.rs`](../src/verification.rs)                                 | High-level card and VU verification entry points (`verify_card`, `verify_vu`, file path helpers) |
| **Gen2 Verification Engine** | [`src/tachograph_gen2/verification.rs`](../src/tachograph_gen2/verification.rs) | CVC parsing, chain checks, Link Cert rollover, card EF and VU TREP signature validation          |
| **Card Data Model**          | [`src/tachograph_gen2/card_data.rs`](../src/tachograph_gen2/card_data.rs)       | Gen2 / Combined card parsing and elementary file storage                                         |
| **VU Data Model**            | [`src/tachograph_gen2/vu_data.rs`](../src/tachograph_gen2/vu_data.rs)           | Gen2 VU parsing, storing raw TREP bytes and preserving signature arrays                          |
| **Verification Results**     | [`src/tachograph/verify_result.rs`](../src/tachograph/verify_result.rs)         | Shared result models (`VerifyResult`, `VUVerifyResult`, `VerifyItem`, `VUVerifyItem`)            |
| **CLI & Export Handlers**    | [`examples/helpers/export.rs`](../examples/helpers/export.rs)                   | Dispatches verification for `esm2json` and `esm2xml`, isolating combined and VU outputs          |

---

## 3. Public Verification Contracts

### Card Verification

`verify_card`, `verify_card_with_time`, and `verify_card_with_erca_path` verify exactly one card application at a time:

```rust
pub fn verify_card(
    data_files: &CardFilesMap,
    generation: CardGeneration,
    erca_pk: &[u8],
) -> Result<VerifyResult>
```

- `CardGeneration::Gen1` requires a 144-byte ERCA RSA public key.
- `CardGeneration::Gen2` requires a 205-byte ERCA CVC.
- `CardGeneration::Combined` is rejected. A combined card contains both Gen1 and Gen2 application maps, which may require two separate ERCA root certificates. Callers must dispatch each application independently:
  ```text
  combined Gen1 application + Gen1 ERCA (144 bytes) -> Gen1 result
  combined Gen2 application + Gen2 ERCA (205 bytes) -> Gen2 result
  ```

### Vehicle Unit (VU) Verification

`verify_vu`, `verify_vu_with_time`, and `verify_vu_with_erca_path` verify extracted VU transfer response parameter files:

```rust
pub fn verify_vu_with_time(
    data_files: &VUFilesList,
    erca_pk: &[u8],
    validation_time: Option<u32>,
) -> Result<VUVerifyResult>
```

- Key length automatically selects the verifier:
  - **144 bytes**: Gen1 VU verifier (`tachograph_gen1::verification::verify_vu_with_time`, RSA-1024 / SHA-1).
  - **205 bytes**: Gen2 VU verifier (`tachograph_gen2::verification::verify_vu_with_time`, ECDSA P-256 / SHA-256).
  - Other lengths: Returns `Error::VerifyError("ERCA Public Key size ... is not supported")`.

---

## 4. Certificate Validation & Link Certificate Rollover

### Strict CVC Parsing

The Gen2 CVC parser requires strict TLV structure rooted at tag `7F21`, with inner body `7F4E`:

- CPI must be `0x00`.
- Profile fields checked: CAR (`0x42`), CHA (`0x5F4C`), Public Key OID and Point (`0x7F49`), CHR (`0x5F20`), Effective Date (`0x5F25`), Expiration Date (`0x5F24`), and Signature (`0x5F37`).

### Chain Verification Sequence

1. **ERCA Root**:
   - Must be self-signed (`CAR == CHR`).
   - Equipment type must be ERCA (`CHA[6] == 0x06` or `0x01`).
2. **Link Certificate (Optional ERCA Rollover)**:
   - If MSCA `CAR` does not match ERCA `CHR`, the verifier checks for an intermediate `LinkCertificate`.
   - The Link Certificate is verified using the current ERCA key.
   - If valid, the Link Certificate's public key replaces the root key for verifying the MSCA certificate.
3. **MSCA Certificate**:
   - `CAR` must match the ERCA (or Link Certificate) `CHR`.
   - Equipment type must be MSCA (`CHA[6] == 0x07`).
   - CVC signature is verified with the ERCA / Link public key.
4. **Target Signing Certificate**:
   - **Card**: Read from `CardSignCertificate` (falling back to `CardCertificate`). Role must be `DriverCardSign` (`0x11`) or `WorkshopCardSign` (`0x12`).
   - **VU**: Extracted from Overview TREP raw bytes (`raw_bytes[..205]` MSCA, optional Link Cert, followed by VU_Sign). Role must be `VehicleUnitSign` (`0x13`).
   - CVC signature is verified with the MSCA public key.
5. **Validity Periods**:
   - Verified against `validation_time` (or current UTC wall-clock time if `None`).

---

## 5. File & TREP Signature Verification

### Card Elementary File Verification

After the certificate chain is established, each signed Gen2 elementary file is hashed with SHA-256 and verified using the card-signing ECDSA public key.

The following non-signed elementary files are excluded from per-file card signature checks (aligned with DDP_035 of Regulation (EU) 2021/1228):

- `IC`, `ICC`
- `CACertificate`, `CardCertificate`, `CardSignCertificate`, `LinkCertificate`
- `CardDownload` (excluded in Gen2; Gen1 maintains its own handling)

### VU TREP Data Verification

Every positive response TREP has a 64-byte plain ECDSA signature appended:

- **TREP 01 (Overview)**: Signs data excluding the embedded certificates prefix (`skip_prefix_bytes = 410` without Link Cert, or `615` with Link Cert).
- **TREP 02 (Activities)**: Signs the data payload preceding the signature.
- **TREP 03 (Events and Faults)**: Signs the data payload preceding the signature.
- **TREP 04 (Detailed Speed)**: Signs the data payload preceding the signature.
- **TREP 05 (Technical Data)**: Signs the data payload preceding the signature.
- **Multi-Signature Support**: Preserves both single (`signature`) and multiple (`signatures: Vec<Vec<u8>>`) signature blocks when present.

### Result Statuses

Each checked item yields a `VerifyStatus`:

| Status                 | Meaning                                                    |
| ---------------------- | ---------------------------------------------------------- |
| `Valid`                | ECDSA signature is cryptographically valid                 |
| `Invalid`              | Signature failed verification against payload SHA-256 hash |
| `NotHaveData`          | File or TREP record has no data payload                    |
| `NotHaveSignature`     | File or TREP record has no signature appended              |
| `InvalidSignatureSize` | Signature is present but not exactly 64 bytes              |

The aggregate result status (`VerifyResultStatus`) is computed as:

- **`Valid`**: Every checked file is `Valid`.
- **`PartiallyValid`**: At least one file is `Valid`, and at least one file is non-valid.
- **`Invalid`**: No checked files are `Valid`.
- **`Unsigned`**: No signed data files or signatures were present to verify.

---

## 6. CLI and Export Tools Integration

The `esm2json` and `esm2xml` CLI examples support both Card and VU verification:

```bash
# Verify Gen2 Vehicle Unit
cargo run --example esm2json -- -d VU_Gen2.DDD -e EC_PK_GEN2.bin --pretty

# Verify Gen1 Vehicle Unit
cargo run --example esm2json -- -d VU_Gen1.DDD -e EC_PK.bin --pretty

# Verify Gen2 Driver Card
cargo run --example esm2json -- -d C_Gen2.ddd -E EC_PK_GEN2.bin --pretty

# Verify Combined Driver Card (produces card_verify_gen1.json and card_verify_gen2.json)
cargo run --example esm2json -- -d C_Combined.ddd -e EC_PK.bin -E EC_PK_GEN2.bin --pretty
```

- Outputs generated:
  - Cards: `<filename>_verify.json` / `<filename>_verify.xml` (or `_gen1` / `_gen2` for combined cards).
  - Vehicle Units: `<filename>_verify.json` / `<filename>_verify.xml`.
