# Gen2 Signature Verification — Implementation Reference

This document describes the current Gen2 card-signature verification in the
crate.

## Scope

The implementation verifies signed Gen2 **Driver** and **Workshop** card
applications. It validates the direct certificate chain and each signed
application elementary file:

```text
ERCA (self-signed)
  -> MSCA card certificate
    -> DriverCardSign or WorkshopCardSign certificate
      -> signed card application files
```

The Gen2 CVC values handled by this implementation have a fixed 205-byte
boundary. The accepted CS#1 profile is deliberately narrow:

| Property | Implemented value |
|---|---|
| Curve | `brainpoolP256r1` (`1.3.36.3.3.2.8.1.1.7`) |
| Hash | SHA-256 |
| Public-key encoding | 65-byte uncompressed SEC1 point |
| ECDSA signature encoding | 64-byte plain `r || s` |
| Certificate format | strict DER-TLV CVC, rooted at `7F21` |

Other profiles fail closed. CS#2/CS#3, larger brainpool/NIST curves,
`LinkCertificate` rollover resolution, and VU signature-record verification are
not implemented.

The implementation follows the Common Security Mechanisms in Annex IC,
Appendix 11 of [Commission Implementing Regulation (EU) 2016/799](https://eur-lex.europa.eu/eli/reg_impl/2016/799/2016-05-26/eng).

## Code map

| Area | Implementation |
|---|---|
| Public verification API | [`src/verification.rs`](../src/verification.rs) |
| Gen2 CVC parsing, chain checks, and file checks | [`src/tachograph_gen2/verification.rs`](../src/tachograph_gen2/verification.rs) |
| Gen2/combined card parsing | [`src/tachograph_gen2/card_data.rs`](../src/tachograph_gen2/card_data.rs) |
| CLI/export handling | [`examples/helpers/export.rs`](../examples/helpers/export.rs) |
| Deterministic and fixture-backed tests | [`tests/verification_tests.rs`](../tests/verification_tests.rs) |

## Public verification contract

`verify_card` and `verify_card_with_erca_path` verify exactly one card
application at a time.

- `CardGeneration::Gen1` requires a 144-byte ERCA value.
- `CardGeneration::Gen2` requires a 205-byte ERCA CVC.
- `CardGeneration::Combined` is rejected. A combined card contains two separate
  application maps and may require two unrelated ERCA certificates; passing one
  map and one certificate would be ambiguous.

Generation is an explicit contract, not something inferred from the ERCA length.
The API validates the requested generation and certificate length before calling
the corresponding Gen1 or Gen2 verifier.

For a combined card, callers must split the parser result and make independent
calls:

```text
combined Gen1 application + Gen1 ERCA (144 bytes) -> Gen1 result
combined Gen2 application + Gen2 ERCA (205 bytes) -> Gen2 result
```

## CVC parsing and certificate validation

The Gen2 verifier parses the 205-byte CVC strictly. It requires the expected
outer certificate and certificate-body TLVs (`7F21` and `7F4E`) and validates
the profile, CAR, CHA, public-key OID and point, CHR, effective date, expiration
date, and certificate signature fields.

At the current UTC verification time it performs these checks in order:

1. Parse and verify the ERCA CVC as self-signed. Its CAR must equal its CHR and
   its equipment type must be ERCA.
2. Parse the `CACertificate` as the MSCA certificate. Its CAR must equal the
   ERCA CHR, its equipment type must be MSCA, and its CVC signature must verify
   with the ERCA public key.
3. Read `CardSignCertificate`, falling back to `CardCertificate` for compatible
   input. Its CAR must equal the MSCA CHR, it must be a DriverCardSign or
   WorkshopCardSign certificate, and its CVC signature must verify with the
   MSCA public key.
4. Validate every certificate's effective and expiration dates at the current
   time.

Any failed certificate or chain check returns `Error::VerifyError`; it is never
converted into a successful partial result. A supplied root must
cryptographically verify the embedded MSCA certificate. Matching references by
themselves are not enough.

## Application-file verification

After the certificate chain succeeds, each signed Gen2 application file is
hashed with SHA-256 and verified with the card-signing public key. The verifier
records one `VerifyItem` per checked file.

These elementary files are intentionally excluded from per-file card-data
signature checks:

- `IC`
- `ICC`
- `CACertificate`
- `CardCertificate`
- `CardSignCertificate`
- `LinkCertificate`
- `CardDownload`

`CardDownload` is excluded only from the Gen2 card-data verifier. Gen1 keeps
its own existing handling for that file. The non-signed-card-file treatment is
aligned with DDP_035 of [Commission Implementing Regulation (EU) 2021/1228](https://eur-lex.europa.eu/eli/reg_impl/2021/1228/oj/eng).

For every other file, the result is one of:

| Condition | Result |
|---|---|
| valid 64-byte signature | `Valid` |
| signature does not verify | `Invalid` |
| missing data | `NotHaveData` |
| missing signature | `NotHaveSignature` |
| signature has a size other than 64 bytes | `InvalidSignatureSize` |

The aggregate status is `Valid` when every recorded item is valid,
`PartiallyValid` when at least one item is valid and another is not, and `Invalid`
when no recorded item is valid. If `IC` or `ICC` is absent, the Gen2 card is
reported as `Unsigned` before certificate processing.

## Combined-card CLI behavior

The example exporters detect `ParsedCard::Combined` and keep the two
applications isolated.

| Combined card type | Verification behavior |
|---|---|
| Driver | Verify Gen1 with `--erca-gen1-file`, then Gen2 with `--erca-gen2-file` |
| Workshop | Verify Gen1 with `--erca-gen1-file`, then Gen2 with `--erca-gen2-file` |
| Company | Verify only the Gen1 application; report that Gen2 `Card_Sign` is not applicable |
| Control | Verify only the Gen1 application; report that Gen2 `Card_Sign` is not applicable |

For a base result path such as `card_verify.json`, a combined Driver or Workshop
run writes:

```text
card_verify_gen1.json
card_verify_gen2.json
```

This prevents one application's result from overwriting the other. If an ERCA
flag is missing, only that application's verification is skipped and the CLI
explains which certificate was not supplied.

## Test coverage

The implementation has three complementary test layers:

1. Unit tests in `src/tachograph_gen2/verification.rs` embed public CVC vectors
   and generate a deterministic ERCA → MSCA → card-signing chain. They verify
   a signed application file, certificate failures, and Gen2 `CardDownload`
   exclusion.
2. `src/verification.rs` tests that the public API rejects an ambiguous
   `CardGeneration::Combined` request.
3. `tests/verification_tests.rs` contains ignored real-fixture tests. They
   verify observable aggregate and per-file results, then flip one signed byte
   to require detection of tampering.

Run the reproducible suite with:

```bash
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
```

Real DDD fixtures are deliberately ignored because they may contain private card
data. To run the combined Driver/Workshop integration test, supply matching
fixture paths and roots:

```bash
ESM_PARSER_COMBINED_DDD=/path/to/card.ddd \
ESM_PARSER_COMBINED_GEN1_ERCA=/path/to/gen1-erca.bin \
ESM_PARSER_COMBINED_GEN2_ERCA=/path/to/gen2-erca.bin \
cargo test --test verification_tests test_verify_real_combined_card_and_rejects_gen2_tampering -- --ignored --exact
```

The Gen1 ERCA must be 144 bytes and the Gen2 ERCA must be 205 bytes. Both
certificate chains must be valid at the current time. The test then modifies a
signed Gen2 application file and requires that exact file to become `Invalid`.

For Gen1 fixture coverage, use `ESM_PARSER_GEN1_DDD` or
`ESM_PARSER_GEN1_DDD_DIR` together with `ESM_PARSER_GEN1_ERCA`.

## Operational limitations

- Certificate validity is evaluated against the current system time. Historical
  test material may fail because it has expired even when its signatures are
  structurally correct.
- Root rollover is not resolved through `LinkCertificate`; provide the direct
  ERCA that verifies the embedded MSCA certificate.
- The verifier intentionally supports only the fixed P-256 profile above. Do
  not reinterpret different curves, point widths, or signature widths as this
  profile.
- Company and Control Gen2 card applications have no supported `Card_Sign`
  verification path in this implementation.
