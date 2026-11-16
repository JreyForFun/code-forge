#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
//! Pixel Learn: learn to code in a pixel-art RPG. Offline, single player.
mod art;
mod content;
mod css;
mod data;
mod game;
mod interp;
mod lesson;
mod onboarding;
mod preview;
mod save;
mod screens;
mod sql;
mod tabs;

use art::*;
use preview::*;
use data::*;
use eframe::egui::{self, Align2, Color32, FontId, Pos2, Rect, RichText, Sense, Stroke, Vec2};
use game::{HINT_COST, LEAGUES, LEAGUE_UP_XP, REFILL_COST, SAVER_COST, XP_PER_LEVEL};
use save::{Save, MAX_HEARTS};
use std::collections::VecDeque;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
const HDR: Color32 = Color32::from_rgb(0x0b, 0x12, 0x28);

fn now() -> u64 { std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0) }
fn today() -> u64 { game::day_of(now()) }

// ---------------- state ----------------
#[derive(Clone, Copy, PartialEq)]
enum Tab { Learn, Train, Ranks, Create, Hero }
#[derive(Clone, Copy, PartialEq)]
enum Modal { None, Notif, Shop, Courses, NoHearts, Reset, Avatar, DeleteSpell(usize) }

#[derive(Clone)]
enum Kind { Lesson(usize, usize, usize), Coach, Duel(usize) }

struct RunState {
    title: String, kind: Kind, lang: &'static str, slides: Vec<(&'static str, &'static str)>, qs: Vec<Q>, stage: usize, queue: Vec<usize>,
    sel: Option<usize>, text: String, fb: Option<(bool, String)>, mistakes: u32, right: u32, total: usize, last: Option<usize>, hint_q: Option<usize>, project: bool,
}
struct Complete { title: String, xp: u32, gems: u32, perfect: bool, score: u32, best: u32, duel: Option<(u32, u32, String, bool)>, new_streak: bool, next: Option<(usize, usize, usize)>, project: bool }
struct Editor { idx: Option<usize>, title: String, lang: String, code: String, out: Vec<(interp::Kind, String)> }

enum Screen { Main, Settings, Run(Box<RunState>), Complete(Complete), Editor(Editor), Preview(usize) }

struct App {
    s: Save, tab: Tab, screen: Screen, modal: Modal, courses: Vec<Course>, cur: usize, open_mod: Option<usize>,
    opp: usize, sub_train: usize, sub_create: usize, input: String, cat: usize, query: String,
    view_key: u32, view_t0: f64, toasts: VecDeque<(String, f64)>, demo: bool, shot: Option<String>, frames: u32, save_warned: bool,
    onboard: Option<onboarding::Onboard>,
}

impl App {
    fn new(shot: Option<String>) -> Self {
        let demo = shot.is_some();
        let mut s = if demo { Save::new() } else { Save::load() };
        content::migrate(&mut s);
        if s.heart_t == 0 { s.heart_t = now(); }
        if demo {
            // In-memory demo profile for screenshots. Never written to disk.
            s.onboarded = true; s.name = "Ada".into(); s.avatar = 1; s.xp = 399; s.gems = 65; s.streak = 3; s.streak_max = 4; s.last_day = today();
            s.week_id = game::week_of(today()); s.week_xp = 120; s.week_mask = 0b0000101; s.league = 2; s.hearts = 4;
            for k in ["html/your-first-tags", "html/headings", "html/paragraphs-and-breaks", "html/bold-italic-and-more"] { s.done.insert(k.into()); }
            s.ach.insert("first_lesson".into(), today() - 3); s.ach.insert("streak3".into(), today() - 1); s.ach.insert("xp100".into(), today() - 2);
            s.codes.push(("Torch Counter".into(), "python".into(), "for i in range(3):\n  print(\"Torch\", i + 1)\n".into()));
            s.add_log(today() - 1, "cleared 'Tags and Elements'");
            s.add_log(today(), "reached a 3 day streak");
        }
        let courses = courses();
        let cur = (s.course as usize).min(courses.len() - 1);
        let onboard = if s.onboarded { None } else { Some(onboarding::Onboard::default()) };
        let mut a = App { s, tab: Tab::Learn, screen: Screen::Main, modal: Modal::None, courses, cur, open_mod: None, opp: 0, sub_train: 0, sub_create: 0,
            input: String::new(), cat: 0, query: String::new(), view_key: 999, view_t0: 0.0, toasts: VecDeque::new(), demo, shot, frames: 0, save_warned: false, onboard };
        a.week_check();
        a.open_mod = a.current_module(a.cur);
        a
    }

    /// Save to disk (atomic). Warns once if the save folder can't be written.
    fn persist(&mut self) {
        if self.demo { return; }
        if let Err(e) = self.s.store()
            && !self.save_warned { self.save_warned = true; self.toast(format!("Can't save progress: {}", e)); }
    }
    fn toast(&mut self, t: impl Into<String>) {
        let t = t.into();
        if self.toasts.iter().any(|(x, _)| *x == t) { return; }
        self.toasts.push_back((t, 2.8));
    }
    fn log(&mut self, t: impl Into<String>) { let d = today(); self.s.add_log(d, t); }

    fn week_check(&mut self) {
        if let Some(r) = game::week_roll(&mut self.s, today()) {
            let name = LEAGUES[self.s.league as usize];
            let msg = match r { game::LeagueResult::Up => format!("Promoted to the {} league!", name), game::LeagueResult::Down => format!("Dropped to the {} league", name), game::LeagueResult::Stay => format!("Still in the {} league", name) };
            self.log(msg.clone());
            self.toast(msg);
            self.s.notif_read = false;
            self.award();
            self.persist();
        }
    }
    fn courses_cleared(&self) -> usize { (0..self.courses.len()).filter(|c| self.progress(*c) >= 1.0).count() }
    /// Grant newly earned achievements, with a toast and log entry for each.
    fn award(&mut self) {
        for c in 0..self.courses.len() {
            let id = self.courses[c].badge_id();
            if self.progress(c) >= 1.0 && !self.s.ach.contains_key(&id) {
                self.s.ach.insert(id, today());
                let name = self.courses[c].badge.name;
                self.toast(format!("Badge earned: {}!", name));
                self.log(format!("earned the {} badge for {}", name, self.courses[c].name));
                self.s.notif_read = false;
            }
        }
        let ctx = game::Ctx { courses_cleared: self.courses_cleared() };
        for a in game::grant_achievements(&mut self.s, &ctx, today()) {
            self.toast(format!("Achievement: {}", a.title));
            self.log(format!("earned the '{}' achievement", a.title));
            self.s.notif_read = false;
        }
    }

    fn level(&self) -> u32 { game::level(self.s.xp) }
    fn lkey(&self, c: usize, m: usize, l: usize) -> String { self.courses[c].lesson_id(m, l) }
    fn is_done(&self, c: usize, m: usize, l: usize) -> bool { self.s.done.contains(&self.lkey(c, m, l)) }
    fn module_done(&self, c: usize, m: usize) -> bool { (0..self.courses[c].modules[m].lessons.len()).all(|l| self.is_done(c, m, l)) }
    fn current_module(&self, c: usize) -> Option<usize> { (0..self.courses[c].modules.len()).find(|m| !self.module_done(c, *m)) }
    fn lesson_unlocked(&self, c: usize, m: usize, l: usize) -> bool {
        (0..m).all(|mm| self.module_done(c, mm)) && (0..l).all(|ll| self.is_done(c, m, ll))
    }
    fn next_lesson(&self, c: usize) -> Option<(usize, usize)> {
        for mi in 0..self.courses[c].modules.len() { for li in 0..self.courses[c].modules[mi].lessons.len() { if !self.is_done(c, mi, li) { return Some((mi, li)); } } }
        None
    }
    fn progress(&self, c: usize) -> f32 {
        let total = self.courses[c].lesson_count();
        let done = (0..self.courses[c].modules.len()).map(|m| (0..self.courses[c].modules[m].lessons.len()).filter(|l| self.is_done(c, m, *l)).count()).sum::<usize>();
        done as f32 / total.max(1) as f32
    }
    fn select_course(&mut self, c: usize) {
        self.cur = c; self.s.course = c as u32; self.open_mod = self.current_module(c).or(Some(0)); self.tab = Tab::Learn; self.persist();
    }
    fn tick(&mut self) {
        let before = self.s.hearts;
        game::tick_hearts(&mut self.s, now());
        if self.s.hearts != before { self.persist(); }
    }

    fn open_lesson(&mut self, c: usize, m: usize, l: usize) {
        let ls = self.courses[c].modules[m].lessons[l].clone();
        self.start_run(ls.title, Kind::Lesson(c, m, l), ls.slides, ls.qs);
    }

    fn start_run(&mut self, title: &str, kind: Kind, slides: Vec<(&'static str, &'static str)>, qs: Vec<Q>) {
        self.tick();
        if self.s.hearts == 0 && !matches!(kind, Kind::Duel(_)) { self.modal = Modal::NoHearts; return; }
        let n = qs.len();
        let (mut stage, mut queue, mut right) = (0usize, (0..n).collect::<Vec<usize>>(), 0u32);
        if let Kind::Lesson(c, mi, l) = &kind {
            let k = self.lkey(*c, *mi, *l);
            // An unfinished lesson resumes at the first question not yet answered correctly.
            if !self.s.done.contains(&k)
                && let Some(d) = self.s.qdone.get(&k)
                    && !d.is_empty() { queue.retain(|i| !d.contains(i)); right = d.len() as u32; stage = slides.len(); }
        }
        let lang = match &kind { Kind::Lesson(c, _, _) => self.courses[*c].lang, _ => "" };
        let project = matches!(kind, Kind::Lesson(c, m, l) if self.courses[c].modules[m].lessons[l].project);
        self.screen = Screen::Run(Box::new(RunState { title: title.into(), lang, kind, slides, qs, stage, queue, sel: None, text: String::new(), fb: None, mistakes: 0, right, total: n, last: None, hint_q: None, project }));
    }

    fn finish(&mut self, rs: &RunState) {
        self.week_check();
        let (mut xp, gems);
        let (mut duel, mut next) = (None, None);
        let perfect = rs.mistakes == 0;
        match rs.kind {
            Kind::Lesson(c, m, l) => {
                let first = !self.is_done(c, m, l);
                self.s.qdone.remove(&self.lkey(c, m, l));
                (xp, gems) = game::lesson_reward(first, perfect);
                if first && self.courses[c].modules[m].lessons[l].project { xp += game::PROJECT_BONUS; }
                if first {
                    self.s.done.insert(self.lkey(c, m, l));
                    self.log(format!("cleared '{}' ({})", rs.title, self.courses[c].name));
                    if perfect { self.s.flawless += 1; }
                    if self.progress(c) >= 1.0 { self.log(format!("finished the {} course!", self.courses[c].name)); }
                }
                self.open_mod = self.current_module(c).or(Some(m));
                next = self.next_lesson(c).map(|(m, l)| (c, m, l));
            }
            Kind::Coach => { (xp, gems) = (8, 2); self.log(format!("solved the '{}' challenge", rs.title)); }
            Kind::Duel(o) => {
                let me = rs.right;
                let bot = opponents()[o].skill.min(rs.total as u32);
                let won = me > bot || (me == bot && me as usize == rs.total);
                if won { (xp, gems) = (20, 5); self.s.duel_wins += 1; } else { (xp, gems) = (5, 0); }
                duel = Some((me, bot, opponents()[o].name.to_string(), won));
                self.log(format!("{} a duel against {}", if won { "won" } else { "lost" }, opponents()[o].name));
            }
        }
        let score = game::score(rs.total as u32, rs.mistakes);
        let mut best = score;
        if let Kind::Lesson(c, mi, l) = rs.kind {
            let e = self.s.best.entry(self.lkey(c, mi, l)).or_insert((0, 0));
            e.0 = e.0.max(score); e.1 += 1; best = e.0;
        }
        let new_streak = game::bump_streak(&mut self.s, today());
        game::add_xp(&mut self.s, xp, gems);
        self.award();
        self.persist();
        self.screen = Screen::Complete(Complete { title: rs.title.clone(), xp, gems, perfect, score, best, duel, new_streak, next, project: rs.project });
    }
}

// ---------------- shared ui helpers ----------------
fn chip(ui: &mut egui::Ui, art: &str, dim: bool, text: &str) -> egui::Response {
    let (r, resp) = ui.allocate_exact_size(Vec2::new(44.0 + text.len() as f32 * 12.0, 34.0), Sense::click());
    let p = ui.painter();
    p.rect_filled(r.expand(2.0), 0.0, EDGE);
    p.rect_filled(r, 0.0, if resp.hovered() { PANEL_HI } else { Color32::from_rgb(0x10, 0x0c, 0x1c) });
    draw_icon(p, Rect::from_min_size(Pos2::new(r.min.x + 4.0, r.min.y + 2.0), Vec2::splat(30.0)), art, if dim { Color32::from_gray(80) } else { Color32::WHITE });
    p.text(Pos2::new(r.min.x + 38.0, r.center().y + 1.0), Align2::LEFT_CENTER, text, FontId::new(11.0, press()), INK);
    resp
}

fn stats_row(ui: &mut egui::Ui, app: &mut App, show_hearts: bool) {
    let streak = game::live_streak(&app.s, today());
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 8.0;
        if show_hearts {
            let r = chip(ui, "heart", app.s.hearts == 0, &app.s.hearts.to_string());
            let tip = match game::next_heart_in(&app.s, now()) { Some(sec) => format!("Next heart in {}:{:02}", sec / 60, sec % 60), None => "Hearts are full".into() };
            if r.on_hover_text(tip).clicked() { app.modal = Modal::Shop; }
        }
        let st = chip(ui, "flame", streak == 0, &streak.to_string()).on_hover_text("Day streak");
        if st.clicked() { app.tab = Tab::Hero; }
        if chip(ui, "crystal_purple", false, &app.s.gems.to_string()).on_hover_text("Gems").clicked() { app.modal = Modal::Shop; }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.available_width() > 70.0 && pbtn(ui, "SHOP", GOLD, 80.0).clicked() { app.modal = Modal::Shop; }
        });
    });
}

fn draw_avatar(p: &egui::Painter, r: Rect, idx: u32) {
    let a = avatar(idx);
    bevel(p, r, a.bg, false);
    draw_icon(p, r.shrink(r.width() * 0.14), a.art, Color32::WHITE);
}

fn header(ctx: &mut egui::Ui, app: &mut App, title_text: &str, back: bool, dropdown: bool) {
    egui::Panel::top("hdr").frame(egui::Frame::new().fill(HDR).inner_margin(egui::Margin { left: 12, right: 12, top: 12, bottom: 12 })).show(ctx, |ui| {
        let (r, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 30.0), Sense::hover());
        let rr = ui.max_rect();
        ui.painter().rect_filled(Rect::from_min_size(Pos2::new(rr.min.x - 12.0, rr.max.y + 8.0), Vec2::new(rr.width() + 24.0, 4.0)), 0.0, GOLD);
        let mut x = r.min.x;
        if back {
            let br = Rect::from_min_size(Pos2::new(x, r.min.y), Vec2::new(30.0, 30.0));
            let resp = ui.interact(br, ui.scope_id().with("back"), Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand);
            ui.painter().text(br.center(), Align2::CENTER_CENTER, "<", FontId::new(16.0, press()), if resp.hovered() { GOLD } else { Color32::WHITE });
            if resp.clicked() { app.screen = Screen::Main; }
            x += 36.0;
        }
        let galley = ui.painter().layout_no_wrap(title_text.to_string(), FontId::new(12.0, press()), Color32::WHITE);
        let tw = galley.size().x.min(r.width() - 130.0);
        let th = galley.size().y;
        let tr = Rect::from_min_size(Pos2::new(x, r.min.y), Vec2::new(tw + if dropdown { 28.0 } else { 0.0 }, 30.0));
        ui.painter().with_clip_rect(Rect::from_min_size(Pos2::new(x, r.min.y), Vec2::new(tw, 30.0))).galley(Pos2::new(x, r.center().y - th / 2.0), galley, Color32::WHITE);
        if dropdown {
            let c = Pos2::new(x + tw + 14.0, r.center().y);
            for i in 0..4 { ui.painter().rect_filled(Rect::from_min_size(c + Vec2::new(-8.0 + i as f32 * 2.0, -3.0 + i as f32 * 2.0), Vec2::new(16.0 - i as f32 * 4.0, 2.0)), 0.0, Color32::WHITE); }
            if ui.interact(tr, ui.scope_id().with("drop"), Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_text("Choose a course").clicked() { app.modal = Modal::Courses; }
        }
        let mr = Rect::from_min_size(Pos2::new(r.max.x - 24.0, r.min.y), Vec2::new(24.0, 30.0));
        let mresp = ui.interact(mr, ui.scope_id().with("menu"), Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_text("Settings");
        for i in 0..3 { ui.painter().rect_filled(Rect::from_min_size(Pos2::new(mr.center().x - 2.0, mr.min.y + 5.0 + i as f32 * 8.0), Vec2::splat(5.0)), 0.0, if mresp.hovered() { GOLD } else { Color32::WHITE }); }
        if mresp.clicked() { app.screen = Screen::Settings; }
        let br = Rect::from_min_size(Pos2::new(r.max.x - 72.0, r.min.y), Vec2::new(36.0, 30.0));
        draw_icon(ui.painter(), br, "letter", Color32::WHITE);
        if !app.s.notif_read {
            ui.painter().rect_filled(Rect::from_min_size(Pos2::new(br.max.x - 8.0, br.min.y - 3.0), Vec2::splat(11.0)), 0.0, RED);
        }
        if ui.interact(br, ui.scope_id().with("bell"), Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_text("Messages").clicked() { app.modal = Modal::Notif; app.s.notif_read = true; app.persist(); }
    });
}

fn subtabs(ui: &mut egui::Ui, labels: &[&str], cur: &mut usize) {
    let (r, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 38.0), Sense::hover());
    ui.painter().rect_filled(r, 0.0, HDR);
    let n = labels.len() as f32;
    for (i, lab) in labels.iter().enumerate() {
        let tr = Rect::from_min_size(Pos2::new(r.min.x + r.width() / n * i as f32, r.min.y), Vec2::new(r.width() / n, 38.0));
        let resp = ui.interact(tr, ui.scope_id().with(("st", labels[0], i)), Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand);
        let on = *cur == i;
        ui.painter().text(tr.center() - Vec2::new(0.0, 2.0), Align2::CENTER_CENTER, *lab, FontId::new(9.0, press()), if on { Color32::WHITE } else if resp.hovered() { INK } else { DIM });
        if on { ui.painter().rect_filled(Rect::from_min_size(Pos2::new(tr.min.x, tr.max.y - 4.0), Vec2::new(tr.width(), 4.0)), 0.0, GOLD); }
        if resp.clicked() { *cur = i; }
    }
    ui.add_space(SP_S);
}

fn nav(ctx: &mut egui::Ui, app: &mut App) {
    egui::Panel::bottom("nav").frame(egui::Frame::new().fill(Color32::from_rgb(0x1d, 0x16, 0x32)).inner_margin(egui::Margin { left: 4, right: 4, top: 6, bottom: 6 })).show(ctx, |ui| {
        let (r, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 56.0), Sense::hover());
        let rr = ui.max_rect();
        ui.painter().rect_filled(Rect::from_min_size(Pos2::new(rr.min.x - 4.0, rr.min.y - 6.0), Vec2::new(rr.width() + 8.0, 3.0)), 0.0, GOLD);
        let items = [(Tab::Learn, "LEARN", "book"), (Tab::Train, "TRAIN", "sword"), (Tab::Ranks, "RANKS", "banner_royal"), (Tab::Create, "CREATE", "staff"), (Tab::Hero, "HERO", "helmet")];
        let w = r.width() / 5.0;
        for (i, (t, name, art)) in items.iter().enumerate() {
            let tr = Rect::from_min_size(Pos2::new(r.min.x + w * i as f32, r.min.y), Vec2::new(w, 56.0));
            let resp = ui.interact(tr, ui.scope_id().with(("nav", i)), Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand);
            let on = app.tab == *t;
            if on {
                ui.painter().rect_filled(tr.shrink2(Vec2::new(3.0, 0.0)).translate(Vec2::new(0.0, 3.0)), 0.0, EDGE);
                ui.painter().rect_filled(tr.shrink2(Vec2::new(3.0, 0.0)), 0.0, Color32::from_rgb(0x3a, 0x2e, 0x6b));
                ui.painter().rect_stroke(tr.shrink2(Vec2::new(3.0, 0.0)), 0.0, Stroke::new(2.0, GOLD), egui::StrokeKind::Inside);
            } else if resp.hovered() {
                ui.painter().rect_filled(tr.shrink2(Vec2::new(3.0, 0.0)), 0.0, Color32::from_rgb(0x2a, 0x22, 0x48));
            }
            draw_icon(ui.painter(), Rect::from_center_size(Pos2::new(tr.center().x, tr.min.y + 20.0), Vec2::splat(30.0)), art, if on || resp.hovered() { Color32::WHITE } else { Color32::from_gray(150) });
            ui.painter().text(Pos2::new(tr.center().x, tr.max.y - 9.0), Align2::CENTER_CENTER, *name, FontId::new(8.0, press()), if on { GOLD } else { DIM });
            if resp.clicked() { app.tab = *t; }
        }
    });
}

fn c32(c: [u8; 3]) -> Color32 { Color32::from_rgb(c[0], c[1], c[2]) }

fn out_view(ui: &mut egui::Ui, out: &[(interp::Kind, String)]) {
    use interp::Kind::*;
    for (i, (k, t)) in out.iter().enumerate() {
        match k {
            Out => { ui.label(RichText::new(format!("> {}", t)).monospace().size(14.0).color(GREEN)); }
            Err => { ui.label(RichText::new(format!("! {}", t)).monospace().size(13.0).color(RED)); }
            H1 => { ui.label(title(t.clone(), 15.0).color(PARCH)); ui.add_space(SP_XS); }
            H2 => { ui.label(title(t.clone(), 11.0).color(PARCH)); ui.add_space(3.0); }
            P => { ui.label(body(t.clone(), 15.0)); }
            Li => { ui.label(body(format!("  * {}", t), 15.0)); }
            Btn => { let _ = pbtn(ui, t, BLUE, 180.0); }
            Page => {
                ui.horizontal_wrapped(|ui| {
                    let (r, _) = ui.allocate_exact_size(Vec2::new(46.0, 18.0), Sense::hover());
                    ui.painter().rect_filled(r, 0.0, TEAL);
                    ui.painter().text(r.center(), Align2::CENTER_CENTER, "PAGE", FontId::new(8.0, press()), EDGE);
                    ui.label(RichText::new(t).monospace().size(14.0).color(TEAL));
                });
            }
            Img => {
                ui.horizontal(|ui| { icon(ui, "map", 34.0); ui.label(body(format!("[picture: {}]", t), 14.0).color(DIM)); });
            }
            Table => {
                let rows: Vec<Vec<&str>> = t.lines().map(|l| l.split('\u{1f}').collect()).collect();
                egui::ScrollArea::horizontal().id_salt(("tbl", i)).show(ui, |ui| {
                    egui::Grid::new(("grid", i)).striped(true).spacing([18.0, 4.0]).show(ui, |ui| {
                        for (ri, r) in rows.iter().enumerate() {
                            for c in r { ui.label(RichText::new(*c).monospace().size(14.0).color(if ri == 0 { GOLD } else { PARCH })); }
                            ui.end_row();
                        }
                    });
                });
            }
            Styled => {
                let (st, text) = css::Style::decode(t);
                let size = st.size.unwrap_or(15.0);
                let pad = st.pad.unwrap_or(0.0);
                let (r, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), size + 18.0 + pad * 2.0), Sense::hover());
                ui.painter().rect_filled(r, 0.0, Color32::WHITE);
                let inner = r.shrink(6.0);
                ui.painter().rect_filled(inner, 0.0, st.bg.map(c32).unwrap_or(Color32::WHITE));
                if let Some((w, c)) = st.border { ui.painter().rect_stroke(inner, 0.0, Stroke::new(w, c32(c)), egui::StrokeKind::Inside); }
                let tr = inner.shrink(pad + 4.0);
                let (pos, al) = match st.align.as_deref() { Some("center") => (tr.center(), Align2::CENTER_CENTER), Some("right") => (Pos2::new(tr.max.x, tr.center().y), Align2::RIGHT_CENTER), _ => (Pos2::new(tr.min.x, tr.center().y), Align2::LEFT_CENTER) };
                let col = st.color.map(c32).unwrap_or(Color32::BLACK);
                let p = ui.painter().with_clip_rect(inner);
                let font = FontId::new(size, if st.italic { egui::FontFamily::Monospace } else { egui::FontFamily::Proportional });
                let rect = p.text(pos, al, &text, font.clone(), col);
                if st.bold { p.text(pos + Vec2::new(1.0, 0.0), al, &text, font, col); }
                if st.underline { p.rect_filled(Rect::from_min_size(Pos2::new(rect.min.x, rect.max.y - 1.0), Vec2::new(rect.width(), 2.0)), 0.0, col); }
                ui.painter().rect_stroke(r, 0.0, Stroke::new(2.0, EDGE), egui::StrokeKind::Inside);
                ui.add_space(SP_XS);
            }
        }
    }
}

fn dim_back(ctx: &egui::Context) {
    let r = ctx.content_rect();
    egui::Area::new(egui::Id::unique("dimmer")).order(egui::Order::Middle).fixed_pos(r.min).show(ctx, |ui| {
        ui.painter().rect_filled(r, 0.0, Color32::from_black_alpha(190));
        let _ = ui.allocate_rect(r, Sense::click());
    });
}

fn dialog(ctx: &egui::Context, id: &str, add: impl FnOnce(&mut egui::Ui)) {
    let w = (ctx.content_rect().width() - 40.0).clamp(260.0, 360.0);
    egui::Window::new(id).title_bar(false).collapsible(false).resizable(false).anchor(Align2::CENTER_CENTER, [0.0, 0.0]).order(egui::Order::Foreground)
        .min_width(w).max_width(w)
        .frame(egui::Frame::new().fill(PANEL).stroke(Stroke::new(4.0, GOLD)).inner_margin(16.0).shadow(egui::epaint::Shadow { offset: [0, 6], blur: 0, spread: 0, color: EDGE }))
        .show(ctx, |ui| {
            ui.set_width(w);
            egui::ScrollArea::vertical().auto_shrink([false, true]).max_height((ctx.content_rect().height() - 90.0).max(200.0)).show(ui, add);
        });
}

fn option_btn(ui: &mut egui::Ui, text: &str, selected: bool, locked: bool, sel: &mut Option<usize>, i: usize) {
    let (r, resp) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 48.0), Sense::click());
    let fill = if selected { Color32::from_rgb(0x3a, 0x5a, 0xb8) } else if resp.hovered() && !locked { PANEL_HI } else { PANEL };
    bevel(ui.painter(), r, fill, true);
    if selected { ui.painter().rect_stroke(r.expand(2.0), 0.0, Stroke::new(3.0, GOLD), egui::StrokeKind::Outside); }
    ui.painter().text(Pos2::new(r.min.x + 12.0, r.center().y), Align2::LEFT_CENTER, format!("{}", i + 1), FontId::new(9.0, press()), DIM);
    ui.painter().with_clip_rect(r).text(Pos2::new(r.min.x + 32.0, r.center().y), Align2::LEFT_CENTER, text, FontId::new(16.0, egui::FontFamily::Proportional), INK);
    if resp.clicked() && !locked { *sel = Some(i); }
    ui.add_space(SP_S);
}

fn toggle(ui: &mut egui::Ui, name: &str, sub: &str, v: &mut bool) -> bool {
    let mut ch = false;
    ui.horizontal(|ui| {
        ui.vertical(|ui| { ui.label(body(name, 15.0)); ui.label(body(sub, 12.0).color(DIM)); });
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let (r, resp) = ui.allocate_exact_size(Vec2::new(60.0, 28.0), Sense::click());
            bevel(ui.painter(), r, if *v { GREEN } else { Color32::from_rgb(0x4a, 0x44, 0x66) }, false);
            let kx = if *v { r.max.x - 22.0 } else { r.min.x + 4.0 };
            ui.painter().rect_filled(Rect::from_min_size(Pos2::new(kx, r.min.y + 4.0), Vec2::new(18.0, 20.0)), 0.0, Color32::WHITE);
            if resp.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() { *v = !*v; ch = true; }
        });
    });
    ui.add_space(SP_S);
    ch
}

// ---------------- app glue ----------------
impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _f: &mut eframe::Frame) { self.frame_ui(ui); }
}

impl App {
    /// One frame of the whole app. Separate from eframe so tests can drive it headlessly.
    fn frame_ui(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();
        self.tick();
        if self.s.hearts < MAX_HEARTS { ctx.request_repaint_after(std::time::Duration::from_secs(1)); }
        if self.onboard.is_some() {
            self.onboarding_screen(ui);
            self.draw_toast(&ctx);
            self.screenshot(&ctx);
            return;
        }
        let tnow = ctx.input(|i| i.time);
        let vkey = (self.tab as u32) * 10 + self.sub_train as u32 + self.sub_create as u32 * 3;
        if vkey != self.view_key { self.view_key = vkey; self.view_t0 = tnow; }
        let fade_t = ((tnow - self.view_t0) / 0.18).clamp(0.0, 1.0);
        let alpha = (1.0 - (1.0 - fade_t).powi(3)) as f32;
        if fade_t < 1.0 { ctx.request_repaint(); }
        let screen = std::mem::replace(&mut self.screen, Screen::Main);
        match screen {
            Screen::Main => {
                let (t, dd) = match self.tab { Tab::Learn => (self.courses[self.cur].name.to_string(), true), Tab::Train => ("Training".into(), false), Tab::Ranks => ("Guild Ranks".into(), false), Tab::Create => ("Spell Forge".into(), false), Tab::Hero => (self.s.name.clone(), false) };
                header(ui, self, &t, false, dd);
                nav(ui, self);
                let tab = self.tab;
                egui::CentralPanel::default().frame(egui::Frame::new().fill(BG).inner_margin(egui::Margin { left: SCREEN_PAD, right: SCREEN_PAD, top: SCREEN_PAD, bottom: 0 })).show(ui, |ui| {
                    if self.s.floor { floor(ui.painter(), ui.max_rect(), self.s.scan); }
                    ui.set_opacity(alpha);
                    egui::ScrollArea::vertical().id_salt(("main", tab as u32)).auto_shrink([false, false]).show(ui, |ui| match tab {
                        Tab::Learn => self.learn(ui),
                        Tab::Train => self.train(ui),
                        Tab::Ranks => self.ranks(ui),
                        Tab::Create => self.create(ui),
                        Tab::Hero => self.hero(ui),
                    });
                });
            }
            Screen::Settings => { self.screen = Screen::Settings; self.settings(ui); }
            Screen::Run(rs) => { if let Some(rs) = self.run_screen(ui, rs) { self.screen = Screen::Run(rs); } }
            Screen::Complete(c) => { if !self.complete_screen(ui, &c) && matches!(self.screen, Screen::Main) { self.screen = Screen::Complete(c); } }
            Screen::Editor(ed) => { if let Some(ed) = self.editor_screen(ui, ed) { self.screen = Screen::Editor(ed); } }
            Screen::Preview(c) => { self.screen = Screen::Preview(c); self.preview_screen(ui, c); }
        }
        self.modals(&ctx);
        self.draw_toast(&ctx);
        self.screenshot(&ctx);
    }

    fn modals(&mut self, ctx: &egui::Context) {
        if self.modal == Modal::None { return; }
        dim_back(ctx);
        let mut close = ctx.input(|i| i.key_pressed(egui::Key::Escape));
        match self.modal {
            Modal::Notif => dialog(ctx, "notif", |ui| {
                ui.horizontal(|ui| { icon(ui, "letter", 36.0); ui.label(title("MESSAGES", 11.0).color(GOLD)); });
                ui.add_space(SP_S);
                let day = today();
                let streak = game::live_streak(&self.s, day);
                let msg_streak = if self.s.last_day == day { format!("Today's lesson is done. Streak: {} day{}.", streak, if streak == 1 { "" } else { "s" }) }
                    else if streak > 0 { format!("Clear a lesson today to keep your {} day streak.", streak) }
                    else { "Clear a lesson today to start a streak.".into() };
                ui.horizontal_wrapped(|ui| { icon(ui, "flame", 28.0); ui.label(body(msg_streak, 14.0)); });
                let hearts = match game::next_heart_in(&self.s, now()) { Some(s) => format!("{} of {} hearts. Next heart in {}:{:02}.", self.s.hearts, MAX_HEARTS, s / 60, s % 60), None => "Your hearts are full.".into() };
                ui.horizontal_wrapped(|ui| { icon(ui, "heart", 28.0); ui.label(body(hearts, 14.0)); });
                if let Some((id, d)) = self.s.ach.iter().max_by_key(|(_, d)| **d)
                    && let Some(a) = game::achievement(id) { ui.horizontal_wrapped(|ui| { icon(ui, a.icon, 28.0); ui.label(body(format!("Latest achievement: {} ({})", a.title, day_label(*d)), 14.0)); }); }
                if !self.s.last_result.is_empty() {
                    let lr = match self.s.last_result.as_str() { "LEVEL_UP" => "Last week: promoted!", "LEVEL_DOWN" => "Last week: dropped a league.", _ => "Last week: league unchanged." };
                    ui.horizontal_wrapped(|ui| { icon(ui, "banner_royal", 28.0); ui.label(body(lr, 14.0)); });
                }
                ui.add_space(SP_M);
                if pbtn(ui, "CLOSE", BLUE, 0.0).clicked() { close = true; }
            }),
            Modal::Shop => dialog(ctx, "shop", |ui| {
                ui.horizontal(|ui| { icon(ui, "money_bag", 36.0); ui.label(title("SHOP", 12.0).color(GOLD)); ui.add_space(SP_M); icon(ui, "crystal_purple", 28.0); ui.label(title(format!("{}", self.s.gems), 11.0)); });
                ui.add_space(SP_M);
                ui.horizontal_wrapped(|ui| { icon(ui, "heart", 34.0); ui.label(body(format!("Refill all hearts ({} gems)", REFILL_COST), 14.0)); });
                if pbtn(ui, "BUY HEARTS", RED, 0.0).clicked() { match game::buy_refill(&mut self.s) { Ok(()) => { self.persist(); self.toast("Hearts refilled!"); } Err(e) => self.toast(e.msg()) } }
                let claimed = self.s.free_day == today();
                if pbtn(ui, if claimed { "FREE HEART (TOMORROW)" } else { "FREE HEART (1 PER DAY)" }, if claimed { PANEL_HI } else { GREEN }, 0.0).clicked() {
                    match game::claim_free_heart(&mut self.s, today()) { Ok(()) => { self.persist(); self.toast("+1 heart!"); } Err(e) => self.toast(e.msg()) }
                }
                ui.add_space(SP_M);
                ui.horizontal_wrapped(|ui| { icon(ui, "hourglass", 34.0); ui.label(body(format!("Streak saver ({} gems, hold up to 3). Protects your streak if you miss one day.", SAVER_COST), 14.0)); });
                if pbtn(ui, &format!("BUY SAVER ({}/3)", self.s.savers), BLUE, 0.0).clicked() { match game::buy_saver(&mut self.s) { Ok(()) => { self.persist(); self.toast("Streak saver added!"); } Err(e) => self.toast(e.msg()) } }
                ui.add_space(SP_M);
                ui.label(body("Earn gems by clearing lessons, challenges and duels.", 12.0).color(DIM));
                if pbtn(ui, "CLOSE", PANEL_HI, 0.0).clicked() { close = true; }
            }),
            Modal::Courses => dialog(ctx, "courses", |ui| {
                ui.label(title("CHOOSE A COURSE", 11.0).color(GOLD)); ui.add_space(SP_S);
                for ci in 0..self.courses.len() {
                    ui.horizontal(|ui| {
                        emblem(ui, self.courses[ci].tag, self.courses[ci].color, 44.0);
                        let label = format!("{}  {}%", self.courses[ci].name, (self.progress(ci) * 100.0) as u32);
                        let w = (ui.available_width() - 52.0).max(80.0);
                        if pbtn(ui, &label, if ci == self.cur { GOLD } else { PANEL_HI }, w).clicked() { self.select_course(ci); close = true; }
                        if pbtn(ui, "i", BLUE, 44.0).on_hover_text("Course details").clicked() { self.screen = Screen::Preview(ci); close = true; }
                    });
                    ui.add_space(SP_XS);
                }
                if pbtn(ui, "CLOSE", PANEL_HI, 0.0).clicked() { close = true; }
            }),
            Modal::NoHearts => dialog(ctx, "nohearts", |ui| {
                ui.vertical_centered(|ui| {
                    let (r, _) = ui.allocate_exact_size(Vec2::splat(96.0), Sense::hover());
                    draw_icon(ui.painter(), r, "heart", Color32::from_gray(70));
                    ui.label(title("OUT OF HEARTS", 12.0).color(RED)); ui.add_space(SP_S);
                    let wait = game::next_heart_in(&self.s, now()).map(|s| format!("Next heart in {}:{:02}. ", s / 60, s % 60)).unwrap_or_default();
                    ui.label(body(format!("{}Hearts refill 1 every 5 minutes. Your lesson progress is saved.", wait), 14.0)); ui.add_space(SP_S);
                });
                if pbtn(ui, &format!("REFILL ({} GEMS)", REFILL_COST), RED, 0.0).clicked() {
                    match game::buy_refill(&mut self.s) { Ok(()) => { self.persist(); close = true; } Err(e) => self.toast(e.msg()) }
                }
                if self.s.free_day != today() && pbtn(ui, "FREE DAILY HEART", GREEN, 0.0).clicked()
                    && game::claim_free_heart(&mut self.s, today()).is_ok() { self.persist(); close = true; }
                ui.add_space(SP_S);
                if pbtn(ui, "PRACTISE A DUEL (NO HEARTS)", PANEL_HI, 0.0).clicked() { self.tab = Tab::Train; self.sub_train = 1; self.screen = Screen::Main; close = true; }
                if pbtn(ui, "LATER", PANEL_HI, 0.0).clicked() { close = true; }
            }),
            Modal::Reset => dialog(ctx, "reset", |ui| {
                ui.label(title("RESET PROGRESS?", 11.0).color(RED)); ui.add_space(SP_S);
                ui.label(body("This erases XP, gems, streak, achievements, saved spells and finished lessons. It cannot be undone.", 14.0)); ui.add_space(SP_S);
                if pbtn(ui, "YES, RESET", RED, 0.0).clicked() {
                    let (n, a) = (self.s.name.clone(), self.s.avatar);
                    self.s = Save::new(); self.s.heart_t = now(); self.s.name = n; self.s.avatar = a; self.s.onboarded = true;
                    game::week_roll(&mut self.s, today());
                    self.persist(); self.cur = 0; self.open_mod = self.current_module(0); self.screen = Screen::Main; self.toast("Progress reset");
                    close = true;
                }
                ui.add_space(SP_S);
                if pbtn(ui, "CANCEL", PANEL_HI, 0.0).clicked() { close = true; }
            }),
            Modal::Avatar => dialog(ctx, "avatar", |ui| {
                ui.label(title("CHOOSE YOUR HERO", 11.0).color(GOLD)); ui.add_space(SP_S);
                if avatar_grid(ui, &mut self.s.avatar) { self.persist(); close = true; }
                ui.add_space(SP_S);
                if pbtn(ui, "CLOSE", PANEL_HI, 0.0).clicked() { close = true; }
            }),
            Modal::DeleteSpell(i) => dialog(ctx, "delspell", |ui| {
                let name = self.s.codes.get(i).map(|c| c.0.clone()).unwrap_or_default();
                ui.label(title("DELETE SPELL?", 11.0).color(RED)); ui.add_space(SP_S);
                ui.label(body(format!("'{}' will be gone for good.", name), 14.0)); ui.add_space(SP_S);
                if pbtn(ui, "DELETE", RED, 0.0).clicked() {
                    if i < self.s.codes.len() { self.s.codes.remove(i); self.persist(); }
                    self.screen = Screen::Main; close = true;
                }
                if pbtn(ui, "KEEP IT", PANEL_HI, 0.0).clicked() { close = true; }
            }),
            Modal::None => {}
        }
        if close { self.modal = Modal::None; }
    }

    fn draw_toast(&mut self, ctx: &egui::Context) {
        let dt = ctx.input(|i| i.stable_dt).min(0.1) as f64;
        let Some((t, left)) = self.toasts.front_mut() else { return };
        *left -= dt;
        let t = t.clone();
        if *left <= 0.0 { self.toasts.pop_front(); }
        ctx.request_repaint();
        let r = ctx.content_rect();
        let w = (r.width() - 32.0).min(320.0);
        egui::Area::new(egui::Id::unique("toast")).order(egui::Order::Tooltip).fixed_pos(Pos2::new(r.center().x - w / 2.0, r.max.y - 140.0)).interactable(false).show(ctx, |ui| {
            let (rr, _) = ui.allocate_exact_size(Vec2::new(w, 40.0), Sense::hover());
            bevel(ui.painter(), rr, Color32::from_rgb(0x1f, 0x1a, 0x35), false);
            ui.painter().rect_stroke(rr, 0.0, Stroke::new(2.0, GOLD), egui::StrokeKind::Inside);
            ui.painter().with_clip_rect(rr).text(rr.center(), Align2::CENTER_CENTER, t, FontId::new(15.0, egui::FontFamily::Proportional), INK);
        });
    }

    fn screenshot(&mut self, ctx: &egui::Context) {
        let Some(path) = self.shot.clone() else { return };
        self.frames += 1;
        ctx.request_repaint();
        if self.frames == 120 {
            ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(egui::ScreenshotCallback::new(move |img| {
                let (w, h) = (img.width() as u32, img.height() as u32);
                let raw: Vec<u8> = img.pixels.iter().flat_map(|c| c.to_array()).collect();
                if let Some(b) = image::RgbaImage::from_raw(w, h, raw) { let _ = b.save(&path); }
                std::process::exit(0);
            })));
        }
    }
}

/// "today", "yesterday" or "N days ago".
fn day_label(d: u64) -> String {
    match today().saturating_sub(d) { 0 => "today".into(), 1 => "yesterday".into(), n => format!("{} days ago", n) }
}

/// Grid of avatar tiles. Returns true when one was clicked.
fn avatar_grid(ui: &mut egui::Ui, cur: &mut u32) -> bool {
    let mut picked = false;
    let cols = 4;
    let size = ((ui.available_width() - (cols as f32 - 1.0) * 10.0) / cols as f32).clamp(52.0, 76.0);
    for row in AVATARS.chunks(cols).enumerate() {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 10.0;
            for (j, a) in row.1.iter().enumerate() {
                let i = (row.0 * cols + j) as u32;
                ui.vertical(|ui| {
                    let (r, resp) = ui.allocate_exact_size(Vec2::splat(size), Sense::click());
                    draw_avatar(ui.painter(), r, i);
                    if *cur == i { ui.painter().rect_stroke(r.expand(3.0), 0.0, Stroke::new(3.0, GOLD), egui::StrokeKind::Outside); }
                    else if resp.hovered() { ui.painter().rect_stroke(r.expand(3.0), 0.0, Stroke::new(2.0, DIM), egui::StrokeKind::Outside); }
                    ui.label(body(a.name, 12.0).color(if *cur == i { GOLD } else { DIM }));
                    if resp.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() { *cur = i; picked = true; }
                });
            }
        });
        ui.add_space(SP_XS);
    }
    picked
}

fn main() -> eframe::Result {
    let args: Vec<String> = std::env::args().collect();
    let arg = |k: &str| args.iter().position(|a| a == k).and_then(|i| args.get(i + 1).cloned());
    if args.iter().any(|a| a == "--version") { println!("Pixel Learn {}", VERSION); return Ok(()); }
    // Release builds have no console: record any crash next to the save so it can be reported.
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = std::fs::create_dir_all(Save::dir());
        let _ = std::fs::write(Save::dir().join("crash.log"), format!("Pixel Learn {} crashed (day {}):
{}
", VERSION, today(), info));
        default_hook(info);
    }));
    let shot = arg("--screenshot");
    let tab = arg("--tab");
    let screen = arg("--screen");
    let lesson = arg("--lesson");
    let force_onboarding = args.iter().any(|a| a == "--onboarding");
    let num = |k: &str| arg(k).and_then(|v| v.parse::<usize>().ok()).unwrap_or(0);
    let (stage, skip) = (num("--stage"), num("--skip"));
    let mut vp = egui::ViewportBuilder::default().with_inner_size([440.0, 780.0]).with_min_inner_size([360.0, 600.0]).with_title(format!("Pixel Learn {}", VERSION)).with_app_id("pixel_learn");
    if let Ok(icon) = eframe::icon_data::from_png_bytes(include_bytes!("../assets/icon.png")) { vp = vp.with_icon(std::sync::Arc::new(icon)); }
    if let Some(sz) = arg("--size").and_then(|s| s.split_once('x').and_then(|(w, h)| Some((w.parse::<f32>().ok()?, h.parse::<f32>().ok()?)))) { vp = vp.with_inner_size([sz.0, sz.1]); }
    if shot.is_some() { vp = vp.with_active(false).with_window_level(egui::WindowLevel::AlwaysOnBottom); }
    eframe::run_native("Pixel Learn", eframe::NativeOptions { viewport: vp, ..Default::default() }, Box::new(move |cc| {
        install_fonts(&cc.egui_ctx);
        Box::leak(Box::new(load_textures(&cc.egui_ctx)));
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        cc.egui_ctx.global_style_mut(|s| {
            s.visuals.panel_fill = BG;
            s.visuals.window_fill = PANEL;
            s.visuals.override_text_color = Some(INK);
            s.visuals.extreme_bg_color = Color32::from_rgb(0x0f, 0x0b, 0x1a);
            s.visuals.selection.bg_fill = Color32::from_rgb(0x3a, 0x5a, 0xb8);
            s.visuals.widgets.inactive.bg_stroke = Stroke::new(2.0, EDGE);
            for w in [&mut s.visuals.widgets.noninteractive, &mut s.visuals.widgets.inactive, &mut s.visuals.widgets.hovered, &mut s.visuals.widgets.active, &mut s.visuals.widgets.open] { w.corner_radius = 0.0.into(); }
            s.visuals.widgets.inactive.weak_bg_fill = PANEL_HI;
            s.spacing.item_spacing = Vec2::new(8.0, 6.0);
            s.visuals.window_corner_radius = 0.0.into();
        });
        let mut app = App::new(shot);
        if force_onboarding { app.onboard = Some(onboarding::Onboard::default()); }
        if let Some(t) = tab { app.tab = match t.as_str() { "train" | "community" => Tab::Train, "ranks" | "leader" => Tab::Ranks, "create" => Tab::Create, "hero" | "profile" => Tab::Hero, _ => Tab::Learn }; }
        if let Some(sc) = screen {
            match sc.as_str() {
                "settings" => app.screen = Screen::Settings,
                "editor" => app.screen = Screen::Editor(Editor { idx: None, title: "Royal Banner".into(), lang: "css".into(), code: EXAMPLES[5].code.into(), out: interp::run("css", EXAMPLES[5].code) }),
                "sql" => app.screen = Screen::Editor(Editor { idx: None, title: "Strongest Heroes".into(), lang: "sql".into(), code: EXAMPLES[6].code.into(), out: interp::run("sql", EXAMPLES[6].code) }),
                "complete" => app.screen = Screen::Complete(Complete { title: "Project: Wanted Poster".into(), xp: 25, gems: 5, perfect: true, score: 100, best: 100, duel: None, new_streak: true, next: Some((0, 1, 0)), project: true }),
                "preview" => app.screen = Screen::Preview(1),
                "preview-js" => app.screen = Screen::Preview(2),
                "shop" => app.modal = Modal::Shop,
                "notif" => app.modal = Modal::Notif,
                "avatar" => app.modal = Modal::Avatar,
                "nohearts" => app.modal = Modal::NoHearts,
                _ => {}
            }
        }
        // QA helper: --lesson C.M.L [--stage N] [--skip N] opens a lesson directly.
        if let Some(spec) = lesson {
            let p: Vec<usize> = spec.split('.').filter_map(|x| x.parse().ok()).collect();
            if p.len() == 3 && p[0] < app.courses.len() && p[1] < app.courses[p[0]].modules.len() && p[2] < app.courses[p[0]].modules[p[1]].lessons.len() {
                app.open_lesson(p[0], p[1], p[2]);
                if let Screen::Run(rs) = &mut app.screen { rs.stage = stage.min(rs.slides.len()); for _ in 0..skip { if rs.queue.len() > 1 { rs.queue.remove(0); } } }
            }
        }
        Ok(Box::new(app))
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// End-to-end over the real App logic (no window): first launch -> onboarding -> clear a
    /// lesson -> restart -> everything persisted (US1 + US3).
    #[test]
    fn first_launch_to_cleared_lesson_persists() {
        let dir = std::env::temp_dir().join(format!("pixel_learn_e2e_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        // SAFETY: only this test reads or writes APPDATA/HOME in the test binary.
        unsafe { std::env::set_var("APPDATA", &dir); std::env::set_var("HOME", &dir); }

        let mut app = App::new(None);
        assert!(app.onboard.is_some(), "first launch shows the welcome flow");
        app.s.name = Save::clean_name("  Ada  ");
        app.s.avatar = 2;
        app.cur = 3; app.s.course = 3; // Python
        app.s.onboarded = true; app.onboard = None;
        app.persist();

        app.open_lesson(3, 0, 0);
        let Screen::Run(mut rs) = std::mem::replace(&mut app.screen, Screen::Main) else { panic!("lesson did not open") };
        rs.queue.clear(); rs.right = rs.total as u32; // answered everything right first time
        app.finish(&rs);
        assert!(matches!(app.screen, Screen::Complete(_)));

        let again = App::new(None);
        assert!(again.onboard.is_none(), "welcome flow is not shown twice");
        assert_eq!(again.s.name, "Ada");
        assert_eq!(again.s.avatar, 2);
        assert_eq!(again.cur, 3);
        assert!(again.s.done.contains("python/printing"));
        assert_eq!(again.s.xp, 15, "first clear + flawless bonus");
        assert_eq!(again.s.gems, 25);
        assert_eq!(again.s.streak, 1);
        assert!(again.s.ach.contains_key("first_lesson"));
        assert!(again.s.ach.contains_key("flawless"));
        assert!(again.s.log.iter().any(|(_, t)| t.contains("cleared")));
        assert!(dir.join("pixel_learn").join("save.bak").exists(), "previous save kept as backup");
        let _ = std::fs::remove_dir_all(&dir);
    }

    // ---------- headless UI driving ----------
    fn key(k: egui::Key, ctrl: bool) -> egui::Event {
        let m = if ctrl { egui::Modifiers { ctrl: true, command: true, ..Default::default() } } else { egui::Modifiers::NONE };
        egui::Event::Key { key: k, physical_key: None, pressed: true, repeat: false, modifiers: m }
    }
    fn ui_ctx() -> egui::Context {
        let ctx = egui::Context::default();
        install_fonts(&ctx);
        Box::leak(Box::new(load_textures(&ctx)));
        ctx
    }
    fn frame(ctx: &egui::Context, app: &mut App, events: Vec<egui::Event>) {
        let modifiers = events.iter().find_map(|e| if let egui::Event::Key { modifiers, .. } = e { Some(*modifiers) } else { None }).unwrap_or_default();
        let mut events = events;
        events.insert(0, egui::Event::ModifiersChanged(modifiers));
        let raw = egui::RawInput { screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(440.0, 780.0))), events, ..Default::default() };
        let mut out = ctx.run_ui(raw, |ui| app.frame_ui(ui));
        out.textures_delta.clear();
    }
    fn demo_app() -> App { let mut a = App::new(Some(String::new())); a.shot = None; a }

    /// Play a running lesson with the keyboard until it completes. `wrong_first` answers the
    /// first question wrongly once. Returns the number of frames used.
    fn play(ctx: &egui::Context, app: &mut App, wrong_first: bool) -> usize {
        let mut wrong_left = wrong_first;
        for f in 0..400 {
            let mut ev = vec![];
            match &mut app.screen {
                Screen::Run(rs) => {
                    if rs.stage < rs.slides.len() || rs.fb.is_some() { ev.push(key(egui::Key::Enter, false)); }
                    else if let Some(&qi) = rs.queue.first() {
                        let mut ctrl = false;
                        match &rs.qs[qi] {
                            Q::Mc { right, opts, .. } | Q::Fill { right, opts, .. } => rs.sel = Some(if wrong_left { (right + 1) % opts.len() } else { *right }),
                            Q::Type { answer, .. } => { rs.text = if wrong_left { "nope".into() } else { answer.to_string() }; ctrl = answer.contains('\n'); }
                        }
                        wrong_left = false;
                        ev.push(key(egui::Key::Enter, ctrl));
                    }
                }
                Screen::Complete(_) => return f,
                _ => panic!("left the lesson unexpectedly"),
            }
            frame(ctx, app, ev);
        }
        let st = match &app.screen { Screen::Run(rs) => format!("{} stage {}/{} queue {:?} fb {:?} sel {:?} text {:?}", rs.title, rs.stage, rs.slides.len(), rs.queue, rs.fb, rs.sel, rs.text), _ => "?".into() };
        panic!("lesson did not finish: {}", st);
    }

    /// Regression for the START THE QUIZ crash: click through every lesson of every course.
    #[test]
    fn every_lesson_can_be_played_through_the_ui() {
        let ctx = ui_ctx();
        let mut app = demo_app();
        app.s.hearts = MAX_HEARTS;
        for c in 0..app.courses.len() {
            for m in 0..app.courses[c].modules.len() {
                for l in 0..app.courses[c].modules[m].lessons.len() {
                    app.open_lesson(c, m, l);
                    frame(&ctx, &mut app, vec![]);
                    play(&ctx, &mut app, false);
                    assert!(app.s.done.contains(&app.lkey(c, m, l)), "{}.{}.{} not cleared", c, m, l);
                    app.screen = Screen::Main;
                    frame(&ctx, &mut app, vec![]);
                }
            }
        }
        for t in [Tab::Learn, Tab::Train, Tab::Ranks, Tab::Create, Tab::Hero] { app.tab = t; frame(&ctx, &mut app, vec![]); }
    }

    #[test]
    fn wrong_answer_costs_a_heart_and_comes_back() {
        let ctx = ui_ctx();
        let mut app = demo_app();
        app.s.hearts = MAX_HEARTS;
        app.open_lesson(2, 0, 0);
        frame(&ctx, &mut app, vec![]);
        play(&ctx, &mut app, true);
        assert_eq!(app.s.hearts, MAX_HEARTS - 1);
        let Screen::Complete(c) = &app.screen else { panic!() };
        assert!(!c.perfect);
        assert!(c.score < 100);
    }

    #[test]
    fn course_preview_starts_the_next_lesson() {
        let ctx = ui_ctx();
        let mut app = demo_app();
        app.screen = Screen::Preview(1);
        frame(&ctx, &mut app, vec![]);
        assert!(matches!(app.screen, Screen::Preview(1)), "preview stays open until an action");
        frame(&ctx, &mut app, vec![key(egui::Key::Enter, false)]);
        let Screen::Run(rs) = &app.screen else { panic!("START did not open a lesson") };
        assert_eq!(rs.title, app.courses[1].modules[0].lessons[0].title);
        assert_eq!(app.cur, 1, "previewed course becomes the active course");
        for c in 0..app.courses.len() { app.screen = Screen::Preview(c); frame(&ctx, &mut app, vec![]); }
    }

    #[test]
    fn finishing_a_course_earns_its_badge_once() {
        let mut app = demo_app();
        assert!(app.badge_earned(0).is_none());
        for m in 0..app.courses[0].modules.len() { for l in 0..app.courses[0].modules[m].lessons.len() { let k = app.lkey(0, m, l); app.s.done.insert(k); } }
        app.award();
        assert!(app.badge_earned(0).is_some());
        assert!(app.badge_earned(1).is_none());
        let n = app.s.log.len();
        app.award();
        assert_eq!(app.s.log.len(), n, "badge is granted only once");
    }

    #[test]
    fn project_lessons_give_bonus_xp() {
        let ctx = ui_ctx();
        let mut app = demo_app();
        let (m, l) = (0, app.courses[0].modules[0].lessons.len() - 1);
        assert!(app.courses[0].modules[m].lessons[l].project);
        let xp0 = app.s.xp;
        app.open_lesson(0, m, l);
        frame(&ctx, &mut app, vec![]);
        play(&ctx, &mut app, false);
        assert_eq!(app.s.xp - xp0, 15 + game::PROJECT_BONUS);
    }

    #[test]
    fn no_hearts_blocks_lessons_but_not_duels() {
        let mut app = demo_app();
        app.s.hearts = 0; app.s.heart_t = now();
        app.open_lesson(0, 0, 0);
        assert!(matches!(app.screen, Screen::Main));
        assert!(app.modal == Modal::NoHearts);
        app.modal = Modal::None;
        app.start_run("duel", Kind::Duel(0), vec![], duel_questions(1));
        assert!(matches!(app.screen, Screen::Run(_)));
    }
}
