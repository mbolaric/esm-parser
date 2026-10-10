// @ts-check

(() => {
    const button = document.querySelector('#menu-toggle');
    const navigation = document.querySelector('#site-navigation');
    if (!(button instanceof HTMLButtonElement) || !(navigation instanceof HTMLElement)) return;

    const compact = window.matchMedia('(max-width: 48rem)');
    document.documentElement.classList.add('menu-ready');

    /** @param {boolean} open */
    function setOpen(open) {
        navigation.dataset.open = String(open);
        button.setAttribute('aria-expanded', String(open));
    }

    function synchronizeMenu() {
        button.hidden = !compact.matches;
        setOpen(!compact.matches);
        if (compact.matches && navigation.contains(document.activeElement)) button.focus();
    }

    button.addEventListener('click', () => setOpen(button.getAttribute('aria-expanded') !== 'true'));
    navigation.addEventListener('click', (event) => {
        const anchor = event.target instanceof Element ? event.target.closest('a') : null;
        if (!(anchor instanceof HTMLAnchorElement) || !compact.matches) return;
        setOpen(false);
        const target = anchor.hash ? document.getElementById(anchor.hash.slice(1)) : null;
        if (!(target instanceof HTMLElement)) return;
        // Closing the sticky menu changes the available space above the destination.
        window.requestAnimationFrame(() => {
            target.scrollIntoView({ block: 'start' });
            const heading = target.querySelector('h1, h2');
            if (heading instanceof HTMLElement) {
                heading.tabIndex = -1;
                heading.focus({ preventScroll: true });
            }
        });
    });
    document.addEventListener('keydown', (event) => {
        if (event.key === 'Escape' && compact.matches && button.getAttribute('aria-expanded') === 'true') {
            setOpen(false);
            button.focus();
        }
    });
    compact.addEventListener('change', synchronizeMenu);
    synchronizeMenu();
})();
