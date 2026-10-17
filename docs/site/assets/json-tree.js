export function* entries(value) {
    if (value === null || typeof value !== 'object') return;
    if (Array.isArray(value)) {
        for (let index = 0; index < value.length; index += 1) yield [String(index), value[index]];
    } else {
        for (const key of Object.keys(value)) yield [key, value[key]];
    }
}

export function* walkJson(value) {
    const stack = [{ iterator: entries({ $: value }), path: [] }];
    while (stack.length) {
        const frame = stack[stack.length - 1];
        const next = frame.iterator.next();
        if (next.done) {
            stack.pop();
            continue;
        }
        const [key, child] = next.value;
        const path = [...frame.path, key];
        yield { key, value: child, path };
        if (child !== null && typeof child === 'object') stack.push({ iterator: entries(child), path });
    }
}

export function matchesJson(entry, query) {
    return entry.key.toLowerCase().includes(query) ||
        (entry.value === null || typeof entry.value !== 'object') &&
        String(entry.value).toLowerCase().includes(query);
}

export function describe(value) {
    if (value === null) return 'null';
    if (Array.isArray(value)) return `Array (${value.length})`;
    if (typeof value === 'object') return `Object (${Object.keys(value).length})`;
    return JSON.stringify(value);
}

export function createJsonTree(container, search, status) {
    let root;
    let revision = 0;
    let timer;
    const pageSize = 100;
    const searchLimit = 200;

    function element(tag, className, text) {
        const node = document.createElement(tag);
        node.className = className;
        if (text !== undefined) node.textContent = text;
        return node;
    }

    function row(key, value) {
        const complex = value !== null && typeof value === 'object';
        const node = element(complex ? 'details' : 'div', 'json-node');
        const line = complex ? element('summary', 'json-line') : node;
        line.append(element('span', 'json-key', key), document.createTextNode(': '),
            element('span', complex ? 'json-type' : `json-value json-${typeof value}`, describe(value)));
        if (complex) {
            node.append(line);
            node.addEventListener('toggle', () => {
                if (!node.open || node.childElementCount > 1) return;
                const children = element('div', 'json-children');
                node.append(children);
                populate(children, entries(value));
            });
        }
        return node;
    }

    function populate(parent, iterator) {
        const more = element('button', 'json-more', 'Show next 100 entries');
        more.type = 'button';
        function nextPage() {
            more.remove();
            const fragment = document.createDocumentFragment();
            for (let count = 0; count < pageSize; count += 1) {
                const next = iterator.next();
                if (next.done) {
                    parent.append(fragment);
                    return;
                }
                fragment.append(row(...next.value));
            }
            parent.append(fragment, more);
        }
        more.addEventListener('click', nextPage);
        nextPage();
    }

    function browse() {
        container.replaceChildren();
        container.scrollTop = 0;
        populate(container, entries({ $: root }));
        const first = container.querySelector('details');
        if (first) first.open = true;
        status.textContent = 'Expand objects and arrays to explore. All data is searchable. Signatures have not been verified.';
    }

    async function filter(current) {
        const query = search.value.trim().toLowerCase();
        if (!query) {
            browse();
            return;
        }
        container.replaceChildren();
        container.scrollTop = 0;
        status.textContent = 'Searching keys and values...';
        let found = 0;
        let scanned = 0;
        for (const entry of walkJson(root)) {
            if (current !== revision) return;
            if (matchesJson(entry, query)) {
                const match = element('div', 'json-match');
                match.append(element('p', 'json-path', entry.path.map(key => `[${JSON.stringify(key)}]`).join('')),
                    row(entry.key, entry.value));
                container.append(match);
                found += 1;
                if (found === searchLimit) {
                    status.textContent = `Showing the first ${searchLimit} matches. Refine your search for more specific results.`;
                    return;
                }
            }
            scanned += 1;
            if (scanned % 500 === 0) await new Promise(resolve => setTimeout(resolve, 0));
        }
        status.textContent = found ? `${found} matching keys or values. Expand a matching object to inspect its contents.` : 'No matching keys or values.';
    }

    search.addEventListener('input', () => {
        clearTimeout(timer);
        const current = ++revision;
        timer = setTimeout(() => { void filter(current); }, 150);
    });

    return {
        show(value) {
            this.clear();
            root = value;
            search.disabled = false;
            browse();
        },
        clear() {
            revision += 1;
            clearTimeout(timer);
            root = undefined;
            search.value = '';
            search.disabled = true;
            status.textContent = '';
            container.replaceChildren();
        },
    };
}
