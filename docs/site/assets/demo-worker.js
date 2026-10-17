self.onmessage = async ({ data: buffer }) => {
    let parser;
    try {
        parser = await import('./wasm/esm_parser.js');
        await parser.default();
    } catch {
        self.postMessage({
            type: 'error',
            message: 'Could not load the WebAssembly parser. Check your connection, rebuild the site if previewing locally, and select the file again.',
        });
        return;
    }

    self.postMessage({ type: 'parsing' });
    try {
        const result = parser.parse_from_memory(new Uint8Array(buffer));
        self.postMessage({ type: 'result', kind: result.kind, json: JSON.stringify(result, null, 2) });
    } catch (error) {
        const detail = error instanceof Error ? error.message : String(error);
        self.postMessage({
            type: 'error',
            message: `Could not parse this file: ${detail}. Choose a supported, complete tachograph download. For very large files, try the native CLI.`,
        });
    }
};
