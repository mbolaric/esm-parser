import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { test } from 'node:test';
import { packages, renderRelease, resolveRelease } from './releases.mjs';

const repository = 'https://github.com/mbolaric/esm-parser';
const html = await readFile(new URL('index.html', import.meta.url), 'utf8');

function fixture() {
    return {
        tag_name: 'v1.2.3',
        html_url: `${repository}/releases/tag/v1.2.3`,
        draft: false,
        prerelease: false,
        assets: packages.map((item) => {
            const name = `${item.prefix}v1.2.3${item.suffix}`;
            return { name, browser_download_url: `${repository}/releases/download/v1.2.3/${name}` };
        }),
    };
}

test('all five exact platform archives are rendered as direct links', () => {
    const release = resolveRelease(fixture());
    const rendered = renderRelease(html, release);
    for (const item of release.packages) assert.ok(rendered.includes(`href="${item.url}"`));
    assert.ok(rendered.includes('v1.2.3 release notes'));
    assert.ok(!rendered.includes('Source preview:'));
});

test('missing formats are explicitly unavailable, never redirected', () => {
    const input = fixture();
    input.assets.splice(0, 1);
    const rendered = renderRelease(html, resolveRelease(input));
    assert.ok(rendered.includes('not available in this release'));
    assert.ok(!rendered.includes('data-release-asset="windows"'));
});

test('drafts, prereleases, malformed tags and unexpected URLs are rejected', () => {
    for (const change of [
        { draft: true }, { prerelease: true }, { tag_name: 'v1.2.3-rc1' },
        { tag_name: '<script>' }, { html_url: 'https://example.com/release' },
        { assets: [] }, { assets: null },
    ]) assert.throws(() => resolveRelease({ ...fixture(), ...change }));
    const input = fixture();
    input.assets[0].browser_download_url = 'https://example.com/archive.zip';
    assert.throws(() => resolveRelease(input), /Invalid release archive URL/);
});

test('debug builds do not match and duplicate archives fail', () => {
    const input = fixture();
    input.assets.push({ name: 'examples-v1.2.3-debug-windows.zip', browser_download_url: 'https://example.com/debug.zip' });
    assert.equal(resolveRelease(input).packages.length, 5);
    input.assets.push(input.assets[0]);
    assert.throws(() => resolveRelease(input), /Ambiguous/);
});

test('missing download placeholders and release status fail', () => {
    const release = resolveRelease(fixture());
    assert.throws(() => renderRelease(html.replace('data-release-asset="linux"', ''), release), /linux/);
    assert.throws(() => renderRelease(html.replace('id="release-status"', ''), release), /release status/);
});

test('local navigation and authored assets resolve without a build', async () => {
    const ids = [...html.matchAll(/\bid="([^"]+)"/g)].map((match) => match[1]);
    assert.equal(ids.length, new Set(ids).size, 'IDs must be unique');
    for (const [, id] of html.matchAll(/href="#([^"]+)"/g)) assert.ok(ids.includes(id), `Missing anchor ${id}`);
    for (const [, path] of html.matchAll(/(?:href|src)="(\.\/assets\/[^"]+)"/g)) {
        await readFile(new URL(path, import.meta.url));
    }
    assert.ok(!html.includes('../../src/'));
});

test('Viewer-style footer retains grouped navigation and parser project links', () => {
    const footer = html.match(/<footer class="site-footer">[\s\S]*?<\/footer>/)?.[0];
    assert.ok(footer);
    for (const heading of ['Product', 'Get started', 'Project']) {
        assert.ok(footer.includes(`<h2>${heading}</h2>`));
    }
    assert.ok(footer.includes('class="footer-bottom"'));
    assert.ok(footer.includes('Back to top'));
    assert.ok(footer.includes('https://github.com/mbolaric/esm-parser/issues'));
    assert.ok(footer.includes('https://mbolaric.github.io/esm-viewer/'));
});
