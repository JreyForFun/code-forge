use std::collections::HashMap;
use std::sync::OnceLock;
use eframe::egui::{self, Color32, FontFamily, FontId, Pos2, Rect, RichText, Sense, Stroke, Vec2};

// ---------- RPG dungeon palette ----------
pub const BG: Color32 = Color32::from_rgb(0x02, 0x06, 0x17);
pub const BG2: Color32 = Color32::from_rgb(0x0b, 0x12, 0x28);
pub const PANEL: Color32 = Color32::from_rgb(0x0f, 0x17, 0x2a);
pub const PANEL_HI: Color32 = Color32::from_rgb(0x1e, 0x29, 0x3b);
pub const EDGE: Color32 = Color32::from_rgb(0x00, 0x03, 0x0c);
pub const PARCH: Color32 = Color32::from_rgb(0xf0, 0xe2, 0xb6);
pub const INK: Color32 = Color32::from_rgb(0xf8, 0xfa, 0xfc);
pub const DIM: Color32 = Color32::from_rgb(0x94, 0xa3, 0xb8);
pub const GREEN: Color32 = Color32::from_rgb(0x4f, 0xc8, 0x3c);
pub const GOLD: Color32 = Color32::from_rgb(0xfa, 0xcc, 0x15);
pub const ORANGE: Color32 = Color32::from_rgb(0xf0, 0x8a, 0x24);
pub const RED: Color32 = Color32::from_rgb(0xe0, 0x36, 0x4c);
pub const BLUE: Color32 = Color32::from_rgb(0x14, 0xad, 0xff);
pub const PURPLE: Color32 = Color32::from_rgb(0xa0, 0x66, 0xe8);
pub const TEAL: Color32 = Color32::from_rgb(0x2f, 0xb9, 0xa5);

pub fn shade(c: Color32, f: f32) -> Color32 {
    let m = |v: u8| ((v as f32 * f).clamp(0.0, 255.0)) as u8;
    Color32::from_rgb(m(c.r()), m(c.g()), m(c.b()))
}

// ---------- fonts ----------
pub fn press() -> FontFamily { FontFamily::Name("press".into()) }
pub fn title(s: impl Into<String>, size: f32) -> RichText {
    RichText::new(s.into()).font(FontId::new(size, press())).color(INK)
}
pub fn body(s: impl Into<String>, size: f32) -> RichText {
    RichText::new(s.into()).font(FontId::new(size, FontFamily::Proportional)).color(INK)
}

pub fn install_fonts(ctx: &egui::Context) {
    let mut f = egui::FontDefinitions::default();
    f.font_data.insert("silk".into(), std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/Silkscreen.ttf"))));
    f.font_data.insert("press".into(), std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/PressStart2P.ttf"))));
    f.families.get_mut(&FontFamily::Proportional).unwrap().insert(0, "silk".into());
    f.families.insert(press(), vec!["press".into()]);
    ctx.set_fonts(f);
}

// ---------- code-drawn sprites ----------
#[derive(Clone, Copy, PartialEq)]
pub enum Spr { Check }

pub fn rows(s: Spr) -> Vec<String> {
    match s {
        Spr::Check => ["............", "..........ww", ".........ww.", "........ww..", "ww.....ww...", ".ww...ww....", "..ww.ww.....", "...www......", "....w.......", "............", "............", "............"]
            .iter().map(|x| x.to_string()).collect(),
    }
}

pub fn draw_spr(p: &egui::Painter, at: Pos2, s: Spr, px: f32, over: &[(char, Color32)]) {
    for (y, row) in rows(s).iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            let col = over.iter().find(|(a, _)| *a == c).map(|(_, c)| *c).or(if c == 'w' { Some(Color32::WHITE) } else { None });
            if let Some(col) = col {
                p.rect_filled(Rect::from_min_size(at + Vec2::new(x as f32 * px, y as f32 * px), Vec2::splat(px + 0.5)), 0.0, col);
            }
        }
    }
}

pub fn spr_size(s: Spr, px: f32) -> Vec2 {
    let r = rows(s);
    Vec2::new(r[0].chars().count() as f32 * px, r.len() as f32 * px)
}

pub fn spr(ui: &mut egui::Ui, s: Spr, px: f32, over: &[(char, Color32)]) {
    let (r, _) = ui.allocate_exact_size(spr_size(s, px), Sense::hover());
    draw_spr(ui.painter(), r.min, s, px, over);
}

// ---------- pixel widgets ----------
pub fn bevel(p: &egui::Painter, r: Rect, fill: Color32, shadow: bool) {
    if shadow {
        p.rect_filled(r.translate(Vec2::new(0.0, 4.0)), 0.0, EDGE);
    }
    p.rect_filled(r.expand(2.0), 0.0, EDGE);
    p.rect_filled(r, 0.0, fill);
    p.rect_filled(Rect::from_min_size(r.min, Vec2::new(r.width(), 3.0)), 0.0, shade(fill, 1.25));
    p.rect_filled(Rect::from_min_size(Pos2::new(r.min.x, r.max.y - 3.0), Vec2::new(r.width(), 3.0)), 0.0, shade(fill, 0.72));
}

pub fn pbtn(ui: &mut egui::Ui, label: &str, color: Color32, width: f32) -> egui::Response {
    let w = if width <= 0.0 { ui.available_width() } else { width };
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(w, 42.0), Sense::click());
    let down = resp.is_pointer_button_down_on();
    let hov = ui.ctx().animate_bool_with_time(resp.id.with("hov"), resp.hovered(), 0.12);
    let fill = shade(color, 1.0 + 0.14 * hov);
    let r = if down { rect.translate(Vec2::new(0.0, 3.0)) } else { rect.translate(Vec2::new(0.0, -2.0)) };
    bevel(ui.painter(), r, fill, !down);
    let txt_col = if color.r() as u32 + color.g() as u32 + color.b() as u32 > 460 { EDGE } else { Color32::WHITE };
    ui.painter().text(r.center(), egui::Align2::CENTER_CENTER, label, FontId::new(11.0, press()), txt_col);
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
}

pub fn panel<R>(ui: &mut egui::Ui, fill: Color32, edge: Color32, f: impl FnOnce(&mut egui::Ui) -> R) -> R {
    let out = egui::Frame::new().fill(fill).stroke(Stroke::new(3.0, edge)).corner_radius(0.0).inner_margin(12.0)
        .shadow(egui::epaint::Shadow { offset: [0, 4], blur: 0, spread: 0, color: EDGE }).show(ui, f);
    let r = out.response.rect;
    ui.painter().rect_filled(Rect::from_min_size(r.min + Vec2::new(3.0, 3.0), Vec2::new(r.width() - 6.0, 2.0)), 0.0, shade(fill, 1.3));
    ui.add_space(6.0);
    out.inner
}

pub fn bar(ui: &mut egui::Ui, frac: f32, color: Color32, h: f32) {
    let (r, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), h), Sense::hover());
    let frac = ui.ctx().animate_value_with_time(egui::Id::unique(("bar", r.min.x as i32, r.min.y as i32 / 8)), frac, 0.25);
    let p = ui.painter();
    p.rect_filled(r.expand(2.0), 0.0, EDGE);
    p.rect_filled(r, 0.0, Color32::from_rgb(0x10, 0x0c, 0x1c));
    let w = (r.width() * frac.clamp(0.0, 1.0)).floor();
    if w > 0.0 {
        let fr = Rect::from_min_size(r.min, Vec2::new(w, h));
        p.rect_filled(fr, 0.0, color);
        p.rect_filled(Rect::from_min_size(fr.min, Vec2::new(w, (h / 3.0).floor())), 0.0, shade(color, 1.35));
    }
    let mut x = r.min.x + 12.0;
    while x < r.max.x {
        p.rect_filled(Rect::from_min_size(Pos2::new(x, r.min.y), Vec2::new(2.0, h)), 0.0, Color32::from_black_alpha(70));
        x += 12.0;
    }
}

// course emblem: pixel shield tile with label and 3 level dots
pub fn emblem(ui: &mut egui::Ui, label: &str, color: Color32, size: f32) {
    let (r, _) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
    draw_emblem(ui.painter(), r, label, color);
}
pub fn draw_emblem(p: &egui::Painter, r: Rect, label: &str, color: Color32) {
    let c = 6.0;
    p.rect_filled(r, 0.0, EDGE);
    p.rect_filled(r.shrink2(Vec2::new(0.0, c)), 0.0, EDGE);
    let inner = r.shrink(3.0);
    p.rect_filled(inner, 0.0, color);
    p.rect_filled(Rect::from_min_size(inner.min, Vec2::new(inner.width(), 4.0)), 0.0, shade(color, 1.3));
    p.rect_filled(Rect::from_min_size(Pos2::new(inner.min.x, inner.max.y - 5.0), Vec2::new(inner.width(), 5.0)), 0.0, shade(color, 0.7));
    for (dx, dy) in [(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (1.0, 1.0)] {
        let cp = Pos2::new(if dx == 0.0 { r.min.x } else { r.max.x - 4.0 }, if dy == 0.0 { r.min.y } else { r.max.y - 4.0 });
        p.rect_filled(Rect::from_min_size(cp, Vec2::splat(4.0)), 0.0, BG);
    }
    let fs = if label.len() > 3 { r.width() * 0.2 } else { r.width() * 0.3 };
    p.text(r.center() - Vec2::new(0.0, r.height() * 0.08), egui::Align2::CENTER_CENTER, label, FontId::new(fs.max(8.0), press()), Color32::WHITE);
    for i in 0..3 {
        let d = r.width() * 0.07;
        let cx = r.center().x + (i as f32 - 1.0) * d * 2.2;
        let cy = r.max.y - r.height() * 0.2;
        p.rect_filled(Rect::from_center_size(Pos2::new(cx, cy), Vec2::splat(d)), 0.0, if i == 0 { Color32::WHITE } else { shade(color, 0.6) });
    }
}

// dungeon-floor background texture
pub fn floor(p: &egui::Painter, r: Rect, scan: bool) {
    p.rect_filled(r, 0.0, BG);
    let t = 24.0;
    let mut y = r.min.y;
    let mut row = 0;
    while y < r.max.y {
        let mut x = r.min.x + if row % 2 == 0 { 0.0 } else { t / 2.0 } - t;
        while x < r.max.x {
            let rr = Rect::from_min_size(Pos2::new(x, y), Vec2::new(t, t)).intersect(r);
            if rr.width() > 0.0 && rr.height() > 0.0 {
                p.rect_filled(Rect::from_min_size(rr.min, Vec2::new(rr.width().min(t - 2.0), 1.0)), 0.0, BG2);
            }
            x += t;
        }
        y += t;
        row += 1;
    }
    if scan {
        let mut y = r.min.y;
        while y < r.max.y {
            p.rect_filled(Rect::from_min_size(Pos2::new(r.min.x, y), Vec2::new(r.width(), 1.0)), 0.0, Color32::from_black_alpha(28));
            y += 3.0;
        }
    }
}

pub fn panel_resp(ui: &mut egui::Ui, fill: Color32, edge: Color32, f: impl FnOnce(&mut egui::Ui)) -> egui::Response {
    let out = egui::Frame::new().fill(fill).stroke(Stroke::new(3.0, edge)).corner_radius(0.0).inner_margin(12.0)
        .shadow(egui::epaint::Shadow { offset: [0, 4], blur: 0, spread: 0, color: EDGE }).show(ui, f);
    let rect = out.response.rect;
    ui.add_space(6.0);
    let r = ui.interact(rect, ui.scope_id().with(("pr", rect.min.x as i32, rect.min.y as i32)), Sense::click());
    r.on_hover_cursor(egui::CursorIcon::PointingHand)
}

// ---------- pixel art from Pixelrepo's free collection ----------
pub static TEX: OnceLock<HashMap<&'static str, (egui::TextureId, Vec2)>> = OnceLock::new();

/// Every embedded picture: Pixelrepo Free collection (credited in Settings and README).
pub const ART: &[(&str, &[u8])] = &[
    ("potion_red", include_bytes!("../assets/art/potion_red.png")),
    ("crystal_purple", include_bytes!("../assets/art/crystal_purple.png")),
    ("crystal_blue", include_bytes!("../assets/art/crystal_blue.png")),
    ("torch", include_bytes!("../assets/art/torch.png")),
    ("chest", include_bytes!("../assets/art/chest.png")),
    ("book", include_bytes!("../assets/art/book.png")),
    ("sword", include_bytes!("../assets/art/sword.png")),
    ("shield", include_bytes!("../assets/art/shield.png")),
    ("helmet", include_bytes!("../assets/art/helmet.png")),
    ("map", include_bytes!("../assets/art/map.png")),
    ("key", include_bytes!("../assets/art/key.png")),
    ("staff", include_bytes!("../assets/art/staff.png")),
    ("cauldron", include_bytes!("../assets/art/cauldron.png")),
    ("scroll_fire", include_bytes!("../assets/art/scroll_fire.png")),
    ("coin_gold", include_bytes!("../assets/art/coin_gold.png")),
    ("gem_green", include_bytes!("../assets/art/gem_green.png")),
    ("orb_red", include_bytes!("../assets/art/orb_red.png")),
    ("axe", include_bytes!("../assets/art/axe.png")),
    ("banner", include_bytes!("../assets/art/banner.png")),
    ("banner_royal", include_bytes!("../assets/art/banner_royal.png")),
    ("bow", include_bytes!("../assets/art/bow.png")),
    ("campfire", include_bytes!("../assets/art/campfire.png")),
    ("chalice", include_bytes!("../assets/art/chalice.png")),
    ("coins", include_bytes!("../assets/art/coins.png")),
    ("console", include_bytes!("../assets/art/console.png")),
    ("cpu", include_bytes!("../assets/art/cpu.png")),
    ("crown", include_bytes!("../assets/art/crown.png")),
    ("cup_silver", include_bytes!("../assets/art/cup_silver.png")),
    ("flame", include_bytes!("../assets/art/flame.png")),
    ("gear", include_bytes!("../assets/art/gear.png")),
    ("hammer", include_bytes!("../assets/art/hammer.png")),
    ("hat_wizard", include_bytes!("../assets/art/hat_wizard.png")),
    ("heart", include_bytes!("../assets/art/heart.png")),
    ("helmet_bronze", include_bytes!("../assets/art/helmet_bronze.png")),
    ("helmet_knight", include_bytes!("../assets/art/helmet_knight.png")),
    ("hourglass", include_bytes!("../assets/art/hourglass.png")),
    ("keyhole", include_bytes!("../assets/art/keyhole.png")),
    ("lantern", include_bytes!("../assets/art/lantern.png")),
    ("letter", include_bytes!("../assets/art/letter.png")),
    ("medallion", include_bytes!("../assets/art/medallion.png")),
    ("money_bag", include_bytes!("../assets/art/money_bag.png")),
    ("mystic_figure", include_bytes!("../assets/art/mystic_figure.png")),
    ("potion_magic", include_bytes!("../assets/art/potion_magic.png")),
    ("scroll_lightning", include_bytes!("../assets/art/scroll_lightning.png")),
    ("spellbook", include_bytes!("../assets/art/spellbook.png")),
    ("amulet", include_bytes!("../assets/art/chest2.png")),
];

#[cfg_attr(not(test), allow(dead_code))]
pub fn art_exists(name: &str) -> bool { ART.iter().any(|(n, _)| *n == name) }

pub fn load_textures(ctx: &egui::Context) -> Vec<egui::TextureHandle> {
    let mut map = HashMap::new();
    let mut handles = vec![];
    for (name, bytes) in ART {
        if let Ok(img) = image::load_from_memory(bytes) {
            let rgba = img.to_rgba8();
            let size = [rgba.width() as usize, rgba.height() as usize];
            let ci = egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());
            let h = ctx.load_texture(*name, ci, egui::TextureOptions::NEAREST);
            map.insert(*name, (h.id(), Vec2::new(size[0] as f32, size[1] as f32)));
            handles.push(h);
        }
    }
    let _ = TEX.set(map);
    handles
}

/// Draw a texture centred in `bx`, scaled by a whole number so pixels stay sharp.
pub fn draw_icon(p: &egui::Painter, bx: Rect, name: &str, tint: Color32) {
    let Some((id, sz)) = TEX.get().and_then(|m| m.get(name)) else { return };
    let k = (bx.width() / sz.x).min(bx.height() / sz.y).floor().max(1.0);
    let r = Rect::from_center_size(bx.center(), *sz * k);
    p.image(*id, r, Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)), tint);
}

pub fn icon(ui: &mut egui::Ui, name: &str, size: f32) {
    let (r, _) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
    draw_icon(ui.painter(), r, name, Color32::WHITE);
}


#[cfg(test)]
mod tests {
    #[test]
    fn all_art_decodes() {
        for (n, b) in super::ART { assert!(image::load_from_memory(b).is_ok(), "{} does not decode", n); }
    }
}
