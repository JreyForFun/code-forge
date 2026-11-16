use crate::art::*;
use eframe::egui::Color32;

#[derive(Clone)]
pub enum Q {
    Mc { q: &'static str, opts: Vec<&'static str>, right: usize, why: &'static str },
    /// Pick or drag the missing piece into the gap marked ___ in `code`.
    Fill { prompt: &'static str, code: &'static str, opts: Vec<&'static str>, right: usize, why: &'static str },
    /// Type code. `guide` shows the code to copy; without it the learner repeats it from memory.
    Type { prompt: &'static str, answer: &'static str, why: &'static str, guide: bool },
}

#[derive(Clone)]
pub struct Lesson { pub title: &'static str, pub slides: Vec<(&'static str, &'static str)>, pub qs: Vec<Q>, pub project: bool }
pub struct Module { pub name: &'static str, pub desc: &'static str, pub lessons: Vec<Lesson> }
/// A course badge, earned by clearing every lesson in the course.
pub struct Badge { pub name: &'static str, pub art: &'static str, pub desc: &'static str }
pub struct Course {
    pub name: &'static str, pub tag: &'static str, pub slug: &'static str, pub color: Color32, pub desc: &'static str, pub lang: &'static str,
    pub cat: &'static str, pub level: &'static str, pub hours: u32, pub teaches: Vec<&'static str>, pub snippet: &'static str, pub badge: Badge,
    pub modules: Vec<Module>,
}

impl Course {
    pub fn lesson_count(&self) -> usize { self.modules.iter().map(|m| m.lessons.len()).sum() }
    pub fn project_count(&self) -> usize { self.modules.iter().flat_map(|m| &m.lessons).filter(|l| l.project).count() }
    /// Stable progress key for a lesson: `course/lesson-slug`.
    pub fn lesson_id(&self, m: usize, l: usize) -> String { format!("{}/{}", self.slug, slugify(self.modules[m].lessons[l].title)) }
    pub fn badge_id(&self) -> String { format!("badge:{}", self.slug) }
}

pub fn slugify(t: &str) -> String {
    let mut o = String::new();
    for c in t.chars() {
        if c.is_ascii_alphanumeric() { o.push(c.to_ascii_lowercase()); } else if !o.ends_with('-') && !o.is_empty() { o.push('-'); }
    }
    o.trim_end_matches('-').to_string()
}

pub fn mc(q: &'static str, opts: Vec<&'static str>, right: usize, why: &'static str) -> Q { Q::Mc { q, opts, right, why } }

pub fn courses() -> Vec<Course> { crate::content::all() }

// ----- avatars (Pixelrepo art) -----
pub struct Avatar { pub name: &'static str, pub art: &'static str, pub bg: Color32 }
pub const AVATARS: &[Avatar] = &[
    Avatar { name: "Knight", art: "helmet_knight", bg: Color32::from_rgb(0x2a, 0x4a, 0x78) },
    Avatar { name: "Wizard", art: "hat_wizard", bg: Color32::from_rgb(0x3a, 0x2e, 0x6b) },
    Avatar { name: "Ranger", art: "bow", bg: Color32::from_rgb(0x1f, 0x5a, 0x3a) },
    Avatar { name: "Warrior", art: "helmet_bronze", bg: Color32::from_rgb(0x6b, 0x3a, 0x1f) },
    Avatar { name: "Berserker", art: "axe", bg: Color32::from_rgb(0x6b, 0x1f, 0x2e) },
    Avatar { name: "Smith", art: "hammer", bg: Color32::from_rgb(0x4a, 0x4a, 0x5a) },
    Avatar { name: "Alchemist", art: "potion_magic", bg: Color32::from_rgb(0x4a, 0x1f, 0x6b) },
    Avatar { name: "Mystic", art: "mystic_figure", bg: Color32::from_rgb(0x1f, 0x3a, 0x5a) },
];
pub fn avatar(i: u32) -> &'static Avatar { &AVATARS[(i as usize).min(AVATARS.len() - 1)] }

// ----- practice challenges -----
pub struct Coach { pub title: &'static str, pub level: &'static str, pub color: Color32, pub art: &'static str, pub qs: Vec<Q> }
pub fn coaches() -> Vec<Coach> {
    vec![
        Coach { title: "FizzBuzz", level: "Easy", color: ORANGE, art: "scroll_fire", qs: vec![
            mc("Which number prints 'Fizz'?", vec!["5", "3", "7", "10"], 1, "Multiples of 3 print Fizz."),
            mc("15 prints...", vec!["Fizz", "Buzz", "FizzBuzz", "15"], 2, "15 is a multiple of both 3 and 5."),
            mc("Which operator gives the remainder of a division?", vec!["/", "%", "//", "*"], 1, "% is the remainder (modulo) operator."),
        ]},
        Coach { title: "Camel to Snake", level: "Medium", color: TEAL, art: "scroll_lightning", qs: vec![
            mc("Convert 'myHeroName' to snake_case:", vec!["my_hero_name", "myheroname", "My_Hero_Name", "my-hero-name"], 0, "Lowercase words joined by _."),
            mc("Which is camelCase?", vec!["hero_name", "HeroName", "heroName", "hero-name"], 2, "camelCase starts lowercase; each new word starts with a capital."),
        ]},
        Coach { title: "It's a Sign", level: "Hard", color: Color32::from_rgb(0x60, 0x7d, 0x8b), art: "hourglass", qs: vec![
            mc("What does -5 % 3 give in Python?", vec!["-2", "1", "2", "-1"], 1, "Python keeps the divisor's sign, so -5 % 3 is 1."),
            mc("What is abs(-7)?", vec!["-7", "7", "0", "error"], 1, "abs removes the sign."),
        ]},
        Coach { title: "Card Check", level: "Medium", color: GOLD, art: "coins", qs: vec![
            mc("The Luhn check starts doubling from which digit?", vec!["The first", "Second from the right", "The last", "Random"], 1, "Every second digit from the right is doubled."),
            mc("If a doubled digit is over 9, you...", vec!["drop it", "subtract 9", "add 9", "keep it"], 1, "Subtract 9 (the same as adding its two digits)."),
        ]},
        Coach { title: "Pig Latin", level: "Medium", color: Color32::from_rgb(0xe9, 0x1e, 0x63), art: "spellbook", qs: vec![
            mc("'pig' in Pig Latin is...", vec!["igpay", "pigay", "gipay", "pig"], 0, "Move 'p' to the end and add 'ay'."),
            mc("In Python, which gives the first letter of word?", vec!["word[0]", "word[1]", "word.first", "first(word)"], 0, "Indexes start at 0."),
        ]},
    ]
}

// ----- duel opponents (computer characters) -----
pub struct Opponent { pub name: &'static str, pub skill: u32, pub color: Color32, pub art: &'static str }
pub fn opponents() -> Vec<Opponent> {
    vec![
        Opponent { name: "Slime Scout", skill: 1, color: GREEN, art: "potion_magic" },
        Opponent { name: "Goblin Coder", skill: 2, color: ORANGE, art: "axe" },
        Opponent { name: "Dark Compiler", skill: 3, color: PURPLE, art: "mystic_figure" },
    ]
}

const DUEL_POOL: usize = 12;
fn duel_pool() -> [Q; DUEL_POOL] {
    [
        mc("What prints text in Python?", vec!["print()", "echo()", "say()"], 0, "print() writes to the screen."),
        mc("Which HTML tag is a link?", vec!["<a>", "<l>", "<href>"], 0, "<a> is an anchor (link)."),
        mc("2 + 3 * 2 = ?", vec!["10", "8", "12"], 1, "Multiplication happens first."),
        mc("Which CSS property changes text colour?", vec!["font", "color", "paint"], 1, "color sets the text colour."),
        mc("Which keyword makes a JS variable that can change?", vec!["let", "const", "fix"], 0, "let can be reassigned; const cannot."),
        mc("SQL: which word filters rows?", vec!["ORDER", "WHERE", "LIMIT"], 1, "WHERE keeps only matching rows."),
        mc("What is len(\"hero\") in Python?", vec!["3", "4", "5"], 1, "h-e-r-o is 4 characters."),
        mc("Which HTML tag makes the biggest heading?", vec!["<h6>", "<h1>", "<head>"], 1, "<h1> is the top-level heading."),
        mc("In JS, === checks...", vec!["value only", "value and type", "type only"], 1, "=== is strict equality."),
        mc("SQL: which sorts results?", vec!["ORDER BY", "SORT", "GROUP BY"], 0, "ORDER BY sorts rows."),
        mc("Python: range(3) gives...", vec!["1, 2, 3", "0, 1, 2", "0, 1, 2, 3"], 1, "range stops before the end number."),
        mc("CSS: a class selector starts with...", vec!["#", ".", "@"], 1, ".name selects class=\"name\"."),
    ]
}

/// Three questions for a duel, varying by a seed (e.g. the day and opponent).
pub fn duel_questions(seed: u64) -> Vec<Q> {
    let pool = duel_pool();
    let start = (seed.wrapping_mul(7) % DUEL_POOL as u64) as usize;
    (0..3).map(|k| pool[(start + k * 5) % DUEL_POOL].clone()).collect()
}

// ----- weekly league rivals (computer characters) -----
pub struct Rival { pub name: &'static str, pub pace: f32, pub art: &'static str }
pub const RIVALS: &[Rival] = &[
    Rival { name: "Sir Byte", pace: 2.6, art: "helmet_knight" },
    Rival { name: "Hex the Wise", pace: 2.0, art: "hat_wizard" },
    Rival { name: "Loop Ranger", pace: 1.6, art: "bow" },
    Rival { name: "Syntax Smith", pace: 1.25, art: "hammer" },
    Rival { name: "Null Mage", pace: 0.95, art: "potion_magic" },
    Rival { name: "Pixel Squire", pace: 0.7, art: "helmet_bronze" },
    Rival { name: "Bug Hunter", pace: 0.45, art: "axe" },
    Rival { name: "Echo Mystic", pace: 0.25, art: "mystic_figure" },
];

/// A rival's XP this week, growing with the weekday so the table moves during the week.
pub fn rival_xp(r: &Rival, league: u32, weekday: usize) -> u32 {
    let base = 20.0 + league as f32 * 12.0;
    let progress = (weekday as f32 + 1.0) / 7.0;
    ((base * r.pace * progress * 1.6) as u32) / 5 * 5
}

// ----- bundled example programs (no fake authors or stats) -----
pub struct Example { pub title: &'static str, pub lang: &'static str, pub desc: &'static str, pub code: &'static str }
pub const EXAMPLES: &[Example] = &[
    Example { title: "Hello, Pixel Realm", lang: "python", desc: "Variables, print and a for loop", code: "name = \"Hero\"\nprint(\"Welcome,\", name)\nfor i in range(3):\n  print(\"Torch\", i + 1)\n" },
    Example { title: "Level Up Table", lang: "python", desc: "Running totals in a loop", code: "xp = 0\nfor lvl in range(1, 6):\n  xp += lvl * 50\n  print(\"Level\", lvl, \"needs\", xp, \"XP\")\n" },
    Example { title: "Damage Calculator", lang: "javascript", desc: "Arithmetic with let", code: "let attack = 12;\nlet armor = 5;\nlet damage = attack - armor;\nconsole.log(\"Damage:\", damage);\n" },
    Example { title: "Countdown Spell", lang: "javascript", desc: "A counting-down for loop", code: "for (let i = 3; i > 0; i--) {\n  console.log(i);\n}\nconsole.log(\"Cast!\");\n" },
    Example { title: "Tavern Menu", lang: "html", desc: "Headings, a list and a button", code: "<h1>The Pixel Tavern</h1>\n<h2>Menu</h2>\n<ul>\n  <li>Potion of Healing</li>\n  <li>Roast Boar</li>\n</ul>\n<button>Order</button>\n" },
    Example { title: "Royal Banner", lang: "css", desc: "Colours, size and alignment", code: "h1 { color: gold; font-size: 28px; text-align: center; }\np { color: white; background-color: #3a2e6b; }\n" },
    Example { title: "Strongest Heroes", lang: "sql", desc: "Filter, sort and limit", code: "SELECT name, class, hp FROM heroes WHERE hp > 10 ORDER BY hp DESC LIMIT 3;\n" },
    Example { title: "Shop Report", lang: "sql", desc: "Totals per group", code: "SELECT type, COUNT(*), SUM(price) FROM items GROUP BY type;\n" },
];

pub const LANGS: [(&str, &str); 5] = [("python", "PY"), ("javascript", "JS"), ("html", "HTML"), ("css", "CSS"), ("sql", "SQL")];
pub fn starter(lang: &str) -> &'static str {
    match lang {
        "javascript" => "console.log(\"Hello, pixels!\");\n",
        "html" => "<h1>Hello, pixels!</h1>\n<p>My first page.</p>\n",
        "css" => "p { color: gold; font-size: 20px; }\n",
        "sql" => "SELECT * FROM heroes;\n",
        _ => "print(\"Hello, pixels!\")\n",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interp::{run, Kind};

    #[test]
    fn examples_run_clean() {
        for e in EXAMPLES {
            let out = run(e.lang, e.code);
            assert!(!out.iter().any(|(k, _)| *k == Kind::Err), "example '{}' failed: {:?}", e.title, out.iter().map(|x| &x.1).collect::<Vec<_>>());
        }
        for (l, _) in LANGS { assert!(!run(l, starter(l)).iter().any(|(k, _)| *k == Kind::Err), "starter {}", l); }
    }

    #[test]
    fn art_references_exist() {
        for a in AVATARS { assert!(art_exists(a.art), "{}", a.art); }
        for r in RIVALS { assert!(art_exists(r.art), "{}", r.art); }
        for o in opponents() { assert!(art_exists(o.art), "{}", o.art); }
        for c in coaches() { assert!(art_exists(c.art), "{}", c.art); }
        for a in crate::game::ACHIEVEMENTS { assert!(art_exists(a.icon), "{}", a.icon); }
    }

    #[test]
    fn duel_questions_are_valid_and_vary() {
        let a: Vec<String> = duel_questions(1).iter().map(|q| if let Q::Mc { q, .. } = q { q.to_string() } else { String::new() }).collect();
        let b: Vec<String> = duel_questions(2).iter().map(|q| if let Q::Mc { q, .. } = q { q.to_string() } else { String::new() }).collect();
        assert_eq!(a.len(), 3);
        assert_ne!(a, b);
        for q in duel_pool() { if let Q::Mc { opts, right, .. } = q { assert!(right < opts.len()); } }
    }

    #[test]
    fn rivals_scale_with_week() {
        assert!(rival_xp(&RIVALS[0], 0, 6) > rival_xp(&RIVALS[0], 0, 0));
        assert!(rival_xp(&RIVALS[0], 3, 3) > rival_xp(&RIVALS[0], 0, 3));
    }
}
