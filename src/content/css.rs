use super::*;
use eframe::egui::Color32;

pub fn course() -> Course {
    Course {
        name: "CSS", tag: "CSS", slug: "css", color: Color32::from_rgb(0x23, 0x7b, 0xe0), lang: "css", cat: "Web", level: "Beginner", hours: 12,
        desc: "Paint and arrange your pages with CSS: colors, fonts, spacing, positioning and flexbox.",
        teaches: vec!["Selectors: type, class, id, groups", "Colors, units, fonts and backgrounds", "The box model: border, padding, margin", "Positioning: relative, absolute, fixed, sticky", "Flexbox layouts"],
        snippet: r##"h1 { color: gold; }"##,
        badge: Badge { name: "Prism Painter", art: "star", desc: "Styled every CSS quest" },
        modules: vec![selectors(), properties(), box_model(), layout(), flexbox()],
    }
}

fn selectors() -> Module {
    ch("Selectors", "Pick which elements to style.", vec![
        l("What Is CSS?", &[
            (r##"HTML builds the castle; CSS paints it. CSS (Cascading Style Sheets) sets colors, sizes, spacing and layout.

A RULE has a selector and declarations:

p { color: red; }"##, "potion_magic"),
            (r##"p          - the SELECTOR: which elements
color      - the PROPERTY: what to change
red        - the VALUE
;          - ends each declaration
{ }        - wrap the declarations"##, "spellbook"),
        ], vec![
            quiz("In p { color: red; } what is p?", &["The property", "The selector", "The value"], 1, "The selector picks the elements."),
            fill("End the declaration.", "p { color: red___ }", &[";", ":", ","], 0, "Each declaration ends with a semicolon."),
            copy("Type this rule.", "p { color: red; }", "selector { property: value; }"),
            recall("From memory: make every h1 blue.", "h1 { color: blue; }", "h1 { color: blue; }"),
        ]),
        l("Linking CSS", &[
            (r##"CSS usually lives in its own file, like style.css. The HTML page links to it inside <head>:

<link rel="stylesheet" href="style.css">"##, "key"),
            (r##"You can also write CSS in a <style> tag in the head, or in a style="..." attribute. A separate file keeps things tidy and lets many pages share one look."##, "book"),
        ], vec![
            quiz("Where does the <link> to a stylesheet go?", &["In <head>", "At the end of <body>", "Inside <p>"], 0, "Styles load from the head."),
            fill("Link the stylesheet.", r##"<link rel="___" href="style.css">"##, &["stylesheet", "style", "css"], 0, "rel=\"stylesheet\""),
            copy("Type this rule for style.css.", "body { background-color: black; }", "A rule for the whole page."),
            recall("From memory: make every h2 gold.", "h2 { color: gold; }", "h2 { color: gold; }"),
        ]),
        l("Type Selectors", &[
            (r##"A TYPE selector is just a tag name. It styles every element of that type:

li { color: green; }

Every <li> on the page turns green."##, "gem_green"),
            (r##"body { ... } styles the whole page.
a { ... } styles every link.

Use type selectors for the basic look of your site."##, "map"),
        ], vec![
            quiz("Which selector styles every link?", &["link", "a", "href"], 1, "Links are <a> elements."),
            fill("Style every list item.", "___ { color: green; }", &["li", ".li", "#li"], 0, "Tag name, no symbol."),
            copy("Type this rule.", "li { color: green; }", "Every li is green."),
            recall("From memory: make every link orange.", "a { color: orange; }", "a { color: orange; }"),
        ]),
        l("Class and ID Selectors", &[
            (r##"A dot selects a CLASS:
.warning { color: red; }
matches <p class="warning">.

A hash selects an ID:
#title { color: gold; }
matches <h1 id="title">."##, "scroll_fire"),
            (r##"Classes are reusable; ids are for one element. Prefer classes for styling."##, "medallion"),
        ], vec![
            quiz("Which selects class=\"note\"?", &["#note", ".note", "note"], 1, "Dot = class."),
            fill("Select the id title.", "___title { color: gold; }", &["#", ".", "*"], 0, "Hash = id."),
            copy("Type this rule.", ".warning { color: red; }", "Every .warning turns red."),
            recall("From memory: give #title the color gold.", "#title { color: gold; }", "#title { color: gold; }"),
        ]),
        l("Grouping and Nesting", &[
            (r##"Group selectors with commas to share styles:

h1, h2 { color: purple; }"##, "crystal_purple"),
            (r##"A space means "inside":  nav a { } styles links inside nav.
> means "direct child":    ul > li { }
* means "everything":      * { margin: 0; }"##, "gear"),
        ], vec![
            quiz("What does nav a select?", &["nav and a", "a elements inside nav", "nav inside a"], 1, "A space means descendant."),
            fill("Share one rule.", "h1___ h2 { color: purple; }", &[",", ">", "."], 0, "Commas group selectors."),
            copy("Type this rule.", "h1, h2 { color: purple; }", "Both headings purple."),
            recall("From memory: make links inside nav white.", "nav a { color: white; }", "nav a { color: white; }"),
        ]),
        p("Project: Bard Concert Flyer", &[
            (r##"CHECKPOINT PROJECT

The bards play the Pixel Tavern on Friday! Style their flyer with type, group, class and id selectors."##, "banner_royal"),
            (r##"body { background-color: #1e1b4b; }
h1, h2 { color: gold; text-align: center; }
.band { color: white; font-size: 20px; }
#date { color: pink; }"##, "cup_silver"),
        ], vec![
            fill("Style both headings at once.", "h1___ h2 { color: gold; }", &[",", " >", "."], 0, "Comma groups."),
            fill("Select the date by id.", "___date { color: pink; }", &["#", ".", "@"], 0, "Hash = id."),
            copy("Style the flyer.", "body { background-color: #1e1b4b; }\nh1, h2 { color: gold; text-align: center; }\n.band { color: white; font-size: 20px; }\n#date { color: pink; }", "Four rules, four kinds of selector."),
            recall("From memory: make .band white.", ".band { color: white; }", ".band { color: white; }"),
        ]),
    ])
}

fn properties() -> Module {
    ch("Properties", "Colors, units, fonts, text and backgrounds.", vec![
        l("Colors", &[
            (r##"Three common ways to write a color:

red                - a name
#ffcc00            - hex: red, green, blue in 00-ff
rgb(255, 204, 0)   - red, green, blue in 0-255"##, "crystal_blue"),
            (r##"color changes TEXT color.
background-color changes the fill behind it.

h1 { color: #ffcc00; }"##, "potion_red"),
        ], vec![
            quiz("rgb(255, 0, 0) is...", &["Red", "Green", "Blue"], 0, "All red, no green or blue."),
            fill("Write the hex color.", "h1 { color: ___ffcc00; }", &["#", ".", "0x"], 0, "Hex colors start with #."),
            copy("Type this rule.", "h1 { color: #ffcc00; }", "A golden hex color."),
            recall("From memory: p with color rgb(255, 0, 0).", "p { color: rgb(255, 0, 0); }", "p { color: rgb(255, 0, 0); }"),
        ]),
        l("Units", &[
            (r##"px  - pixels, a fixed size
%   - a share of the parent's size
em  - relative to the current font size
rem - relative to the page's root font size"##, "hourglass"),
            (r##".box { width: 50%; }   half its parent
p { font-size: 2em; }  twice the normal text"##, "chest"),
        ], vec![
            quiz("width: 50% means...", &["50 pixels", "Half the parent's width", "Half the screen always"], 1, "% is relative to the parent."),
            fill("Use pixels.", "h1 { font-size: 32___; }", &["px", "pt", "pix"], 0, "px = pixels."),
            copy("Type this rule.", ".box { width: 50%; }", "Half as wide as its parent."),
            recall("From memory: p with font-size 2em.", "p { font-size: 2em; }", "p { font-size: 2em; }"),
        ]),
        l("Fonts", &[
            (r##"font-family picks the typeface. List backups after commas:

p { font-family: Arial, sans-serif; }

The last one (serif, sans-serif, monospace) is a generic fallback."##, "letter"),
            (r##"font-size: 18px;   how big
font-weight: bold;  how thick (or 100-900)"##, "scroll_lightning"),
        ], vec![
            quiz("Why list several fonts?", &["To mix them", "As backups if one is missing", "It is required"], 1, "The browser uses the first one it has."),
            fill("Make the heading thick.", "h1 { font-___: bold; }", &["weight", "size", "style"], 0, "font-weight: bold."),
            copy("Type this rule.", "p { font-family: Arial, sans-serif; }", "Arial, or any sans-serif."),
            recall("From memory: make h1 bold.", "h1 { font-weight: bold; }", "h1 { font-weight: bold; }"),
        ]),
        l("Text Styling", &[
            (r##"text-align: left | center | right
text-decoration: underline | line-through | none
font-style: italic"##, "spellbook"),
            (r##"Links are underlined by default. Remove it with:

a { text-decoration: none; }"##, "key"),
        ], vec![
            quiz("Which removes a link's underline?", &["text-decoration: none", "underline: off", "text-style: plain"], 0, "text-decoration: none."),
            fill("Center the heading.", "h2 { text-align: ___; }", &["center", "middle", "centre"], 0, "CSS uses American spelling: center."),
            copy("Type this rule.", "a { text-decoration: none; }", "No underline."),
            recall("From memory: center the text of h2.", "h2 { text-align: center; }", "h2 { text-align: center; }"),
        ]),
        l("Backgrounds", &[
            (r##"background-color fills the element.
background-image puts a picture behind it:

body { background-image: url("stars.png"); }"##, "lantern"),
            (r##"background-size: cover;  stretch to fill
background-repeat: no-repeat;"##, "map"),
        ], vec![
            quiz("Which property sets a picture behind an element?", &["background-image", "image", "img-src"], 0, "background-image: url(...)."),
            fill("Load the picture.", r##"body { background-image: ___("stars.png"); }"##, &["url", "src", "img"], 0, "url(\"...\")."),
            copy("Type this rule.", r##"body { background-image: url("stars.png"); }"##, "A starry backdrop."),
            recall("From memory: give .card a navy background-color.", ".card { background-color: navy; }", ".card { background-color: navy; }"),
        ]),
        l("Shorthands", &[
            (r##"Shorthands set several properties at once:

border: 2px solid gold;
  = width, style, color"##, "shield"),
            (r##"font: bold 16px Arial;
  = weight, size, family

Less typing, same result."##, "hammer"),
        ], vec![
            quiz("border: 2px solid gold sets...", &["Only the color", "Width, style and color", "The margin"], 1, "Three values in one."),
            fill("Draw a solid border.", ".card { border: 2px ___ gold; }", &["solid", "line", "full"], 0, "Styles: solid, dashed, dotted..."),
            copy("Type this rule.", ".card { border: 2px solid gold; }", "A golden frame."),
            recall("From memory: p with font bold 16px Arial.", "p { font: bold 16px Arial; }", "p { font: bold 16px Arial; }"),
        ]),
        p("Project: Royal Invitation", &[
            (r##"CHECKPOINT PROJECT

The queen is throwing a feast! Style the invitation card with color, alignment, padding, a border and a fancy font."##, "crown"),
            (r##".invite {
  background-color: #3a2e6b;
  color: white;
  text-align: center;
  padding: 20px;
  border: 3px solid gold;
}
h1 { font-family: Georgia, serif; }"##, "chalice"),
        ], vec![
            fill("Center the text.", ".invite { text-align: ___; }", &["center", "left", "middle"], 0, "center."),
            fill("Frame it in gold.", ".invite { border: 3px solid ___; }", &["gold", "#gold", "golden"], 0, "gold is a named color."),
            copy("Style the invitation.", ".invite {\n  background-color: #3a2e6b;\n  color: white;\n  text-align: center;\n  padding: 20px;\n  border: 3px solid gold;\n}\nh1 { font-family: Georgia, serif; }", "Royal!"),
            recall("From memory: give .invite a 3px solid gold border.", ".invite { border: 3px solid gold; }", ".invite { border: 3px solid gold; }"),
        ]),
    ])
}

fn box_model() -> Module {
    ch("Box Model", "Every element is a box: content, padding, border, margin.", vec![
        l("Every Element Is a Box", &[
            (r##"Every element is a rectangle made of layers:

content -> padding -> border -> margin

width and height size the content area."##, "chest"),
            (r##".box { width: 200px; height: 100px; }

Images keep their shape if you set only one of width or height."##, "crystal_blue"),
        ], vec![
            quiz("Which layer is outside the border?", &["padding", "margin", "content"], 1, "Margin is the outermost space."),
            fill("Set the height.", ".box { width: 200px; ___: 100px; }", &["height", "tall", "size"], 0, "width and height."),
            copy("Type this rule.", ".box { width: 200px; height: 100px; }", "A 200 x 100 box."),
            recall("From memory: make img 120px wide.", "img { width: 120px; }", "img { width: 120px; }"),
        ]),
        l("Borders and Corners", &[
            (r##"border draws a frame around the padding.

border-radius rounds the corners:
.badge { border: 2px solid gold; border-radius: 8px; }"##, "medallion"),
            (r##"border-radius: 50% turns a square image into a circle, great for avatars."##, "helmet_knight"),
        ], vec![
            quiz("Which makes corners round?", &["border-round", "border-radius", "corner"], 1, "border-radius."),
            fill("Make the avatar a circle.", "img { border-radius: ___; }", &["50%", "100px", "round"], 0, "50% of each side = a circle."),
            copy("Type this rule.", ".badge { border: 2px solid gold; border-radius: 8px; }", "A rounded golden badge."),
            recall("From memory: make img a circle with border-radius.", "img { border-radius: 50%; }", "img { border-radius: 50%; }"),
        ]),
        l("Padding", &[
            (r##"padding is space INSIDE the border, around the content:

.card { padding: 16px; }"##, "potion_red"),
            (r##"One value: all sides. Two values: top/bottom, left/right.

button { padding: 8px 16px; }
padding-left, padding-top... set one side."##, "gear"),
        ], vec![
            quiz("padding: 8px 16px gives left and right...", &["8px", "16px", "24px"], 1, "Second value = left and right."),
            fill("Add space inside the card.", ".card { ___: 16px; }", &["padding", "margin", "space"], 0, "Inside = padding."),
            copy("Type this rule.", ".card { padding: 16px; }", "Room to breathe inside."),
            recall("From memory: button with padding 8px 16px.", "button { padding: 8px 16px; }", "button { padding: 8px 16px; }"),
        ]),
        l("Margin", &[
            (r##"margin is space OUTSIDE the border, pushing other elements away:

h1 { margin-bottom: 24px; }"##, "map"),
            (r##"margin: 0 auto; centers a block that has a width:

.page { width: 600px; margin: 0 auto; }"##, "shield"),
        ], vec![
            quiz("To push a neighbour away, use...", &["padding", "margin", "border"], 1, "Margin is outside space."),
            fill("Center the page.", ".page { margin: 0 ___; }", &["auto", "center", "50%"], 0, "auto splits the free space."),
            copy("Type this rule.", ".page { margin: 0 auto; }", "Centered block."),
            recall("From memory: h1 with margin-bottom 24px.", "h1 { margin-bottom: 24px; }", "h1 { margin-bottom: 24px; }"),
        ]),
        l("Box Sizing", &[
            (r##"By default, width covers only the content. Padding and border are ADDED, so boxes grow bigger than you asked."##, "hourglass"),
            (r##"box-sizing: border-box makes width include padding and border. Most coders set it everywhere:

* { box-sizing: border-box; }"##, "hammer"),
        ], vec![
            quiz("With border-box, padding...", &["is added to width", "fits inside width", "is ignored"], 1, "width includes padding and border."),
            fill("Apply to everything.", "___ { box-sizing: border-box; }", &["*", "all", "body"], 0, "* selects every element."),
            copy("Type this rule.", "* { box-sizing: border-box; }", "Predictable box sizes."),
            recall("From memory: .card with box-sizing border-box.", ".card { box-sizing: border-box; }", ".card { box-sizing: border-box; }"),
        ]),
        p("Project: Library Card", &[
            (r##"CHECKPOINT PROJECT

The Pixel Library needs member cards: fixed width, padded, centered, framed and rounded."##, "book"),
            (r##".card {
  width: 300px;
  padding: 16px;
  margin: 20px auto;
  border: 2px solid brown;
  border-radius: 10px;
  box-sizing: border-box;
}"##, "spellbook"),
        ], vec![
            fill("Center the card.", ".card { margin: 20px ___; }", &["auto", "center", "0"], 0, "auto centers horizontally."),
            fill("Include padding in the width.", ".card { box-sizing: ___; }", &["border-box", "content-box", "padding-box"], 0, "border-box."),
            copy("Style the card.", ".card {\n  width: 300px;\n  padding: 16px;\n  margin: 20px auto;\n  border: 2px solid brown;\n  border-radius: 10px;\n  box-sizing: border-box;\n}", "Every layer of the box model."),
            recall("From memory: .card with margin 20px auto.", ".card { margin: 20px auto; }", ".card { margin: 20px auto; }"),
        ]),
    ])
}

fn layout() -> Module {
    ch("Layout", "Block vs inline, and positioning elements anywhere.", vec![
        l("Block and Inline", &[
            (r##"BLOCK elements (div, p, h1) start on a new line and stretch full width.
INLINE elements (span, a, b) sit inside a line."##, "chest"),
            (r##"display changes this:
display: block | inline | inline-block | none

inline-block sits in a line but accepts width and height. none hides it."##, "gear"),
        ], vec![
            quiz("Which hides an element completely?", &["display: none", "display: hidden", "visible: no"], 0, "display: none removes it from the layout."),
            fill("Put list items side by side.", "li { display: ___; }", &["inline-block", "block", "none"], 0, "inline-block flows in a line."),
            copy("Type this rule.", "span { display: block; }", "The span now takes its own line."),
            recall("From memory: li with display inline-block.", "li { display: inline-block; }", "li { display: inline-block; }"),
        ]),
        l("Position Relative", &[
            (r##"position: relative nudges an element from where it would normally be, using top, left, right and bottom."##, "map"),
            (r##".icon { position: relative; top: 4px; }

Moves down 4px. The space it left behind stays reserved."##, "key"),
        ], vec![
            quiz("With position: relative, top: 4px moves the element...", &["up 4px", "down 4px", "to the top of the page"], 1, "top pushes it down from its top edge."),
            fill("Nudge it right.", ".tag { position: relative; ___: 10px; }", &["left", "right", "move"], 0, "left: 10px pushes it right."),
            copy("Type this rule.", ".icon { position: relative; top: 4px; }", "A small nudge."),
            recall("From memory: .tag relative with left 10px.", ".tag { position: relative; left: 10px; }", ".tag { position: relative; left: 10px; }"),
        ]),
        l("Position Absolute", &[
            (r##"position: absolute lifts an element OUT of the flow and places it inside its nearest positioned parent."##, "crystal_purple"),
            (r##".badge { position: absolute; top: 0; right: 0; }

Pins the badge to the parent's top-right corner. Give the parent position: relative."##, "medallion"),
        ], vec![
            quiz("An absolute element is placed relative to...", &["The previous sibling", "Its nearest positioned parent", "The mouse"], 1, "The nearest ancestor with a position."),
            fill("Pin it to the corner.", ".badge { position: ___; top: 0; right: 0; }", &["absolute", "relative", "static"], 0, "absolute."),
            copy("Type this rule.", ".badge { position: absolute; top: 0; right: 0; }", "Top-right corner."),
            recall("From memory: .close absolute with top 8px.", ".close { position: absolute; top: 8px; }", ".close { position: absolute; top: 8px; }"),
        ]),
        l("Fixed and Sticky", &[
            (r##"position: fixed sticks to the WINDOW. It stays put while you scroll, like a banner bar.

nav { position: fixed; top: 0; width: 100%; }"##, "banner"),
            (r##"position: sticky scrolls normally until it reaches its top value, then sticks:

header { position: sticky; top: 0; }"##, "lantern"),
        ], vec![
            quiz("Which stays in view even while scrolling the whole page?", &["fixed", "relative", "static"], 0, "fixed is tied to the window."),
            fill("Stick when it reaches the top.", "header { position: ___; top: 0; }", &["sticky", "fixed", "stuck"], 0, "sticky."),
            copy("Type this rule.", "nav { position: fixed; top: 0; width: 100%; }", "A bar that never scrolls away."),
            recall("From memory: header sticky with top 0.", "header { position: sticky; top: 0; }", "header { position: sticky; top: 0; }"),
        ]),
        l("Stacking with z-index", &[
            (r##"When positioned elements overlap, z-index decides who is on top. Bigger numbers sit higher."##, "cpu"),
            (r##".modal { position: absolute; z-index: 10; }

z-index works on positioned elements (not static)."##, "potion_magic"),
        ], vec![
            quiz("Which appears on top?", &["z-index: 1", "z-index: 5", "z-index: 0"], 1, "Higher z-index wins."),
            fill("Raise the popup.", ".popup { ___: 99; }", &["z-index", "layer", "top"], 0, "z-index."),
            copy("Type this rule.", ".modal { position: absolute; z-index: 10; }", "Above the rest."),
            recall("From memory: .popup with z-index 99.", ".popup { z-index: 99; }", ".popup { z-index: 99; }"),
        ]),
        p("Project: Art Gallery", &[
            (r##"CHECKPOINT PROJECT

Lay out the royal art gallery: framed pictures in a row, labels pinned to each frame, and a sticky menu."##, "amulet"),
            (r##".gallery { position: relative; }
.frame { display: inline-block; width: 150px; margin: 8px; }
.label { position: absolute; bottom: 0; background-color: black; color: white; }
nav { position: sticky; top: 0; }"##, "crystal_blue"),
        ], vec![
            fill("Frames side by side.", ".frame { display: ___; }", &["inline-block", "none", "block"], 0, "inline-block."),
            fill("Keep the menu visible.", "nav { position: ___; top: 0; }", &["sticky", "relative", "static"], 0, "sticky."),
            copy("Lay out the gallery.", ".gallery { position: relative; }\n.frame { display: inline-block; width: 150px; margin: 8px; }\n.label { position: absolute; bottom: 0; background-color: black; color: white; }\nnav { position: sticky; top: 0; }", "Flow, absolute and sticky together."),
            recall("From memory: .frame with display inline-block.", ".frame { display: inline-block; }", ".frame { display: inline-block; }"),
        ]),
    ])
}

fn flexbox() -> Module {
    ch("Flexbox", "Line things up in rows and columns the easy way.", vec![
        l("Display Flex", &[
            (r##"Flexbox lays out children in a row or column. Put display: flex on the PARENT:

.row { display: flex; }

Its children line up side by side."##, "chest"),
            (r##"gap adds space between the children:

.row { display: flex; gap: 8px; }"##, "coins"),
        ], vec![
            quiz("display: flex goes on...", &["Each child", "The parent container", "The body only"], 1, "The container becomes a flex box."),
            fill("Turn on flexbox.", ".row { display: ___; }", &["flex", "row", "box"], 0, "display: flex."),
            copy("Type this rule.", ".row { display: flex; }", "Children in a row."),
            recall("From memory: .row flex with gap 8px.", ".row { display: flex; gap: 8px; }", ".row { display: flex; gap: 8px; }"),
        ]),
        l("Direction and Wrap", &[
            (r##"flex-direction: row (default) or column.

.col { display: flex; flex-direction: column; }"##, "map"),
            (r##"flex-wrap: wrap lets items drop to the next line when they run out of room."##, "scroll_fire"),
        ], vec![
            quiz("Stack items top to bottom with...", &["flex-direction: column", "flex-wrap: wrap", "display: block"], 0, "column stacks vertically."),
            fill("Let items wrap.", ".row { flex-wrap: ___; }", &["wrap", "yes", "true"], 0, "flex-wrap: wrap."),
            copy("Type this rule.", ".col { display: flex; flex-direction: column; }", "A vertical stack."),
            recall("From memory: .row with flex-wrap wrap.", ".row { flex-wrap: wrap; }", ".row { flex-wrap: wrap; }"),
        ]),
        l("Justify Content", &[
            (r##"justify-content spreads items along the MAIN axis (left to right in a row):

flex-start | center | flex-end | space-between | space-around"##, "banner_royal"),
            (r##".row { display: flex; justify-content: space-between; }

First item at the start, last at the end, equal gaps between."##, "sword"),
        ], vec![
            quiz("space-between puts the first and last items...", &["in the middle", "at the edges", "on top of each other"], 1, "Edges, with equal gaps between."),
            fill("Center the items.", ".row { justify-content: ___; }", &["center", "middle", "align"], 0, "center."),
            copy("Type this rule.", ".row { display: flex; justify-content: space-between; }", "Spread to the edges."),
            recall("From memory: .row with justify-content center.", ".row { justify-content: center; }", ".row { justify-content: center; }"),
        ]),
        l("Align Items", &[
            (r##"align-items lines items up on the CROSS axis (top to bottom in a row):

stretch | flex-start | center | flex-end"##, "gear"),
            (r##".row { display: flex; align-items: center; }

justify + align center = perfectly centered."##, "crystal_purple"),
        ], vec![
            quiz("In a row, align-items controls...", &["Left to right", "Top to bottom", "Font size"], 1, "The cross axis."),
            fill("Line items up at the bottom.", ".bar { align-items: ___; }", &["flex-end", "bottom", "end-line"], 0, "flex-end."),
            copy("Type this rule.", ".row { display: flex; align-items: center; }", "Vertically centered."),
            recall("From memory: .bar with align-items flex-end.", ".bar { align-items: flex-end; }", ".bar { align-items: flex-end; }"),
        ]),
        l("Flexible Sizing", &[
            (r##"flex: 1 lets an item grow to fill free space. Items with flex: 1 share it equally.

.card { flex: 1; }"##, "potion_magic"),
            (r##"flex-grow sets the share: an item with flex-grow: 2 takes twice as much extra space as one with 1."##, "hammer"),
        ], vec![
            quiz("Two cards with flex: 1 get...", &["Equal extra space", "No space", "Random space"], 0, "Equal shares."),
            fill("Grow twice as much.", ".main { flex-grow: ___; }", &["2", "200%", "double"], 0, "A number share."),
            copy("Type this rule.", ".card { flex: 1; }", "Fill the free space."),
            recall("From memory: .main with flex-grow 2.", ".main { flex-grow: 2; }", ".main { flex-grow: 2; }"),
        ]),
        p("Final Project: Monster Cards", &[
            (r##"FINAL PROJECT

Show the monster collection as a row of cards that wraps, centers and spaces itself, with equal-width framed cards."##, "crown"),
            (r##".cards {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 16px;
}
.card {
  flex: 1;
  padding: 12px;
  border: 2px solid gold;
  border-radius: 8px;
}"##, "mystic_figure"),
        ], vec![
            fill("Space between cards.", ".cards { ___: 16px; }", &["gap", "margin", "space"], 0, "gap spaces flex items."),
            fill("Equal-width cards.", ".card { flex: ___; }", &["1", "auto", "equal"], 0, "flex: 1."),
            copy("Style the collection.", ".cards {\n  display: flex;\n  flex-wrap: wrap;\n  justify-content: center;\n  gap: 16px;\n}\n.card {\n  flex: 1;\n  padding: 12px;\n  border: 2px solid gold;\n  border-radius: 8px;\n}", "A real responsive layout!"),
            recall("From memory: .cards flex with gap 16px.", ".cards { display: flex; gap: 16px; }", ".cards { display: flex; gap: 16px; }"),
        ]),
    ])
}
