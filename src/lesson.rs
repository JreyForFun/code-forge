use super::*;

fn norm_code(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect::<String>().to_lowercase().trim_end_matches(';').to_string()
}

impl App {
    /// One lesson, three steps: LEARN (text + picture) -> COMPLETE (pick or drag code) -> CODE (type it).
    pub fn run_screen(&mut self, ctx: &mut egui::Ui, mut rs: Box<RunState>) -> Option<Box<RunState>> {
        let mut keep = true;
        let mut finished = false;
        let n_slides = rs.slides.len();
        let in_slides = rs.stage < n_slides;
        let duel = matches!(rs.kind, Kind::Duel(_));
        let cur_q = rs.queue.first().copied();
        let shown_q = if rs.fb.is_some() { rs.last } else { cur_q };
        let phase = if in_slides { "1  LEARN" } else {
            match shown_q.map(|i| &rs.qs[i]) { Some(Q::Fill { .. }) => "2  COMPLETE THE CODE", Some(Q::Type { guide: true, .. }) => "3  TYPE THE CODE", Some(Q::Type { .. }) => "3  CODE IT FROM MEMORY", _ => "QUIZ" }
        };

        let (enter, ctrl_enter, num_key) = ctx.ctx().input(|i| {
            let n = [egui::Key::Num1, egui::Key::Num2, egui::Key::Num3, egui::Key::Num4, egui::Key::Num5].iter().position(|k| i.key_pressed(*k));
            (i.key_pressed(egui::Key::Enter), i.key_pressed(egui::Key::Enter) && i.modifiers.command, n)
        });
        let typing_multiline = matches!(cur_q.map(|i| &rs.qs[i]), Some(Q::Type { answer, .. }) if answer.contains('\n')) && rs.fb.is_none() && !in_slides;
        let go = if typing_multiline { ctrl_enter } else { enter };
        if !in_slides && rs.fb.is_none()
            && let (Some(n), Some(qi)) = (num_key, cur_q) {
                let len = match &rs.qs[qi] { Q::Mc { opts, .. } | Q::Fill { opts, .. } => opts.len(), _ => 0 };
                if n < len { rs.sel = Some(n); }
            }
        if ctx.ctx().input(|i| i.key_pressed(egui::Key::Escape)) && self.modal == Modal::None { keep = false; }

        egui::Panel::top("rtop").frame(egui::Frame::new().fill(BG2).inner_margin(egui::Margin { left: 16, right: 16, top: 12, bottom: 12 })).show(ctx, |ui| {
            ui.horizontal(|ui| {
                let (r, resp) = ui.allocate_exact_size(Vec2::splat(28.0), Sense::click());
                ui.painter().text(r.center(), Align2::CENTER_CENTER, "X", FontId::new(14.0, press()), if resp.hovered() { INK } else { DIM });
                if resp.on_hover_text("Leave (progress is kept)").clicked() { keep = false; }
                let total = (n_slides + rs.total).max(1) as f32;
                let done = if in_slides { rs.stage as f32 / total } else { (n_slides as f32 + rs.right as f32) / total };
                ui.allocate_ui(Vec2::new((ui.available_width() - 70.0).max(40.0), 18.0), |ui| bar(ui, done, GREEN, 12.0));
                if !duel { icon(ui, "heart", 26.0); ui.label(title(self.s.hearts.to_string(), 11.0)); }
            });
        });

        let bot_fill = match &rs.fb { Some((true, _)) => Color32::from_rgb(0x10, 0x32, 0x1c), Some((false, _)) => Color32::from_rgb(0x3a, 0x12, 0x1e), None => BG2 };
        egui::Panel::bottom("rbot").frame(egui::Frame::new().fill(bot_fill).inner_margin(16.0)).show(ctx, |ui| {
            if in_slides {
                ui.horizontal(|ui| {
                    if rs.stage > 0 && pbtn(ui, "BACK", PANEL_HI, 100.0).clicked() { rs.stage -= 1; }
                    let w = ui.available_width();
                    if pbtn(ui, if rs.stage + 1 == n_slides { "START THE QUIZ" } else { "NEXT" }, GOLD, w).clicked() || go { rs.stage += 1; }
                });
            } else if let Some((ok, why)) = rs.fb.clone() {
                ui.label(title(if ok { "CORRECT" } else { "NOT QUITE" }, 12.0).color(if ok { GREEN } else { RED }));
                ui.add_space(2.0);
                ui.label(body(why, 14.0));
                ui.add_space(8.0);
                if pbtn(ui, "CONTINUE", if ok { GREEN } else { RED }, 0.0).clicked() || go {
                    rs.fb = None; rs.sel = None; rs.text.clear();
                    if rs.queue.is_empty() { finished = true; }
                    else if self.s.hearts == 0 && !duel { keep = false; self.modal = Modal::NoHearts; }
                }
            } else if let Some(qi) = cur_q {
                let can = match &rs.qs[qi] { Q::Type { .. } => !rs.text.trim().is_empty(), _ => rs.sel.is_some() };
                ui.horizontal(|ui| {
                    let hint_w = 120.0;
                    if rs.hint_q != Some(qi)
                        && pbtn(ui, &format!("HINT {}G", HINT_COST), PANEL_HI, hint_w).clicked() {
                            if self.s.gems >= HINT_COST { self.s.gems -= HINT_COST; rs.hint_q = Some(qi); self.persist(); } else { self.toast("Not enough gems"); }
                        }
                    let w = ui.available_width();
                    if (pbtn(ui, "CHECK", if can { GOLD } else { PANEL_HI }, w).clicked() || go) && can {
                        let (ok, why) = match &rs.qs[qi] {
                            Q::Mc { right, why, .. } | Q::Fill { right, why, .. } => (rs.sel == Some(*right), why.to_string()),
                            Q::Type { answer, why, .. } => (norm_code(&rs.text) == norm_code(answer), if norm_code(&rs.text) == norm_code(answer) { why.to_string() } else { format!("{}  Answer: {}", why, answer) }),
                        };
                        rs.last = Some(qi);
                        rs.queue.remove(0);
                        if ok {
                            rs.right += 1;
                            if let Kind::Lesson(c, mi, l) = rs.kind {
                                let e = self.s.qdone.entry(Self::key(c, mi, l)).or_default();
                                if !e.contains(&qi) { e.push(qi); }
                                self.persist();
                            }
                        }
                        else { rs.mistakes += 1; if !duel { rs.queue.push(qi); game::lose_heart(&mut self.s, now()); self.persist(); } }
                        rs.fb = Some((ok, why));
                    }
                });
            } else { finished = true; }
        });

        egui::CentralPanel::default().frame(egui::Frame::new().fill(BG).inner_margin(16.0)).show(ctx, |ui| {
            floor(ui.painter(), ui.max_rect(), self.s.scan);
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                ui.horizontal(|ui| {
                    let (r, _) = ui.allocate_exact_size(Vec2::new(8.0, 14.0), Sense::hover());
                    ui.painter().rect_filled(r, 0.0, GOLD);
                    ui.label(title(phase, 9.0).color(GOLD));
                });
                ui.label(body(rs.title.clone(), 13.0).color(DIM));
                ui.add_space(12.0);
                if in_slides {
                    let (text, img) = rs.slides[rs.stage];
                    // picture
                    let (r, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 128.0), Sense::hover());
                    let p = ui.painter();
                    p.rect_filled(r.expand(2.0), 0.0, EDGE);
                    p.rect_filled(r, 0.0, Color32::from_rgb(0x0b, 0x12, 0x28));
                    p.rect_filled(Rect::from_min_max(Pos2::new(r.min.x, r.max.y - 26.0), r.max), 0.0, Color32::from_rgb(0x14, 0x20, 0x3a));
                    p.rect_filled(Rect::from_center_size(Pos2::new(r.center().x, r.max.y - 26.0), Vec2::new(84.0, 6.0)), 0.0, Color32::from_black_alpha(110));
                    draw_icon(p, Rect::from_center_size(Pos2::new(r.center().x, r.center().y - 6.0), Vec2::splat(96.0)), img, Color32::WHITE);
                    for i in 0..n_slides {
                        p.rect_filled(Rect::from_min_size(Pos2::new(r.center().x - n_slides as f32 * 8.0 + i as f32 * 16.0, r.max.y - 12.0), Vec2::new(10.0, 4.0)), 0.0, if i == rs.stage { GOLD } else { PANEL_HI });
                    }
                    ui.add_space(12.0);
                    panel(ui, PARCH, EDGE, |ui| {
                        ui.set_width(ui.available_width());
                        ui.label(RichText::new(text).monospace().size(15.0).color(EDGE));
                    });
                } else if let Some(qi) = shown_q {
                    let locked = rs.fb.is_some();
                    let ok_now = matches!(rs.fb, Some((true, _)));
                    match rs.qs[qi].clone() {
                        Q::Mc { q, opts, right, .. } => {
                            ui.label(body(q, 19.0)); ui.add_space(12.0);
                            let struck = if rs.hint_q == Some(qi) { (0..opts.len()).find(|i| *i != right) } else { None };
                            for (i, o) in opts.iter().enumerate() {
                                let lab = if Some(i) == struck { format!("X  {}", o) } else { o.to_string() };
                                option_btn(ui, &lab, rs.sel == Some(i) && Some(i) != struck, locked || Some(i) == struck, &mut rs.sel, i);
                            }
                        }
                        Q::Fill { prompt, code, opts, right, .. } => {
                            ui.label(body(prompt, 17.0));
                            ui.add_space(10.0);
                            let chosen = rs.sel.map(|i| opts[i]);
                            let mut zone = Rect::NOTHING;
                            panel(ui, Color32::from_rgb(0x07, 0x0c, 0x1c), PANEL_HI, |ui| {
                                ui.set_width(ui.available_width());
                                for line in code.lines() {
                                    if let Some((a, b)) = line.split_once("___") {
                                        ui.horizontal(|ui| {
                                            ui.spacing_mut().item_spacing.x = 0.0;
                                            ui.label(RichText::new(a).monospace().size(16.0).color(PARCH));
                                            let tw = chosen.map(|c| c.chars().count() as f32 * 10.0 + 16.0).unwrap_or(0.0).max(56.0);
                                            let (zr, zresp) = ui.allocate_exact_size(Vec2::new(tw, 28.0), Sense::click());
                                            zone = zr;
                                            let hot = ui.ctx().input(|i| i.pointer.interact_pos()).map(|p| zr.expand(10.0).contains(p)).unwrap_or(false) && ui.ctx().dragged_id().is_some();
                                            let pp = ui.painter();
                                            pp.rect_filled(zr, 0.0, if chosen.is_some() { Color32::from_rgb(0x3a, 0x30, 0x08) } else if hot { PANEL_HI } else { EDGE });
                                            pp.rect_stroke(zr, 0.0, Stroke::new(2.0, if hot || chosen.is_some() { GOLD } else { DIM }), egui::StrokeKind::Inside);
                                            pp.text(zr.center(), Align2::CENTER_CENTER, chosen.unwrap_or("?"), FontId::monospace(15.0), if chosen.is_some() { GOLD } else { DIM });
                                            if zresp.clicked() && !locked { rs.sel = None; }
                                            ui.label(RichText::new(b).monospace().size(16.0).color(PARCH));
                                        });
                                    } else {
                                        ui.label(RichText::new(if line.is_empty() { " " } else { line }).monospace().size(16.0).color(PARCH));
                                    }
                                }
                            });
                            ui.add_space(6.0);
                            ui.label(body("Click a piece (or press 1-4), or drag it into the gap.", 12.0).color(DIM));
                            ui.add_space(6.0);
                            let struck = if rs.hint_q == Some(qi) { (0..opts.len()).find(|i| *i != right) } else { None };
                            ui.horizontal_wrapped(|ui| {
                                ui.spacing_mut().item_spacing = Vec2::new(10.0, 10.0);
                                for (i, o) in opts.iter().enumerate() {
                                    if Some(i) == struck { continue; }
                                    let w = 28.0 + o.chars().count() as f32 * 11.0;
                                    let used = rs.sel == Some(i);
                                    let (r, resp) = ui.allocate_exact_size(Vec2::new(w, 38.0), Sense::click_and_drag());
                                    if used { ui.painter().rect_filled(r, 0.0, Color32::from_rgb(0x0a, 0x10, 0x20)); ui.painter().rect_stroke(r, 0.0, Stroke::new(2.0, PANEL_HI), egui::StrokeKind::Inside); }
                                    else { bevel(ui.painter(), r, if resp.hovered() || resp.dragged() { PANEL_HI } else { PANEL }, true); }
                                    ui.painter().text(r.center(), Align2::CENTER_CENTER, *o, FontId::monospace(15.0), if used { PANEL_HI } else { INK });
                                    if locked { continue; }
                                    if resp.clicked() { rs.sel = Some(i); }
                                    if resp.dragged()
                                        && let Some(pos) = ui.ctx().input(|inp| inp.pointer.interact_pos()) {
                                            let lp = ui.ctx().layer_painter(egui::LayerId::new(egui::Order::Tooltip, egui::Id::unique("dragchip")));
                                            let fr = Rect::from_center_size(pos, Vec2::new(w, 38.0));
                                            bevel(&lp, fr, GOLD, true);
                                            lp.text(fr.center(), Align2::CENTER_CENTER, *o, FontId::monospace(15.0), EDGE);
                                        }
                                    if resp.drag_stopped()
                                        && let Some(pos) = ui.ctx().input(|inp| inp.pointer.interact_pos()) && zone.expand(14.0).contains(pos) { rs.sel = Some(i); }
                                }
                            });
                        }
                        Q::Type { prompt, answer, guide, .. } => {
                            ui.label(body(prompt, 17.0));
                            ui.add_space(10.0);
                            if guide {
                                panel(ui, Color32::from_rgb(0x07, 0x0c, 0x1c), PANEL_HI, |ui| {
                                    ui.set_width(ui.available_width());
                                    ui.label(body("COPY THIS", 11.0).color(DIM));
                                    ui.label(RichText::new(answer).monospace().size(15.0).color(PARCH));
                                });
                            }
                            if rs.hint_q == Some(qi) { ui.label(body(format!("Hint: {} characters, starts with '{}'", answer.chars().filter(|c| !c.is_whitespace()).count(), answer.chars().next().unwrap_or(' ')), 14.0).color(GOLD)); ui.add_space(4.0); }
                            let multi = answer.contains('\n');
                            panel(ui, Color32::from_rgb(0x07, 0x0c, 0x1c), if locked { if ok_now { GREEN } else { RED } } else { GOLD }, |ui| {
                                let te = if multi { egui::TextEdit::multiline(&mut rs.text).desired_rows(answer.lines().count() + 1) } else { egui::TextEdit::singleline(&mut rs.text) };
                                let r = ui.add_enabled(!locked, te.code_editor().desired_width(f32::INFINITY).hint_text("type here").text_color(PARCH).frame(egui::Frame::NONE));
                                if !locked && !r.has_focus() && rs.text.is_empty() { r.request_focus(); }
                            });
                            ui.label(body(if multi { "Ctrl+Enter to check" } else { "Enter to check" }, 11.0).color(DIM));
                            if locked && ok_now && !self.course_lang(&rs).is_empty() {
                                let out = interp::run(self.course_lang(&rs), &rs.text);
                                panel(ui, PANEL, EDGE, |ui| { ui.set_width(ui.available_width()); ui.label(title("OUTPUT", 9.0).color(GOLD)); ui.add_space(4.0); out_view(ui, &out); });
                            }
                        }
                    }
                }
            });
        });
        if finished { self.finish(&rs); return None; }
        if keep { Some(rs) } else { None }
    }

    fn course_lang(&self, rs: &RunState) -> &'static str { rs.lang }
}
