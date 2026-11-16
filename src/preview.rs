//! Course Preview screen, the Learn tab's quest-card header, and course badges.
use super::*;

/// A course badge: a stepped pixel medal in the course color with a gold rim (grey when locked),
/// the course icon in the middle and two ribbon tails.
pub fn draw_badge(p: &egui::Painter, r: Rect, course: &Course, earned: bool) {
    let s = (r.width().min(r.height()) / 16.0).floor().max(1.0);
    let w = s * 16.0;
    let o = Pos2::new(r.center().x - w / 2.0, r.center().y - w / 2.0);
    let px = |x: f32, y: f32, ww: f32, hh: f32, c: Color32| p.rect_filled(Rect::from_min_size(o + Vec2::new(x * s, y * s), Vec2::new(ww * s, hh * s)), 0.0, c);
    let body = if earned { course.color } else { Color32::from_rgb(0x3a, 0x36, 0x52) };
    let rim = if earned { GOLD } else { Color32::from_rgb(0x6b, 0x66, 0x88) };
    // ribbon tails
    px(4.0, 11.0, 3.0, 5.0, shade(body, 0.7));
    px(9.0, 11.0, 3.0, 5.0, shade(body, 0.7));
    px(4.0, 15.0, 1.0, 1.0, Color32::TRANSPARENT);
    // stepped round medal: outline, rim, body
    for (y, x0, x1) in [(0.0, 5.0, 11.0), (1.0, 3.0, 13.0), (2.0, 2.0, 14.0), (3.0, 1.0, 15.0), (4.0, 1.0, 15.0), (5.0, 0.0, 16.0), (6.0, 0.0, 16.0), (7.0, 0.0, 16.0), (8.0, 0.0, 16.0), (9.0, 1.0, 15.0), (10.0, 1.0, 15.0), (11.0, 2.0, 14.0), (12.0, 3.0, 13.0), (13.0, 5.0, 11.0)] {
        px(x0, y, x1 - x0, 1.0, EDGE);
    }
    for (y, x0, x1) in [(1.0, 5.0, 11.0), (2.0, 3.0, 13.0), (3.0, 2.0, 14.0), (4.0, 2.0, 14.0), (5.0, 1.0, 15.0), (6.0, 1.0, 15.0), (7.0, 1.0, 15.0), (8.0, 1.0, 15.0), (9.0, 2.0, 14.0), (10.0, 2.0, 14.0), (11.0, 3.0, 13.0), (12.0, 5.0, 11.0)] {
        px(x0, y, x1 - x0, 1.0, rim);
    }
    for (y, x0, x1) in [(2.0, 5.0, 11.0), (3.0, 4.0, 12.0), (4.0, 3.0, 13.0), (5.0, 3.0, 13.0), (6.0, 2.0, 14.0), (7.0, 2.0, 14.0), (8.0, 3.0, 13.0), (9.0, 3.0, 13.0), (10.0, 4.0, 12.0), (11.0, 5.0, 11.0)] {
        px(x0, y, x1 - x0, 1.0, body);
    }
    px(5.0, 2.0, 6.0, 1.0, shade(body, 1.3)); // highlight
    let ir = Rect::from_center_size(o + Vec2::new(8.0 * s, 6.8 * s), Vec2::splat(8.0 * s));
    draw_icon(p, ir, course.badge.art, if earned { Color32::WHITE } else { Color32::from_gray(70) });
}

pub fn badge(ui: &mut egui::Ui, course: &Course, earned: bool, size: f32) -> egui::Response {
    let (r, resp) = ui.allocate_exact_size(Vec2::splat(size), Sense::click());
    draw_badge(ui.painter(), r, course, earned);
    resp
}

impl App {
    pub fn badge_earned(&self, c: usize) -> Option<u64> { self.s.ach.get(&self.courses[c].badge_id()).copied() }

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
                badge(ui, &self.courses[c], earned, 72.0).on_hover_text(format!("{} badge: {}", self.courses[c].badge.name, if earned { "earned!" } else { "clear every lesson to earn it" }));
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
        egui::CentralPanel::default().frame(screen_frame(BG)).show(ctx, |ui| {
            if self.s.floor { floor(ui.painter(), ui.max_rect(), self.s.scan); }
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                let cs = &self.courses[c];
                // hero block
                panel(ui, PANEL, EDGE, |ui| {
                    ui.set_width(ui.available_width());
                    let top = ui.max_rect();
                    ui.painter().rect_filled(Rect::from_min_size(top.min - Vec2::new(SP_M - 3.0, SP_M - 3.0), Vec2::new(top.width() + 2.0 * SP_M - 6.0, 5.0)), 0.0, cs.color);
                    ui.add_space(SP_XS);
                    ui.horizontal(|ui| {
                        badge(ui, cs, earned.is_some(), 96.0);
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
                        badge(ui, cs, earned.is_some(), 64.0);
                        ui.add_space(SP_S);
                        ui.vertical(|ui| {
                            ui.label(title(cs.badge.name, 11.0).color(if earned.is_some() { GOLD } else { INK }));
                            ui.add_space(SP_XS);
                            ui.label(body(cs.badge.desc, 13.0).color(DIM));
                            ui.label(body(match earned { Some(d) => format!("Earned {}", day_label(d)), None => format!("Clear all {} lessons to earn it ({} to go)", total, total - done) }, 12.0).color(if earned.is_some() { GREEN } else { DIM }));
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
                            });
                        }
                    });
                }
                ui.add_space(SP_L);
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
