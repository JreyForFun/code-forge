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
        egui::CentralPanel::default().frame(screen_frame(BG)).show(ctx, |ui| {
            if self.s.floor { floor(ui.painter(), ui.clip_rect(), self.s.scan); }
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
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
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
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

/// A small filled label chip.
pub fn tag(ui: &mut egui::Ui, text: &str, color: Color32) {
    let w = 12.0 + text.chars().count() as f32 * 7.0;
    let (r, _) = ui.allocate_exact_size(Vec2::new(w, 18.0), Sense::hover());
    ui.painter().rect_filled(r, 0.0, color);
    let ink = if color.r() as u32 + color.g() as u32 + color.b() as u32 > 460 { EDGE } else { Color32::WHITE };
    ui.painter().text(r.center(), Align2::CENTER_CENTER, text.to_uppercase(), FontId::new(7.0, press()), ink);
}
