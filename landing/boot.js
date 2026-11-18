/* Runs before first paint: opt into entrance motion unless the visitor prefers reduced motion. */
if (!matchMedia('(prefers-reduced-motion: reduce)').matches) document.documentElement.classList.add('js-motion');
