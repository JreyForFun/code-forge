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
pub struct Lesson { pub title: &'static str, pub slides: Vec<(&'static str, &'static str)>, pub qs: Vec<Q> }
pub struct Module { pub name: &'static str, pub lessons: Vec<Lesson> }
pub struct Course {
    pub name: &'static str, pub tag: &'static str, pub color: Color32, pub desc: &'static str, pub lang: &'static str,
    pub cat: &'static str, pub level: &'static str, pub modules: Vec<Module>,
}

pub fn mc(q: &'static str, opts: Vec<&'static str>, right: usize, why: &'static str) -> Q { Q::Mc { q, opts, right, why } }

pub fn courses() -> Vec<Course> { crate::content::all() }

// ----- community content -----
pub struct Coach { pub title: &'static str, pub level: &'static str, pub color: Color32, pub glyph: &'static str, pub qs: Vec<Q> }
pub fn coaches() -> Vec<Coach> {
    vec![
        Coach { title: "Camel to Snake", level: "Medium", color: TEAL, glyph: "_", qs: vec![mc("Convert 'myHeroName' to snake_case:", vec!["my_hero_name", "myheroname", "My_Hero_Name", "my-hero-name"], 0, "Lowercase words joined by _.")] },
        Coach { title: "It's a Sign", level: "Hard", color: Color32::from_rgb(0x60, 0x7d, 0x8b), glyph: ">", qs: vec![mc("What does -5 % 3 give in Python?", vec!["-2", "1", "2", "-1"], 1, "Python keeps the divisor's sign, so -5 % 3 is 1."), mc("What is abs(-7)?", vec!["-7", "7", "0", "error"], 1, "abs removes the sign.")] },
        Coach { title: "Credit Card Validator", level: "Medium", color: GOLD, glyph: "#", qs: vec![mc("The Luhn check starts doubling from which digit?", vec!["The first", "Second from the right", "The last", "Random"], 1, "Every second digit from the right is doubled.")] },
        Coach { title: "Pig Latin", level: "Medium", color: Color32::from_rgb(0xe9, 0x1e, 0x63), glyph: "P", qs: vec![mc("'pig' in Pig Latin is...", vec!["igpay", "pigay", "gipay", "pig"], 0, "Move 'p' to the end and add 'ay'.")] },
        Coach { title: "FizzBuzz", level: "Easy", color: ORANGE, glyph: "%", qs: vec![mc("Which number prints 'Fizz'?", vec!["5", "3", "7", "10"], 1, "Multiples of 3 print Fizz."), mc("15 prints...", vec!["Fizz", "Buzz", "FizzBuzz", "15"], 2, "Multiple of 3 and 5.")] },
    ]
}

pub struct Opponent { pub name: &'static str, pub skill: u32, pub color: Color32 }
pub fn opponents() -> Vec<Opponent> {
    vec![
        Opponent { name: "Slime Scout", skill: 1, color: GREEN },
        Opponent { name: "Goblin Coder", skill: 2, color: ORANGE },
        Opponent { name: "Dark Compiler", skill: 3, color: PURPLE },
    ]
}
pub fn duel_questions() -> Vec<Q> {
    vec![
        mc("What prints text in Python?", vec!["print()", "echo()", "say()"], 0, "print()"),
        mc("Which HTML tag is a link?", vec!["<a>", "<l>", "<href>"], 0, "<a>"),
        mc("2 + 3 * 2 = ?", vec!["10", "8", "12"], 1, "Multiplication first."),
    ]
}

pub struct Post { pub title: &'static str, pub author: &'static str, pub tag: &'static str, pub votes: i32, pub comments: u32, pub when: &'static str, pub views: u32, pub lang: &'static str, pub code: &'static str }
pub fn posts() -> Vec<Post> {
    vec![
        Post { title: "Hello, Pixel Realm", author: "PixelFox", tag: "python", votes: 30, comments: 33, when: "1 day", views: 746, lang: "python", code: "name = \"Hero\"\nprint(\"Welcome,\", name)\nfor i in range(3):\n  print(\"Torch\", i + 1)\n" },
        Post { title: "Tavern Menu", author: "NesGal", tag: "web", votes: 12, comments: 40, when: "1 day", views: 389, lang: "html", code: "<h1>The Pixel Tavern</h1>\n<h2>Menu</h2>\n<ul>\n  <li>Potion of Healing</li>\n  <li>Roast Boar</li>\n</ul>\n<button>Order</button>\n" },
        Post { title: "Damage Calculator", author: "ByteKnight", tag: "javascript", votes: 14, comments: 34, when: "1 day", views: 295, lang: "javascript", code: "let attack = 12;\nlet armor = 5;\nlet damage = attack - armor;\nconsole.log(\"Damage:\", damage);\n" },
        Post { title: "Level Up Table", author: "LoopLord", tag: "python", votes: 10, comments: 15, when: "7 hrs", views: 318, lang: "python", code: "xp = 0\nfor lvl in range(1, 6):\n  xp += lvl * 50\n  print(\"Level\", lvl, \"needs\", xp, \"XP\")\n" },
        Post { title: "Countdown Spell", author: "Sprite99", tag: "javascript", votes: 7, comments: 13, when: "22 hrs", views: 246, lang: "javascript", code: "for (let i = 3; i > 0; i--) {\n  console.log(i);\n}\nconsole.log(\"Cast!\");\n" },
    ]
}

pub struct Thread { pub title: &'static str, pub tags: Vec<&'static str>, pub votes: i32, pub answers: Vec<&'static str>, pub author: &'static str, pub when: &'static str, pub views: u32 }
pub fn threads() -> Vec<Thread> {
    vec![
        Thread { title: "How do I learn C fast?", tags: vec!["c", "program"], votes: 1, answers: vec!["Write tiny programs every day.", "Read other people's code.", "Start with variables and loops."], author: "QZERT", when: "1 day", views: 130 },
        Thread { title: "My if statement always runs, why?", tags: vec!["javascript", "web"], votes: 0, answers: vec!["Check that you used === and not =.", "Print the condition to see its value."], author: "Alyssia", when: "1 day", views: 56 },
        Thread { title: "What is a Buffer in Node.js?", tags: vec!["nodejs"], votes: 1, answers: vec!["It holds raw binary data."], author: "Monkey", when: "11 hrs", views: 29 },
        Thread { title: "Best way to remember HTML tags?", tags: vec!["html"], votes: 2, answers: vec!["Build a small page each day.", "Flash cards help!"], author: "Davide", when: "13 hrs", views: 31 },
    ]
}
