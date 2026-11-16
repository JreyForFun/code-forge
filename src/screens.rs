use super::*;

impl App {
    /// Returns true when the learner leaves the screen.
    pub fn complete_screen(&mut self, ctx: &mut egui::Ui, c: &Complete) -> bool {
        let mut done = false;
        let enter = ctx.ctx().input(|i| i.key_pressed(egui::Key::Enter));
        egui::CentralPanel::default().frame(egui::Frame::new().fill(BG).inner_margin(20.0)).show(ctx, |ui| {
            floor(ui.painter(), ui.max_rect(), self.s.scan);
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(12.0);
                    let lost = c.duel.as_ref().map(|d| !d.3).unwrap_or(false);
                    let (r, _) = ui.allocate_exact_size(Vec2::new(130.0, 120.0), Sense::hover());
                    draw_icon(ui.painter(), r, if lost { "heart" } else if c.duel.is_some() { "chalice" } else { "chest" }, if lost { Color32::from_gray(80) } else { Color32::WHITE });
                    ui.add_space(8.0);
                    let head = if c.duel.is_some() { if lost { "DEFEATED..." } else { "VICTORY!" } } else { "QUEST COMPLETE!" };
                    ui.label(title(head, 15.0).color(GOLD));
                    ui.add_space(6.0);
                    ui.label(body(c.title.clone(), 15.0).color(DIM));
                    if let Some((m, b, n, _)) = &c.duel { ui.label(body(format!("You {}  -  {} {}", m, n, b), 15.0)); }
                    ui.add_space(14.0);
                    panel(ui, PANEL, GOLD, |ui| {
                        ui.set_width(ui.available_width());
                        ui.horizontal(|ui| { icon(ui, "scroll_lightning", 30.0); ui.label(title(format!("+{} XP", c.xp), 12.0).color(BLUE)); });
                        if c.gems > 0 { ui.horizontal(|ui| { icon(ui, "crystal_purple", 30.0); ui.label(title(format!("+{} gems", c.gems), 12.0).color(PURPLE)); }); }
                        ui.horizontal(|ui| { icon(ui, "flame", 30.0); ui.label(body(format!("Streak {} days (best {}){}", self.s.streak, self.s.streak_max, if c.new_streak { "  +1 today!" } else { "" }), 14.0)); });
                        if c.duel.is_none() { ui.label(body(format!("Score {}%   (best {}%)", c.score, c.best), 14.0).color(DIM)); }
                        if c.perfect && c.duel.is_none() && c.xp >= 15 { ui.label(body("Flawless! +5 bonus XP", 14.0).color(GREEN)); }
                        ui.add_space(4.0);
                        ui.label(title(format!("LEVEL {}", self.level()), 10.0));
                        bar(ui, game::level_frac(self.s.xp), BLUE, 14.0);
                    });
                    ui.add_space(10.0);
                    if let Some((ci, m, l)) = c.next {
                        let name = self.courses[ci].modules[m].lessons[l].title;
                        if pbtn(ui, &format!("NEXT: {}", name.to_uppercase()), GOLD, 0.0).clicked() || enter {
                            done = true; self.open_lesson(ci, m, l);
                        }
                        ui.add_space(6.0);
                    }
                    if pbtn(ui, "BACK TO MAP", GREEN, 0.0).clicked() || (enter && c.next.is_none()) { done = true; }
                    ui.add_space(6.0);
                    if pbtn(ui, "GUILD RANKS", BLUE, 0.0).clicked() { done = true; self.tab = Tab::Ranks; }
                });
            });
        });
        done
    }

    pub fn settings(&mut self, ctx: &mut egui::Ui) {
        header(ctx, self, "Settings", true, false);
        egui::CentralPanel::default().frame(egui::Frame::new().fill(BG).inner_margin(14.0)).show(ctx, |ui| {
            floor(ui.painter(), ui.max_rect(), self.s.scan);
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                panel(ui, PANEL, EDGE, |ui| {
                    ui.set_width(ui.available_width());
                    ui.label(title("HERO", 10.0).color(GOLD));
                    ui.add_space(6.0);
                    ui.horizontal(|ui| {
                        let (r, resp) = ui.allocate_exact_size(Vec2::splat(64.0), Sense::click());
                        draw_avatar(ui.painter(), r, self.s.avatar);
                        if resp.on_hover_text("Change avatar").on_hover_cursor(egui::CursorIcon::PointingHand).clicked() { self.modal = Modal::Avatar; }
                        ui.vertical(|ui| {
                            ui.label(body("Name", 13.0).color(DIM));
                            let r = ui.add(egui::TextEdit::singleline(&mut self.s.name).desired_width(f32::INFINITY).char_limit(save::NAME_MAX));
                            if r.lost_focus() { self.s.name = Save::clean_name(&self.s.name); self.persist(); }
                            if ui.link(body("Change avatar", 13.0).color(BLUE)).clicked() { self.modal = Modal::Avatar; }
                        });
                    });
                    ui.add_space(8.0);
                    ui.label(body("Daily streak goal", 13.0).color(DIM));
                    ui.horizontal(|ui| {
                        let gw = ((ui.available_width() - 24.0) / 4.0).max(40.0);
                        for g in save::GOALS { if pbtn(ui, &g.to_string(), if self.s.goal == g { GOLD } else { PANEL_HI }, gw).clicked() { self.s.goal = g; self.persist(); } }
                    });
                });
                panel(ui, PANEL, EDGE, |ui| {
                    ui.set_width(ui.available_width());
                    ui.label(title("DISPLAY", 10.0).color(GOLD));
                    ui.add_space(6.0);
                    if toggle(ui, "Scanlines", "CRT-style pixel filter", &mut self.s.scan) { self.persist(); }
                    if toggle(ui, "Dungeon floor", "Brick texture behind the screen", &mut self.s.floor) { self.persist(); }
                });
                panel(ui, PANEL, EDGE, |ui| {
                    ui.set_width(ui.available_width());
                    ui.label(title("HOW IT WORKS", 10.0).color(GOLD));
                    ui.add_space(6.0);
                    let league_rule = format!("Earn {} XP in a week to move up a league. Rivals are computer characters.", LEAGUE_UP_XP);
                    for (ic, t) in [("heart", "Wrong answers cost a heart. Hearts refill 1 every 5 minutes (max 5)."), ("scroll_lightning", "100 XP per level. First clears give 10 XP (+5 if flawless) and 5 gems."),
                        ("flame", "Clear one lesson a day to grow your streak. A streak saver covers one missed day."), ("banner_royal", league_rule.as_str())] {
                        ui.horizontal_wrapped(|ui| { icon(ui, ic, 26.0); ui.label(body(t, 13.0)); });
                    }
                    ui.add_space(4.0);
                    ui.label(body("Keys: Enter = next/check/continue, 1-5 = pick an answer, Esc = leave a lesson.", 12.0).color(DIM));
                });
                panel(ui, PANEL, EDGE, |ui| {
                    ui.set_width(ui.available_width());
                    ui.label(title("DATA", 10.0).color(GOLD));
                    ui.add_space(6.0);
                    ui.label(body("Progress is saved on this computer only. Nothing is sent anywhere.", 13.0));
                    ui.label(RichText::new(Save::path().display().to_string()).monospace().size(11.0).color(DIM));
                    ui.add_space(8.0);
                    if pbtn(ui, "RESET PROGRESS", RED, 0.0).clicked() { self.modal = Modal::Reset; }
                });
                panel(ui, PANEL, EDGE, |ui| {
                    ui.set_width(ui.available_width());
                    ui.label(title("CREDITS", 10.0).color(GOLD));
                    ui.add_space(6.0);
                    ui.horizontal_wrapped(|ui| { icon(ui, "scroll_fire", 32.0); ui.label(body("Assets by Pixelrepo (https://pixelrepo.com), used under the Pixelrepo Free Asset License.", 13.0)); });
                    ui.add_space(4.0);
                    ui.label(body("Fonts: Press Start 2P (CodeMan38) and Silkscreen (Jason Kottke), SIL Open Font License 1.1.", 13.0).color(DIM));
                    ui.label(body("Python and HTML chapter topics follow common beginner outlines. All lesson text is original.", 13.0).color(DIM));
                    ui.add_space(6.0);
                    ui.label(title(format!("PIXEL LEARN v{}", VERSION), 9.0).color(DIM));
                });
            });
        });
    }

    pub fn editor_screen(&mut self, ctx: &mut egui::Ui, mut ed: Editor) -> Option<Editor> {
        let mut keep = true;
        let run_key = ctx.ctx().input(|i| i.key_pressed(egui::Key::Enter) && i.modifiers.command) || ctx.ctx().input(|i| i.key_pressed(egui::Key::F5));
        egui::Panel::top("etop").frame(egui::Frame::new().fill(HDR).inner_margin(12.0)).show(ctx, |ui| {
            ui.horizontal(|ui| {
                if pbtn(ui, "<", Color32::from_rgb(0x3a, 0x4a, 0xa8), 44.0).on_hover_text("Back (unsaved changes are lost)").clicked() { keep = false; }
                ui.add(egui::TextEdit::singleline(&mut ed.title).hint_text("Spell name").desired_width(ui.available_width()).char_limit(30).font(FontId::new(16.0, egui::FontFamily::Proportional)));
            });
        });
        egui::CentralPanel::default().frame(egui::Frame::new().fill(BG).inner_margin(12.0)).show(ctx, |ui| {
            floor(ui.painter(), ui.max_rect(), self.s.scan);
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(6.0, 6.0);
                    let w = ((ui.available_width() - 24.0) / 5.0).max(56.0);
                    for (k, n) in LANGS {
                        if pbtn(ui, n, if ed.lang == k { GOLD } else { PANEL_HI }, w).clicked() && ed.lang != k {
                            let starter_like = ed.code.trim().is_empty() || LANGS.iter().any(|(l, _)| data::starter(l) == ed.code);
                            ed.lang = k.into();
                            if starter_like { ed.code = data::starter(k).into(); }
                            ed.out.clear();
                        }
                    }
                });
                ui.add_space(8.0);
                panel(ui, Color32::from_rgb(0x0f, 0x0b, 0x1a), GREEN, |ui| {
                    ui.add(egui::TextEdit::multiline(&mut ed.code).code_editor().desired_rows(11).desired_width(f32::INFINITY).text_color(PARCH).char_limit(20_000));
                });
                ui.horizontal(|ui| {
                    if pbtn(ui, "RUN", GREEN, 100.0).on_hover_text("Ctrl+Enter or F5").clicked() || run_key { ed.out = interp::run(&ed.lang, &ed.code); }
                    if pbtn(ui, "SAVE", BLUE, 100.0).clicked() {
                        let t = if ed.title.trim().is_empty() { "Untitled".to_string() } else { ed.title.trim().to_string() };
                        ed.title = t.clone();
                        let e = (t, ed.lang.clone(), ed.code.clone());
                        match ed.idx { Some(i) if i < self.s.codes.len() => self.s.codes[i] = e, _ => { self.s.codes.push(e); ed.idx = Some(self.s.codes.len() - 1); self.log(format!("forged a new spell: '{}'", ed.title)); } }
                        self.award();
                        self.persist(); self.toast("Saved to My Spells");
                    }
                    if let Some(i) = ed.idx && pbtn(ui, "DELETE", RED, 100.0).clicked() { self.modal = Modal::DeleteSpell(i); }
                });
                ui.add_space(8.0);
                panel(ui, PANEL, EDGE, |ui| {
                    ui.set_width(ui.available_width());
                    ui.label(title("OUTPUT", 9.0).color(GOLD));
                    ui.add_space(4.0);
                    if ed.out.is_empty() { ui.label(body("Press RUN to cast your code.", 14.0).color(DIM)); } else { out_view(ui, &ed.out); }
                });
                if ed.lang == "sql" {
                    panel(ui, PANEL, EDGE, |ui| {
                        ui.set_width(ui.available_width());
                        ui.label(title("SAMPLE TABLES", 9.0).color(GOLD));
                        for t in sql::sample_tables() { ui.label(RichText::new(format!("{}({})", t.name, t.cols.join(", "))).monospace().size(13.0).color(PARCH)); }
                    });
                }
            });
        });
        if keep { Some(ed) } else { None }
    }
}
