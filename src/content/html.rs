use super::*;
use eframe::egui::Color32;

pub fn course() -> Course {
    Course {
        name: "HTML", tag: "HTML", slug: "html", color: Color32::from_rgb(0xe8, 0x5a, 0x1e), lang: "html", cat: "Web", level: "Beginner", hours: 10,
        desc: "Build your first web pages with HTML, the stone and timber every website is made of.",
        teaches: vec!["Tags, headings, text and lists", "Links and pictures", "The skeleton of a real page", "Forms people can fill in", "Semantic layout: header, main, footer"],
        snippet: r##"<h1>Hello, hero!</h1>"##,
        badge: Badge { name: "Stonemason", art: "shield", desc: "Built every HTML structure" },
        modules: vec![elements(), structure(), forms(), semantic()],
    }
}

fn elements() -> Module {
    ch("Elements", "Tags, text, lists, links and pictures: the bricks of every page.", vec![
        l("Your First Tags", &[
            (r##"HTML (HyperText Markup Language) tells a browser what each part of a page IS: a title, a paragraph, a picture.

You mark content with TAGS written in angle brackets."##, "scroll_fire"),
            (r##"Most tags come in pairs around the content:

<h1>Hello, hero!</h1>

<h1> opens, </h1> closes (note the slash). Together they make one ELEMENT."##, "book"),
        ], vec![
            quiz("What does HTML describe?", &["How a page looks", "What each part of a page is", "How a page moves"], 1, "HTML gives content structure and meaning; CSS handles looks."),
            fill("Close the heading.", "<h1>Hello, hero!</___>", &["h1", "/h1", "h1/"], 0, "A closing tag is </ + the same name + >."),
            copy("Type this element.", "<h1>Hello, hero!</h1>", "Opening tag, content, closing tag."),
            recall("From memory: a paragraph that says Welcome to the guild.", "<p>Welcome to the guild.</p>", "<p> is a paragraph."),
        ]),
        l("Headings", &[
            (r##"There are six heading levels:

<h1> is the biggest, <h6> the smallest.

Use one <h1> for the page title, then <h2> for sections, <h3> inside those."##, "banner"),
            (r##"<h1>The Guild Hall</h1>
<h2>Quests</h2>
<h3>Slay the Slime</h3>

Headings are like a book's chapters: they show the outline."##, "map"),
        ], vec![
            quiz("Which heading is the smallest?", &["<h1>", "<h3>", "<h6>"], 2, "Numbers go from 1 (biggest) to 6 (smallest)."),
            fill("Make a section heading.", "<___>Quests</h2>", &["h2", "h7", "head"], 0, "There are only h1 to h6."),
            copy("Type this heading.", "<h2>Chapter One</h2>", "Level-2 heading."),
            recall("From memory: a level-3 heading that says The Tavern.", "<h3>The Tavern</h3>", "<h3>The Tavern</h3>"),
        ]),
        l("Paragraphs and Breaks", &[
            (r##"<p> wraps a paragraph of text. Browsers add space between paragraphs.

Pressing Enter inside your code does NOT start a new line on the page!"##, "letter"),
            (r##"<br> forces a line break (it has no closing tag).
<hr> draws a dividing line.

<p>Roses are red,<br>potions are blue.</p>
<hr>"##, "scroll_lightning"),
        ], vec![
            quiz("Which tag breaks a line inside a paragraph?", &["<br>", "<lb>", "<break>"], 0, "<br> = break. It stands alone."),
            fill("Draw a divider line.", "<___>", &["hr", "line", "div"], 0, "hr = horizontal rule."),
            copy("Type this poem.", "<p>Roses are red,<br>potions are blue.</p>", "<br> breaks the line."),
            recall("From memory: a paragraph Line one, a break, then Line two.", "<p>Line one<br>Line two</p>", "<p>Line one<br>Line two</p>"),
        ]),
        l("Bold, Italic and More", &[
            (r##"Inside text you can stress words:

<b> or <strong>  - bold
<i> or <em>      - italic
<u> underline, <s> strikethrough"##, "spellbook"),
            (r##"<strong> and <em> also tell screen readers the words are important, so prefer them for real emphasis.

<p>A <strong>brave</strong> and <em>clever</em> hero.</p>"##, "hat_wizard"),
        ], vec![
            quiz("Which tag means strong importance?", &["<strong>", "<big>", "<i>"], 0, "<strong> shows bold AND marks importance."),
            fill("Make 'clever' italic.", "<p>A ___clever</em> hero.</p>", &["<em>", "<b>", "<s>"], 0, "<em> pairs with </em>."),
            copy("Type this sentence.", "<p>A <b>brave</b> and <i>clever</i> hero.</p>", "Tags can sit inside a paragraph."),
            recall("From memory: a paragraph where Danger is strong, followed by ' ahead!'.", "<p><strong>Danger</strong> ahead!</p>", "<strong>Danger</strong>"),
        ]),
        l("Lists", &[
            (r##"<ul> makes a bullet list (unordered).
<ol> makes a numbered list (ordered).

Each item inside is an <li> (list item)."##, "chest"),
            (r##"<ol>
  <li>Wake up</li>
  <li>Fight a slime</li>
</ol>

Indent the <li> lines so the list is easy to read."##, "sword"),
        ], vec![
            quiz("A recipe's steps in order should use...", &["<ul>", "<ol>", "<li>"], 1, "Order matters, so use an ordered list."),
            fill("Add a list item.", "<ul>\n  ___Sword</li>\n</ul>", &["<li>", "<ul>", "<ol>"], 0, "Each item is an <li>."),
            copy("Type this numbered list.", "<ol>\n  <li>Wake up</li>\n  <li>Fight a slime</li>\n</ol>", "ol wraps li items."),
            recall("From memory: a bullet list with Sword and Shield.", "<ul><li>Sword</li><li>Shield</li></ul>", "<ul><li>Sword</li><li>Shield</li></ul>"),
        ]),
        l("Links", &[
            (r##"<a> (anchor) makes a link. The href ATTRIBUTE holds where it goes:

<a href="https://example.com">Guild Hall</a>

Attributes live inside the opening tag: name="value"."##, "map"),
            (r##"target="_blank" opens the link in a new tab:

<a href="map.html" target="_blank">Map</a>

href can be a full web address or a file in your site."##, "key"),
        ], vec![
            quiz("Which attribute says where a link goes?", &["src", "href", "link"], 1, "href = hypertext reference."),
            fill("Open in a new tab.", r##"<a href="map.html" ___="_blank">Map</a>"##, &["target", "tab", "open"], 0, "target=\"_blank\" opens a new tab."),
            copy("Type this link.", r##"<a href="https://example.com">Guild Hall</a>"##, "Attribute values go in quotes."),
            recall("From memory: link to map.html that opens in a new tab, text Map.", r##"<a href="map.html" target="_blank">Map</a>"##, r##"<a href="map.html" target="_blank">Map</a>"##),
        ]),
        l("Images", &[
            (r##"<img> shows a picture. It has NO closing tag.

<img src="dragon.png" alt="A red dragon">

src = the picture file. alt = words that describe it."##, "helmet"),
            (r##"alt text is read aloud to blind visitors and shown if the picture fails to load. Always write it!

width sets the size in pixels:
<img src="hero.png" alt="Hero" width="200">"##, "crystal_blue"),
        ], vec![
            quiz("Why write alt text?", &["It makes the image bigger", "It describes the image for everyone", "It is required by CSS"], 1, "Screen readers and broken images rely on alt."),
            fill("Which attribute loads the picture?", r##"<img ___="dragon.png" alt="A red dragon">"##, &["src", "href", "file"], 0, "src = source."),
            copy("Type this image.", r##"<img src="dragon.png" alt="A red dragon">"##, "No closing tag needed."),
            recall("From memory: image hero.png, alt Hero, width 200.", r##"<img src="hero.png" alt="Hero" width="200">"##, r##"<img src="hero.png" alt="Hero" width="200">"##),
        ]),
        p("Project: Wanted Poster", &[
            (r##"CHECKPOINT PROJECT

The sheriff needs a WANTED poster for the Goblin King. Use everything from this chapter: a heading, a picture, bold text and a list."##, "scroll_fire"),
            (r##"<h1>WANTED</h1>
<img src="goblin.png" alt="Goblin King">
<p>Reward: <b>500 gold</b></p>
<ul>
  <li>Green skin</li>
  <li>Loves shiny coins</li>
</ul>"##, "coins"),
        ], vec![
            fill("Describe the picture.", r##"<img src="goblin.png" ___="Goblin King">"##, &["alt", "title", "name"], 0, "alt describes the image."),
            fill("Make the reward bold.", "<p>Reward: <b>500 gold</___></p>", &["b", "p", "h1"], 0, "Close the <b> you opened."),
            copy("Build the poster.", "<h1>WANTED</h1>\n<img src=\"goblin.png\" alt=\"Goblin King\">\n<p>Reward: <b>500 gold</b></p>\n<ul>\n  <li>Green skin</li>\n  <li>Loves shiny coins</li>\n</ul>", "Heading, image, text, list: a real page!"),
            recall("From memory: heading WANTED, then a paragraph Reward: with 100 gold in bold.", "<h1>WANTED</h1><p>Reward: <b>100 gold</b></p>", "<h1>WANTED</h1><p>Reward: <b>100 gold</b></p>"),
        ]),
    ])
}

fn structure() -> Module {
    ch("Structure", "Page skeletons, nesting, boxes, classes and ids.", vec![
        l("Page Skeleton", &[
            (r##"Every full page has the same skeleton:

<!DOCTYPE html> says "this is modern HTML".
<html> wraps everything.
<head> holds info ABOUT the page.
<body> holds what you SEE."##, "shield"),
            (r##"<!DOCTYPE html>
<html>
  <head>
    <title>My Guild</title>
  </head>
  <body>
    <h1>Welcome</h1>
  </body>
</html>

<title> is the text on the browser tab."##, "book"),
        ], vec![
            quiz("Where does visible content go?", &["<head>", "<body>", "<title>"], 1, "The body is what visitors see."),
            fill("Name the browser tab.", "<head>\n  <___>My Guild</title>\n</head>", &["title", "h1", "name"], 0, "<title> sets the tab text."),
            copy("Type the skeleton.", "<!DOCTYPE html>\n<html>\n  <head>\n    <title>My Guild</title>\n  </head>\n  <body>\n    <h1>Welcome</h1>\n  </body>\n</html>", "doctype, html, head, body."),
            recall("From memory: a title element that says Quest Log.", "<title>Quest Log</title>", "<title>Quest Log</title>"),
        ]),
        l("Nesting and Indentation", &[
            (r##"Elements live INSIDE other elements, like boxes in boxes. The inner one is a CHILD, the outer one its PARENT.

Close children before their parent:
<p>I am <i>very</i> brave.</p>"##, "chest"),
            (r##"Indent each child two spaces deeper. The browser doesn't care, but humans do:

<ul>
  <li><a href="#">Home</a></li>
</ul>"##, "map"),
        ], vec![
            quiz("Which is correctly nested?", &["<p><i>hi</p></i>", "<p><i>hi</i></p>", "<i><p>hi</i></p>"], 1, "Last opened, first closed."),
            fill("Close the child first.", "<p>I am <i>very___ brave.</p>", &["</i>", "</p>", "<i>"], 0, "The <i> must close before </p>."),
            copy("Type this nested list.", "<ul>\n  <li><a href=\"#\">Home</a></li>\n</ul>", "a inside li inside ul."),
            recall("From memory: a paragraph I am very brave. with very in <i>.", "<p>I am <i>very</i> brave.</p>", "<p>I am <i>very</i> brave.</p>"),
        ]),
        l("Div and Span", &[
            (r##"<div> is a plain BOX that groups other elements. It starts on a new line.

<div>
  <h2>Inventory</h2>
  <p>3 potions</p>
</div>"##, "chest"),
            (r##"<span> is a plain wrapper for a few words INSIDE a line.

<p>HP: <span>10</span></p>

Neither changes the look by itself; CSS will style them later."##, "gem_green"),
        ], vec![
            quiz("Which wraps a few words inside a sentence?", &["<div>", "<span>", "<p>"], 1, "span is inline; div is a block."),
            fill("Group these elements in a box.", "<___>\n  <h2>Inventory</h2>\n</div>", &["div", "span", "box"], 0, "div = division, a block box."),
            copy("Type this box.", "<div>\n  <h2>Inventory</h2>\n  <p>3 potions</p>\n</div>", "div groups a section."),
            recall("From memory: paragraph HP: with 10 inside a span.", "<p>HP: <span>10</span></p>", "<p>HP: <span>10</span></p>"),
        ]),
        l("Classes and IDs", &[
            (r##"class gives elements a group name. Many elements can share one class:

<p class="note">Rest at the inn.</p>
<p class="note">Buy potions.</p>"##, "scroll_fire"),
            (r##"id gives ONE element a unique name:

<h1 id="title">Guild</h1>

CSS and JavaScript use classes and ids to find elements."##, "key"),
        ], vec![
            quiz("Which must be unique on the page?", &["class", "id", "both"], 1, "Only one element may use each id."),
            fill("Share a style name.", r##"<p ___="note">Rest at the inn.</p>"##, &["class", "id", "style"], 0, "class can repeat."),
            copy("Type this paragraph.", r##"<p class="note">Rest at the inn.</p>"##, "class=\"note\""),
            recall("From memory: an h1 with id title that says Guild.", r##"<h1 id="title">Guild</h1>"##, r##"<h1 id="title">Guild</h1>"##),
        ]),
        l("Comments and Attributes", &[
            (r##"Comments are notes for coders. The browser hides them:

<!-- shop section -->

Use them to label parts of a long page."##, "letter"),
            (r##"More useful attributes:

title="..."  - tooltip on hover
width="..."  - size of images
href="#shop" - jump to the element with id="shop""##, "gear"),
        ], vec![
            quiz("Will visitors see an HTML comment?", &["Yes, in grey", "No, it is hidden", "Only on hover"], 1, "Comments never show on the page."),
            fill("Start a comment.", "___ shop section -->", &["<!--", "//", "#"], 0, "HTML comments: <!-- ... -->"),
            copy("Type this labelled section.", "<!-- shop section -->\n<h2>Shop</h2>", "The comment stays hidden."),
            recall("From memory: a link to #shop with title Go to shop and text Shop.", r##"<a href="#shop" title="Go to shop">Shop</a>"##, r##"<a href="#shop" title="Go to shop">Shop</a>"##),
        ]),
        p("Project: Hero Profile Card", &[
            (r##"CHECKPOINT PROJECT

Make a profile card for your hero: a box with a picture, a name with an id and a role line with a class."##, "helmet_knight"),
            (r##"<div class="card">
  <img src="ada.png" alt="Ada" width="120">
  <h2 id="name">Ada the Brave</h2>
  <p class="role">Level 4 <span>Wizard</span></p>
</div>"##, "hat_wizard"),
        ], vec![
            fill("Wrap the card in a box.", "<___ class=\"card\">", &["div", "span", "card"], 0, "A div groups the card."),
            fill("Give the name a unique id.", r##"<h2 ___="name">Ada the Brave</h2>"##, &["id", "class", "alt"], 0, "Unique name = id."),
            copy("Build the card.", "<div class=\"card\">\n  <img src=\"ada.png\" alt=\"Ada\" width=\"120\">\n  <h2 id=\"name\">Ada the Brave</h2>\n  <p class=\"role\">Level 4 <span>Wizard</span></p>\n</div>", "Box, picture, heading, text."),
            recall("From memory: a div with class card holding an h2 Bo.", r##"<div class="card"><h2>Bo</h2></div>"##, r##"<div class="card"><h2>Bo</h2></div>"##),
        ]),
    ])
}

fn forms() -> Module {
    ch("Forms", "Text boxes, buttons, choices and validation.", vec![
        l("Your First Form", &[
            (r##"Forms collect answers from visitors: names, searches, orders.

<form> wraps the fields.
<input> is a field. It has no closing tag."##, "letter"),
            (r##"<form>
  <input type="text" placeholder="Hero name">
  <input type="submit" value="Join">
</form>

placeholder is grey hint text. type="submit" makes the send button."##, "scroll_fire"),
        ], vec![
            quiz("Which input type sends the form?", &["send", "submit", "button-go"], 1, "type=\"submit\" makes the send button."),
            fill("Show hint text inside the box.", r##"<input type="text" ___="Hero name">"##, &["placeholder", "value", "hint"], 0, "placeholder shows grey hint text."),
            copy("Type this form.", "<form>\n  <input type=\"text\" placeholder=\"Hero name\">\n  <input type=\"submit\" value=\"Join\">\n</form>", "A field and a button."),
            recall("From memory: a text input with placeholder Search.", r##"<input type="text" placeholder="Search">"##, r##"<input type="text" placeholder="Search">"##),
        ]),
        l("Input Types", &[
            (r##"type changes what a field accepts:

text, email, password, number, date, color...

The browser gives each a fitting keyboard or picker."##, "gear"),
            (r##"<input type="email" placeholder="you@mail.com">
<input type="password" placeholder="Secret">
<input type="number" placeholder="Age">

Password fields hide what you type."##, "keyhole"),
        ], vec![
            quiz("Which type hides typed letters?", &["hidden", "secret", "password"], 2, "type=\"password\" shows dots."),
            fill("Ask for an email address.", r##"<input type="___" placeholder="you@mail.com">"##, &["email", "mail", "text"], 0, "type=\"email\" checks for an @."),
            copy("Type this field.", r##"<input type="email" placeholder="you@mail.com">"##, "An email field."),
            recall("From memory: a password input with placeholder Secret.", r##"<input type="password" placeholder="Secret">"##, r##"<input type="password" placeholder="Secret">"##),
        ]),
        l("Labels", &[
            (r##"A <label> names a field. Clicking the label focuses the field, and screen readers read it out.

Connect them: label for="..." matches the input's id."##, "medallion"),
            (r##"<label for="hp">Health</label>
<input type="number" id="hp">

for="hp" points at id="hp"."##, "heart"),
        ], vec![
            quiz("label for=\"pet\" connects to the input with...", &["name=\"pet\"", "id=\"pet\"", "class=\"pet\""], 1, "for matches an id."),
            fill("Connect the label.", "<label ___=\"hp\">Health</label>", &["for", "id", "to"], 0, "label for=\"...\""),
            copy("Type this labelled field.", "<label for=\"hp\">Health</label>\n<input type=\"number\" id=\"hp\">", "for and id match."),
            recall("From memory: a label for pet that says Pet name.", r##"<label for="pet">Pet name</label>"##, r##"<label for="pet">Pet name</label>"##),
        ]),
        l("Checkboxes and Radios", &[
            (r##"Checkboxes: pick ANY number of options.
<input type="checkbox" value="Potion"> Potion

Radio buttons: pick only ONE from a group."##, "potion_red"),
            (r##"Radios in a group share the same name:

<input type="radio" name="class" value="Mage"> Mage
<input type="radio" name="class" value="Knight"> Knight

Picking one un-picks the other."##, "hat_wizard"),
        ], vec![
            quiz("Choose exactly one class: use...", &["checkbox", "radio", "text"], 1, "Radio buttons allow one choice per group."),
            fill("Group the radios.", r##"<input type="radio" ___="class" value="Mage">"##, &["name", "id", "group"], 0, "Same name = same group."),
            copy("Type this radio.", r##"<input type="radio" name="class" value="Mage"> Mage"##, "name groups radios."),
            recall("From memory: a checkbox with value Potion.", r##"<input type="checkbox" value="Potion">"##, r##"<input type="checkbox" value="Potion">"##),
        ]),
        l("Text Areas and Dropdowns", &[
            (r##"<textarea> is a big box for long answers. Unlike input, it HAS a closing tag:

<textarea placeholder="Your story"></textarea>"##, "spellbook"),
            (r##"<select> makes a dropdown. Each choice is an <option>:

<select>
  <option>Sword</option>
  <option>Bow</option>
</select>"##, "bow"),
        ], vec![
            quiz("Which element makes a dropdown?", &["<dropdown>", "<select>", "<list>"], 1, "select + option."),
            fill("Add a choice.", "<select>\n  <___>Sword</option>\n</select>", &["option", "li", "choice"], 0, "Each choice is an option."),
            copy("Type this dropdown.", "<select>\n  <option>Sword</option>\n  <option>Bow</option>\n</select>", "select wraps options."),
            recall("From memory: a textarea with placeholder Your story.", r##"<textarea placeholder="Your story"></textarea>"##, r##"<textarea placeholder="Your story"></textarea>"##),
        ]),
        l("Validation", &[
            (r##"The browser can check answers before sending:

required  - must not be empty
minlength / maxlength - text length
min / max - number range"##, "shield"),
            (r##"<input type="text" placeholder="Name" required minlength="3">
<input type="number" min="1" max="10">

Bad answers get a warning and the form will not send."##, "hourglass"),
        ], vec![
            quiz("Which attribute stops an empty answer?", &["needed", "required", "must"], 1, "required blocks empty fields."),
            fill("Allow only 1 to 10.", r##"<input type="number" min="1" ___="10">"##, &["max", "top", "end"], 0, "min and max set the range."),
            copy("Type this field.", r##"<input type="text" placeholder="Name" required minlength="3">"##, "required + minlength."),
            recall("From memory: a number input with min 1 and max 10.", r##"<input type="number" min="1" max="10">"##, r##"<input type="number" min="1" max="10">"##),
        ]),
        p("Project: Tavern Sign-Up", &[
            (r##"CHECKPOINT PROJECT

The Pixel Tavern needs a sign-up form: a heading, a required name field with a label, a class dropdown and a button."##, "cup_silver"),
            (r##"<form>
  <h2>Join the Tavern</h2>
  <label for="nm">Name</label>
  <input type="text" id="nm" required>
  <select>
    <option>Knight</option>
    <option>Mage</option>
  </select>
  <input type="submit" value="Sign up">
</form>"##, "letter"),
        ], vec![
            fill("Make the name required.", r##"<input type="text" id="nm" ___>"##, &["required", "needed", "min"], 0, "required = can't be empty."),
            fill("Add the send button.", r##"<input type="___" value="Sign up">"##, &["submit", "send", "text"], 0, "type=\"submit\"."),
            copy("Build the form.", "<form>\n  <h2>Join the Tavern</h2>\n  <label for=\"nm\">Name</label>\n  <input type=\"text\" id=\"nm\" required>\n  <select>\n    <option>Knight</option>\n    <option>Mage</option>\n  </select>\n  <input type=\"submit\" value=\"Sign up\">\n</form>", "Heading, label, field, dropdown, button."),
            recall("From memory: a form with a required email input and a submit input.", r##"<form><input type="email" required><input type="submit"></form>"##, r##"<form><input type="email" required><input type="submit"></form>"##),
        ]),
    ])
}

fn semantic() -> Module {
    ch("Semantic HTML", "Tags that say what each part of a page means.", vec![
        l("Why Semantics?", &[
            (r##"A <div> says nothing about what it holds. SEMANTIC tags do:

<header> <nav> <main> <article> <section> <footer>

Screen readers, search engines and other coders understand the page better."##, "crystal_blue"),
            (r##"<header>
  <h1>The Pixel Times</h1>
</header>

It looks the same as a div, but now the page explains itself."##, "scroll_lightning"),
        ], vec![
            quiz("Why use <header> instead of <div>?", &["It is bigger", "It tells what the content means", "It is faster"], 1, "Semantic tags carry meaning."),
            fill("Mark the page's top area.", "<___>\n  <h1>The Pixel Times</h1>\n</header>", &["header", "head", "top"], 0, "<head> is page info; <header> is the visible top."),
            copy("Type this header.", "<header>\n  <h1>The Pixel Times</h1>\n</header>", "A semantic header."),
            recall("From memory: a footer that says Made by heroes.", "<footer>Made by heroes</footer>", "<footer>Made by heroes</footer>"),
        ]),
        l("Header and Nav", &[
            (r##"<header> holds the top of a page: logo, title, menu.

<nav> holds the main navigation links."##, "banner_royal"),
            (r##"<nav>
  <a href="#">Home</a>
  <a href="#quests">Quests</a>
</nav>

Usually the nav sits inside the header."##, "map"),
        ], vec![
            quiz("Which tag holds the main menu links?", &["<menu>", "<nav>", "<links>"], 1, "nav = navigation."),
            fill("Wrap the menu.", "<___>\n  <a href=\"#\">Home</a>\n</nav>", &["nav", "menu", "ul"], 0, "nav opens, /nav closes."),
            copy("Type this nav.", "<nav>\n  <a href=\"#\">Home</a>\n  <a href=\"#quests\">Quests</a>\n</nav>", "Links inside nav."),
            recall("From memory: a header holding an h1 Guild.", "<header><h1>Guild</h1></header>", "<header><h1>Guild</h1></header>"),
        ]),
        l("Main and Footer", &[
            (r##"<main> holds the page's main content. Use only one per page.

<footer> holds the bottom: credits, contact, copyright."##, "book"),
            (r##"<main>
  <p>Today's quests</p>
</main>
<footer>Guild 2026</footer>"##, "lantern"),
        ], vec![
            quiz("How many <main> elements should a page have?", &["One", "Two", "As many as you like"], 0, "One main per page."),
            fill("Mark the bottom area.", "<___>Guild 2026</footer>", &["footer", "bottom", "end"], 0, "footer = page bottom."),
            copy("Type main and footer.", "<main>\n  <p>Today's quests</p>\n</main>\n<footer>Guild 2026</footer>", "Content, then credits."),
            recall("From memory: a main holding an h2 Quests.", "<main><h2>Quests</h2></main>", "<main><h2>Quests</h2></main>"),
        ]),
        l("Article and Section", &[
            (r##"<article> is a complete piece that makes sense alone: a blog post, a news story, a forum reply."##, "letter"),
            (r##"<section> groups related content under a heading:

<section>
  <h2>News</h2>
  <article>
    <h3>Dragon Spotted</h3>
    <p>Near the mountain.</p>
  </article>
</section>"##, "scroll_fire"),
        ], vec![
            quiz("A single blog post fits best in...", &["<article>", "<nav>", "<footer>"], 0, "It stands on its own."),
            fill("Group related news.", "<___>\n  <h2>News</h2>\n</section>", &["section", "group", "div"], 0, "section groups by topic."),
            copy("Type this article.", "<article>\n  <h2>Dragon Spotted</h2>\n  <p>Near the mountain.</p>\n</article>", "A self-contained story."),
            recall("From memory: a section holding an h2 News.", "<section><h2>News</h2></section>", "<section><h2>News</h2></section>"),
        ]),
        l("Figures", &[
            (r##"<figure> wraps a picture (or chart, or code) with its caption.

<figcaption> is the caption text."##, "map"),
            (r##"<figure>
  <img src="map.png" alt="Old map">
  <figcaption>The lost map</figcaption>
</figure>"##, "crystal_purple"),
        ], vec![
            quiz("Which tag is a caption?", &["<caption>", "<figcaption>", "<alt>"], 1, "figcaption goes inside figure."),
            fill("Add the caption.", "<figure>\n  <img src=\"map.png\" alt=\"Old map\">\n  <___>The lost map</figcaption>\n</figure>", &["figcaption", "caption", "p"], 0, "figure + figcaption."),
            copy("Type this figure.", "<figure>\n  <img src=\"map.png\" alt=\"Old map\">\n  <figcaption>The lost map</figcaption>\n</figure>", "Picture with a caption."),
            recall("From memory: a figcaption that says A brave hero.", "<figcaption>A brave hero</figcaption>", "<figcaption>A brave hero</figcaption>"),
        ]),
        p("Final Project: Adventure Blog", &[
            (r##"FINAL PROJECT

Build Ada's adventure blog with real semantic structure: a header with nav, a main with an article, and a footer."##, "crown"),
            (r##"<header>
  <h1>Ada's Adventure Log</h1>
  <nav><a href="#day1">Day 1</a></nav>
</header>
<main>
  <article id="day1">
    <h2>Day 1: The Cave</h2>
    <p>I found a <b>glowing gem</b>.</p>
  </article>
</main>
<footer>Thanks for reading!</footer>"##, "gem_green"),
        ], vec![
            fill("The menu belongs in...", "<header>\n  <___><a href=\"#day1\">Day 1</a></nav>\n</header>", &["nav", "main", "footer"], 0, "nav holds menu links."),
            fill("A blog post is an...", "<___ id=\"day1\">", &["article", "aside", "span"], 0, "A post stands alone: article."),
            copy("Build the blog.", "<header>\n  <h1>Ada's Adventure Log</h1>\n  <nav><a href=\"#day1\">Day 1</a></nav>\n</header>\n<main>\n  <article id=\"day1\">\n    <h2>Day 1: The Cave</h2>\n    <p>I found a <b>glowing gem</b>.</p>\n  </article>\n</main>\n<footer>Thanks for reading!</footer>", "A complete semantic page!"),
            recall("From memory: an article with an h2 Day 2 and a paragraph Rain.", "<article><h2>Day 2</h2><p>Rain.</p></article>", "<article><h2>Day 2</h2><p>Rain.</p></article>"),
        ]),
    ])
}
