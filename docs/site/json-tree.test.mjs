import assert from 'node:assert/strict';
import { test } from 'node:test';
import { entries, walkJson, matchesJson, describe } from './assets/json-tree.js';

test('walk visits every key and value including large arrays and deep paths', () => {
    const value = { items: Array.from({ length: 1000 }, (_, index) => ({ value: index })), end: null };
    const rows = [...walkJson(value)];
    assert.equal(rows.length, 2003);
    assert.deepEqual(rows.find(entry => entry.value === 999).path, ['$', 'items', '999', 'value']);
    assert.deepEqual([...entries(null)], []);
});

test('search matches keys, strings, numbers, booleans and null, case insensitively', () => {
    const rows = [...walkJson({ DriverName: 'Alice', distance: 12345, signed: false, missing: null })];
    for (const query of ['driver', 'alice', '12345', 'false', 'null']) {
        assert.equal(rows.filter(entry => matchesJson(entry, query)).length, 1);
    }
    assert.equal(rows.filter(entry => matchesJson(entry, 'not present')).length, 0);
});

test('searchable keys preserve special characters and prototype-like names', () => {
    const value = JSON.parse('{"__proto__": {"<script>": "a/b~c"}}');
    const match = [...walkJson(value)].find(entry => matchesJson(entry, 'a/b~c'));
    assert.deepEqual(match.path, ['$', '__proto__', '<script>']);
});

test('summaries distinguish objects, arrays and primitive types', () => {
    assert.equal(describe({ value: true }), 'Object (1)');
    assert.equal(describe([1, 2]), 'Array (2)');
    assert.equal(describe(null), 'null');
    assert.equal(describe('text'), '"text"');
    assert.equal(describe(false), 'false');
});
