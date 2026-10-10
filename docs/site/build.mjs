import { createHash } from 'node:crypto';
import { copyFile, cp, mkdir, readFile, writeFile } from 'node:fs/promises';
import { renderRelease, resolveRelease } from './releases.mjs';

const source = new URL('./', import.meta.url);
const output = new URL('../../dist/site/', import.meta.url);
const headers = { Accept: 'application/vnd.github+json', 'X-GitHub-Api-Version': '2022-11-28' };
if (process.env.GITHUB_TOKEN) headers.Authorization = `Bearer ${process.env.GITHUB_TOKEN}`;
const response = await fetch('https://api.github.com/repos/mbolaric/esm-parser/releases/latest', {
    headers,
    signal: AbortSignal.timeout(30_000),
});
if (!response.ok) throw new Error(`GitHub release lookup failed: HTTP ${response.status}`);
const release = resolveRelease(await response.json());
let html = renderRelease(await readFile(new URL('index.html', source), 'utf8'), release);

for (const name of ['site.css', 'site.js']) {
    const content = await readFile(new URL(`assets/${name}`, source));
    const version = createHash('sha256').update(content).digest('hex').slice(0, 12);
    html = html.replaceAll(`"./assets/${name}"`, `"./assets/${name}?v=${version}"`);
}

await mkdir(output, { recursive: true });
await Promise.all([
    writeFile(new URL('index.html', output), html),
    copyFile(new URL('.nojekyll', source), new URL('.nojekyll', output)),
    cp(new URL('assets/', source), new URL('assets/', output), { recursive: true }),
]);
console.info(`Product site built in dist/site with ${release.tag} downloads.`);
