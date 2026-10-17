const input = document.querySelector('#demo-file');
const dropzone = document.querySelector('#demo-dropzone');
const clear = document.querySelector('#demo-clear');
const workspace = document.querySelector('#demo-workspace');
const status = document.querySelector('#demo-status');
const error = document.querySelector('#demo-error');
const result = document.querySelector('#demo-result');
const summary = document.querySelector('#demo-summary');
const output = document.querySelector('#demo-output');
const note = document.querySelector('#demo-preview-note');
const download = document.querySelector('#demo-download');
const search = document.querySelector('#demo-search');
const tree = createJsonTree(output, search, note);
let worker;
let downloadUrl;
let request = 0;
let dragDepth = 0;

function clearDrag() {
    dragDepth = 0;
    dropzone.setAttribute('data-dragging', 'false');
}

// Prevent file drops outside the demo from navigating away from the page.
for (const name of ['dragover', 'drop']) {
    window.addEventListener(name, (event) => {
        if (Array.from(event.dataTransfer?.types ?? []).includes('Files')) event.preventDefault();
        if (name === 'drop') clearDrag();
    });
}
window.addEventListener('dragend', clearDrag);

function stopWorker() {
    worker?.terminate();
    worker = undefined;
    workspace.setAttribute('aria-busy', 'false');
    input.disabled = false;
}

function reset() {
    clearDrag();
    request += 1;
    stopWorker();
    if (downloadUrl) URL.revokeObjectURL(downloadUrl);
    downloadUrl = undefined;
    download.removeAttribute('href');
    download.removeAttribute('download');
    result.hidden = true;
    error.hidden = true;
    error.textContent = '';
    tree.clear();
    summary.textContent = '';
    note.textContent = '';
    input.value = '';
    clear.disabled = true;
}

function fail(message) {
    stopWorker();
    error.textContent = message;
    error.hidden = false;
    status.textContent = 'No result. Select another file to try again.';
}

if (!['http:', 'https:'].includes(location.protocol)) {
    status.textContent = 'To use the demo, build the site and serve dist/site over HTTP. See the website preview instructions in the repository.';
} else if (typeof Worker === 'undefined' || typeof WebAssembly === 'undefined') {
    status.textContent = 'This browser does not support WebAssembly or Web Workers. Use a current browser or the native CLI examples.';
} else {
    input.disabled = false;
    status.textContent = 'Ready. Choose a file to load the parser. No file data leaves your browser.';

    clear.addEventListener('click', () => {
        reset();
        status.textContent = 'Cleared. Choose a file to start again.';
        input.focus();
    });

    async function parseFile(file) {
        if (!file) return;
        reset();
        const current = request;
        clear.disabled = false;
        if (file.size === 0) {
            fail('This file is empty. Choose a complete tachograph download.');
            return;
        }
        input.disabled = true;
        workspace.setAttribute('aria-busy', 'true');
        status.textContent = `Reading ${file.name} locally and loading the WebAssembly parser...`;
        try {
            const buffer = await file.arrayBuffer();
            if (current !== request) return;
            worker = new Worker(new URL('./demo-worker.js', import.meta.url), { type: 'module' });
            worker.onmessage = ({ data }) => {
                if (current !== request) return;
                if (data.type === 'parsing') {
                    status.textContent = `Parsing ${file.name} in your browser...`;
                } else if (data.type === 'error') {
                    fail(data.message);
                } else if (data.type === 'result') {
                    stopWorker();
                    downloadUrl = URL.createObjectURL(new Blob([data.json], { type: 'application/json' }));
                    download.href = downloadUrl;
                    download.download = `${file.name.replace(/\.[^.]+$/, '') || 'tachograph'}.json`;
                    summary.textContent = `${file.name} / ${file.size.toLocaleString()} bytes / ${data.kind}`;
                    tree.show(JSON.parse(data.json));
                    result.hidden = false;
                    status.textContent = 'Parsed locally. Signatures have not been verified.';
                }
            };
            worker.onerror = (event) => {
                event.preventDefault();
                if (current === request) fail('The parser worker stopped unexpectedly. Try another file, or use the native CLI for this download.');
            };
            worker.postMessage(buffer, [buffer]);
        } catch (cause) {
            if (current !== request) return;
            const detail = cause instanceof Error ? cause.message : String(cause);
            fail(`Could not read the file or start the parser: ${detail}. Select the file again.`);
        }
    }

    input.addEventListener('change', () => parseFile(input.files?.[0]));
    dropzone.addEventListener('dragenter', (event) => {
        if (!Array.from(event.dataTransfer?.types ?? []).includes('Files')) return;
        event.preventDefault();
        dragDepth += 1;
        dropzone.setAttribute('data-dragging', 'true');
    });
    dropzone.addEventListener('dragover', (event) => {
        if (!Array.from(event.dataTransfer?.types ?? []).includes('Files')) return;
        event.preventDefault();
        event.dataTransfer.dropEffect = 'copy';
    });
    dropzone.addEventListener('dragleave', () => {
        dragDepth = Math.max(0, dragDepth - 1);
        if (dragDepth === 0) clearDrag();
    });
    dropzone.addEventListener('drop', (event) => {
        event.preventDefault();
        clearDrag();
        const files = event.dataTransfer?.files;
        if (!files?.length) return;
        if (files.length !== 1) {
            reset();
            fail('Drop one tachograph file at a time. Choose a single download to parse.');
            return;
        }
        return parseFile(files[0]);
    });
}

window.addEventListener('pagehide', reset);
import { createJsonTree } from './json-tree.js';
