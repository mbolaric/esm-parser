# ESM Parser product website

## Preview and build

Open `docs/site/index.html` directly for a source preview, or serve it with:

```bash
python3 -m http.server 8080 --directory docs/site
```

The source preview's download links explicitly open GitHub Releases. Build a
self-contained artifact with Node.js 24 (no npm dependencies):

```bash
node --test docs/site/site.test.mjs
node docs/site/build.mjs
python3 -m http.server 8080 --directory dist/site
```

## Maintenance

Update content alongside the Rust API, CLI options and verification contract.
Keep privacy warnings. Do not add personal DDD files or certificate material.
The code preview is actual API usage, not fabricated decoded data. The SVG mark
is a parser-specific code symbol.

Release filenames follow `.github/workflows/rust-release.yml`. Update
`releases.mjs` and its tests if that packaging contract changes. Repository
renames also require updating canonical/Open Graph URLs and project links.
