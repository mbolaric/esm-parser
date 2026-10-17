import { createHash } from 'node:crypto';
import { copyFile, cp, mkdir, readFile, writeFile } from 'node:fs/promises';
import { renderRelease, resolveRelease } from './releases.mjs';

const source = new URL('./', import.meta.url);
const output = new URL('../../dist/site/', import.meta.url);
const packageSource = new URL('../../pkg/', import.meta.url);
const wasmFiles = ['esm_parser.js', 'esm_parser_bg.wasm'];
const wasmContents = await Promise.all(wasmFiles.map((name) => readFile(new URL(name, packageSource))));
const wasmVersion = createHash('sha256');
for (const content of wasmContents) wasmVersion.update(content);
const wasmDirectory = `wasm/${wasmVersion.digest('hex').slice(0, 12)}/`;
const workerSource = (await readFile(new URL('assets/demo-worker.js', source), 'utf8'))
    .replace('./wasm/esm_parser.js', `./${wasmDirectory}esm_parser.js`);
const workerVersion = createHash('sha256').update(workerSource).digest('hex').slice(0, 12);
const treeSource = await readFile(new URL('assets/json-tree.js', source));
const treeVersion = createHash('sha256').update(treeSource).digest('hex').slice(0, 12);
const demoSource = (await readFile(new URL('assets/demo.js', source), 'utf8'))
    .replace('./demo-worker.js', `./demo-worker.js?v=${workerVersion}`)
    .replace('./json-tree.js', `./json-tree.js?v=${treeVersion}`);
const headers = { Accept: 'application/vnd.github+json', 'X-GitHub-Api-Version': '2022-11-28' };
if (process.env.GITHUB_TOKEN) headers.Authorization = `Bearer ${process.env.GITHUB_TOKEN}`;
const response = await fetch('https://api.github.com/repos/mbolaric/esm-parser/releases/latest', {
    headers,
    signal: AbortSignal.timeout(30_000),
});
if (!response.ok) throw new Error(`GitHub release lookup failed: HTTP ${response.status}`);
const release = resolveRelease(await response.json());
let html = renderRelease(await readFile(new URL('index.html', source), 'utf8'), release);
let demoHtml = await readFile(new URL('demo.html', source), 'utf8');

for (const name of ['site.css', 'site.js', 'demo.js', 'demo-page.css']) {
    const content = name === 'demo.js' ? demoSource : await readFile(new URL(`assets/${name}`, source));
    const version = createHash('sha256').update(content).digest('hex').slice(0, 12);
    html = html.replaceAll(`"./assets/${name}"`, `"./assets/${name}?v=${version}"`);
    demoHtml = demoHtml.replaceAll(`"./assets/${name}"`, `"./assets/${name}?v=${version}"`);
}

await mkdir(output, { recursive: true });
await Promise.all([
    writeFile(new URL('index.html', output), html),
    writeFile(new URL('demo.html', output), demoHtml),
    copyFile(new URL('.nojekyll', source), new URL('.nojekyll', output)),
    cp(new URL('assets/', source), new URL('assets/', output), { recursive: true }),
]);
await mkdir(new URL(`assets/${wasmDirectory}`, output), { recursive: true });
await Promise.all([
    writeFile(new URL('assets/demo-worker.js', output), workerSource),
    writeFile(new URL('assets/demo.js', output), demoSource),
    ...wasmFiles.map((name) => copyFile(new URL(name, packageSource), new URL(`assets/${wasmDirectory}${name}`, output))),
]);
console.info(`Product site built in dist/site with ${release.tag} downloads.`);
