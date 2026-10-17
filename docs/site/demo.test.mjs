import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { test } from 'node:test';
import { runInNewContext } from 'node:vm';

const source = (await readFile(new URL('assets/demo.js', import.meta.url), 'utf8'))
    .replace("import { createJsonTree } from './json-tree.js';", '')
    .replaceAll('import.meta.url', '"https://example.test/esm-parser/assets/demo.js"');

function harness(protocol = 'https:') {
    const elements = new Map();
    const workers = [];
    const revoked = [];
    const created = [];
    const windowEvents = {};
    for (const id of ['file', 'dropzone', 'clear', 'workspace', 'status', 'error', 'result', 'summary', 'output', 'preview-note', 'download', 'search']) {
        elements.set(`#demo-${id}`, {
            disabled: true, hidden: true, textContent: '', value: '', attributes: {}, events: {},
            addEventListener(name, handler) { this.events[name] = handler; },
            setAttribute(name, value) { this.attributes[name] = value; },
            removeAttribute(name) { delete this[name]; },
            focus() { this.focused = true; },
        });
    }
    class MockURL extends URL {
        static createObjectURL(blob) { created.push(blob); return `blob:result-${created.length}`; }
        static revokeObjectURL(url) { revoked.push(url); }
    }
    class MockWorker {
        constructor(url) { this.url = url.href; workers.push(this); }
        postMessage(buffer, transfer) { this.buffer = buffer; this.transfer = transfer; }
        terminate() { this.terminated = true; }
        send(data) { this.onmessage({ data }); }
    }
    runInNewContext(source, {
        document: { querySelector: (selector) => elements.get(selector) },
        window: { addEventListener: (name, handler) => { windowEvents[name] = handler; } },
        location: { protocol }, Worker: MockWorker, WebAssembly: {}, URL: MockURL, Blob,
        createJsonTree(output, search, note) {
            return {
                show(value) { output.textContent = JSON.stringify(value); search.disabled = false; },
                clear() { output.textContent = ''; search.value = ''; search.disabled = true; note.textContent = ''; },
            };
        },
    });
    const get = (id) => elements.get(`#demo-${id}`);
    async function select(file) {
        get('file').files = [file];
        await get('file').events.change();
    }
    return { get, workers, revoked, created, windowEvents, select };
}

const file = (name = 'card.ddd') => ({ name, size: 2, arrayBuffer: async () => new ArrayBuffer(2) });

test('demo remains disabled on file URLs with actionable HTTP guidance', () => {
    const h = harness('file:');
    assert.equal(h.get('file').disabled, true);
    assert.match(h.get('status').textContent, /serve dist\/site over HTTP/);
});

test('file bytes go to a module worker and complete JSON is downloadable', async () => {
    const h = harness();
    await h.select(file('<test>.ddd'));
    const worker = h.workers[0];
    assert.equal(worker.url, 'https://example.test/esm-parser/assets/demo-worker.js');
    assert.equal(worker.transfer[0], worker.buffer);
    worker.send({ type: 'parsing' });
    assert.match(h.get('status').textContent, /Parsing/);
    const json = JSON.stringify({ kind: 'vuGen1', data: { text: '<script>example</script>' } });
    worker.send({ type: 'result', kind: 'vuGen1', json });
    assert.equal(h.get('output').textContent, json);
    assert.match(h.get('summary').textContent, /<test>\.ddd/);
    assert.equal(await h.created[0].text(), json);
    assert.equal(h.get('download').download, '<test>.json');
    assert.equal(h.get('result').hidden, false);
    assert.equal(worker.terminated, true);
    h.get('clear').events.click();
    assert.deepEqual(h.revoked, ['blob:result-1']);
    assert.equal(h.get('output').textContent, '');
    assert.equal(h.get('download').href, undefined);
    assert.equal(h.get('result').hidden, true);
    assert.equal(h.get('file').focused, true);
});

test('large results are passed completely to the lazy tree and download', async () => {
    const h = harness();
    await h.select(file());
    const json = JSON.stringify({ kind: 'vuGen1', data: 'x'.repeat(50_000) });
    h.workers[0].send({ type: 'result', kind: 'vuGen1', json });
    assert.equal(h.get('output').textContent, json);
    assert.equal(await h.created[0].text(), json);
    assert.equal(h.get('search').disabled, false);
});

test('empty files and read failures are visible and retryable', async () => {
    const h = harness();
    await h.select({ ...file(), size: 0 });
    assert.match(h.get('error').textContent, /empty/);
    assert.equal(h.workers.length, 0);
    await h.select({ ...file(), arrayBuffer: async () => { throw new Error('Read denied'); } });
    assert.match(h.get('error').textContent, /Read denied/);
    assert.equal(h.get('file').disabled, false);
    await h.select(file());
    assert.equal(h.get('error').hidden, true);
    assert.equal(h.workers.length, 1);
});

test('parser and worker errors clear busy state and allow recovery', async () => {
    const h = harness();
    await h.select(file());
    h.workers[0].send({ type: 'error', message: 'Could not parse this file.' });
    assert.equal(h.get('error').hidden, false);
    assert.equal(h.get('workspace').attributes['aria-busy'], 'false');
    assert.equal(h.workers[0].terminated, true);
    await h.select(file());
    h.workers[1].onerror({ preventDefault() {} });
    assert.match(h.get('error').textContent, /worker stopped/);
    assert.equal(h.get('file').disabled, false);
});

test('cancel discards an outstanding read and late worker results', async () => {
    const h = harness();
    let finishRead;
    const reading = h.select({ ...file(), arrayBuffer: () => new Promise(resolve => { finishRead = resolve; }) });
    h.get('clear').events.click();
    finishRead(new ArrayBuffer(2));
    await reading;
    assert.equal(h.workers.length, 0);
    await h.select(file());
    h.get('clear').events.click();
    h.workers[0].send({ type: 'result', kind: 'vuGen1', json: '{}' });
    assert.equal(h.created.length, 0);
    assert.equal(h.get('result').hidden, true);
    assert.equal(h.workers[0].terminated, true);
});

test('page exit releases worker and download resources', async () => {
    const h = harness();
    await h.select(file());
    h.workers[0].send({ type: 'result', kind: 'vuGen1', json: '{}' });
    h.windowEvents.pagehide();
    assert.deepEqual(h.revoked, ['blob:result-1']);
    assert.equal(h.get('output').textContent, '');
});

test('file drag highlights the drop zone without flickering across children', () => {
    const h = harness();
    const zone = h.get('dropzone');
    const event = { dataTransfer: { types: ['Files'] }, preventDefault() {} };
    zone.events.dragenter(event);
    zone.events.dragenter(event);
    zone.events.dragleave();
    assert.equal(zone.attributes['data-dragging'], 'true');
    zone.events.dragleave();
    assert.equal(zone.attributes['data-dragging'], 'false');
    zone.events.dragenter({ dataTransfer: { types: ['text/plain'] } });
    assert.equal(zone.attributes['data-dragging'], 'false');
});

test('a dropped file uses the parser and cancels any previous worker', async () => {
    const h = harness();
    await h.select(file());
    let prevented = false;
    await h.get('dropzone').events.drop({
        preventDefault() { prevented = true; },
        dataTransfer: { files: [file('dropped.ddd')] },
    });
    assert.equal(prevented, true);
    assert.equal(h.workers[0].terminated, true);
    assert.equal(h.workers.length, 2);
    h.workers[1].send({ type: 'result', kind: 'vuGen1', json: '{}' });
    assert.equal(h.get('download').download, 'dropped.json');
    assert.match(h.get('summary').textContent, /dropped\.ddd/);
});

test('multiple dropped files are rejected explicitly and non-file drops are ignored', async () => {
    const h = harness();
    const zone = h.get('dropzone');
    await zone.events.drop({ preventDefault() {}, dataTransfer: { files: [file(), file()] } });
    assert.match(h.get('error').textContent, /one tachograph file/);
    assert.equal(h.workers.length, 0);
    await zone.events.drop({ preventDefault() {}, dataTransfer: { files: [] } });
    assert.equal(h.workers.length, 0);
});

test('file drops outside the demo cannot navigate away', () => {
    const h = harness();
    let prevented = false;
    h.windowEvents.drop({ dataTransfer: { types: ['Files'] }, preventDefault() { prevented = true; } });
    assert.equal(prevented, true);
    h.windowEvents.dragover({ dataTransfer: { types: ['text/plain'] }, preventDefault() { assert.fail('Not a file'); } });
});
