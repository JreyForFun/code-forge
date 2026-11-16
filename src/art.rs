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

// ---------- sprites ----------
fn pal(c: char) -> Option<Color32> {
    Some(match c {
        'k' => Color32::from_rgb(0x12, 0x0d, 0x1c),
        'r' => RED,
        'R' => Color32::from_rgb(0xa1, 0x1f, 0x3a),
        'p' => Color32::from_rgb(0xff, 0x9a, 0xa8),
        'g' => GREEN,
        'G' => Color32::from_rgb(0x2e, 0x8a, 0x2a),
        'y' => GOLD,
        'o' => ORANGE,
        'w' => Color32::WHITE,
        'b' => BLUE,
        'B' => Color32::from_rgb(0x23, 0x58, 0xb0),
        'l' => Color32::from_rgb(0x9f, 0xd0, 0xff),
        'v' => PURPLE,
        'V' => Color32::from_rgb(0x6c, 0x3f, 0xb0),
        's' => Color32::from_rgb(0xa8, 0xaa, 0xb8),
        'S' => Color32::from_rgb(0x6a, 0x6c, 0x80),
        'n' => Color32::from_rgb(0x8a, 0x5a, 0x32),
        'N' => Color32::from_rgb(0x2a, 0x1a, 0x2e),
        't' => Color32::from_rgb(0xf1, 0xc9, 0xa0),
        _ => return None,
    })
}

#[derive(Clone, Copy, PartialEq)]
pub enum Spr { Heart, Gem, Bolt, Trophy, Lock, Check, Play, Star, Book, Bell, Hero, People, Chat, Eye, Up }

fn mirror(half: &[&str], sub: &[(char, char)]) -> Vec<String> {
    half.iter().map(|r| {
        let right: String = r.chars().rev().map(|c| sub.iter().find(|(a, _)| *a == c).map(|(_, b)| *b).unwrap_or(c)).collect();
        format!("{}{}", r, right)
    }).collect()
}

pub fn rows(s: Spr) -> Vec<String> {
    let full = |v: &[&str]| v.iter().map(|x| x.to_string()).collect::<Vec<_>>();
    match s {
        Spr::Heart => mirror(&["......", ".kkk..", "krrrk.", "krprrr", "krrrrr", "krrrrr", ".krrrR", "..krrR", "...krR", "....kR", ".....k", "......"], &[('p', 'r')]),
        Spr::Gem => mirror(&["......", "...kkk", ".kklll", "klwlvv", "kllvvv", "kvvvvv", ".kvvvV", "..kvvV", "...kvV", "....kV", ".....k", "......"], &[('w', 'l')]),
        Spr::Trophy => mirror(&["......", ".kkkkk", "kkywyy", "kykyyy", "kykyyy", ".kkkyy", "...kyy", "....ky", "....ko", "..koyy", ".knnnn", ".kkkkk"], &[('w', 'y')]),
        Spr::Lock => mirror(&["......", "..kkkk", ".kss..", ".ks...", ".ks...", "kkkkkk", "kaaaaa", "kaaaaa", "kaaakk", "kaaaka", "kkkkkk", "......"], &[]),
        Spr::Star => mirror(&["......", ".....y", "....yy", "....yy", "yyyyyy", ".yyyyy", "..yyyy", "..yyyy", ".yyy..", ".yy...", ".y....", "......"], &[]),
        Spr::Book => mirror(&["......", ".kkkkk", "kwwwwk", "kwSSwk", "kwwwwk", "kwSSwk", "kwwwwk", ".kkkkk", ".nnnnn", "..kkkk", "......", "......"], &[]),
        Spr::Bell => mirror(&["......", ".....k", "..kkyy", ".kyyyy", ".kyyyy", ".kyyyy", ".kyyyy", "kkkkkk", ".....k", "......", "......", "......"], &[]),
        Spr::Chat => mirror(&["......", ".kkkkk", "kwwwww", "kwwwww", "kwSwww", "kwwwwS", "kwwwww", ".kkkkk", "..kw..", "...k..", "......", "......"], &[]),
        Spr::Eye => mirror(&["......", "......", "...kkk", ".kkwww", "kwwkbb", "kwwkbb", ".kkwww", "...kkk", "......", "......", "......", "......"], &[]),
        Spr::Up => mirror(&["......", ".....w", "....ww", "...www", "..wwww", ".wwwww", "...www", "...www", "...www", "......", "......", "......"], &[]),
        Spr::Bolt => full(&[
            "............", "...kkkkkk...", "..kaaaaaak..", ".kaaaaawwak.", "kaaaaawwaaak", "kaaaawwwwaak",
            "kaaawwwwwaak", "kaaaaaawwaak", ".kaaaawwaak.", ".kaaawwaaak.", "..kaawaaak..", "...kkkkkk...",
        ]),
        Spr::Check => full(&[
            "............", "..........ww", ".........ww.", "........ww..", "ww.....ww...", ".ww...ww....", "..ww.ww.....", "...www......", "....w.......", "............", "............", "............",
        ]),
        Spr::Play => full(&[
            "............", "............", ".ww.........", ".wwww.......", ".wwwwww.....", ".wwwwwwww...", ".wwwwww.....", ".wwww.......", ".ww.........", "............", "............", "............",
        ]),
        Spr::People => mirror(&["......", "...kk.", "..kttk", "..kttk", "...kk.", ".kkkkk", "kbbbbb", "kbbbbb", "kkkkkk", "......", "......", "......"], &[]),
        Spr::Hero => mirror(&[
            "........", "...kkkkk", "..kNNNNN", ".kNNNNNN", ".kNNNNNN", ".kNNtttt", ".kNttttt", ".kttkwtt",
            ".kttktyt", ".ktttttt", "..kttttt", "...kkttt", "..kbbbbb", ".kbbbbbb", ".kbbbBBB", ".kkkkkkk",
        ], &[('y', 't'), ('w', 't')]),
    }
}

pub fn draw_spr(p: &egui::Painter, at: Pos2, s: Spr, px: f32, over: &[(char, Color32)]) {
    let tex = match s { Spr::Heart => Some("potion_red"), Spr::Gem => Some("crystal_purple"), Spr::Bolt => Some("torch"), Spr::Trophy => Some("chest"), Spr::Book => Some("book"), Spr::Star => Some("crystal_blue"), Spr::Bell => Some("scroll_fire"), Spr::People => Some("helmet"), Spr::Lock => Some("key"), Spr::Hero => Some("helmet"), _ => None };
    if let Some(n) = tex {
        if TEX.get().is_some() {
            let off = |c: char| over.iter().find(|(a, _)| *a == c).map(|(_, v)| *v);
            let grey = |v: Color32| v == Color32::GRAY || v == Color32::from_rgb(0x6b, 0x66, 0x88) || v == Color32::from_rgb(0x55, 0x4a, 0x70);
            let dull = match s { Spr::Heart => off('r').map(grey).unwrap_or(false), Spr::Bolt => off('a').map(grey).unwrap_or(false), _ => false };
            let tint = if s == Spr::Lock { Color32::from_gray(110) } else if dull { Color32::from_gray(70) } else { Color32::WHITE };
            draw_icon(p, Rect::from_min_size(at, spr_size(s, px)), n, tint);
            return;
        }
    }
    for (y, row) in rows(s).iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            let col = over.iter().find(|(a, _)| *a == c).map(|(_, c)| *c).or_else(|| pal(c));
            if let Some(col) = col {
                let r = Rect::from_min_size(at + Vec2::new(x as f32 * px, y as f32 * px), Vec2::splat(px + 0.5));
                p.rect_filled(r, 0.0, col);
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
    let frac = ui.ctx().animate_value_with_time(egui::Id::new(("bar", r.min.x as i32, r.min.y as i32 / 8)), frac, 0.25);
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
    let r = ui.interact(rect, ui.id().with(("pr", rect.min.x as i32, rect.min.y as i32)), Sense::click());
    r.on_hover_cursor(egui::CursorIcon::PointingHand)
}

// ---------- pixel art from Pixelrepo's free collection ----------
pub static TEX: OnceLock<HashMap<&'static str, (egui::TextureId, Vec2)>> = OnceLock::new();

pub fn load_textures(ctx: &egui::Context) -> Vec<egui::TextureHandle> {
    let files: [(&'static str, &'static [u8]); 18] = [
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
        ("amulet", include_bytes!("../assets/art/chest2.png")),
    ];
    let mut map = HashMap::new();
    let mut handles = vec![];
    for (name, bytes) in files {
        if let Ok(img) = image::load_from_memory(bytes) {
            let rgba = img.to_rgba8();
            let size = [rgba.width() as usize, rgba.height() as usize];
            let ci = egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());
            let h = ctx.load_texture(name, ci, egui::TextureOptions::NEAREST);
            map.insert(name, (h.id(), Vec2::new(size[0] as f32, size[1] as f32)));
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

