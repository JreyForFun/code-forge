// Five complete courses. Every lesson follows the same three steps:
//   1. LEARN    - short text with a picture
//   2. COMPLETE - pick or drag the missing code into the gap
//   3. CODE     - type the code (first copy it, then repeat it from memory)
// Chapter names for Python and HTML follow the public outlines on codedex.io; all lesson text is written for this app.
use crate::data::*;
use eframe::egui::Color32;

type S = (&'static str, &'static str);

fn fill(prompt: &'static str, code: &'static str, opts: &[&'static str], right: usize, why: &'static str) -> Q {
    Q::Fill { prompt, code, opts: opts.to_vec(), right, why }
}
fn copy(prompt: &'static str, answer: &'static str, why: &'static str) -> Q { Q::Type { prompt, answer, why, guide: true } }
fn recall(prompt: &'static str, answer: &'static str, why: &'static str) -> Q { Q::Type { prompt, answer, why, guide: false } }
fn l(title: &'static str, slides: &[S], qs: Vec<Q>) -> Lesson { Lesson { title, slides: slides.to_vec(), qs } }
fn m(name: &'static str, lessons: Vec<Lesson>) -> Module { Module { name, lessons } }

pub fn all() -> Vec<Course> { vec![html(), css(), javascript(), python(), sql()] }

fn html() -> Course {
    Course { name: "HTML", tag: "HTML", color: Color32::from_rgb(0xe8, 0x5a, 0x1e), lang: "html", cat: "Web", level: "Beginner", desc: "Create your first website with HTML, the building blocks of the web", modules: vec![
        m("Elements", vec![
            l("Tags and Elements", &[
                (r#"HTML is the language that gives a web page its structure.

Think of it as the stone walls of a castle: text, pictures and links are all placed inside them."#, "shield"),
                (r#"HTML is built from ELEMENTS. Most have an opening tag and a closing tag around the content:

<p>Hello, hero!</p>"#, "scroll_fire"),
            ], vec![
                fill("Finish the closing tag.", "<p>Hello</___>", &["p", "/p", "h1"], 0, "The closing tag repeats the name after the slash: </p>."),
                fill("Which tag makes a paragraph?", "___Hello</p>", &["<p>", "<h1>", "<a>"], 0, "p stands for paragraph."),
                copy("Type this element.", "<p>Hello</p>", "Opening tag, content, closing tag."),
                recall("From memory: a paragraph that says Hi.", "<p>Hi</p>", "<p>Hi</p>"),
            ]),
            l("Headings and Paragraphs", &[
                (r#"Headings are titles. <h1> is the biggest and <h6> the smallest.

Use one <h1> per page: it is the main title."#, "book"),
                (r#"<h1>Quest Log</h1>
<h2>Chapter 1</h2>
<p>You wake in a tavern.</p>"#, "map"),
            ], vec![
                fill("Make the biggest heading.", "<___>Quest Log</h1>", &["h1", "h6", "p"], 0, "h1 is the largest heading."),
                fill("Close the second-level heading.", "<h2>Chapter 1___", &["</h2>", "</h1>", "<h2>"], 0, "Names must match: <h2> closes with </h2>."),
                copy("Type this heading.", "<h1>Quest Log</h1>", "h1 opens and closes."),
                recall("From memory: a level-2 heading that says Chapter 1.", "<h2>Chapter 1</h2>", "<h2>Chapter 1</h2>"),
            ]),
        ]),
        m("Lists and Links", vec![
            l("Lists", &[
                (r#"<ul> makes a bullet list and <ol> a numbered list.

Every item inside is an <li>."#, "chest"),
                (r#"<ul>
  <li>Sword</li>
  <li>Shield</li>
</ul>"#, "sword"),
            ], vec![
                fill("Add one list item.", "<ul>\n  ___Sword</li>\n</ul>", &["<li>", "<ul>", "<ol>"], 0, "Each item is an <li>."),
                fill("Make a numbered list.", "<___>\n  <li>Sword</li>\n</ol>", &["ol", "ul", "li"], 0, "ol = ordered list."),
                copy("Type this list.", "<ul><li>Sword</li></ul>", "ul wraps li items."),
                recall("From memory: a bullet list with one item, Map.", "<ul><li>Map</li></ul>", "<ul><li>Map</li></ul>"),
            ]),
            l("Links and Images", &[
                (r#"Links use <a> with an href attribute that holds the address:

<a href="https://example.com">Visit</a>"#, "map"),
                (r#"Images use <img>. It has no closing tag, and alt describes the picture:

<img src="hero.png" alt="A hero">"#, "helmet"),
            ], vec![
                fill("Where does the link go?", r#"<a ___="https://example.com">Visit</a>"#, &["href", "src", "alt"], 0, "href = hypertext reference."),
                fill("Which attribute loads the picture file?", r#"<img ___="hero.png" alt="A hero">"#, &["src", "href", "text"], 0, "src = source."),
                copy("Type this link.", r#"<a href="https://example.com">Visit</a>"#, "Attribute value in quotes."),
                recall("From memory: an image hero.png with alt text Hero.", r#"<img src="hero.png" alt="Hero">"#, r#"<img src="hero.png" alt="Hero">"#),
            ]),
        ]),
        m("Structure", vec![
            l("Page Skeleton", &[
                (r#"Every page has a skeleton. <head> holds info about the page; <body> holds everything you see."#, "shield"),
                (r#"<!DOCTYPE html>
<html>
  <head><title>Hi</title></head>
  <body>Hello</body>
</html>"#, "book"),
            ], vec![
                fill("Where does visible content go?", "<html>\n  <___>Hello</body>\n</html>", &["body", "head", "title"], 0, "Visible content lives in <body>."),
                fill("Which tag names the browser tab?", "<head><___>Hi</title></head>", &["title", "h1", "meta"], 0, "<title> inside <head>."),
                copy("Type this body.", "<body>Hello</body>", "body wraps what you see."),
                recall("From memory: a head containing a title, Hi.", "<head><title>Hi</title></head>", "<head><title>Hi</title></head>"),
            ]),
            l("Attributes", &[
                (r#"Attributes add extra info to an element. They sit inside the opening tag as name="value"."#, "key"),
                (r#"<p id="intro" class="big">Welcome</p>

An id must be unique on the page. A class can be shared."#, "scroll_fire"),
            ], vec![
                fill("Give the paragraph a unique name.", r#"<p ___="intro">Welcome</p>"#, &["id", "href", "src"], 0, "id must be unique."),
                fill("Add a shared style name.", r#"<p ___="big">Welcome</p>"#, &["class", "id", "alt"], 0, "class can repeat across elements."),
                copy("Type this paragraph.", r#"<p id="intro">Welcome</p>"#, "Attributes live in the opening tag."),
                recall("From memory: a paragraph with class big that says Hi.", r#"<p class="big">Hi</p>"#, r#"<p class="big">Hi</p>"#),
            ]),
        ]),
        m("Forms and Meaning", vec![
            l("Inputs and Buttons", &[
                (r#"Forms collect input. <input> makes a box and <button> sends it."#, "staff"),
                (r#"<input type="text">
<button>Go</button>

The type attribute picks the kind of input."#, "key"),
            ], vec![
                fill("Make a text box.", r#"<input ___="text">"#, &["type", "href", "id"], 0, "type picks the input kind."),
                fill("Make a clickable button.", "<___>Go</button>", &["button", "click", "press"], 0, "Use <button>."),
                copy("Type this input.", r#"<input type="text">"#, "No closing tag needed."),
                recall("From memory: a button that says Go.", "<button>Go</button>", "<button>Go</button>"),
            ]),
            l("Semantic Tags", &[
                (r#"Semantic tags describe their purpose: <header>, <nav>, <main> and <footer>.

They help search engines and screen readers."#, "shield"),
                (r#"<header>Top</header>
<main>Story</main>
<footer>End</footer>"#, "chest"),
            ], vec![
                fill("Mark the bottom of the page.", "<___>End</footer>", &["footer", "bottom", "end"], 0, "<footer> holds the page bottom."),
                fill("Mark the main content.", "<___>Story</main>", &["main", "body", "div"], 0, "<main> holds the main content."),
                copy("Type this footer.", "<footer>End</footer>", "footer opens and closes."),
                recall("From memory: a header that says Top.", "<header>Top</header>", "<header>Top</header>"),
            ]),
        ]),
    ]}
}

fn css() -> Course {
    Course { name: "CSS", tag: "CSS", color: Color32::from_rgb(0x23, 0x7b, 0xe0), lang: "css", cat: "Web", level: "Beginner", desc: "Style web pages with colors, fonts, spacing and layout", modules: vec![
        m("Selectors", vec![
            l("Your First Rule", &[
                (r#"CSS styles HTML. A rule has a selector (what to style) and declarations (how).

p {
  color: red;
}"#, "staff"),
                (r#"Each declaration is property: value; The braces { } wrap them."#, "scroll_fire"),
            ], vec![
                fill("Style every paragraph.", "___ {\n  color: red;\n}", &["p", "<p>", "red"], 0, "The selector is the tag name, no brackets."),
                fill("Finish the declaration.", "p {\n  color___ red;\n}", &[":", "=", ";"], 0, "A property and its value are separated by a colon."),
                copy("Type this rule.", "p { color: red; }", "selector { property: value; }"),
                recall("From memory: make every h1 blue.", "h1 { color: blue; }", "h1 { color: blue; }"),
            ]),
            l("Classes and IDs", &[
                (r#"A class selector starts with a dot and matches every element with that class.

.big { font-size: 24px; }"#, "key"),
                (r#"An id selector starts with # and matches ONE element.

#intro { color: gold; }"#, "shield"),
            ], vec![
                fill("Select the class big.", "___big { font-size: 24px; }", &[".", "#", "*"], 0, "Classes use a dot."),
                fill("Select the id intro.", "___intro { color: gold; }", &["#", ".", "&"], 0, "IDs use a hash."),
                copy("Type this rule.", ".big { font-size: 24px; }", "Dot, class name, braces."),
                recall("From memory: give #intro the color gold.", "#intro { color: gold; }", "#intro { color: gold; }"),
            ]),
        ]),
        m("Color and Text", vec![
            l("Colors", &[
                (r#"CSS has color names (red), hex codes (#facc15) and rgb() values.

color sets text color; background-color sets the background."#, "crystal_purple"),
                (r#"body {
  background-color: #020617;
  color: #f8fafc;
}"#, "potion_red"),
            ], vec![
                fill("Set the background.", "body { ___: #020617; }", &["background-color", "color-bg", "paint"], 0, "background-color."),
                fill("Which value is a hex color?", "p { color: ___; }", &["#facc15", "yellow()", "facc15"], 0, "Hex codes start with #."),
                copy("Type this rule.", "body { background-color: #020617; }", "property: value;"),
                recall("From memory: body text color white.", "body { color: white; }", "body { color: white; }"),
            ]),
            l("Fonts and Text", &[
                (r#"font-size sets the size, font-weight how bold, text-align the alignment."#, "book"),
                (r#"h1 {
  font-size: 32px;
  text-align: center;
}"#, "scroll_fire"),
            ], vec![
                fill("Center the heading text.", "h1 { text-align: ___; }", &["center", "middle", "align"], 0, "text-align: center."),
                fill("Make text bigger.", "h1 { ___: 32px; }", &["font-size", "text-size", "size"], 0, "font-size."),
                copy("Type this rule.", "h1 { text-align: center; }", "property: value;"),
                recall("From memory: font-size 20px for p.", "p { font-size: 20px; }", "p { font-size: 20px; }"),
            ]),
        ]),
        m("Box Model", vec![
            l("Margin and Padding", &[
                (r#"Every element is a box. Padding is space INSIDE the border. Margin is space OUTSIDE."#, "chest"),
                (r#".card {
  padding: 16px;
  margin: 8px;
}"#, "shield"),
            ], vec![
                fill("Add space inside the box.", ".card { ___: 16px; }", &["padding", "margin", "gap"], 0, "padding is inside."),
                fill("Add space outside the box.", ".card { ___: 8px; }", &["margin", "padding", "border"], 0, "margin is outside."),
                copy("Type this rule.", ".card { padding: 16px; }", "padding: 16px;"),
                recall("From memory: margin 8px on .card.", ".card { margin: 8px; }", ".card { margin: 8px; }"),
            ]),
            l("Borders", &[
                (r#"border takes a width, a style and a color.

border: 2px solid gold;"#, "shield"),
                (r#"border-radius rounds the corners:

.card { border-radius: 8px; }"#, "amulet"),
            ], vec![
                fill("Pick the border style.", ".card { border: 2px ___ gold; }", &["solid", "thick", "line"], 0, "solid draws a straight line."),
                fill("Round the corners.", ".card { ___: 8px; }", &["border-radius", "corner", "round"], 0, "border-radius."),
                copy("Type this rule.", ".card { border: 2px solid gold; }", "width style color."),
                recall("From memory: border-radius 4px on .card.", ".card { border-radius: 4px; }", ".card { border-radius: 4px; }"),
            ]),
        ]),
        m("Layout", vec![
            l("Flexbox", &[
                (r#"display: flex puts children in a row. justify-content spreads them along the row."#, "map"),
                (r#".row {
  display: flex;
  justify-content: center;
}"#, "key"),
            ], vec![
                fill("Turn on flexbox.", ".row { display: ___; }", &["flex", "row", "inline"], 0, "display: flex."),
                fill("Center along the row.", ".row { justify-content: ___; }", &["center", "middle", "mid"], 0, "justify-content: center."),
                copy("Type this rule.", ".row { display: flex; }", "display: flex;"),
                recall("From memory: a row that uses flex and has gap 8px.", ".row { display: flex; gap: 8px; }", ".row { display: flex; gap: 8px; }"),
            ]),
            l("Sizing", &[
                (r#"width and height set the size. Use % to size relative to the parent, and max-width to stop growth."#, "helmet"),
                (r#".box {
  width: 50%;
  max-width: 400px;
}"#, "chest"),
            ], vec![
                fill("Make it half the parent's width.", ".box { width: ___; }", &["50%", "50", "half"], 0, "% is relative to the parent."),
                fill("Stop it growing past 400px.", ".box { ___: 400px; }", &["max-width", "limit", "width-max"], 0, "max-width."),
                copy("Type this rule.", ".box { width: 50%; }", "width: 50%;"),
                recall("From memory: max-width 400px on .box.", ".box { max-width: 400px; }", ".box { max-width: 400px; }"),
            ]),
        ]),
    ]}
}

fn javascript() -> Course {
    Course { name: "JavaScript", tag: "JS", color: Color32::from_rgb(0xf0, 0xa0, 0x1a), lang: "javascript", cat: "Code", level: "Beginner", desc: "Make web pages come alive with variables, logic and functions", modules: vec![
        m("Basics", vec![
            l("console.log", &[
                (r#"console.log() writes a message to the console.

Text goes in quotes. Numbers do not need them."#, "scroll_fire"),
                (r#"console.log("Hello, hero!");
console.log(42);"#, "torch"),
            ], vec![
                fill("Print a message.", r#"console.___("Hello");"#, &["log", "print", "say"], 0, "console.log writes to the console."),
                fill("Text needs quotes.", "console.log(___Hi\");", &["\"", "'", "`"], 0, "Open the string with a quote."),
                copy("Type this line.", r#"console.log("Hello");"#, "console.log(...);"),
                recall("From memory: print the number 5.", "console.log(5);", "console.log(5);"),
            ]),
            l("Variables", &[
                (r#"let makes a variable you can change. const makes one you cannot.

let hp = 10;
const name = "Pix";"#, "potion_red"),
                (r#"Change a let variable later:

hp = hp - 3;
console.log(hp); // 7"#, "crystal_blue"),
            ], vec![
                fill("Make a changeable variable.", "___ hp = 10;", &["let", "const", "print"], 0, "let can change."),
                fill("Make a variable that never changes.", r#"___ name = "Pix";"#, &["const", "let", "var()"], 0, "const is constant."),
                copy("Type this and see 7.", "let hp = 10;\nhp = hp - 3;\nconsole.log(hp);", "10 - 3 = 7."),
                recall("From memory: let gold be 5 then print gold.", "let gold = 5;\nconsole.log(gold);", "let gold = 5; console.log(gold);"),
            ]),
        ]),
        m("Numbers and Strings", vec![
            l("Math and Strings", &[
                (r#"JavaScript can do math: + - * / and % (remainder).

Joining strings with + is called concatenation."#, "coin_gold"),
                (r#"console.log(7 % 4); // 3
console.log("Hi " + "hero");"#, "gem_green"),
            ], vec![
                fill("Join two strings.", r#"console.log("Hi " ___ "hero");"#, &["+", "&", "."], 0, "+ joins strings."),
                fill("Get the remainder.", "console.log(7 ___ 4);", &["%", "/", "#"], 0, "% gives the remainder: 3."),
                copy("Type this line.", r#"console.log("Hi " + "hero");"#, "+ joins text."),
                recall("From memory: print 2 times 6.", "console.log(2 * 6);", "console.log(2 * 6);"),
            ]),
            l("Comparing", &[
                (r#"=== checks if two values are equal. > and < compare size. The result is true or false."#, "shield"),
                (r#"console.log(3 > 2);   // true
console.log(1 === 2); // false"#, "sword"),
            ], vec![
                fill("Check equality.", "console.log(1 ___ 1);", &["===", "=", "=>"], 0, "=== compares values."),
                fill("Is 3 bigger than 2?", "console.log(3 ___ 2);", &[">", "<", "=="], 0, "> means greater than."),
                copy("Type this line.", "console.log(3 > 2);", "Prints true."),
                recall("From memory: print whether 1 equals 2.", "console.log(1 === 2);", "console.log(1 === 2);"),
            ]),
        ]),
        m("Control Flow", vec![
            l("If and Else", &[
                (r#"if runs code only when its condition is true. else runs when it is false."#, "key"),
                (r#"let hp = 5;
if (hp > 0) {
  console.log("alive");
} else {
  console.log("fallen");
}"#, "potion_red"),
            ], vec![
                fill("Check hp before printing.", "___ (hp > 0) {\n  console.log(\"alive\");\n}", &["if", "when", "check"], 0, "if (condition) { ... }."),
                fill("Run this when it is false.", "if (hp > 0) {\n} ___ {\n  console.log(\"fallen\");\n}", &["else", "or", "then"], 0, "else runs otherwise."),
                copy("Type this and see alive.", "let hp = 5;\nif (hp > 0) {\n  console.log(\"alive\");\n}", "5 > 0 is true."),
                recall("From memory: if 2 > 1 print yes.", "if (2 > 1) {\n  console.log(\"yes\");\n}", "if (2 > 1) { console.log(\"yes\"); }"),
            ]),
            l("Loops", &[
                (r#"A for loop repeats code. It has a start, a stop condition and a step."#, "orb_red"),
                (r#"for (let i = 0; i < 3; i++) {
  console.log(i);
}

This prints 0, 1, 2."#, "torch"),
            ], vec![
                fill("Add one each time.", "for (let i = 0; i < 3; ___) {\n  console.log(i);\n}", &["i++", "i--", "i + 1"], 0, "i++ adds one."),
                fill("Stop before 3.", "for (let i = 0; ___; i++) {\n  console.log(i);\n}", &["i < 3", "i > 3", "i = 3"], 0, "Loop while i < 3."),
                copy("Type this loop.", "for (let i = 0; i < 3; i++) {\n  console.log(i);\n}", "Prints 0, 1, 2."),
                recall("From memory: count 0 to 1.", "for (let i = 0; i < 2; i++) {\n  console.log(i);\n}", "i < 2 prints 0 and 1."),
            ]),
        ]),
        m("Functions and Arrays", vec![
            l("Functions", &[
                (r#"A function is a reusable spell. Give it a name, parameters in ( ) and a body in { }.

return hands back a result."#, "staff"),
                (r#"function heal(hp) {
  return hp + 10;
}
console.log(heal(5)); // 15"#, "potion_red"),
            ], vec![
                fill("Define a function.", "___ heal(hp) {\n  return hp + 10;\n}", &["function", "def", "fun"], 0, "function name(params) { }."),
                fill("Send back a result.", "function heal(hp) {\n  ___ hp + 10;\n}", &["return", "give", "print"], 0, "return."),
                copy("Type this and see 15.", "function heal(hp) {\n  return hp + 10;\n}\nconsole.log(heal(5));", "5 + 10 = 15."),
                recall("From memory: double(n) returns n * 2.", "function double(n) {\n  return n * 2;\n}", "function double(n) { return n * 2; }"),
            ]),
            l("Arrays", &[
                (r#"An array holds a list of values. Counting starts at 0, so loot[0] is the first item."#, "chest"),
                (r#"let loot = ["gem", "coin", "key"];
console.log(loot[0]);
console.log(loot.length);"#, "key"),
            ], vec![
                fill("Make an array.", r#"let loot = ___"gem", "coin"];"#, &["[", "(", "{"], 0, "Arrays use square brackets."),
                fill("Get the first item.", "console.log(loot[___]);", &["0", "1", "first"], 0, "Index 0 is the first."),
                copy("Type this array.", r#"let loot = ["gem", "coin"];"#, "Square brackets, commas."),
                recall("From memory: an array nums holding 1 and 2.", "let nums = [1, 2];", "let nums = [1, 2];"),
            ]),
        ]),
    ]}
}

fn python() -> Course {
    Course { name: "Python", tag: "PY", color: Color32::from_rgb(0x2f, 0x6f, 0xd6), lang: "python", cat: "Code", level: "Beginner", desc: "Learn programming fundamentals such as variables, control flow, and loops", modules: vec![
        m("Hello World", vec![
            l("Printing", &[
                (r#"print() shows text on the screen.

Text goes between quotes. Anything between quotes is a STRING."#, "scroll_fire"),
                (r#"print("Hello, World!")

Code runs top to bottom, one line at a time."#, "torch"),
            ], vec![
                fill("Show a message.", r#"___("Hello, World!")"#, &["print", "echo", "say"], 0, "print() writes to the screen."),
                fill("Close the text.", r#"print("Hello___"#, &["\")", ")", "'"], 0, "Close the quote, then the bracket."),
                copy("Type this line.", r#"print("Hello, World!")"#, "print(\"text\")"),
                recall("From memory: print Hi.", r#"print("Hi")"#, r#"print("Hi")"#),
            ]),
            l("Strings and Comments", &[
                (r#"Print several lines to draw a pattern. Each print() starts a new line.

A line starting with # is a comment: Python ignores it."#, "book"),
                (r#"# draw a tower
print("*")
print("**")
print("***")"#, "chest"),
            ], vec![
                fill("Write a comment.", "___ this is a note", &["#", "//", "--"], 0, "Python comments start with #."),
                fill("Print two stars.", r#"print("___")"#, &["**", "2", "*+*"], 0, "Whatever is in the quotes is printed."),
                copy("Type this pattern.", "print(\"*\")\nprint(\"**\")", "Two prints, two lines."),
                recall("From memory: print A then B on two lines.", "print(\"A\")\nprint(\"B\")", "Two prints."),
            ]),
        ]),
        m("Variables", vec![
            l("Variables", &[
                (r#"A variable stores a value under a name. No keyword needed.

lives = 3
name = "Pix""#, "potion_red"),
                (r#"lives = 3
print(lives)  # 3"#, "orb_red"),
            ], vec![
                fill("Store 3 in lives.", "lives ___ 3", &["=", "==", ":"], 0, "A single = stores a value."),
                fill("Show the variable.", "lives = 3\n___(lives)", &["print", "show", "lives"], 0, "print(lives) shows 3."),
                copy("Type this and see 3.", "lives = 3\nprint(lives)", "Prints 3."),
                recall("From memory: gold is 7, print gold.", "gold = 7\nprint(gold)", "gold = 7; print(gold)"),
            ]),
            l("Math", &[
                (r#"Python does math: + - * / and % (remainder) and ** (power). Strings join with +."#, "coin_gold"),
                (r#"print(2 + 3 * 2)   # 8
print(7 % 4)       # 3
print("Hi " + "hero")"#, "gem_green"),
            ], vec![
                fill("Get the remainder.", "print(7 ___ 4)", &["%", "/", "//"], 0, "% is the remainder: 3."),
                fill("Join strings.", r#"print("a" ___ "b")"#, &["+", "&", "."], 0, "+ joins strings."),
                copy("Type this line.", "print(2 + 3 * 2)", "Multiplication first: 8."),
                recall("From memory: print 2 to the power 3.", "print(2 ** 3)", "print(2 ** 3)"),
            ]),
        ]),
        m("Control Flow", vec![
            l("If Statements", &[
                (r#"if runs code only when its condition is True. The line ends with a colon, and the code inside is indented."#, "key"),
                (r#"hp = 5
if hp > 0:
  print("alive")"#, "potion_red"),
            ], vec![
                fill("End the if line.", "if hp > 0___\n  print(\"alive\")", &[":", ";", "."], 0, "An if line ends with a colon."),
                fill("Test hp is above zero.", "if hp ___ 0:\n  print(\"alive\")", &[">", "<", "=="], 0, "> means greater than."),
                copy("Type this and see alive.", "hp = 5\nif hp > 0:\n  print(\"alive\")", "5 > 0 is True."),
                recall("From memory: if 2 > 1 print yes.", "if 2 > 1:\n  print(\"yes\")", "if 2 > 1:"),
            ]),
            l("Else", &[
                (r#"else runs when the condition is False. == checks equality."#, "shield"),
                (r#"hp = 0
if hp > 0:
  print("alive")
else:
  print("fallen")"#, "sword"),
            ], vec![
                fill("Run this otherwise.", "if hp > 0:\n  print(\"alive\")\n___:\n  print(\"fallen\")", &["else", "elif", "otherwise"], 0, "else:"),
                fill("Check equality.", "if hp ___ 0:\n  print(\"zero\")", &["==", "=", "=>"], 0, "== compares values."),
                copy("Type this and see fallen.", "hp = 0\nif hp > 0:\n  print(\"alive\")\nelse:\n  print(\"fallen\")", "0 > 0 is False."),
                recall("From memory: if 1 > 2 print a, else print b.", "if 1 > 2:\n  print(\"a\")\nelse:\n  print(\"b\")", "else branch"),
            ]),
        ]),
        m("Loops", vec![
            l("For Loops", &[
                (r#"for repeats code. range(3) gives 0, 1, 2 - it starts at 0 and stops before 3."#, "orb_red"),
                (r#"for i in range(3):
  print(i)"#, "torch"),
            ], vec![
                fill("Count three times.", "for i in ___(3):\n  print(i)", &["range", "count", "loop"], 0, "range()."),
                fill("Loop over the variable i.", "for ___ in range(3):\n  print(i)", &["i", "range", "print"], 0, "The loop variable is i."),
                copy("Type this loop.", "for i in range(3):\n  print(i)", "Prints 0, 1, 2."),
                recall("From memory: loop twice printing hi.", "for i in range(2):\n  print(\"hi\")", "range(2)"),
            ]),
            l("While Loops", &[
                (r#"while repeats as long as its condition is True. Change the variable inside or it never stops!"#, "cauldron"),
                (r#"n = 3
while n > 0:
  print(n)
  n -= 1"#, "potion_red"),
            ], vec![
                fill("Keep going while n is above 0.", "while n ___ 0:\n  print(n)\n  n -= 1", &[">", "<", "=="], 0, "Loop while n > 0."),
                fill("Count down by one.", "while n > 0:\n  print(n)\n  n ___ 1", &["-=", "+=", "=="], 0, "-= subtracts."),
                copy("Type this loop.", "n = 3\nwhile n > 0:\n  print(n)\n  n -= 1", "Prints 3, 2, 1."),
                recall("From memory: n = 2, loop while n > 0 printing n, then subtract 1.", "n = 2\nwhile n > 0:\n  print(n)\n  n -= 1", "Prints 2, 1."),
            ]),
        ]),
        m("Lists and Functions", vec![
            l("Lists", &[
                (r#"A list holds many values. Counting starts at 0, so loot[0] is the first item. len() counts items."#, "chest"),
                (r#"loot = ["gem", "coin", "key"]
print(loot[0])
print(len(loot))"#, "key"),
            ], vec![
                fill("Make a list.", "loot = ___\"gem\", \"coin\"]", &["[", "(", "{"], 0, "Lists use square brackets."),
                fill("Count the items.", "print(___(loot))", &["len", "count", "size"], 0, "len() counts items."),
                copy("Type this list.", "loot = [\"gem\", \"coin\"]\nprint(len(loot))", "Prints 2."),
                recall("From memory: nums holds 1 and 2, print how many.", "nums = [1, 2]\nprint(len(nums))", "len(nums) is 2."),
            ]),
            l("Functions", &[
                (r#"A function is a reusable spell. def starts it, return hands back a result."#, "staff"),
                (r#"def heal(hp):
  return hp + 10

print(heal(5))  # 15"#, "potion_red"),
            ], vec![
                fill("Define a function.", "___ heal(hp):\n  return hp + 10", &["def", "func", "make"], 0, "def starts a function."),
                fill("Send back a result.", "def heal(hp):\n  ___ hp + 10", &["return", "give", "print"], 0, "return."),
                copy("Type this and see 15.", "def heal(hp):\n  return hp + 10\n\nprint(heal(5))", "5 + 10 = 15."),
                recall("From memory: double(n) returns n * 2.", "def double(n):\n  return n * 2", "def double(n):"),
            ]),
        ]),
    ]}
}

fn sql() -> Course {
    Course { name: "SQL", tag: "SQL", color: Color32::from_rgb(0x1f, 0x9d, 0x8a), lang: "sql", cat: "Data", level: "Beginner", desc: "Learn database basics, queries, filtering and totals", modules: vec![
        m("Querying", vec![
            l("SELECT", &[
                (r#"SQL talks to databases. SELECT reads rows from a table.

SELECT * FROM heroes;

* means every column."#, "book"),
                (r#"A table is like a spreadsheet: columns are fields, rows are records. heroes might hold name, hp and class."#, "chest"),
            ], vec![
                fill("Read from the table.", "___ * FROM heroes;", &["SELECT", "GET", "READ"], 0, "SELECT reads rows."),
                fill("Name the table.", "SELECT * ___ heroes;", &["FROM", "IN", "AT"], 0, "FROM names the table."),
                copy("Type this query.", "SELECT * FROM heroes;", "End with a semicolon."),
                recall("From memory: select everything from items.", "SELECT * FROM items;", "SELECT * FROM items;"),
            ]),
            l("Choosing Columns", &[
                (r#"List the columns you want instead of *, separated by commas."#, "map"),
                (r#"SELECT name, hp FROM heroes;"#, "scroll_fire"),
            ], vec![
                fill("Pick two columns.", "SELECT name___ hp FROM heroes;", &[",", ".", ";"], 0, "Separate columns with commas."),
                fill("Pick the column hp.", "SELECT ___ FROM heroes;", &["hp", "*", "FROM"], 0, "Name the column."),
                copy("Type this query.", "SELECT name, hp FROM heroes;", "Columns then table."),
                recall("From memory: select name from heroes.", "SELECT name FROM heroes;", "SELECT name FROM heroes;"),
            ]),
        ]),
        m("Filtering", vec![
            l("WHERE", &[
                (r#"WHERE keeps only the rows that match a condition."#, "key"),
                (r#"SELECT * FROM heroes
WHERE hp > 10;"#, "shield"),
            ], vec![
                fill("Filter the rows.", "SELECT * FROM heroes ___ hp > 10;", &["WHERE", "IF", "HAVING"], 0, "WHERE filters rows."),
                fill("Only strong heroes.", "SELECT * FROM heroes WHERE hp ___ 10;", &[">", "<", "=>"], 0, "> means greater than."),
                copy("Type this query.", "SELECT * FROM heroes WHERE hp > 10;", "WHERE then a condition."),
                recall("From memory: heroes with hp above 5.", "SELECT * FROM heroes WHERE hp > 5;", "WHERE hp > 5"),
            ]),
            l("AND / OR", &[
                (r#"Combine conditions: AND needs both to be true, OR needs at least one. Text values use single quotes."#, "sword"),
                (r#"SELECT * FROM heroes
WHERE class = 'mage' AND hp > 5;"#, "staff"),
            ], vec![
                fill("Need both conditions.", "SELECT * FROM heroes WHERE class = 'mage' ___ hp > 5;", &["AND", "OR", "ALSO"], 0, "AND needs both."),
                fill("Match text.", "SELECT * FROM heroes WHERE class = ___mage';", &["'", "\"", "`"], 0, "SQL text uses single quotes."),
                copy("Type this query.", "SELECT * FROM heroes WHERE class = 'mage';", "Single quotes around text."),
                recall("From memory: heroes where class is 'knight'.", "SELECT * FROM heroes WHERE class = 'knight';", "class = 'knight'"),
            ]),
        ]),
        m("Sorting", vec![
            l("ORDER BY", &[
                (r#"ORDER BY sorts the results. ASC goes low to high (default), DESC goes high to low."#, "crystal_blue"),
                (r#"SELECT * FROM heroes
ORDER BY hp DESC;"#, "coin_gold"),
            ], vec![
                fill("Sort the rows.", "SELECT * FROM heroes ___ BY hp DESC;", &["ORDER", "SORT", "GROUP"], 0, "ORDER BY."),
                fill("Highest first.", "SELECT * FROM heroes ORDER BY hp ___;", &["DESC", "ASC", "TOP"], 0, "DESC is high to low."),
                copy("Type this query.", "SELECT * FROM heroes ORDER BY hp DESC;", "ORDER BY column direction."),
                recall("From memory: order heroes by name.", "SELECT * FROM heroes ORDER BY name;", "ORDER BY name"),
            ]),
            l("LIMIT", &[
                (r#"LIMIT caps how many rows come back. Combine it with ORDER BY for a top list."#, "chest"),
                (r#"SELECT * FROM heroes
ORDER BY hp DESC
LIMIT 3;"#, "shield"),
            ], vec![
                fill("Only three rows.", "SELECT * FROM heroes ___ 3;", &["LIMIT", "TOP", "MAX"], 0, "LIMIT n."),
                fill("Build a top list.", "SELECT * FROM heroes ___ BY hp DESC LIMIT 3;", &["ORDER", "GROUP", "WHERE"], 0, "Sort, then limit."),
                copy("Type this query.", "SELECT * FROM heroes LIMIT 3;", "LIMIT goes last."),
                recall("From memory: the top 1 hero by hp.", "SELECT * FROM heroes ORDER BY hp DESC LIMIT 1;", "ORDER BY hp DESC LIMIT 1"),
            ]),
        ]),
        m("Totals", vec![
            l("COUNT and SUM", &[
                (r#"Aggregate functions summarise many rows into one value: COUNT, SUM, AVG, MAX and MIN."#, "coin_gold"),
                (r#"SELECT COUNT(*) FROM heroes;
SELECT SUM(hp) FROM heroes;"#, "gem_green"),
            ], vec![
                fill("Count the rows.", "SELECT ___(*) FROM heroes;", &["COUNT", "TOTAL", "NUM"], 0, "COUNT(*) counts rows."),
                fill("Add up the hp.", "SELECT ___(hp) FROM heroes;", &["SUM", "ADD", "PLUS"], 0, "SUM adds values."),
                copy("Type this query.", "SELECT COUNT(*) FROM heroes;", "COUNT(*)."),
                recall("From memory: the largest hp.", "SELECT MAX(hp) FROM heroes;", "MAX(hp)"),
            ]),
            l("GROUP BY", &[
                (r#"GROUP BY makes one result row per value, so you can total each group."#, "cauldron"),
                (r#"SELECT class, COUNT(*)
FROM heroes
GROUP BY class;"#, "map"),
            ], vec![
                fill("One row per class.", "SELECT class, COUNT(*) FROM heroes ___ BY class;", &["GROUP", "ORDER", "SORT"], 0, "GROUP BY."),
                fill("Count each group.", "SELECT class, ___(*) FROM heroes GROUP BY class;", &["COUNT", "SUM", "ALL"], 0, "COUNT(*) per group."),
                copy("Type this query.", "SELECT class, COUNT(*) FROM heroes GROUP BY class;", "GROUP BY goes last."),
                recall("From memory: total hp per class.", "SELECT class, SUM(hp) FROM heroes GROUP BY class;", "SUM(hp) GROUP BY class"),
            ]),
        ]),
    ]}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interp::{run, Kind};

    #[test]
    fn content_is_well_formed() {
        let cs = all();
        assert_eq!(cs.len(), 5);
        for c in &cs {
            assert!(!c.modules.is_empty(), "{} has no chapters", c.name);
            for m in &c.modules {
                assert!(!m.lessons.is_empty(), "{} / {} has no lessons", c.name, m.name);
                for l in &m.lessons {
                    let at = format!("{} / {} / {}", c.name, m.name, l.title);
                    assert!(!l.slides.is_empty(), "{at}: no slides");
                    assert!(!l.qs.is_empty(), "{at}: no questions");
                    for (_, img) in &l.slides { assert!(crate::art::art_exists(img), "{at}: unknown picture '{img}'"); }
                    for q in &l.qs {
                        match q {
                            Q::Mc { opts, right, .. } => assert!(*right < opts.len(), "{at}: bad answer index"),
                            Q::Fill { code, opts, right, .. } => {
                                assert!(*right < opts.len(), "{at}: bad answer index");
                                assert!(code.contains("___"), "{at}: fill question has no gap");
                            }
                            Q::Type { answer, .. } => assert!(!answer.trim().is_empty(), "{at}: empty answer"),
                        }
                    }
                }
            }
        }
    }

    /// SC-002: typing the expected answer in any course never produces an error.
    #[test]
    fn content_answers_run_clean() {
        let mut failures = vec![];
        for c in all() {
            for m in &c.modules {
                for l in &m.lessons {
                    for q in &l.qs {
                        if let Q::Type { answer, .. } = q {
                            let out = run(c.lang, answer);
                            if let Some((_, e)) = out.iter().find(|(k, _)| *k == Kind::Err) {
                                failures.push(format!("{} / {}: `{}` -> {}", c.name, l.title, answer.replace('\n', "⏎"), e));
                            }
                        }
                    }
                }
            }
        }
        assert!(failures.is_empty(), "{} answers fail:\n{}", failures.len(), failures.join("\n"));
    }
}
