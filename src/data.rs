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
/// `explain` is the long beginner-friendly explanation ("The Full Scroll"); `summary` lists the key points.
pub struct Lesson { pub title: &'static str, pub slides: Vec<(&'static str, &'static str)>, pub qs: Vec<Q>, pub project: bool, pub explain: &'static str, pub summary: Vec<&'static str> }
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
pub fn fq(prompt: &'static str, code: &'static str, opts: Vec<&'static str>, right: usize, why: &'static str) -> Q { Q::Fill { prompt, code, opts, right, why } }

// ----- quick challenges (Practice tab) -----
pub struct Coach { pub title: &'static str, pub level: &'static str, pub lang: &'static str, pub desc: &'static str, pub color: Color32, pub art: &'static str, pub qs: Vec<Q> }
pub fn coaches() -> Vec<Coach> {
    vec![
        Coach { title: "FizzBuzz", level: "Easy", lang: "Python", desc: "The classic counting puzzle with % and elif.", color: ORANGE, art: "scroll_fire", qs: vec![
            mc("Which number prints 'Fizz'?", vec!["5", "3", "7", "10"], 1, "Multiples of 3 print Fizz."),
            mc("15 prints...", vec!["Fizz", "Buzz", "FizzBuzz", "15"], 2, "15 is a multiple of both 3 and 5."),
            mc("Which operator gives the remainder of a division?", vec!["/", "%", "//", "*"], 1, "% is the remainder (modulo) operator."),
            fq("Test for a multiple of 5.", "if n % 5 ___ 0:\n    print(\"Buzz\")", vec!["==", "=", "!="], 0, "No remainder means a multiple."),
            mc("Why check n % 15 first?", vec!["Otherwise 15 prints Fizz", "It is faster", "15 is prime"], 0, "The first matching branch wins."),
        ]},
        Coach { title: "Tag Detective", level: "Easy", lang: "HTML", desc: "Spot the broken tags before the page falls apart.", color: Color32::from_rgb(0xe8, 0x5a, 0x1e), art: "shield", qs: vec![
            mc("Which line is broken?", vec!["<p>Hi</p>", "<h2>Quest<h2>", "<li>Rope</li>"], 1, "The closing tag needs a slash: </h2>."),
            mc("Which element has no closing tag?", vec!["<img>", "<p>", "<a>"], 0, "<img> is an empty element."),
            fq("Close the link.", "<a href=\"map.html\">Map___", vec!["</a>", "<a/>", "</link>"], 0, "Anchors close with </a>."),
            mc("Which nesting is correct?", vec!["<b><i>Run</b></i>", "<b><i>Run</i></b>", "<i><b>Run</i>"], 1, "Close the inner element first."),
        ]},
        Coach { title: "Camel to Snake", level: "Medium", lang: "Any", desc: "Name things the way each language likes.", color: TEAL, art: "scroll_lightning", qs: vec![
            mc("Convert 'myHeroName' to snake_case:", vec!["my_hero_name", "myheroname", "My_Hero_Name", "my-hero-name"], 0, "Lowercase words joined by _."),
            mc("Which is camelCase?", vec!["hero_name", "HeroName", "heroName", "hero-name"], 2, "camelCase starts lowercase; each new word starts with a capital."),
            mc("Python variables usually use...", vec!["snake_case", "camelCase", "kebab-case"], 0, "Python style is snake_case."),
            mc("CSS class names usually use...", vec!["kebab-case", "snake_case", "PascalCase"], 0, "quest-card is typical CSS."),
            mc("Python class names use...", vec!["PascalCase", "snake_case", "UPPERCASE"], 0, "class BankAccount:"),
        ]},
        Coach { title: "Selector Sniper", level: "Medium", lang: "CSS", desc: "Hit exactly the elements you mean.", color: Color32::from_rgb(0x26, 0x6d, 0xd3), art: "bow", qs: vec![
            mc("Which selects class=\"rare\"?", vec![".rare", "#rare", "rare"], 0, "Classes start with a dot."),
            mc("What does nav a select?", vec!["Links inside nav", "nav and a", "A nav inside a link"], 0, "A space means inside."),
            mc("Which is most specific?", vec!["p", ".note", "#intro"], 2, "id beats class beats type."),
            fq("Style links on hover.", "a___ { color: gold; }", vec![":hover", ".hover", "#hover"], 0, "Pseudo-classes use a colon."),
        ]},
        Coach { title: "Loop Labyrinth", level: "Medium", lang: "JavaScript", desc: "Count the turns before you get lost.", color: GOLD, art: "map", qs: vec![
            mc("for (let i = 0; i < 4; i++) runs how many times?", vec!["3", "4", "5"], 1, "0, 1, 2, 3."),
            mc("What does break do?", vec!["Leaves the loop", "Skips one round", "Restarts the loop"], 0, "break exits immediately."),
            mc("A while loop never ends when...", vec!["its condition never becomes false", "it uses console.log", "it has braces"], 0, "Something must change."),
            fq("Count down.", "for (let i = 3; i > 0; ___) { }", vec!["i--", "i++", "i = 0"], 0, "i-- moves towards 0."),
        ]},
        Coach { title: "Truth Tables", level: "Medium", lang: "Any", desc: "and, or, not: who survives the gate?", color: PURPLE, art: "keyhole", qs: vec![
            mc("True and False is...", vec!["True", "False"], 1, "and needs both."),
            mc("False or True is...", vec!["True", "False"], 0, "or needs one."),
            mc("not (5 > 3) is...", vec!["True", "False"], 1, "5 > 3 is True; not flips it."),
            mc("Which checks 13 to 19?", vec!["age >= 13 and age <= 19", "age >= 13 or age <= 19", "13 < age > 19"], 0, "Both limits must hold."),
        ]},
        Coach { title: "Query Quest", level: "Medium", lang: "SQL", desc: "Ask the vault the right question.", color: Color32::from_rgb(0x1f, 0x9d, 0x8a), art: "book", qs: vec![
            mc("Which keyword filters rows?", vec!["WHERE", "ORDER BY", "LIMIT"], 0, "WHERE keeps matching rows."),
            mc("Names starting with D:", vec!["LIKE 'D%'", "LIKE '%D'", "= 'D'"], 0, "% after the letter."),
            mc("Totals per class need...", vec!["GROUP BY class", "ORDER BY class", "DISTINCT class"], 0, "GROUP BY makes one row per group."),
            mc("Find missing values with...", vec!["IS NULL", "= NULL", "== NULL"], 0, "Comparisons with NULL are never true."),
            fq("Strongest first.", "SELECT name FROM heroes ORDER BY hp ___;", vec!["DESC", "ASC", "UP"], 0, "DESC is largest first."),
        ]},
        Coach { title: "It's a Sign", level: "Hard", lang: "Python", desc: "Tricky maths with negatives and rounding.", color: Color32::from_rgb(0x60, 0x7d, 0x8b), art: "hourglass", qs: vec![
            mc("What does -5 % 3 give in Python?", vec!["-2", "1", "2", "-1"], 1, "Python keeps the divisor's sign, so -5 % 3 is 1."),
            mc("What is abs(-7)?", vec!["-7", "7", "0", "error"], 1, "abs removes the sign."),
            mc("7 // 2 is...", vec!["3.5", "3", "4"], 1, "// drops the decimals."),
            mc("round(2.675, 1) shows about...", vec!["2.7", "2.6", "3"], 0, "Rounded to one decimal."),
        ]},
        Coach { title: "Card Check", level: "Hard", lang: "Any", desc: "The Luhn rule that guards card numbers.", color: Color32::from_rgb(0xc8, 0x9b, 0x2a), art: "coins", qs: vec![
            mc("The Luhn check starts doubling from which digit?", vec!["The first", "Second from the right", "The last", "Random"], 1, "Every second digit from the right is doubled."),
            mc("If a doubled digit is over 9, you...", vec!["drop it", "subtract 9", "add 9", "keep it"], 1, "Subtract 9 (the same as adding its two digits)."),
            mc("A number passes when the total...", vec!["ends in 0", "is even", "is over 50"], 0, "Total % 10 == 0."),
        ]},
        Coach { title: "Pig Latin", level: "Hard", lang: "Python", desc: "Slice and glue words into a secret code.", color: Color32::from_rgb(0xe9, 0x1e, 0x63), art: "spellbook", qs: vec![
            mc("'pig' in Pig Latin is...", vec!["igpay", "pigay", "gipay", "pig"], 0, "Move 'p' to the end and add 'ay'."),
            mc("In Python, which gives the first letter of word?", vec!["word[0]", "word[1]", "word.first", "first(word)"], 0, "Indexes start at 0."),
            mc("word[1:] gives...", vec!["everything after the first letter", "the second letter only", "the last letter"], 0, "A slice from index 1 to the end."),
            fq("Build the Pig Latin word.", "new = word[1:] + word[0] + ___", vec!["\"ay\"", "ay", "'a' 'y'"], 0, "Add the text \"ay\"."),
        ]},
    ]
}

// ----- duel opponents (computer characters) -----
pub struct Opponent { pub name: &'static str, pub title: &'static str, pub skill: u32, pub color: Color32, pub art: &'static str }
pub fn opponents() -> Vec<Opponent> {
    vec![
        Opponent { name: "Slime Scout", title: "Wobbly beginner", skill: 2, color: GREEN, art: "potion_magic" },
        Opponent { name: "Goblin Coder", title: "Sneaky and quick", skill: 3, color: ORANGE, art: "axe" },
        Opponent { name: "Dark Compiler", title: "Rarely wrong", skill: 4, color: PURPLE, art: "mystic_figure" },
        Opponent { name: "Bug Dragon", title: "Only perfection wins", skill: 5, color: RED, art: "flame" },
    ]
}

/// Questions per duel.
pub const DUEL_LEN: usize = 5;
const DUEL_POOL: usize = 30;
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
        mc("Python: which starts a function?", vec!["def", "function", "fn"], 0, "def name():"),
        mc("JS: [10, 20, 30][1] is...", vec!["10", "20", "30"], 1, "Indexes start at 0."),
        mc("CSS: which centres a flex row's items horizontally?", vec!["justify-content: center", "align-items: center", "text-align: left"], 0, "justify-content works on the main axis."),
        mc("HTML: where does <title> go?", vec!["<head>", "<body>", "<footer>"], 0, "Page information lives in the head."),
        mc("SQL: COUNT(*) gives...", vec!["the number of rows", "the sum", "the first row"], 0, "COUNT(*) counts rows."),
        mc("Python: [1, 2, 3][-1] is...", vec!["1", "3", "error"], 1, "-1 is the last item."),
        mc("JS: typeof \"7\" is...", vec!["number", "string", "boolean"], 1, "Anything in quotes is a string."),
        mc("CSS: margin is space...", vec!["outside the border", "inside the border", "between letters"], 0, "Padding is inside; margin is outside."),
        mc("HTML: which input hides what you type?", vec!["type=\"password\"", "type=\"hidden\"", "type=\"secret\""], 0, "Password fields show dots."),
        mc("Python: \"Hi\" * 3 is...", vec!["HiHiHi", "Hi3", "error"], 0, "* repeats a string."),
        mc("SQL: which keeps every row of the left table?", vec!["LEFT JOIN", "INNER JOIN", "WHERE"], 0, "LEFT JOIN keeps unmatched left rows."),
        mc("JS: which adds to the end of an array?", vec!["push", "pop", "shift"], 0, "push adds to the end."),
        mc("Python: which loops over each item?", vec!["for item in bag:", "for (item of bag)", "loop bag"], 0, "for ... in ...:"),
        mc("CSS: position: absolute is placed relative to...", vec!["the nearest positioned ancestor", "the next sibling", "the mouse"], 0, "Give the parent position: relative."),
        mc("HTML: alt text is for...", vec!["describing images", "making links", "bold text"], 0, "Screen readers read alt."),
        mc("Python: 10 // 4 is...", vec!["2.5", "2", "3"], 1, "// drops the decimals."),
        mc("JS: what does return do?", vec!["Sends a value back and ends the function", "Prints a value", "Restarts the loop"], 0, "return ends the function with a value."),
        mc("SQL: which renames a column in the result?", vec!["AS", "RENAME", "TO"], 0, "column AS new_name."),
    ]
}

/// The questions for a duel, varying by a seed (e.g. the day and opponent).
pub fn duel_questions(seed: u64) -> Vec<Q> {
    let pool = duel_pool();
    let start = (seed.wrapping_mul(7) % DUEL_POOL as u64) as usize;
    (0..DUEL_LEN).map(|k| pool[(start + k * 7) % DUEL_POOL].clone()).collect()
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
        assert_eq!(a.len(), DUEL_LEN);
        let set: std::collections::HashSet<_> = a.iter().collect();
        assert_eq!(set.len(), DUEL_LEN, "duel questions repeat");
        assert_ne!(a, b);
        for q in duel_pool() { if let Q::Mc { opts, right, .. } = q { assert!(right < opts.len()); } }
        for c in coaches() { assert!(c.qs.len() >= 3, "{} needs 3+ questions", c.title); for q in &c.qs { match q { Q::Mc { opts, right, .. } | Q::Fill { opts, right, .. } => assert!(*right < opts.len(), "{}", c.title), _ => {} } } }
    }

    #[test]
    fn rivals_scale_with_week() {
        assert!(rival_xp(&RIVALS[0], 0, 6) > rival_xp(&RIVALS[0], 0, 0));
        assert!(rival_xp(&RIVALS[0], 3, 3) > rival_xp(&RIVALS[0], 0, 3));
    }
}
