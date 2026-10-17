# ESM Parser product website

## Preview and build

Open `docs/site/index.html` directly for a static source preview, or serve it with:

```bash
python3 -m http.server 8080 --directory docs/site
```

The source preview's download links explicitly open GitHub Releases. Build a
self-contained artifact with Node.js 24 (no npm dependencies), a stable Rust
toolchain and wasm-pack:

```bash
node --test docs/site/*.test.mjs
cargo install wasm-pack --locked --version 0.13.1
./scripts/build-wasm.sh --release
node docs/site/build.mjs
python3 -m http.server 8080 --directory dist/site
```

The build requires internet access to resolve the latest published stable
release through GitHub's API. An optional `GITHUB_TOKEN` authenticates that
build-time lookup and is never included in the output. The build selects exact
archive names for Windows x86-64, macOS Apple silicon and Intel, Linux x86-64,
and WebAssembly. It inserts direct links and release notes into the HTML.
Missing packages are explicitly unavailable; failed lookups, unexpected URLs,
ambiguous matches and releases with no supported packages fail the build.
Current source capabilities may be newer than the latest stable release.
The demo always uses the parser built from this checkout, not the older release
archive. The build fails if `pkg/esm_parser.js` or `pkg/esm_parser_bg.wasm` is
missing. Build the site and serve `dist/site` to use the demo: `file://` pages
cannot run module workers, and source previews do not stage the WASM package.

Authored assets, `index.html`, `demo.html`, `.nojekyll` and the two generated browser parser
files are staged in `dist/site`.
Rust sources, private fixtures and certificate files are never staged. CSS and
JavaScript URLs, the demo worker and the WASM package directory have content
hashes for cache invalidation. No external fonts, analytics, CDN or browser API
lookup are used. JavaScript enhances mobile navigation and runs the demo;
page content, download links and disclosures work without it.

## Browser demo

The Live demo links open `demo.html`, a viewport-sized workspace separate from
the product page. Desktop file controls stay in a sidebar beside the results.
On narrow screens the controls sit above the results in a height-limited,
scrollable area. The JSON pane fills the remaining height, with its own scroll
and scroll containment so reaching the end does not scroll the surrounding page.
The download action remains visible above the JSON.

The demo accepts a local file through the file picker or drag-and-drop
and lazily loads the same-origin WASM
parser inside a dedicated module worker. File bytes are transferred to that
worker, never sent to a server. The drop zone highlights while dragging a file, accepts one file at a
time, and replaces any active parse when another file is dropped. Multiple-file
drops show an error; dropping files outside the zone cannot navigate away.
Parsing and JSON serialization run off the main
thread. Results appear as expandable objects and arrays. Children are rendered
on demand in batches of 100 rather than mounting the whole document. Search
checks all keys and primitive values case-insensitively, including collapsed
branches, and shows contextual paths with expandable matched objects. Search is
debounced and yields periodically; up to 200 matches are shown with an explicit
refine-search message at the limit. Clearing the query restores the tree.
Download JSON saves the full `{ kind, data }` result. The demo parses
only: it does not load ERCA keys or verify signatures.

Clear / cancel terminates an active worker, revokes the result download URL and
clears the input, summary and preview. Workers are also terminated after success
or failure and on page exit. No localStorage, IndexedDB or other persistent data
storage is used. Read errors, empty files, parser errors, missing WASM assets and
worker failures are visible and allow retry. Browser memory still constrains
large files; the native CLI is the alternative. Filenames and JSON are rendered
as text, not HTML. Never commit real files to demonstrate the parser.

## Publish with GitHub Pages

1. In repository **Settings > Pages > Build and deployment > Source**, select
   **GitHub Actions**.
2. Merge the website changes into `master`. The **Product Website** workflow
   builds on site and parser changes, supports manual runs, and refreshes archive links
   when a stable release is published. Release builds check out `master`, not
   the release tag. Drafts and prereleases do not publish.
   The workflow builds the WASM parser before staging the site, so changes to the
   parser and the full-page demo are deployed together.
3. The expected URL is <https://mbolaric.github.io/esm-parser/>. The deployment
   job reports its actual published URL.

Pull requests build and test the artifact but cannot deploy it. Manual runs
from branches other than `master` also cannot deploy. Only the deploy job has
`pages: write` and `id-token: write` permissions; it uses the `github-pages`
environment. Required reviewers can be configured on that environment.
Enabling Pages and pushing or publishing are separate repository-owner actions.

## Maintenance

Update content alongside the Rust API, CLI options and verification contract.
Keep privacy warnings, verification limitations and the distinction from ESM
Viewer. Do not add personal DDD files or certificate material. The code preview
is actual API usage, not fabricated decoded data. The SVG mark is a
parser-specific code symbol; no Viewer screenshots or desktop icon are reused.

Release filenames follow `.github/workflows/rust-release.yml`. Update
`releases.mjs` and its tests if that packaging contract changes. Repository
renames also require updating canonical/Open Graph URLs and project links.
