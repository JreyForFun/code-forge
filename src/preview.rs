//! Course Preview screen, the Learn tab's quest-card header, and course badges.
use super::*;

/// Badge tiers: 0 = not started, 1 = bronze (first chapter), 2 = silver (half the course), 3 = gold (all).
pub const TIER_NAMES: [&str; 4] = ["Locked", "Bronze", "Silver", "Gold"];

/// A course badge from the GTORAVERSE pack: the course's shape (shield, star, crown...) in the
/// metal of the tier reached. Locked badges are a dark silhouette.
pub fn draw_badge(p: &egui::Painter, r: Rect, course: &Course, tier: u8) {
    let metal = ["bronze", "bronze", "silver", "gold"][tier.min(3) as usize];
    let name = format!("{}_{}", metal, course.badge.art);
    if tier >= 3 {
        // a soft glow behind a gold badge
        let t = p.ctx().input(|i| i.time) as f32;
        let a = (40.0 + 25.0 * (t * 2.0).sin()) as u8;
        p.circle_filled(r.center(), r.width() * 0.46, Color32::from_rgba_unmultiplied(0xfa, 0xcc, 0x15, a));
    }
    draw_icon(p, r, &name, if tier == 0 { Color32::from_rgb(0x30, 0x2c, 0x40) } else { Color32::WHITE });
}

pub fn badge(ui: &mut egui::Ui, course: &Course, tier: u8, size: f32) -> egui::Response {
    let (r, resp) = ui.allocate_exact_size(Vec2::splat(size), Sense::click());
    draw_badge(ui.painter(), r, course, tier);
    resp
}

impl App {
    pub fn badge_earned(&self, c: usize) -> Option<u64> { self.s.ach.get(&self.courses[c].badge_id()).copied() }

    /// How far the course badge has grown: bronze after the first chapter, silver at half, gold at the end.
    pub fn badge_tier(&self, c: usize) -> u8 {
        let prog = self.progress(c);
        if self.badge_earned(c).is_some() || prog >= 1.0 { 3 } else if prog >= 0.5 { 2 } else if self.module_done(c, 0) { 1 } else { 0 }
    }

    /// The Learn tab header: a code-drawn quest card (badge, course info, a self-typing code line, progress).
    pub fn quest_card(&mut self, ui: &mut egui::Ui, c: usize) {
        let prog = self.progress(c);
        let earned = self.badge_earned(c).is_some();
        let total = self.courses[c].lesson_count();
        let done = (prog * total as f32).round() as usize;
        let next = self.next_lesson(c);
        let col = self.courses[c].color;
        let resp = panel_resp(ui, PANEL, EDGE, |ui| {
            ui.set_width(ui.available_width());
            let top = ui.max_rect();
            ui.painter().rect_filled(Rect::from_min_size(top.min - Vec2::new(SP_M - 3.0, SP_M - 3.0), Vec2::new(top.width() + 2.0 * SP_M - 6.0, 5.0)), 0.0, col);
            ui.add_space(SP_XS);
            ui.horizontal(|ui| {
                badge(ui, &self.courses[c], self.badge_tier(c), 72.0).on_hover_text(format!("{} badge ({}): {}", self.courses[c].badge.name, TIER_NAMES[self.badge_tier(c) as usize], if earned { "earned!" } else { "bronze after chapter 1, silver at half, gold when you finish" }));
                ui.add_space(SP_XS);
                ui.vertical(|ui| {
                    let cs = &self.courses[c];
                    ui.label(title(format!("{}  *  {}  *  ~{}H", cs.level.to_uppercase(), cs.cat.to_uppercase(), cs.hours), 7.0).color(DIM));
                    ui.add_space(SP_XS);
                    ui.label(title(cs.name, 16.0));
                    ui.add_space(SP_XS);
                    ui.label(body(cs.desc, 13.0).color(DIM));
                });
            });
            ui.add_space(SP_S);
            // self-typing code line in the course language
            let snippet = self.courses[c].snippet;
            let t = ui.ctx().input(|i| i.time);
            let n = snippet.chars().count();
            let cycle = n + 24;
            let k = ((t * 14.0) as usize % cycle).min(n);
            let shown: String = snippet.chars().take(k).collect();
            let cursor = if (t * 2.0) as i64 % 2 == 0 { "_" } else { " " };
            let (r, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 30.0), Sense::hover());
            let p = ui.painter();
            p.rect_filled(r, 0.0, Color32::from_rgb(0x07, 0x0c, 0x1c));
            p.rect_stroke(r, 0.0, Stroke::new(2.0, PANEL_HI), egui::StrokeKind::Inside);
            p.text(Pos2::new(r.min.x + 10.0, r.center().y), Align2::LEFT_CENTER, ">", FontId::monospace(14.0), col);
            p.with_clip_rect(r.shrink(2.0)).text(Pos2::new(r.min.x + 26.0, r.center().y), Align2::LEFT_CENTER, format!("{}{}", shown, cursor), FontId::monospace(14.0), PARCH);
            ui.ctx().request_repaint_after(std::time::Duration::from_millis(70));
            ui.add_space(SP_S);
            ui.horizontal(|ui| {
                ui.allocate_ui(Vec2::new((ui.available_width() - 120.0).max(60.0), 14.0), |ui| bar(ui, prog, GREEN, 12.0));
                ui.label(body(format!("{} / {} lessons", done, total), 12.0).color(DIM));
            });
            ui.add_space(SP_XS);
            match next {
                Some((mi, li)) => { ui.label(body(format!("Next: {} - {}", self.courses[c].modules[mi].name, self.courses[c].modules[mi].lessons[li].title), 12.0).color(INK)); }
                None => { ui.label(body(format!("Course complete! {} badge earned.", self.courses[c].badge.name), 12.0).color(GOLD)); }
            }
        });
        if resp.on_hover_text("Course details").clicked() { self.screen = Screen::Preview(c); }
    }

    /// Full-screen course overview, opened from anywhere except the Learn map itself.
    pub fn preview_screen(&mut self, ctx: &mut egui::Ui, c: usize) {
        let name = self.courses[c].name;
        header(ctx, self, name, true, false);
        let prog = self.progress(c);
        let next = self.next_lesson(c);
        let earned = self.badge_earned(c);
        let total = self.courses[c].lesson_count();
        let done = (prog * total as f32).round() as usize;
        egui::Panel::bottom("pv_bot").frame(egui::Frame::new().fill(BG2).inner_margin(egui::Margin::same(SCREEN_PAD))).show(ctx, |ui| {
            ledge(ui, false);
            let (label, color) = match (next, prog > 0.0) {
                (Some(_), false) => ("START COURSE".to_string(), GOLD),
                (Some((m, l)), true) => (format!("CONTINUE: {}", self.courses[c].modules[m].lessons[l].title.to_uppercase()), GOLD),
                (None, _) => ("REVIEW FROM THE START".to_string(), BLUE),
            };
            if pbtn(ui, &label, color, 0.0).clicked() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                self.cur = c; self.s.course = c as u32; self.persist();
                let (m, l) = next.unwrap_or((0, 0));
                self.screen = Screen::Main; self.tab = Tab::Learn; self.open_mod = Some(m);
                self.open_lesson(c, m, l);
            }
            ui.add_space(SP_S);
            if pbtn(ui, "OPEN COURSE MAP", PANEL_HI, 0.0).clicked() { self.screen = Screen::Main; self.select_course(c); }
        });
        let mut notes: Option<(usize, usize)> = None;
        let mut cert = false;
        egui::CentralPanel::default().frame(screen_frame(BG)).show(ctx, |ui| {
            if self.s.floor { floor(ui.painter(), ui.clip_rect(), self.s.scan); }
            scroll_centered(ui, |ui| {
                let cs = &self.courses[c];
                // hero block
                panel(ui, PANEL, EDGE, |ui| {
                    ui.set_width(ui.available_width());
                    let top = ui.max_rect();
                    ui.painter().rect_filled(Rect::from_min_size(top.min - Vec2::new(SP_M - 3.0, SP_M - 3.0), Vec2::new(top.width() + 2.0 * SP_M - 6.0, 5.0)), 0.0, cs.color);
                    ui.add_space(SP_XS);
                    ui.horizontal(|ui| {
                        badge(ui, cs, self.badge_tier(c), 96.0);
                        ui.add_space(SP_S);
                        ui.vertical(|ui| {
                            ui.horizontal_wrapped(|ui| { for t in [cs.level, cs.cat] { tag(ui, t, PANEL_HI); } tag(ui, &format!("~{} hours", cs.hours), PANEL_HI); });
                            ui.add_space(SP_S);
                            ui.label(title(cs.name, 18.0));
                            ui.add_space(SP_S);
                            ui.label(body(cs.desc, 14.0).color(DIM));
                        });
                    });
                });
                // stats
                let stats = [(cs.modules.len().to_string(), "CHAPTERS", "map"), (total.to_string(), "LESSONS", "book"), (cs.project_count().to_string(), "PROJECTS", "hammer"), (format!("{}%", (prog * 100.0) as u32), "DONE", "flame")];
                let w = ((ui.available_width() - 3.0 * SP_S) / 4.0).max(60.0);
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = SP_S;
                    for (v, l, ic) in stats {
                        let (r, _) = ui.allocate_exact_size(Vec2::new(w, 70.0), Sense::hover());
                        bevel(ui.painter(), r, PANEL, false);
                        draw_icon(ui.painter(), Rect::from_min_size(r.min + Vec2::new(6.0, 6.0), Vec2::splat(22.0)), ic, Color32::WHITE);
                        ui.painter().text(Pos2::new(r.center().x, r.min.y + 40.0), Align2::CENTER_CENTER, v, FontId::new(13.0, press()), INK);
                        ui.painter().text(Pos2::new(r.center().x, r.max.y - 10.0), Align2::CENTER_CENTER, l, FontId::new(6.0, press()), DIM);
                    }
                });
                ui.add_space(CARD_GAP);
                // progress
                panel(ui, PANEL, if prog > 0.0 { GOLD } else { EDGE }, |ui| {
                    ui.set_width(ui.available_width());
                    card_title(ui, "YOUR PROGRESS");
                    bar(ui, prog, GREEN, 12.0);
                    ui.add_space(SP_S);
                    let msg = match next {
                        Some((m, l)) if prog > 0.0 => format!("{} of {} lessons cleared. Next: {} - {}", done, total, cs.modules[m].name, cs.modules[m].lessons[l].title),
                        Some(_) => "Not started yet. Your first quest is waiting!".to_string(),
                        None => "Every lesson cleared. Well done, hero!".to_string(),
                    };
                    ui.label(body(msg, 13.0));
                });
                // what you'll learn
                panel(ui, PANEL, EDGE, |ui| {
                    ui.set_width(ui.available_width());
                    card_title(ui, "WHAT YOU'LL LEARN");
                    for t in &cs.teaches {
                        ui.horizontal_wrapped(|ui| { spr(ui, Spr::Check, 1.6, &[('w', GREEN)]); ui.label(body(*t, 14.0)); });
                        ui.add_space(SP_XS);
                    }
                });
                // badge
                panel(ui, PANEL, if earned.is_some() { GOLD } else { EDGE }, |ui| {
                    ui.set_width(ui.available_width());
                    card_title(ui, "COURSE BADGE");
                    ui.horizontal(|ui| {
                        badge(ui, cs, self.badge_tier(c), 64.0);
                        ui.add_space(SP_S);
                        ui.vertical(|ui| {
                            ui.label(title(cs.badge.name, 11.0).color(if earned.is_some() { GOLD } else { INK }));
                            ui.add_space(SP_XS);
                            ui.label(body(cs.badge.desc, 13.0).color(DIM));
                            if mini_btn(ui, if earned.is_some() { "VIEW CERTIFICATE" } else { "PREVIEW CERTIFICATE" }, if earned.is_some() { GOLD } else { PANEL_HI }).clicked() { cert = true; }
                            ui.label(body(match earned { Some(d) => format!("Earned {}", day_label(d)), None => format!("{} now. Bronze after chapter 1, silver at half, gold when all {} lessons are clear ({} to go).", TIER_NAMES[self.badge_tier(c) as usize], total, total - done) }, 12.0).color(if earned.is_some() { GREEN } else { DIM }));
                        });
                    });
                });
                // chapters
                section(ui, "CHAPTERS");
                for (mi, m) in cs.modules.iter().enumerate() {
                    let unlocked = (0..mi).all(|x| self.module_done(c, x));
                    let cleared = (0..m.lessons.len()).filter(|l| self.is_done(c, mi, *l)).count();
                    panel(ui, PANEL, if cleared == m.lessons.len() { GREEN } else { EDGE }, |ui| {
                        ui.set_width(ui.available_width());
                        ui.horizontal(|ui| {
                            let (r, _) = ui.allocate_exact_size(Vec2::splat(30.0), Sense::hover());
                            bevel(ui.painter(), r, if cleared == m.lessons.len() { GREEN } else if unlocked { GOLD } else { PANEL_HI }, false);
                            ui.painter().text(r.center(), Align2::CENTER_CENTER, format!("{}", mi + 1), FontId::new(10.0, press()), EDGE);
                            ui.vertical(|ui| {
                                ui.label(title(m.name, 9.0));
                                if !m.desc.is_empty() { ui.label(body(m.desc, 12.0).color(DIM)); }
                            });
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| ui.label(body(format!("{}/{}", cleared, m.lessons.len()), 12.0).color(DIM)));
                        });
                        ui.add_space(SP_S);
                        for (li, l) in m.lessons.iter().enumerate() {
                            let d = self.is_done(c, mi, li);
                            let open = self.lesson_unlocked(c, mi, li);
                            ui.horizontal(|ui| {
                                ui.add_space(SP_XS);
                                let (r, _) = ui.allocate_exact_size(Vec2::splat(18.0), Sense::hover());
                                if d { draw_spr(ui.painter(), r.min - Vec2::new(2.0, 2.0), Spr::Check, 1.8, &[('w', GREEN)]); }
                                else if open { ui.painter().rect_filled(Rect::from_center_size(r.center(), Vec2::splat(8.0)), 0.0, GOLD); }
                                else { ui.painter().rect_filled(Rect::from_center_size(r.center(), Vec2::splat(8.0)), 0.0, PANEL_HI); }
                                ui.label(body(l.title, 14.0).color(if d || open { INK } else { DIM }));
                                if l.project { tag(ui, "PROJECT", ORANGE); }
                                if d { ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| if mini_btn(ui, "NOTES", BLUE).on_hover_text("Re-read the explanation and summary").clicked() { notes = Some((mi, li)); }); }
                            });
                        }
                    });
                }
                ui.add_space(SP_L);
            });
        });
        if let Some((m, l)) = notes { self.screen = Screen::Notes(c, m, l); }
        if cert { self.screen = Screen::Certificate(c); }
    }

    /// Re-read a cleared lesson: its explanation and key points, with a way to practise again.
    pub fn notes_screen(&mut self, ctx: &mut egui::Ui, c: usize, m: usize, l: usize) {
        let title_text = self.courses[c].modules[m].lessons[l].title;
        header(ctx, self, title_text, true, false);
        if matches!(self.screen, Screen::Main) { self.screen = Screen::Preview(c); return; }
        egui::Panel::bottom("notes_bot").frame(egui::Frame::new().fill(BG2).inner_margin(egui::Margin::same(SCREEN_PAD))).show(ctx, |ui| {
            ledge(ui, false);
            ui.horizontal(|ui| {
                if pbtn(ui, "BACK", PANEL_HI, 110.0).clicked() { self.screen = Screen::Preview(c); }
                let w = ui.available_width();
                if pbtn(ui, "PRACTISE AGAIN", GOLD, w).clicked() { self.screen = Screen::Main; self.open_lesson(c, m, l); }
            });
        });
        egui::CentralPanel::default().frame(screen_frame(BG)).show(ctx, |ui| {
            if self.s.floor { floor(ui.painter(), ui.clip_rect(), self.s.scan); }
            scroll_centered(ui, |ui| {
                let ls = &self.courses[c].modules[m].lessons[l];
                ui.label(title(format!("{}  *  {}", self.courses[c].name.to_uppercase(), self.courses[c].modules[m].name.to_uppercase()), 8.0).color(DIM));
                ui.add_space(SP_S);
                if !ls.summary.is_empty() { crate::lesson::summary_card(ui, "KEY POINTS", &ls.summary); }
                if !ls.explain.is_empty() { crate::lesson::explain_page(ui, ls.explain); }
                for (text, _) in &ls.slides {
                    panel(ui, PANEL, EDGE, |ui| { ui.set_width(ui.available_width()); ui.label(RichText::new(*text).monospace().size(14.0).color(PARCH)); });
                }
            });
        });
    }
}

/// Calendar date (YYYY-MM-DD) for a day number counted from 1970-01-01.
pub fn ymd(day: u64) -> String {
    let z = day as i64 + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    format!("{:04}-{:02}-{:02}", y, m, d)
}

/// A short, stable certificate number from the hero name, course and date.
pub fn cert_no(name: &str, slug: &str, day: u64) -> String {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in name.bytes().chain(slug.bytes()).chain(day.to_le_bytes()) { h ^= b as u64; h = h.wrapping_mul(0x0100_0000_01b3); }
    format!("CF-{}-{:04X}-{:04X}", slug.to_uppercase().chars().take(4).collect::<String>(), (h >> 16) & 0xffff, h & 0xffff)
}

thread_local! {
    /// Where the certificate was drawn last frame (screen points), so "Save as image" can crop to it.
    static CERT_RECT: std::cell::Cell<Rect> = const { std::cell::Cell::new(Rect::NOTHING) };
}

/// A pixel laurel: leaves along two arcs around `c`, gold with a dark outline, and red ribbon tails.
fn laurel(p: &egui::Painter, c: Pos2, r: f32, k: f32) {
    let leaf = |at: Pos2, ang: f32, len: f32, wid: f32, col: Color32| {
        let (dx, dy) = (ang.cos(), ang.sin());
        let (nx, ny) = (-dy, dx);
        let pts = vec![
            at + Vec2::new(dx, dy) * len, at + Vec2::new(nx, ny) * wid, at - Vec2::new(dx, dy) * len * 0.6, at - Vec2::new(nx, ny) * wid,
        ];
        p.add(egui::Shape::convex_polygon(pts, col, Stroke::NONE));
    };
    for side in [-1.0f32, 1.0] {
        for i in 0..9 {
            let t = i as f32 / 8.0;
            // arc from the bottom up to the top on each side
            let a = std::f32::consts::FRAC_PI_2 + side * (0.35 + t * 2.15);
            let at = c + Vec2::new(a.cos(), a.sin()) * r;
            let tangent = a - side * std::f32::consts::FRAC_PI_2 + side * 0.5;
            let len = (11.0 - t * 3.0) * k;
            leaf(at, tangent, len + 2.0 * k, 5.5 * k, Color32::from_rgb(0x4a, 0x2a, 0x08));
            leaf(at, tangent, len, 3.8 * k, Color32::from_rgb(0xe8, 0xa8, 0x2a));
            leaf(at + Vec2::new(-1.0, -1.0) * k, tangent, len * 0.5, 1.6 * k, Color32::from_rgb(0xff, 0xe0, 0x80));
        }
    }
    // ribbon tails under the wreath
    for side in [-1.0f32, 1.0] {
        let b = c + Vec2::new(side * 8.0 * k, r + 2.0 * k);
        p.add(egui::Shape::convex_polygon(vec![b, b + Vec2::new(side * 14.0 * k, 16.0 * k), b + Vec2::new(side * 6.0 * k, 13.0 * k), b + Vec2::new(side * 2.0 * k, 20.0 * k)], Color32::from_rgb(0xa8, 0x22, 0x2a), Stroke::new(1.5 * k, Color32::from_rgb(0x4a, 0x0e, 0x12))));
    }
}

/// The Forgemaster's signature: a quick ink flourish.
fn signature(p: &egui::Painter, at: Pos2, k: f32, ink: Color32) {
    let pts: Vec<Pos2> = [(-34.0, 6.0), (-28.0, -10.0), (-22.0, 8.0), (-16.0, -6.0), (-10.0, 10.0), (-4.0, -12.0), (0.0, 6.0), (6.0, -4.0), (14.0, 4.0), (20.0, -8.0), (26.0, 2.0), (40.0, -6.0)]
        .iter().map(|(x, y)| at + Vec2::new(*x, *y) * k).collect();
    p.add(egui::Shape::line(pts, Stroke::new(2.2 * k, ink)));
    p.add(egui::Shape::line(vec![at + Vec2::new(-30.0, 12.0) * k, at + Vec2::new(36.0, 9.0) * k], Stroke::new(1.6 * k, ink)));
}

impl App {
    /// The course certificate: proof that every lesson of a course was cleared. (The badge shows
    /// progress and grows bronze -> silver -> gold; the certificate is the finished document.)
    /// Drawn as a dungeon scene: a parchment in an iron-bound wooden frame, hung on a torch-lit wall.
    pub fn certificate_screen(&mut self, ctx: &mut egui::Ui, c: usize) {
        header(ctx, self, "Certificate", true, false);
        let earned = self.badge_earned(c);
        let cs_name = self.courses[c].name;
        egui::Panel::bottom("cert_bot").frame(egui::Frame::new().fill(BG2).inner_margin(egui::Margin::same(SCREEN_PAD))).show(ctx, |ui| {
            ledge(ui, false);
            centered(ui, |ui| ui.horizontal(|ui| {
                if pbtn(ui, "BACK", PANEL_HI, 110.0).clicked() { self.screen = Screen::Main; self.tab = Tab::Hero; }
                let w = ui.available_width();
                if earned.is_some() {
                    if pbtn(ui, "SAVE AS IMAGE", GOLD, w).clicked() {
                        let base = std::env::var("USERPROFILE").map(|h| std::path::Path::new(&h).join("Pictures")).ok().filter(|p| p.is_dir()).unwrap_or_else(crate::save::Save::dir);
                        let path = base.join(format!("CodeForge-{}-Certificate.png", cs_name.replace(' ', "")));
                        let shown = path.display().to_string();
                        let rect = CERT_RECT.with(|r| r.get());
                        let ppp = ui.ctx().pixels_per_point();
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Screenshot(egui::ScreenshotCallback::new(move |img| {
                            let img = if rect.is_positive() { img.region(&rect, Some(ppp)) } else { (*img).clone() };
                            let (w, h) = (img.width() as u32, img.height() as u32);
                            let raw: Vec<u8> = img.pixels.iter().flat_map(|c| c.to_array()).collect();
                            if let Some(b) = image::RgbaImage::from_raw(w, h, raw) { let _ = b.save(&path); }
                        })));
                        self.toast(format!("Saved to {}", shown));
                    }
                } else if pbtn(ui, "CONTINUE THE COURSE", GOLD, w).clicked() { self.screen = Screen::Preview(c); }
            }));
        });
        egui::CentralPanel::default().frame(screen_frame(BG)).show(ctx, |ui| {
            if self.s.floor { floor(ui.painter(), ui.clip_rect(), self.s.scan); }
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                let cs = &self.courses[c];
                let ink = INK_DARK;
                let brown = Color32::from_rgb(0x7a, 0x5a, 0x30);
                let t = ui.ctx().input(|i| i.time) as f32;
                let w = ui.available_width().min(900.0);
                // everything scales with the certificate; phones get a taller layout with bigger text
                let narrow = w < 600.0;
                let k = if narrow { (w / 440.0).clamp(0.72, 1.0) } else { (w / 760.0).clamp(0.8, 1.2) };
                let (side, top) = (22.0 * k, 64.0 * k);        // stone wall around the frame (torches hang above)
                let inner_w = w - 2.0 * side - 2.0 * 34.0 * k;
                let p = ui.painter();
                let lay = |text: String, font: FontId, col: Color32, wrap: f32| p.layout(text, font, col, wrap);
                let fit = |text: &str, family: egui::FontFamily, size: f32, col: Color32| {
                    let mut s = size;
                    loop { let g = p.layout_no_wrap(text.to_string(), FontId::new(s, family.clone()), col); if g.size().x <= inner_w || s < 8.0 { return g; } s *= 0.9; }
                };
                // text
                let no = earned.map(|d| cert_no(&self.s.name, cs.slug, d)).unwrap_or_else(|| "PREVIEW".into());
                let g_brand = lay("Code Forge".into(), FontId::new(13.0 * k, press()), ink, f32::INFINITY);
                let g_no = lay(format!("Certificate No. {}", no), FontId::new(11.0 * k, read()), brown, f32::INFINITY);
                let g_intro = lay("This Certificate of Achievement is awarded to".into(), FontId::new(15.0 * k, read()), ink, inner_w);
                let g_name = fit(&self.s.name, press(), 26.0 * k, Color32::from_rgb(0x5a, 0x1e, 0x0c));
                let g_for = lay("for forging through every lesson and project of".into(), FontId::new(15.0 * k, read()), ink, inner_w);
                let g_course = fit(cs.name, press(), 24.0 * k, shade(cs.color, 0.7));
                let skills = cs.teaches.iter().map(|s| s.to_lowercase()).collect::<Vec<_>>().join(", ");
                let desc = format!("Heroes who complete the {} course at Code Forge Academy have mastered {}. Their skill is proven by {} checkpoint projects across {} lessons, about {} hours of hands-on coding in the dungeon.", cs.name, skills, cs.project_count(), cs.lesson_count(), cs.hours);
                let mut job = egui::text::LayoutJob::single_section(desc, egui::TextFormat { font_id: FontId::new(12.5 * k, read()), color: brown, italics: true, ..Default::default() });
                job.wrap.max_width = inner_w * 0.92;
                job.halign = Align2::CENTER_TOP.x();
                let g_desc = p.layout_job(job);
                let date = earned.map(ymd).unwrap_or_else(|| "not yet".into());
                let g_date = lay(date, FontId::new(15.0 * k, read()), ink, f32::INFINITY);
                // vertical layout
                let wreath = 60.0 * k;
                let heights = [if narrow { 50.0 * k } else { 34.0 * k }, 2.0 * wreath + 18.0 * k, g_intro.size().y + 6.0 * k, g_name.size().y + 10.0 * k, g_for.size().y + 6.0 * k, g_course.size().y + 14.0 * k, 10.0 * k, g_desc.size().y + 22.0 * k, 92.0 * k];
                let card_h: f32 = heights.iter().sum::<f32>() + 2.0 * 26.0 * k;
                let total_h = card_h + 2.0 * top;
                let (scene, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), total_h), Sense::hover());
                let scene = Rect::from_center_size(scene.center(), Vec2::new(w, total_h));
                CERT_RECT.with(|r| r.set(scene));
                let p = ui.painter().with_clip_rect(scene);
                // the wall
                p.add(tile_shape(scene, "wall", Color32::from_gray(150)));
                p.rect_filled(scene, 0.0, Color32::from_rgba_unmultiplied(16, 12, 28, 120));
                // wall torches above the frame's corners, flickering, and a shield on the wall between them
                for (i, x) in [scene.min.x + side + 70.0 * k, scene.max.x - side - 70.0 * k].iter().enumerate() {
                    let flick = 0.75 + 0.25 * ((t * 7.0 + i as f32 * 2.0).sin() * (t * 3.1).cos());
                    let tc = Pos2::new(*x, scene.min.y + top * 0.5);
                    p.circle_filled(tc, 58.0 * k, Color32::from_rgba_unmultiplied(255, 150, 40, (30.0 * flick) as u8));
                    p.circle_filled(tc, 30.0 * k, Color32::from_rgba_unmultiplied(255, 190, 80, (42.0 * flick) as u8));
                    draw_icon(&p, Rect::from_center_size(tc, Vec2::splat(96.0 * k)), "torch", Color32::WHITE);
                }
                draw_icon(&p, Rect::from_center_size(Pos2::new(scene.center().x, scene.min.y + top * 0.5), Vec2::splat(48.0 * k)), "banner_wall", Color32::WHITE);
                // the framed parchment
                let card = Rect::from_min_size(Pos2::new(scene.min.x + side, scene.min.y + top), Vec2::new(w - 2.0 * side, card_h));
                p.add(frame_shapes(card, Color32::from_rgb(0xf3, 0xe4, 0xbc), GOLD));
                let paper = card.shrink(10.0 * k);
                p.rect_stroke(paper.shrink(8.0 * k), 0.0, Stroke::new(1.5 * k, Color32::from_rgb(0xc8, 0xa4, 0x68)), egui::StrokeKind::Inside);
                for (cx, cy) in [(paper.min.x, paper.min.y), (paper.max.x, paper.min.y), (paper.min.x, paper.max.y), (paper.max.x, paper.max.y)] {
                    // iron corner brackets with a rivet
                    let r = Rect::from_center_size(Pos2::new(cx, cy), Vec2::splat(18.0 * k));
                    p.rect_filled(r, 2.0, Color32::from_rgb(0x3a, 0x3e, 0x4e));
                    p.rect_filled(r.shrink(3.0 * k), 1.0, Color32::from_rgb(0x6a, 0x72, 0x88));
                    p.rect_filled(Rect::from_center_size(r.center(), Vec2::splat(4.0 * k)), 0.0, Color32::from_rgb(0xd6, 0xa8, 0x3c));
                }
                let left = card.min.x + 34.0 * k;
                let right = card.max.x - 34.0 * k;
                let mid = card.center().x;
                let mut y = card.min.y + 26.0 * k;
                // top row: brand and number
                let lr = Rect::from_min_size(Pos2::new(left - 2.0 * k, y - 6.0 * k), Vec2::splat(28.0 * k));
                if let Some((id, _)) = crate::art::TEX.get().and_then(|m| m.get("logo_mark")).copied() {
                    p.image(id, lr, Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)), Color32::WHITE); // exact size, any scale
                }
                p.galley(Pos2::new(left + 34.0 * k, y + 2.0 * k), g_brand, ink);
                if narrow { p.galley(Pos2::new(left, y + 26.0 * k), g_no, brown); } else { p.galley(Pos2::new(right - g_no.size().x, y + 2.0 * k), g_no, brown); }
                y += heights[0];
                // emblem: laurel around the gold course badge
                let ec = Pos2::new(mid, y + wreath);
                p.circle_filled(ec, wreath * 0.62, Color32::from_rgba_unmultiplied(232, 168, 42, 40));
                laurel(&p, ec, wreath * 0.78, k);
                draw_badge(&p, Rect::from_center_size(ec, Vec2::splat(wreath * 1.05)), cs, if earned.is_some() { 3 } else { 0 });
                y += heights[1];
                let center = |p: &egui::Painter, g: std::sync::Arc<egui::Galley>, y: f32| { let x = mid - g.size().x / 2.0; p.galley(Pos2::new(x, y), g, ink); };
                center(&p, g_intro, y); y += heights[2];
                center(&p, g_name, y); y += heights[3];
                center(&p, g_for, y); y += heights[4];
                center(&p, g_course, y); y += heights[5];
                p.rect_filled(Rect::from_center_size(Pos2::new(mid, y), Vec2::new(70.0 * k, 2.0 * k)), 0.0, Color32::from_rgb(0xb0, 0x8a, 0x50));
                y += heights[6];
                p.galley(Pos2::new(mid, y), g_desc, brown); y += heights[7]; // centre-aligned text is laid out around x = 0
                // bottom row: signature, date, wax seal
                let cols = [left + (right - left) / 6.0, mid, right - (right - left) / 6.0];
                let line_y = y + 40.0 * k;
                let small = |text: &str, size: f32, col: Color32| p.layout_no_wrap(text.to_string(), FontId::new(size * k, press()), col);
                signature(&p, Pos2::new(cols[0], line_y - 16.0 * k), k, Color32::from_rgb(0x22, 0x18, 0x30));
                let line_w = (120.0 * k).min((right - left) / 3.0 * 0.85);
                for cx in [cols[0], cols[1]] { p.rect_filled(Rect::from_center_size(Pos2::new(cx, line_y), Vec2::new(line_w, 1.5 * k)), 0.0, Color32::from_rgb(0xb0, 0x8a, 0x50)); }
                let g = small("CODE FORGE ACADEMY", 6.5, brown); let x = cols[0] - g.size().x / 2.0; p.galley(Pos2::new(x, line_y + 22.0 * k), g, brown);
                let g = small("THE FORGEMASTER", 8.0, ink); let x = cols[0] - g.size().x / 2.0; p.galley(Pos2::new(x, line_y + 8.0 * k), g, ink);
                let gd = g_date; let x = cols[1] - gd.size().x / 2.0; p.galley(Pos2::new(x, line_y - gd.size().y - 4.0 * k), gd, ink);
                let g = small("ISSUED ON", 8.0, ink); let x = cols[1] - g.size().x / 2.0; p.galley(Pos2::new(x, line_y + 8.0 * k), g, ink);
                let seal = Pos2::new(cols[2], line_y - 6.0 * k);
                let sr = 30.0 * k;
                for (i, a) in (0..12).map(|i| (i, i as f32 / 12.0 * std::f32::consts::TAU)) { p.circle_filled(seal + Vec2::new(a.cos(), a.sin()) * sr * (if i % 2 == 0 { 1.0 } else { 0.9 }), sr * 0.32, Color32::from_rgb(0x8a, 0x14, 0x1e)); }
                p.circle_filled(seal, sr * 0.92, Color32::from_rgb(0xa8, 0x1e, 0x28));
                p.circle_stroke(seal, sr * 0.7, Stroke::new(2.0 * k, Color32::from_rgb(0x6a, 0x0c, 0x14)));
                draw_icon(&p, Rect::from_center_size(seal, Vec2::splat(sr * 1.15)), "logo_mark", Color32::from_rgb(255, 214, 160));
                let g = small("SEALED IN THE FORGE", 7.0, ink); let x = cols[2] - g.size().x / 2.0; p.galley(Pos2::new(x, line_y + 30.0 * k), g, ink);
                // not earned yet: a stamped PREVIEW across the parchment
                if earned.is_none() {
                    let lb = p.layout_no_wrap("PREVIEW".into(), FontId::new(30.0 * k, press()), Color32::from_rgba_unmultiplied(200, 30, 40, 150));
                    let rc = Rect::from_center_size(card.center(), lb.size() + Vec2::splat(20.0 * k));
                    let mut mesh_text = egui::Shape::galley(rc.min + Vec2::splat(10.0 * k), lb, Color32::TRANSPARENT);
                    if let egui::Shape::Text(ts) = &mut mesh_text { ts.angle = -0.2; }
                    p.rect_stroke(rc, 4.0, Stroke::new(3.0 * k, Color32::from_rgba_unmultiplied(200, 30, 40, 150)), egui::StrokeKind::Inside);
                    p.add(mesh_text);
                }
                ui.add_space(SP_S);
                centered(ui, |ui| {
                    if earned.is_none() {
                        let left = cs.lesson_count() - (self.progress(c) * cs.lesson_count() as f32).round() as usize;
                        ui.label(body(format!("Clear the last {} lesson{} of {} to earn this certificate.", left, if left == 1 { "" } else { "s" }, cs.name), 13.0).color(GOLD));
                    }
                    ui.add(egui::Label::new(body("Your badge grows with your progress (bronze, silver, gold). The certificate is awarded once, when the whole course is done.", 12.0).color(DIM)).wrap());
                });
                ui.ctx().request_repaint(); // torches flicker
            });
        });
    }
}

/// A small filled label chip.
pub fn tag(ui: &mut egui::Ui, text: &str, color: Color32) {
    let w = 12.0 + text.chars().count() as f32 * 7.0;
    let (r, _) = ui.allocate_exact_size(Vec2::new(w, 18.0), Sense::hover());
    ui.painter().rect_filled(r, 0.0, color);
    let ink = if color.r() as u32 + color.g() as u32 + color.b() as u32 > 460 { EDGE } else { Color32::WHITE };
    ui.painter().text(r.center(), Align2::CENTER_CENTER, text.to_uppercase(), FontId::new(7.0, press()), ink);
}
