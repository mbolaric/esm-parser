const repository = 'https://github.com/mbolaric/esm-parser';

export const packages = [
    { id: 'windows', prefix: 'examples-', suffix: '-windows.zip' },
    { id: 'macos-arm64', prefix: 'examples-', suffix: '-macos-arm64.tar.gz' },
    { id: 'macos-x86_64', prefix: 'examples-', suffix: '-macos-x86_64.tar.gz' },
    { id: 'linux', prefix: 'examples-', suffix: '-linux.tar.gz' },
    { id: 'wasm', prefix: 'esm-parser-', suffix: '-wasm.tar.gz' },
];

export function resolveRelease(release) {
    if (
        !release || release.draft !== false || release.prerelease !== false ||
        typeof release.tag_name !== 'string' || !/^v\d+\.\d+\.\d+$/.test(release.tag_name) ||
        release.html_url !== `${repository}/releases/tag/${release.tag_name}` ||
        !Array.isArray(release.assets)
    ) {
        throw new Error('Expected a published stable esm-parser release with valid metadata.');
    }

    const resolved = packages.map((item) => {
        const name = `${item.prefix}${release.tag_name}${item.suffix}`;
        const matches = release.assets.filter((asset) => asset.name === name);
        if (matches.length > 1) throw new Error(`Ambiguous release archive: ${name}`);
        if (!matches.length) return { id: item.id, url: null };
        const url = `${repository}/releases/download/${release.tag_name}/${name}`;
        if (matches[0].browser_download_url !== url) throw new Error(`Invalid release archive URL: ${name}`);
        return { id: item.id, url };
    });
    if (!resolved.some((item) => item.url)) throw new Error('Release has no supported download archives.');
    return { tag: release.tag_name, url: release.html_url, packages: resolved };
}

export function renderRelease(html, release) {
    for (const item of release.packages) {
        const pattern = new RegExp(`<a\\b([^>]*data-release-asset="${item.id}"[^>]*)>([\\s\\S]*?)</a>`, 'g');
        if ([...html.matchAll(pattern)].length !== 1) throw new Error(`Expected one ${item.id} download link.`);
        html = html.replace(pattern, (_match, attributes, content) =>
            item.url
                ? `<a${attributes.replace(/href="[^"]*"/, `href="${item.url}"`)}>${content}</a>`
                : `<span class="package-unavailable">${content} - not available in this release</span>`,
        );
    }
    const status = /(<p\b[^>]*id="release-status"[^>]*>)[\s\S]*?(<\/p>)/;
    if (!status.test(html)) throw new Error('Website is missing its release status.');
    return html.replace(status, (_match, opening, closing) =>
        `${opening}Latest stable archives: <a href="${release.url}">${release.tag} release notes</a>.${closing}`,
    );
}
