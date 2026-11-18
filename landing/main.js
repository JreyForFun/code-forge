/* Code Forge landing page: motion and the live lesson. Motion (motion.dev) drives the animation. */
(() => {
  const root = document.documentElement;
  const M = window.Motion;
  const reduce = matchMedia('(prefers-reduced-motion: reduce)').matches;
  const canAnimate = !!M && !reduce;
  const EASE = [0.23, 1, 0.32, 1];

  // Never leave content hidden: reveal everything if motion is off or slow.
  const reveal = () => root.classList.add('is-ready');
  if (!canAnimate) reveal();
  setTimeout(reveal, 2600);

  const animate = (el, kf, opts) =>
    canAnimate ? M.animate(el, kf, opts) : Promise.resolve();

  /* ---------------- 1. The forge: the logo is built from its own layers ---------------- */
  const forge = document.querySelector('[data-forge]');
  const part = (n) => forge.querySelector(`[data-part="${n}"]`);
  const sparkBox = forge.querySelector('.forge__sparks');
  const glow = forge.querySelector('.forge__glow');
  const stack = forge.querySelector('.forge__stack');

  function sparks(count = 14) {
    if (!canAnimate) return;
    for (let i = 0; i < count; i++) {
      const s = document.createElement('span');
      s.className = 'spark';
      sparkBox.appendChild(s);
      const angle = -Math.PI * (0.05 + Math.random() * 0.9); // upward fan
      const dist = 40 + Math.random() * 70;
      M.animate(
        s,
        {
          x: [0, Math.cos(angle) * dist],
          y: [0, Math.sin(angle) * dist, Math.sin(angle) * dist + 24],
          opacity: [1, 1, 0],
          scale: [1, 0.4],
        },
        { duration: 0.55 + Math.random() * 0.35, ease: 'easeOut' }
      ).then(() => s.remove());
    }
  }

  function strike() {
    if (!canAnimate) return Promise.resolve();
    sparks();
    animate(glow, { opacity: [0.35, 1, 0.45], scale: [1, 1.25, 1] }, { duration: 0.7, ease: EASE });
    return animate(stack, { y: ['0%', '-5%', '1.5%', '0%'], scale: [1, 1.04, 0.98, 1] }, { duration: 0.32, ease: 'easeOut' });
  }

  async function buildHammer() {
    const head = part('head');
    const handle = part('handle');
    const runes = ['lt', 'sl', 'gt'].map(part);

    // Text follows the head down, so the page is readable within half a second.
    const entering = document.querySelectorAll('[data-enter]');
    setTimeout(() => {
      animate(entering, { opacity: [0, 1], y: [14, 0] }, { duration: 0.6, ease: EASE, delay: M.stagger(0.07) });
    }, 220);

    await animate(head, { opacity: [0, 1], y: ['-70%', '0%'] }, { type: 'spring', duration: 0.55, bounce: 0.3 });
    await animate(handle, { opacity: [0, 1], x: ['-30%', '0%'], y: ['30%', '0%'] }, { duration: 0.28, ease: EASE });
    await animate(runes, { opacity: [0, 1], scale: [1.9, 1], y: ['-20%', '0%'] }, { duration: 0.26, ease: EASE, delay: M.stagger(0.07) });
    await strike();
    reveal();
  }

  if (canAnimate) buildHammer();
  forge.addEventListener('click', () => { strike(); });

  /* ---------------- 2. Lesson steps (tabs) ---------------- */
  const tabs = [...document.querySelectorAll('.step[role="tab"]')];
  function select(tab, focus) {
    tabs.forEach((t) => {
      const on = t === tab;
      t.setAttribute('aria-selected', String(on));
      t.tabIndex = on ? 0 : -1;
      const panel = document.getElementById(t.getAttribute('aria-controls'));
      if (on) {
        const wasHidden = panel.hidden;
        panel.hidden = false;
        if (wasHidden) animate(panel, { opacity: [0, 1], y: [10, 0] }, { duration: 0.24, ease: EASE });
      } else {
        panel.hidden = true;
      }
    });
    if (focus) tab.focus();
  }
  tabs.forEach((t, i) => {
    t.addEventListener('click', () => select(t));
    t.addEventListener('keydown', (e) => {
      const keys = { ArrowDown: 1, ArrowRight: 1, ArrowUp: -1, ArrowLeft: -1 };
      if (e.key in keys) { e.preventDefault(); select(tabs[(i + keys[e.key] + tabs.length) % tabs.length], true); }
      else if (e.key === 'Home') { e.preventDefault(); select(tabs[0], true); }
      else if (e.key === 'End') { e.preventDefault(); select(tabs[tabs.length - 1], true); }
    });
  });

  /* ---------------- The live exercise: real content from HTML lesson 1 ---------------- */
  const mini = document.querySelector('[data-fill]');
  const slot = mini.querySelector('[data-slot]');
  const opts = [...mini.querySelectorAll('.opt')];
  const hint = mini.querySelector('[data-hint]');
  const out = mini.querySelector('[data-out]');
  const heartsEl = mini.querySelector('[data-hearts]');
  const xp = mini.querySelector('[data-xp]');
  const reset = mini.querySelector('[data-reset]');
  let hearts = 5;

  function answer(btn, byKeyboard) {
    const correct = btn.hasAttribute('data-correct');
    slot.textContent = btn.dataset.opt;
    slot.classList.toggle('is-ok', correct);
    slot.classList.toggle('is-bad', !correct);

    if (!correct) {
      btn.disabled = true;
      hearts = Math.max(0, hearts - 1);
      heartsEl.textContent = hearts;
      hint.textContent = 'Not quite. A closing tag is </ + the same name + >.';
      animate(slot, { x: [0, -7, 7, -4, 4, 0] }, { duration: 0.34, ease: 'easeInOut' });
      animate(heartsEl.parentElement, { scale: [1, 1.25, 1] }, { duration: 0.3, ease: EASE });
      return;
    }

    opts.forEach((o) => (o.disabled = true));
    btn.classList.add('is-ok');
    hint.textContent = '';
    mini.classList.add('is-done');
    out.hidden = false;
    animate(slot, { scale: [0.8, 1] }, { type: 'spring', duration: 0.35, bounce: 0.35 });
    animate(out, { opacity: [0, 1], clipPath: ['inset(0 0 100% 0)', 'inset(0 0 0% 0)'] }, { duration: 0.4, ease: EASE });
    animate(xp, { opacity: [0, 1], y: [8, 0], scale: [0.9, 1] }, { duration: 0.35, delay: 0.2, ease: EASE });
    // Keep keyboard users in the flow; pointer users don't need a focus ring jumping around.
    if (byKeyboard) reset.focus({ preventScroll: true });
  }

  opts.forEach((o) => o.addEventListener('click', (e) => answer(o, e.detail === 0)));
  mini.addEventListener('keydown', (e) => {
    const n = Number(e.key);
    if (n >= 1 && n <= opts.length && !opts[n - 1].disabled) answer(opts[n - 1], true);
  });
  reset.addEventListener('click', (e) => {
    if (e.detail !== 0) { reset.blur(); }
    hearts = 5;
    heartsEl.textContent = hearts;
    slot.textContent = '___';
    slot.className = 'slot';
    hint.textContent = '';
    out.hidden = true;
    mini.classList.remove('is-done');
    opts.forEach((o) => { o.disabled = false; o.classList.remove('is-ok'); });
    if (e.detail === 0) opts[0].focus({ preventScroll: true });
  });

  /* ---------------- 3. Badges grow bronze -> silver -> gold on hover ---------------- */
  document.querySelectorAll('.ledger tr[data-badge]').forEach((row) => {
    const img = row.querySelector('.ledger__badge');
    const kind = row.dataset.badge;
    const tiers = ['bronze', 'silver', 'gold'];
    tiers.forEach((t) => { const i = new Image(); i.src = `assets/badges/${t}_${kind}.png`; });
    let timers = [];
    const set = (t) => { img.src = `assets/badges/${t}_${kind}.png`; };
    const grow = () => {
      timers.forEach(clearTimeout);
      if (reduce) return set('gold');
      timers = [setTimeout(() => set('silver'), 90), setTimeout(() => { set('gold'); animate(img, { y: [0, -4, 0] }, { duration: 0.26, ease: EASE }); }, 180)];
    };
    const shrink = () => { timers.forEach(clearTimeout); set('bronze'); };
    row.addEventListener('pointerenter', grow);
    row.addEventListener('pointerleave', shrink);
  });

  /* ---------------- The light you carry through the hero ---------------- */
  const hero = document.querySelector('.hero');
  const dark = document.querySelector('[data-carried-light]');
  const finePointer = matchMedia('(hover: hover) and (pointer: fine)').matches;
  if (dark && finePointer && !reduce) {
    let tx = 0, ty = 0, x = 0, y = 0, raf = 0;
    const home = () => {
      const f = forge.getBoundingClientRect(), h = hero.getBoundingClientRect();
      return [f.left - h.left + f.width / 2, f.top - h.top + f.height / 2];
    };
    [x, y] = home(); tx = x; ty = y;
    const paint = () => { dark.style.setProperty('--lx', `${x.toFixed(1)}px`); dark.style.setProperty('--ly', `${y.toFixed(1)}px`); };
    paint();
    const tick = () => {
      // Damped follow: the light trails the hand a little, like a carried torch.
      x += (tx - x) * 0.14; y += (ty - y) * 0.14;
      paint();
      raf = Math.abs(tx - x) + Math.abs(ty - y) > 0.5 ? requestAnimationFrame(tick) : 0;
    };
    hero.addEventListener('pointermove', (e) => {
      const h = hero.getBoundingClientRect();
      tx = e.clientX - h.left; ty = e.clientY - h.top;
      if (!raf) raf = requestAnimationFrame(tick);
    });
    hero.addEventListener('pointerleave', () => {
      [tx, ty] = home();
      if (!raf) raf = requestAnimationFrame(tick);
    });
  } else if (dark) {
    const f = forge.getBoundingClientRect(), h = hero.getBoundingClientRect();
    dark.style.setProperty('--lx', `${f.left - h.left + f.width / 2}px`);
    dark.style.setProperty('--ly', `${f.top - h.top + f.height / 2}px`);
  }

  /* ---------------- The treasure chest opens as you reach for the download ---------------- */
  const chest = document.querySelector('[data-chest]');
  const triggers = document.querySelectorAll('[data-chest-trigger]');
  if (chest && triggers.length) {
    const coinBox = chest.querySelector('.chest__sparks');
    let closeTimer = 0;
    const open = () => {
      clearTimeout(closeTimer);
      if (chest.classList.contains('is-open')) return;
      chest.classList.add('is-open');
      if (!canAnimate) return;
      M.animate(chest, { y: [0, -6, 0], scale: [1, 1.05, 1] }, { duration: 0.32, ease: EASE });
      for (let i = 0; i < 12; i++) {
        const s = document.createElement('span');
        s.className = 'spark';
        coinBox.appendChild(s);
        const dx = (Math.random() - 0.5) * 120;
        const up = 50 + Math.random() * 60;
        M.animate(s, { x: [0, dx], y: [0, -up, -up + 30], opacity: [1, 1, 0] },
          { duration: 0.6 + Math.random() * 0.3, ease: 'easeOut' }).then(() => s.remove());
      }
    };
    const close = () => { closeTimer = setTimeout(() => chest.classList.remove('is-open'), 350); };
    triggers.forEach((t) => {
      t.addEventListener('pointerenter', open);
      t.addEventListener('focus', open);
      t.addEventListener('pointerleave', close);
      t.addEventListener('blur', close);
    });
  }
})();
