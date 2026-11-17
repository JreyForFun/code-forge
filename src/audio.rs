//! Music and sound effects, all synthesised in code (no audio files, nothing to license).
//! Three looping chiptune tracks (title, dungeon, battle) and short effects for every kind of action.
//! If there is no audio device, the game simply stays silent.
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

pub const RATE: u32 = 22_050;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Sfx {
    /// neutral button
    Click,
    /// gold "do it" button
    Confirm,
    /// green "go" button
    Go,
    /// grey back / cancel button
    Back,
    /// red heavy button
    Heavy,
    /// bottom navigation tab
    Nav,
    /// sub-tabs and turning a learn page
    Page,
    /// picking an answer or a code piece
    Select,
    /// a code piece dropped into the gap
    Snap,
    Correct,
    Wrong,
    /// lesson / challenge complete
    Complete,
    LevelUp,
    Coin,
    /// daily quest chest
    Chest,
    /// sword clash at the start of a duel
    DuelStart,
    /// a correct answer in a duel
    Hit,
    /// a wrong answer in a duel
    Miss,
    Victory,
    Defeat,
    /// the hammer striking the anvil in the logo animation
    Forge,
    /// small spark
    Spark,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Track { Title, Dungeon, Battle }

thread_local! {
    /// Sounds requested by widgets this frame: (sound, specific). Generic button clicks are dropped
    /// when a specific sound (correct, wrong, page turn...) was requested in the same frame.
    static QUEUE: RefCell<Vec<(Sfx, bool)>> = const { RefCell::new(Vec::new()) };
}
/// Request a generic UI sound (a plain button press).
pub fn ui(s: Sfx) { QUEUE.with(|q| q.borrow_mut().push((s, false))); }
/// Request a specific, meaningful sound.
pub fn play(s: Sfx) { QUEUE.with(|q| q.borrow_mut().push((s, true))); }
/// Sounds to play this frame.
pub fn take() -> Vec<Sfx> {
    QUEUE.with(|q| {
        let v = std::mem::take(&mut *q.borrow_mut());
        let any_specific = v.iter().any(|(_, sp)| *sp);
        let mut out: Vec<Sfx> = v.into_iter().filter(|(_, sp)| *sp || !any_specific).map(|(s, _)| s).collect();
        out.dedup();
        out
    })
}

// ---------------------------------------------------------------- synthesis
fn midi(m: f32) -> f32 { 440.0 * 2f32.powf((m - 69.0) / 12.0) }
fn secs(s: f32) -> usize { (s * RATE as f32) as usize }

struct Noise(u32);
impl Noise { fn next(&mut self) -> f32 { self.0 ^= self.0 << 13; self.0 ^= self.0 >> 17; self.0 ^= self.0 << 5; (self.0 as f32 / u32::MAX as f32) * 2.0 - 1.0 } }

#[derive(Clone, Copy)]
enum Wave { Square(f32), Tri, Saw, Sine }

fn osc(w: Wave, phase: f32) -> f32 {
    let p = phase.fract();
    match w {
        Wave::Square(d) => if p < d { 1.0 } else { -1.0 },
        Wave::Tri => 4.0 * (p - 0.5).abs() - 1.0,
        Wave::Saw => 2.0 * p - 1.0,
        Wave::Sine => (p * std::f32::consts::TAU).sin(),
    }
}

/// Add a tone into `buf` at `at` seconds: frequency glides from f0 to f1, with attack/decay envelope.
#[allow(clippy::too_many_arguments)]
fn tone(buf: &mut Vec<f32>, at: f32, dur: f32, f0: f32, f1: f32, w: Wave, vol: f32, attack: f32, release: f32) {
    let start = secs(at);
    let n = secs(dur);
    if buf.len() < start + n { buf.resize(start + n, 0.0); }
    let mut ph = 0.0f32;
    for i in 0..n {
        let t = i as f32 / n as f32;
        let f = f0 + (f1 - f0) * t;
        ph += f / RATE as f32;
        let ts = i as f32 / RATE as f32;
        let env = (ts / attack.max(0.001)).min(1.0) * ((dur - ts) / release.max(0.001)).min(1.0);
        buf[start + i] += osc(w, ph) * vol * env;
    }
}

/// Noise burst with a simple low-pass (`bright` 0..1) and exponential decay.
fn noise(buf: &mut Vec<f32>, at: f32, dur: f32, vol: f32, bright: f32, decay: f32, seed: u32) {
    let start = secs(at);
    let n = secs(dur);
    if buf.len() < start + n { buf.resize(start + n, 0.0); }
    let mut ng = Noise(seed | 1);
    let mut lp = 0.0f32;
    for i in 0..n {
        let ts = i as f32 / RATE as f32;
        lp += (ng.next() - lp) * bright;
        buf[start + i] += lp * vol * (-ts * decay).exp() * ((dur - ts) / 0.01).min(1.0);
    }
}

/// Metallic strike: inharmonic partials that ring and decay (anvils, swords).
fn metal(buf: &mut Vec<f32>, at: f32, base: f32, vol: f32, ring: f32) {
    for (k, (ratio, amp)) in [(1.0, 1.0), (2.76, 0.6), (5.40, 0.4), (8.93, 0.25), (13.3, 0.15)].iter().enumerate() {
        let d = ring / (1.0 + k as f32 * 0.6);
        let start = secs(at);
        let n = secs(d);
        if buf.len() < start + n { buf.resize(start + n, 0.0); }
        for i in 0..n {
            let ts = i as f32 / RATE as f32;
            buf[start + i] += (ts * base * ratio * std::f32::consts::TAU).sin() * vol * amp * (-ts * 6.0 / d).exp();
        }
    }
    noise(buf, at, 0.05, vol * 0.8, 0.9, 60.0, (base as u32).wrapping_mul(7919));
}

fn kick(buf: &mut Vec<f32>, at: f32, vol: f32) { tone(buf, at, 0.16, 140.0, 42.0, Wave::Sine, vol, 0.002, 0.08); }
fn snare(buf: &mut Vec<f32>, at: f32, vol: f32) { noise(buf, at, 0.14, vol, 0.6, 22.0, 777); tone(buf, at, 0.06, 220.0, 160.0, Wave::Tri, vol * 0.5, 0.002, 0.04); }
fn hat(buf: &mut Vec<f32>, at: f32, vol: f32) { noise(buf, at, 0.04, vol, 0.95, 90.0, 4242); }

fn finish(mut v: Vec<f32>, gain: f32) -> Vec<f32> {
    let peak = v.iter().fold(0.0f32, |m, x| m.max(x.abs())).max(1e-6);
    let k = gain / peak.max(1.0);
    for x in &mut v { *x = (*x * k).clamp(-1.0, 1.0); }
    v
}

pub fn render_sfx(s: Sfx) -> Vec<f32> {
    let mut b = vec![];
    use Wave::*;
    match s {
        Sfx::Click => tone(&mut b, 0.0, 0.035, 900.0, 820.0, Square(0.5), 0.35, 0.002, 0.02),
        Sfx::Confirm => { tone(&mut b, 0.0, 0.05, midi(76.0), midi(76.0), Square(0.25), 0.35, 0.002, 0.02); tone(&mut b, 0.05, 0.09, midi(83.0), midi(83.0), Square(0.25), 0.35, 0.002, 0.05); }
        Sfx::Go => for (i, m) in [72.0, 76.0, 79.0].iter().enumerate() { tone(&mut b, i as f32 * 0.045, 0.07, midi(*m), midi(*m), Square(0.5), 0.3, 0.002, 0.04); },
        Sfx::Back => { tone(&mut b, 0.0, 0.05, midi(71.0), midi(71.0), Square(0.25), 0.3, 0.002, 0.02); tone(&mut b, 0.05, 0.07, midi(64.0), midi(64.0), Square(0.25), 0.3, 0.002, 0.04); }
        Sfx::Heavy => { kick(&mut b, 0.0, 0.7); tone(&mut b, 0.0, 0.12, 330.0, 220.0, Square(0.5), 0.25, 0.002, 0.06); }
        Sfx::Nav => { tone(&mut b, 0.0, 0.03, 1300.0, 1100.0, Tri, 0.45, 0.001, 0.02); tone(&mut b, 0.0, 0.05, 180.0, 120.0, Sine, 0.4, 0.001, 0.04); }
        Sfx::Page => noise(&mut b, 0.0, 0.16, 0.5, 0.25, 14.0, 99),
        Sfx::Select => tone(&mut b, 0.0, 0.05, midi(81.0), midi(84.0), Square(0.125), 0.3, 0.002, 0.03),
        Sfx::Snap => { tone(&mut b, 0.0, 0.05, 1200.0, 500.0, Square(0.5), 0.3, 0.001, 0.03); noise(&mut b, 0.0, 0.03, 0.4, 0.9, 80.0, 5); }
        Sfx::Correct => for (i, m) in [72.0, 76.0, 79.0, 84.0].iter().enumerate() { tone(&mut b, i as f32 * 0.06, 0.12, midi(*m), midi(*m), Square(0.5), 0.3, 0.002, 0.08); },
        Sfx::Wrong => { tone(&mut b, 0.0, 0.14, 233.0, 220.0, Saw, 0.3, 0.002, 0.05); tone(&mut b, 0.14, 0.26, 175.0, 140.0, Saw, 0.3, 0.002, 0.15); }
        Sfx::Complete => {
            for (i, m) in [67.0, 72.0, 76.0].iter().enumerate() { tone(&mut b, i as f32 * 0.1, 0.12, midi(*m), midi(*m), Square(0.5), 0.3, 0.002, 0.06); }
            for m in [79.0, 84.0, 88.0] { tone(&mut b, 0.32, 0.7, midi(m), midi(m), Square(0.25), 0.2, 0.004, 0.5); }
            tone(&mut b, 0.32, 0.7, midi(48.0), midi(48.0), Tri, 0.5, 0.004, 0.5);
        }
        Sfx::LevelUp => for i in 0..8 { let m = [60.0, 62.0, 64.0, 67.0, 69.0, 72.0, 76.0, 79.0][i]; tone(&mut b, i as f32 * 0.04, if i == 7 { 0.4 } else { 0.06 }, midi(m), midi(m), Square(0.5), 0.28, 0.002, 0.04); },
        Sfx::Coin => { tone(&mut b, 0.0, 0.06, midi(83.0), midi(83.0), Square(0.5), 0.3, 0.001, 0.02); tone(&mut b, 0.06, 0.28, midi(88.0), midi(88.0), Square(0.5), 0.3, 0.001, 0.2); }
        Sfx::Chest => {
            noise(&mut b, 0.0, 0.3, 0.35, 0.05, 6.0, 31); tone(&mut b, 0.0, 0.3, 90.0, 140.0, Saw, 0.12, 0.05, 0.1);
            for (i, m) in [84.0, 88.0, 91.0, 96.0].iter().enumerate() { tone(&mut b, 0.3 + i as f32 * 0.05, 0.25, midi(*m), midi(*m), Tri, 0.3, 0.002, 0.2); }
        }
        Sfx::DuelStart => {
            metal(&mut b, 0.0, 1180.0, 0.5, 0.7); metal(&mut b, 0.09, 1490.0, 0.45, 0.6);
            kick(&mut b, 0.35, 0.8); kick(&mut b, 0.55, 0.8); snare(&mut b, 0.75, 0.6);
            tone(&mut b, 0.35, 0.6, midi(40.0), midi(40.0), Saw, 0.25, 0.01, 0.3);
        }
        Sfx::Hit => { kick(&mut b, 0.0, 0.9); noise(&mut b, 0.0, 0.12, 0.5, 0.5, 30.0, 12); metal(&mut b, 0.02, 1700.0, 0.25, 0.25); }
        Sfx::Miss => { noise(&mut b, 0.0, 0.3, 0.45, 0.15, 6.0, 77); tone(&mut b, 0.0, 0.3, 500.0, 120.0, Sine, 0.2, 0.01, 0.15); }
        Sfx::Victory => {
            let notes = [(0.0, 67.0, 0.12), (0.13, 67.0, 0.12), (0.26, 67.0, 0.12), (0.39, 72.0, 0.6)];
            for (t, m, d) in notes { tone(&mut b, t, d, midi(m), midi(m), Square(0.5), 0.3, 0.002, 0.05); tone(&mut b, t, d, midi(m - 12.0), midi(m - 12.0), Tri, 0.4, 0.002, 0.05); }
            for m in [76.0, 79.0] { tone(&mut b, 0.39, 0.6, midi(m), midi(m), Square(0.25), 0.18, 0.004, 0.4); }
            snare(&mut b, 0.39, 0.5);
        }
        Sfx::Defeat => for (i, m) in [67.0, 66.0, 65.0, 64.0].iter().enumerate() { tone(&mut b, i as f32 * 0.22, if i == 3 { 0.7 } else { 0.22 }, midi(*m), midi(*m - if i == 3 { 0.5 } else { 0.0 }), Square(0.5), 0.28, 0.003, 0.1); },
        Sfx::Forge => { metal(&mut b, 0.0, 820.0, 0.6, 1.2); kick(&mut b, 0.0, 0.7); noise(&mut b, 0.0, 0.08, 0.5, 0.7, 40.0, 3); }
        Sfx::Spark => { noise(&mut b, 0.0, 0.05, 0.3, 0.98, 70.0, 8); tone(&mut b, 0.0, 0.04, 3000.0, 2400.0, Square(0.5), 0.12, 0.001, 0.02); }
    }
    finish(b, 0.9)
}

// ---------------------------------------------------------------- music
/// One loop of a track. Lead/bass are MIDI numbers per 8th note (0 = rest, -1 = hold).
pub fn render_track(t: Track) -> Vec<f32> {
    let mut b = vec![];
    match t {
        Track::Dungeon => {
            // A minor, slow and mysterious: Am F G Em, torch-lit corridors
            let bpm = 92.0; let e = 60.0 / bpm / 2.0; // eighth note
            let chords: [[f32; 3]; 4] = [[57.0, 60.0, 64.0], [53.0, 57.0, 60.0], [55.0, 59.0, 62.0], [52.0, 55.0, 59.0]];
            let lead: [&[f32]; 8] = [
                &[76.0, -1.0, 0.0, 72.0, 74.0, -1.0, 76.0, -1.0], &[77.0, -1.0, -1.0, 76.0, 74.0, -1.0, 72.0, -1.0],
                &[74.0, -1.0, 0.0, 71.0, 72.0, -1.0, 74.0, -1.0], &[71.0, -1.0, -1.0, -1.0, 0.0, 0.0, 0.0, 0.0],
                &[76.0, -1.0, 79.0, -1.0, 77.0, 76.0, 74.0, -1.0], &[72.0, -1.0, 74.0, 72.0, 69.0, -1.0, -1.0, 0.0],
                &[71.0, 72.0, 74.0, -1.0, 76.0, -1.0, 74.0, 71.0], &[69.0, -1.0, -1.0, -1.0, -1.0, -1.0, 0.0, 0.0],
            ];
            for bar in 0..16 {
                let t0 = bar as f32 * 8.0 * e;
                let ch = chords[bar % 4];
                for s in 0..8 {
                    let at = t0 + s as f32 * e;
                    // arpeggio, quiet and thin
                    let m = ch[[0, 1, 2, 1, 0, 1, 2, 1][s]] + 12.0;
                    tone(&mut b, at, e * 0.9, midi(m), midi(m), Wave::Square(0.125), 0.07, 0.003, 0.05);
                    if s % 4 == 0 { tone(&mut b, at, e * 3.6, midi(ch[0] - 12.0), midi(ch[0] - 12.0), Wave::Tri, 0.32, 0.005, 0.2); }
                    if s == 0 { kick(&mut b, at, 0.35); }
                    if s % 2 == 1 { hat(&mut b, at, 0.06); }
                    if s == 4 && bar % 2 == 1 { snare(&mut b, at, 0.16); }
                }
                if bar >= 4 { add_line(&mut b, lead[(bar - 4) % 8], t0, e, Wave::Square(0.5), 0.14); }
            }
        }
        Track::Title => {
            // D minor, slow and grand: Dm Bb C Am, a forge glowing in the deep
            let bpm = 72.0; let e = 60.0 / bpm / 2.0;
            let chords: [[f32; 3]; 4] = [[50.0, 53.0, 57.0], [46.0, 50.0, 53.0], [48.0, 52.0, 55.0], [45.0, 48.0, 52.0]];
            let lead: [&[f32]; 4] = [&[74.0, -1.0, -1.0, 72.0, 69.0, -1.0, -1.0, -1.0], &[70.0, -1.0, 72.0, -1.0, 74.0, -1.0, -1.0, -1.0],
                &[76.0, -1.0, -1.0, 74.0, 72.0, -1.0, 67.0, -1.0], &[69.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0]];
            for bar in 0..8 {
                let t0 = bar as f32 * 8.0 * e;
                let ch = chords[bar % 4];
                for m in ch { tone(&mut b, t0, 8.0 * e, midi(m), midi(m), Wave::Tri, 0.12, 0.4, 0.6); }
                tone(&mut b, t0, 8.0 * e, midi(ch[0] - 12.0), midi(ch[0] - 12.0), Wave::Tri, 0.25, 0.1, 0.5);
                for s in 0..8 { let m = ch[s % 3] + 24.0; tone(&mut b, t0 + s as f32 * e, e * 1.6, midi(m), midi(m), Wave::Sine, 0.06, 0.003, e * 1.4); }
                if bar >= 2 { add_line(&mut b, lead[(bar - 2) % 4], t0, e, Wave::Square(0.25), 0.1); }
                metal(&mut b, t0, 610.0, 0.05, 0.8); // distant anvil on every bar
            }
        }
        Track::Battle => {
            // E minor, fast and driving: Em C D B, swords out
            let bpm = 150.0; let e = 60.0 / bpm / 2.0;
            let roots = [40.0, 36.0, 38.0, 35.0];
            let lead: [&[f32]; 8] = [
                &[76.0, 76.0, 79.0, 76.0, 83.0, -1.0, 81.0, 79.0], &[76.0, -1.0, 74.0, 72.0, 71.0, -1.0, 72.0, 74.0],
                &[74.0, 74.0, 78.0, 74.0, 81.0, -1.0, 79.0, 78.0], &[75.0, -1.0, 78.0, -1.0, 83.0, -1.0, -1.0, 0.0],
                &[88.0, -1.0, 86.0, 84.0, 83.0, 81.0, 79.0, 81.0], &[83.0, -1.0, 79.0, -1.0, 76.0, -1.0, 72.0, 74.0],
                &[78.0, 79.0, 81.0, 78.0, 74.0, 78.0, 81.0, 86.0], &[87.0, -1.0, 83.0, -1.0, 78.0, -1.0, 75.0, -1.0],
            ];
            for bar in 0..16 {
                let t0 = bar as f32 * 8.0 * e;
                let r = roots[bar % 4];
                for s in 0..8 {
                    let at = t0 + s as f32 * e;
                    let m = if s % 2 == 0 { r } else { r + 12.0 };
                    tone(&mut b, at, e * 0.85, midi(m), midi(m), Wave::Square(0.5), 0.16, 0.002, 0.03);
                    if s % 4 == 0 { kick(&mut b, at, 0.6); }
                    if s % 4 == 2 { snare(&mut b, at, 0.35); }
                    hat(&mut b, at, 0.07);
                    let ch = [r + 24.0, r + 27.0 + if bar % 4 == 3 { 1.0 } else { 0.0 }, r + 31.0];
                    tone(&mut b, at, e * 0.5, midi(ch[s % 3]), midi(ch[s % 3]), Wave::Square(0.125), 0.05, 0.002, 0.02);
                }
                add_line(&mut b, lead[bar % 8], t0, e, Wave::Square(0.25), 0.14);
                if bar % 4 == 0 { metal(&mut b, t0, 1250.0, 0.08, 0.4); }
            }
        }
    }
    finish(b, 0.8)
}

fn add_line(b: &mut Vec<f32>, line: &[f32], t0: f32, e: f32, w: Wave, vol: f32) {
    let mut i = 0;
    while i < line.len() {
        let m = line[i];
        if m <= 0.0 { i += 1; continue; }
        let mut len = 1;
        while i + len < line.len() && line[i + len] == -1.0 { len += 1; }
        let d = len as f32 * e;
        tone(b, t0 + i as f32 * e, d * 0.95, midi(m), midi(m), w, vol, 0.004, (d * 0.4).min(0.15));
        // a soft echo, like a stone hall
        tone(b, t0 + i as f32 * e + e * 0.75, d * 0.8, midi(m), midi(m), w, vol * 0.25, 0.004, 0.1);
        i += len;
    }
}

/// Write every track and sound effect as 16-bit WAV files (for listening outside the game).
pub fn export(dir: &std::path::Path) -> std::io::Result<usize> {
    std::fs::create_dir_all(dir)?;
    let wav = |name: &str, v: &[f32]| -> std::io::Result<()> {
        let mut b = Vec::with_capacity(44 + v.len() * 2);
        let data = (v.len() * 2) as u32;
        b.extend_from_slice(b"RIFF"); b.extend_from_slice(&(36 + data).to_le_bytes()); b.extend_from_slice(b"WAVEfmt ");
        b.extend_from_slice(&16u32.to_le_bytes()); b.extend_from_slice(&1u16.to_le_bytes()); b.extend_from_slice(&1u16.to_le_bytes());
        b.extend_from_slice(&RATE.to_le_bytes()); b.extend_from_slice(&(RATE * 2).to_le_bytes()); b.extend_from_slice(&2u16.to_le_bytes()); b.extend_from_slice(&16u16.to_le_bytes());
        b.extend_from_slice(b"data"); b.extend_from_slice(&data.to_le_bytes());
        for x in v { b.extend_from_slice(&((x.clamp(-1.0, 1.0) * 32767.0) as i16).to_le_bytes()); }
        std::fs::write(dir.join(format!("{}.wav", name)), b)
    };
    let mut n = 0;
    for t in [Track::Title, Track::Dungeon, Track::Battle] { wav(&format!("music_{:?}", t).to_lowercase(), &render_track(t))?; n += 1; }
    for s in ALL_SFX { wav(&format!("sfx_{:?}", s).to_lowercase(), &render_sfx(s))?; n += 1; }
    Ok(n)
}

pub const ALL_SFX: [Sfx; 22] = [Sfx::Click, Sfx::Confirm, Sfx::Go, Sfx::Back, Sfx::Heavy, Sfx::Nav, Sfx::Page, Sfx::Select, Sfx::Snap, Sfx::Correct, Sfx::Wrong, Sfx::Complete,
    Sfx::LevelUp, Sfx::Coin, Sfx::Chest, Sfx::DuelStart, Sfx::Hit, Sfx::Miss, Sfx::Victory, Sfx::Defeat, Sfx::Forge, Sfx::Spark];

// ---------------------------------------------------------------- playback
/// Live audio output. `None` when there is no sound device.
pub struct Audio {
    _stream: rodio::OutputStream,
    handle: rodio::OutputStreamHandle,
    music: rodio::Sink,
    current: Option<Track>,
    cache: HashMap<Sfx, Arc<Vec<f32>>>,
    tracks: Arc<std::sync::Mutex<HashMap<Track, Arc<Vec<f32>>>>>,
}

impl Audio {
    pub fn new() -> Option<Audio> {
        let (stream, handle) = rodio::OutputStream::try_default().ok()?;
        let music = rodio::Sink::try_new(&handle).ok()?;
        let tracks: Arc<std::sync::Mutex<HashMap<Track, Arc<Vec<f32>>>>> = Arc::default();
        // compose the music in the background so start-up stays instant
        let t2 = tracks.clone();
        std::thread::spawn(move || {
            for t in [Track::Title, Track::Dungeon, Track::Battle] {
                let v = Arc::new(render_track(t));
                if let Ok(mut m) = t2.lock() { m.insert(t, v); }
            }
        });
        Some(Audio { _stream: stream, handle, music, current: None, cache: HashMap::new(), tracks })
    }

    pub fn sfx(&mut self, s: Sfx, volume: f32) {
        if volume <= 0.0 { return; }
        use rodio::Source;
        let data = self.cache.entry(s).or_insert_with(|| Arc::new(render_sfx(s))).clone();
        let src = rodio::buffer::SamplesBuffer::new(1, RATE, (*data).clone()).amplify(volume);
        let _ = self.handle.play_raw(src.convert_samples());
    }

    /// Switch the looping music (no-op if it is already playing). `None` stops it.
    pub fn music(&mut self, t: Option<Track>, volume: f32) {
        self.music.set_volume(volume * 0.55);
        if volume <= 0.0 { if self.current.is_some() { self.music.stop(); self.current = None; } return; }
        if t == self.current { return; }
        let Some(t) = t else { self.music.stop(); self.current = None; return };
        let Some(data) = self.tracks.lock().ok().and_then(|m| m.get(&t).cloned()) else { return }; // still composing
        use rodio::Source;
        self.music.stop();
        self.music = match rodio::Sink::try_new(&self.handle) { Ok(s) => s, Err(_) => return };
        self.music.set_volume(volume * 0.55);
        self.music.append(rodio::buffer::SamplesBuffer::new(1, RATE, (*data).clone()).repeat_infinite());
        self.current = Some(t);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_sound_and_track_renders_clean() {
        for s in [Sfx::Click, Sfx::Confirm, Sfx::Go, Sfx::Back, Sfx::Heavy, Sfx::Nav, Sfx::Page, Sfx::Select, Sfx::Snap, Sfx::Correct, Sfx::Wrong, Sfx::Complete,
            Sfx::LevelUp, Sfx::Coin, Sfx::Chest, Sfx::DuelStart, Sfx::Hit, Sfx::Miss, Sfx::Victory, Sfx::Defeat, Sfx::Forge, Sfx::Spark] {
            let v = render_sfx(s);
            assert!(!v.is_empty() && v.len() < RATE as usize * 3, "{:?} length", s);
            assert!(v.iter().all(|x| x.is_finite() && x.abs() <= 1.0), "{:?} clips", s);
            assert!(v.iter().any(|x| x.abs() > 0.1), "{:?} is silent", s);
        }
        for t in [Track::Title, Track::Dungeon, Track::Battle] {
            let v = render_track(t);
            assert!(v.len() > RATE as usize * 15, "{:?} too short", t);
            assert!(v.iter().all(|x| x.is_finite() && x.abs() <= 1.0));
        }
    }
    #[test]
    fn specific_sounds_override_generic_clicks() {
        ui(Sfx::Click); play(Sfx::Correct);
        assert_eq!(take(), vec![Sfx::Correct]);
        ui(Sfx::Nav);
        assert_eq!(take(), vec![Sfx::Nav]);
    }
}
