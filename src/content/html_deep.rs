//! HTML: the full explanation, key points and extra practice for every lesson.
use super::*;

pub fn deep() -> Vec<Deep> {
    vec![
        d("Your First Tags", r##"## What HTML really is
Every web page you have ever opened is a plain text file written in HTML. The browser reads that file from top to bottom and turns it into the page you see. HTML does not decide colours or animations. Its only job is to say what each piece of content IS: "this is a heading", "this is a paragraph", "this is a picture".

## Tags and elements
You label content by wrapping it in tags. A tag is a name inside angle brackets.

    <h1>Hello, hero!</h1>

- <h1> is the opening tag. It says "a big heading starts here".
- </h1> is the closing tag. The slash means "it ends here".
- Everything from the opening tag to the closing tag is one element.

## Common mistakes
Forgetting the slash in the closing tag is the most common beginner bug. <h1>Hi<h1> opens a second heading instead of closing the first. Also make sure the closing name matches the opening name exactly: <h1> must close with </h1>, not </h2>.

Tag names are not case sensitive, but everyone writes them in lowercase, so you should too."##,
            &["HTML describes what each part of a page is, not how it looks.", "A tag is a name in angle brackets, like <p>.", "Most elements have an opening tag, content and a closing tag with a slash.", "The closing tag must use the same name as the opening tag."],
            vec![
                quiz("In <p>Hi</p>, which part is the closing tag?", &["<p>", "Hi", "</p>"], 2, "The closing tag has a slash before the name."),
                quiz("What is wrong with <h1>Welcome<h1>?", &["Nothing", "The second tag is missing its slash", "h1 is not a real tag"], 1, "Without the slash, the browser sees a second opening tag."),
                fill("Open the paragraph.", "___Welcome!</p>", &["<p>", "</p>", "<para>"], 0, "An element starts with its opening tag."),
            ]),
        d("Headings", r##"## Six levels of headings
HTML has six heading elements, from <h1> to <h6>. The number is the level, not the size you want. <h1> is the most important heading on the page and <h6> the least important.

    <h1>The Guild Hall</h1>
    <h2>Quests</h2>
    <h3>Slay the Slime</h3>
    <h3>Find the Key</h3>
    <h2>Shop</h2>

## Think of a book
A page is like a book: <h1> is the book title, <h2> are the chapters and <h3> are sections inside a chapter. Search engines and screen readers use this outline to understand and navigate your page, so the levels matter even if you later change the sizes with CSS.

## Good habits
- Use one <h1> per page for the main title.
- Do not skip levels: an <h3> should sit under an <h2>, not straight under the <h1>.
- Never pick a heading just because it looks bigger or smaller. Choose the level that matches the outline, and style it later with CSS.

## Common mistake
There is no <h7>. If you need more depth than six levels, your page probably needs to be split up."##,
            &["There are six heading levels, <h1> to <h6>.", "<h1> is the main title; use it once per page.", "Pick levels by outline (title, chapter, section), not by size.", "Do not skip levels."],
            vec![
                quiz("How many <h1> headings should a page usually have?", &["One", "Six", "As many as you like"], 0, "One <h1> names the whole page."),
                quiz("You want a smaller title. What should guide your choice of heading?", &["Its place in the outline", "How big it looks", "Its colour"], 0, "Choose by meaning; change the size with CSS."),
                fill("A section inside an <h2> chapter:", "<___>Slay the Slime</h3>", &["h3", "h1", "h7"], 0, "Sections inside a chapter use the next level down."),
            ]),
        d("Paragraphs and Breaks", r##"## Paragraphs
Body text goes in <p> elements. Each paragraph becomes its own block with a gap above and below it.

    <p>The tavern is warm and loud.</p>
    <p>A bard is playing in the corner.</p>

## The browser ignores your line breaks
This surprises everyone at first. Pressing Enter in your code does NOT make a new line on the page. The browser collapses all spaces, tabs and line breaks into a single space. So this:

    <p>Roses are red,
    potions are blue.</p>

shows up as one line: "Roses are red, potions are blue."

## Forcing a line break
To break a line inside a paragraph (poems, addresses), use <br>. It is an empty element: it has no content, so it has no closing tag.

    <p>Roses are red,<br>potions are blue.</p>

## A dividing line
<hr> (horizontal rule) draws a line across the page to separate topics. It is also empty.

## When to use what
Use separate <p> elements for separate ideas. Use <br> only when the line break is part of the content itself. Do not stack <br><br><br> to add space; spacing is a job for CSS."##,
            &["<p> wraps a paragraph; paragraphs get space around them.", "Line breaks and extra spaces in your code are collapsed into one space.", "<br> breaks a line inside text and has no closing tag.", "<hr> draws a dividing line; use CSS, not many <br>s, for spacing."],
            vec![
                quiz("You press Enter twice inside a <p> in your code. What happens on the page?", &["Two new lines appear", "Nothing: it becomes a single space", "The paragraph ends"], 1, "Browsers collapse whitespace."),
                quiz("Which element has no closing tag?", &["<p>", "<br>", "<h2>"], 1, "<br> is an empty element."),
                fill("Separate two topics with a line.", "<p>Part one</p>\n<___>\n<p>Part two</p>", &["hr", "br", "line"], 0, "<hr> is a horizontal rule."),
            ]),
        d("Bold, Italic and More", r##"## Inline text elements
Some elements live INSIDE a paragraph and change just a few words. They are called inline elements because they stay in the line of text.

    <p>A <strong>brave</strong> and <em>clever</em> hero.</p>

## Meaning versus looks
There are two pairs that look the same but mean different things:
- <strong> means "this is important". It shows as bold.
- <b> just makes text bold, with no extra meaning.
- <em> means "say this with emphasis". It shows as italic.
- <i> just makes text italic (good for names of ships, spells or foreign words).

Screen readers can change their voice for <strong> and <em>, so use them when the meaning matters. Use <b> and <i> when you only want the look.

## Other handy tags
- <u> underlines (careful: underlined text looks like a link).
- <s> strikes text through, for things no longer true: <s>50 gold</s> 30 gold.
- <mark> highlights text, <small> makes side notes.

## Nesting correctly
Inline elements must close in the reverse order they opened, like stacking boxes:

    <p><strong><em>Run!</em></strong></p>

Closing </strong> before </em> would be broken nesting."##,
            &["Inline elements change a few words inside a line of text.", "<strong> = important (bold), <em> = emphasis (italic).", "<b> and <i> change only the look, with no extra meaning.", "Close nested tags in reverse order."],
            vec![
                quiz("Which tag tells a screen reader the words are important?", &["<b>", "<strong>", "<u>"], 1, "<strong> carries meaning; <b> only looks bold."),
                quiz("Which nesting is correct?", &["<b><i>Hi</b></i>", "<b><i>Hi</i></b>", "<b><i>Hi</b>"], 1, "Close the inner element first."),
                fill("Show an old price crossed out.", "<p><___>50 gold</s> 30 gold</p>", &["s", "u", "em"], 0, "<s> strikes through text."),
            ]),
        d("Lists", r##"## Two kinds of list
HTML has two everyday lists:
- An unordered list <ul> shows bullets. Use it when order does not matter, like an inventory.
- An ordered list <ol> shows numbers. Use it when order matters, like steps in a recipe.

Each item in either list goes inside <li> (list item).

    <ul>
      <li>Sword</li>
      <li>Shield</li>
    </ul>

    <ol>
      <li>Open the door</li>
      <li>Light the torch</li>
    </ol>

## Only <li> goes directly inside
The only element allowed directly inside <ul> or <ol> is <li>. Text or other tags must go inside an <li>.

## Lists inside lists
A list item can hold another list. This is how menus and outlines are built:

    <ul>
      <li>Weapons
        <ul>
          <li>Bow</li>
        </ul>
      </li>
    </ul>

Notice the inner <ul> sits INSIDE the <li>, before its closing tag.

## Why indent?
Browsers do not care about indentation, but people do. Indenting each level makes it easy to see which items belong to which list."##,
            &["<ul> makes a bulleted list; <ol> makes a numbered list.", "Every item goes inside <li>.", "Only <li> may sit directly inside <ul> or <ol>.", "A nested list goes inside an <li>."],
            vec![
                quiz("You are writing the steps of a potion recipe. Which list fits?", &["<ul>", "<ol>", "<li>"], 1, "Steps happen in order, so use an ordered list."),
                quiz("What may sit directly inside a <ul>?", &["Any tag", "Only <li>", "Only text"], 1, "Wrap everything in <li>."),
                fill("Add one more item.", "<ul>\n  <li>Sword</li>\n  <___>Shield</li>\n</ul>", &["li", "ul", "item"], 0, "Items are <li> elements."),
            ]),
        d("Links", r##"## The anchor element
Links are what make the web a web. A link is an <a> (anchor) element. The address it goes to is written in the href attribute.

    <a href="https://example.com">Visit the guild</a>

- href is short for "hypertext reference": the destination.
- The text between the tags is what the reader clicks.

## Attributes
An attribute adds extra information to an element. It always goes inside the OPENING tag, as name="value". You will meet many attributes; href is the first.

## Kinds of addresses
- A full address starts with https:// and points to another website.
- A relative address like about.html points to another page in your own site.
- An address starting with # jumps to an element with that id on the same page: href="#shop".

## Opening in a new tab
Add target="_blank" to open the link in a new tab:

    <a href="https://example.com" target="_blank">Map</a>

## Write good link text
"Click here" tells the reader nothing, and screen-reader users often jump from link to link. Write text that describes where the link goes, like "Read the quest rules"."##,
            &["Links use the <a> element.", "The destination goes in the href attribute.", "Attributes live inside the opening tag as name=\"value\".", "target=\"_blank\" opens a new tab; describe the destination in the link text."],
            vec![
                quiz("Where does an attribute go?", &["In the closing tag", "In the opening tag", "Between the tags"], 1, "Attributes always sit in the opening tag."),
                quiz("Which link text is best?", &["Click here", "Read the quest rules", "Link"], 1, "Good link text says where it goes."),
                fill("Open the link in a new tab.", "<a href=\"map.html\" target=\"___\">Map</a>", &["_blank", "new", "tab"], 0, "target=\"_blank\" opens a new tab."),
            ]),
        d("Images", r##"## Adding a picture
The <img> element shows an image. It is empty (no closing tag) and needs two attributes:

    <img src="dragon.png" alt="A red dragon sleeping on gold">

- src (source) is the file name or web address of the image.
- alt (alternative text) describes the image in words.

## Why alt matters
Alt text is read aloud by screen readers for people who cannot see the image. It also appears if the image fails to load, and search engines read it. Describe what the picture shows and why it is there. If an image is purely decoration, use an empty alt="" so screen readers skip it.

## Where is the file?
If src is just a name like dragon.png, the browser looks in the same folder as the HTML page. A path like img/dragon.png looks inside a folder called img. A full https:// address loads an image from another website.

## Size
You can set width and height in pixels:

    <img src="dragon.png" alt="Dragon" width="200">

Setting only one keeps the shape. Later, CSS is the better place to control size.

## Images as links
Put an <img> inside an <a> to make a clickable picture."##,
            &["<img> shows a picture and has no closing tag.", "src says where the image file is.", "alt describes the image for screen readers and broken images.", "Relative paths are looked up next to the HTML file."],
            vec![
                quiz("Who reads the alt text?", &["Screen readers and search engines", "Only the designer", "Nobody"], 0, "Alt text makes images accessible."),
                quiz("An image is only decoration. What alt should it have?", &["alt=\"image\"", "alt=\"\"", "No alt at all"], 1, "An empty alt tells screen readers to skip it."),
                fill("Load the picture from the img folder.", "<img src=\"___\" alt=\"Knight\">", &["img/knight.png", "knight", "<knight.png>"], 0, "A folder name, a slash, then the file name."),
            ]),
        d("Project: Wanted Poster", r##"## Your first real page
The sheriff needs a WANTED poster for the Goblin King, and the Elements chapter gave you every tool for it. Plan the poster before you write a single tag:

- A big title: WANTED in an <h1>.
- A picture of the outlaw, with alt text for anyone who can't see it.
- The reward in a paragraph, with the amount in bold.
- A list of things to look out for.

## Build it step by step
Write one element, look at the preview, then add the next:

    <h1>WANTED</h1>
    <img src="goblin.png" alt="Goblin King">
    <p>Reward: <b>500 gold</b></p>
    <ul>
      <li>Green skin</li>
      <li>Loves shiny coins</li>
    </ul>

## Why each tag?
- <h1> is the one main title of the page, so screen readers and search engines know what it is about.
- alt="Goblin King" is read aloud when the picture can't be seen, and shown if the image fails to load.
- <b> makes the reward stand out.
- <ul> with <li> items makes a bullet list. The order of the clues doesn't matter, so the list is unordered.

## Check your work
- Does every opening tag have a matching closing tag (except <img>, which has none)?
- Is </b> closed before </p>?
- Would someone who can't see the picture still know who is wanted?

These three checks catch most beginner mistakes on any page."##,
            &["Plan the parts (title, picture, details, list) before coding.", "Build one element at a time and check the preview.", "Every image needs alt text that says what it shows.", "Close tags in the right order: </b> before </p>."],
            vec![
                quiz("What is the poster's main title element?", &["<h1>", "<title>", "<p>"], 0, "The one main title of the page is the <h1>."),
                fill("Make the clues a bullet list.", r##"<___>
  <li>Green skin</li>
</ul>"##, &["ul", "ol", "li"], 0, "<ul> starts an unordered (bullet) list."),
                quiz("Which line is broken?", &["<li>Green skin</li>", "<h1>WANTED<h1>", "<p>Reward: <b>500 gold</b></p>"], 1, "The closing </h1> is missing its slash."),
            ]),
        d("Page Skeleton", r##"## Every page has the same skeleton
A complete HTML file follows the same shape every time:

    <!DOCTYPE html>
    <html>
      <head>
        <title>My Guild</title>
      </head>
      <body>
        <h1>Welcome!</h1>
      </body>
    </html>

## Line by line
- <!DOCTYPE html> tells the browser "this is modern HTML". It is not a tag and has no closing part. It must be the very first line.
- <html> wraps the whole document.
- <head> holds information ABOUT the page that is not shown in the page itself: the title, links to CSS, the character set.
- <title> is the text shown on the browser tab and in search results.
- <body> holds everything visible: headings, paragraphs, images.

## Head versus body
A quick test: if the visitor should SEE it on the page, it goes in <body>. If it is instructions or labels for the browser, it goes in <head>.

## Useful head lines
You will often see <meta charset="UTF-8">, which makes sure letters like é and emoji display correctly. Put it first inside <head>.

## Common mistake
Putting <h1> in the <head> or the <title> in the <body>. The page may still appear, but it is invalid and can behave strangely."##,
            &["<!DOCTYPE html> comes first and declares modern HTML.", "<html> wraps everything; it contains <head> and <body>.", "<head> holds info about the page, like the <title> on the tab.", "<body> holds everything the visitor sees."],
            vec![
                quiz("Where does the <title> go?", &["In <body>", "In <head>", "After </html>"], 1, "The title is page information, so it lives in the head."),
                quiz("Where does an <h1> go?", &["In <head>", "In <body>", "In <title>"], 1, "Visible content goes in the body."),
                fill("The first line of every page:", "<!___ html>", &["DOCTYPE", "HTML", "START"], 0, "<!DOCTYPE html> declares modern HTML."),
            ]),
        d("Nesting and Indentation", r##"## Elements inside elements
HTML is built like boxes inside boxes. When one element sits inside another, we call it nesting. The outer element is the parent and the inner one is the child.

    <body>
      <ul>
        <li>Torch</li>
      </ul>
    </body>

Here <body> is the parent of <ul>, and <ul> is the parent of <li>.

## The golden rule
Close elements in the reverse order you opened them. The last box you opened is the first one you close.

    Right: <p><em>Hello</em></p>
    Wrong: <p><em>Hello</p></em>

When nesting is broken, browsers guess what you meant, and the guess is often wrong.

## Indentation
Each time you go one level deeper, indent by two spaces (or one tab). Indentation does not change the page at all, but it lets you see the family tree at a glance and spot missing closing tags.

## A family tree
Developers talk about parents, children, siblings (elements side by side with the same parent) and ancestors. CSS and JavaScript use the same words, so it pays to learn them now."##,
            &["Nesting means putting elements inside other elements.", "The outer element is the parent, the inner one the child.", "Close tags in reverse order of opening.", "Indent one level per depth so the structure is easy to read."],
            vec![
                quiz("In <ul><li>Torch</li></ul>, what is the <ul>?", &["The child", "The parent", "A sibling"], 1, "The <ul> contains the <li>."),
                quiz("Which is correctly nested?", &["<p><b>Hi</p></b>", "<p><b>Hi</b></p>", "<b><p>Hi</b></p>"], 1, "Close the inner element first."),
                fill("Close the elements in the right order.", "<p><em>Hello</em><___>", &["/p", "/em", "p"], 0, "The paragraph was opened first, so it closes last."),
            ]),
        d("Div and Span", r##"## Containers with no meaning
Sometimes you need to group content just so you can style or move it together, and no meaningful element fits. That is what <div> and <span> are for. They mean nothing on their own.

## <div>: a block box
A <div> (division) groups larger chunks. It is a block element: it starts on a new line and stretches across the full width.

    <div>
      <h2>Inventory</h2>
      <p>3 potions</p>
    </div>

## <span>: an inline box
A <span> wraps a few words inside a line of text without breaking the line.

    <p>You found <span>50 gold</span>!</p>

## Block versus inline
- Block elements (<div>, <p>, <h1>, <ul>) stack on top of each other.
- Inline elements (<span>, <a>, <strong>, <img>) flow inside text.

## Use them last
First ask: is there a meaningful element? A list should be a <ul>, a heading an <h2>, a navigation area a <nav>. Use <div> and <span> only when nothing else fits. You will usually give them a class so CSS can find them, which is the next lesson."##,
            &["<div> and <span> group content but add no meaning.", "<div> is a block: new line, full width.", "<span> is inline: it stays inside the line of text.", "Prefer meaningful elements; use div and span when nothing else fits."],
            vec![
                quiz("Which element stays inside a line of text?", &["<div>", "<span>", "<p>"], 1, "<span> is inline."),
                quiz("You have a list of quests. What should you use?", &["A <div> for each quest", "A <ul> with <li> items", "Many <span>s"], 1, "Use the meaningful element when one exists."),
                fill("Group a heading and a paragraph into one box.", "<___>\n  <h2>Shop</h2>\n  <p>Open daily</p>\n</div>", &["div", "span", "box"], 0, "A <div> groups block content."),
            ]),
        d("Classes and IDs", r##"## Naming your elements
CSS and JavaScript need a way to find specific elements. HTML gives you two attributes for that: class and id.

## class: a label many can share
A class is like a team jersey. Many elements can wear the same class, and one element can wear several classes separated by spaces.

    <p class="warning">Mind the trap!</p>
    <p class="warning big">Dragon ahead!</p>

## id: one unique name
An id is like a name tag: only ONE element on the page may have a given id.

    <div id="inventory">...</div>

ids are also targets for links: <a href="#inventory"> jumps to that element.

## Naming rules
- Names cannot contain spaces (a space separates two class names).
- Use lowercase words joined by hyphens: quest-card, main-menu.
- Choose names that describe WHAT it is (quest-card), not how it looks (red-box), because looks change.

## Class or id?
Use a class for anything that could appear more than once, which is most things. Save ids for truly unique parts, like the main menu or a section you link to."##,
            &["class and id give elements names for CSS and JavaScript.", "Many elements can share a class; an element can have several classes.", "An id must be unique on the page and can be a link target.", "Name by meaning (quest-card), using lowercase and hyphens."],
            vec![
                quiz("How many elements may share one id?", &["One", "Two", "Any number"], 0, "ids are unique."),
                quiz("How do you give an element two classes?", &["class=\"big warning\"", "class=\"big\" class=\"warning\"", "class=\"big,warning\""], 0, "Separate class names with a space."),
                fill("Link to the element with id shop.", "<a href=\"___\">Go to shop</a>", &["#shop", ".shop", "shop.id"], 0, "# + id jumps to that element."),
            ]),
        d("Comments and Attributes", r##"## Comments
A comment is a note for humans. The browser skips it completely.

    <!-- Main menu starts here -->

Comments start with <!-- and end with -->. Use them to explain WHY something is there, to label big sections, or to switch off a piece of code temporarily while testing. Remember that anyone can view your page's source, so never put passwords or secrets in comments.

## Attributes, in depth
You have already used href, src, alt, class and id. All attributes follow the same pattern inside the opening tag:

    <img src="map.png" alt="Old map" width="300">

- Write them as name="value", separated by spaces.
- Always use quotes around values. They are required when the value contains spaces.
- The order of attributes does not matter.

## Boolean attributes
Some attributes are just on or off. Writing the name is enough:

    <input type="checkbox" checked>

## The title attribute
title="..." adds a small tooltip when the mouse rests on an element. It is handy for extra hints, but do not hide important information there, because touch screens cannot hover."##,
            &["Comments look like <!-- note --> and are ignored by the browser.", "Comments are visible in the page source, so never put secrets in them.", "Attributes are name=\"value\" pairs in the opening tag, in any order.", "Boolean attributes like checked are on just by being present."],
            vec![
                quiz("Who sees an HTML comment?", &["Visitors on the page", "Nobody, ever", "Anyone who views the source code"], 2, "Comments are hidden on the page but not in the source."),
                quiz("Which attribute adds a hover tooltip?", &["title", "alt", "tip"], 0, "title shows a tooltip."),
                fill("End the comment.", "<!-- shop section ___", &["-->", "--!>", "/-->"], 0, "Comments close with -->."),
            ]),
        d("Project: Hero Profile Card", r##"## A card made of boxes
Cards are everywhere on the web: profile cards, product cards, game cards. A card is a box (<div>) holding a picture, a name and a few details. In this project you build a profile card for Ada the Brave with the Structure chapter skills.

## Plan the boxes
- An outer <div class="card"> holds everything.
- Inside: an <img> avatar, 120 pixels wide.
- An <h2> with the hero's name, marked with id="name".
- A <p class="role"> with the level, and the role word wrapped in a <span>.

## Build it
    <div class="card">
      <img src="ada.png" alt="Ada" width="120">
      <h2 id="name">Ada the Brave</h2>
      <p class="role">Level 4 <span>Wizard</span></p>
    </div>

## Why class, id and span?
- class="card" is a label many elements can share: every hero card on the page can use it, so the CSS course can style them all at once.
- id="name" is unique: only one element on the page may have it, which suits a single, special part.
- <span> wraps a few words inside a line without starting a new line, so you can style just "Wizard" later.

## Check
Indent each level inside the div, close tags in reverse order (</span> before </p>, </p> before </div>), and give the image honest alt text."##,
            &["A card is an outer <div> that groups related content.", "Classes label parts many elements share; an id is unique.", "<span> marks a few words inside a line.", "Indent each level and close tags in reverse order."],
            vec![
                quiz("Why give the card a class instead of an id?", &["Many cards can share one class", "Classes are faster", "ids are not allowed on divs"], 0, "Every hero card can share class=\"card\"; an id must be unique."),
                fill("Wrap the role word without starting a new line.", r##"<p class="role">Level 4 <___>Wizard</span></p>"##, &["span", "div", "h2"], 0, "<span> is the inline container; a div would start a new line."),
                quiz("How wide is the avatar picture?", &["120 pixels", "120 percent", "As wide as the card"], 0, "width=\"120\" on an <img> means 120 pixels."),
            ]),
        d("Your First Form", r##"## Forms collect information
Sign-up pages, search boxes and checkout pages are all forms. A form gathers what the visitor types or picks and sends it somewhere.

    <form action="/join" method="post">
      <input type="text" name="hero">
      <button type="submit">Join</button>
    </form>

## The parts
- <form> wraps all the fields that belong together.
- action is WHERE the data is sent (a page or server address).
- method is HOW: "get" puts the data in the address bar (good for searches), "post" sends it hidden (good for sign-ups and passwords).
- <input> is a field. It is empty, so it has no closing tag.
- name="hero" is the label the data travels with. Without a name, the value is not sent at all.
- <button type="submit"> sends the form.

## What actually gets sent?
If the visitor types Pix, the server receives hero=Pix. That pairing of name and value is how every form field travels.

## Placeholders
placeholder="Your name" shows faint example text inside an empty field. It disappears when typing starts, so it is a hint, not a replacement for a proper label."##,
            &["<form> groups fields and sends them to the action address.", "method=\"get\" shows data in the address; \"post\" sends it hidden.", "Every field needs a name, or its value is not sent.", "<button type=\"submit\"> sends the form."],
            vec![
                quiz("Which attribute says where the form data goes?", &["method", "action", "name"], 1, "action is the destination."),
                quiz("A field has no name attribute. What happens when the form is sent?", &["Its value is not sent", "It is sent as 'field'", "The form breaks"], 0, "Only named fields are sent."),
                fill("Send the data hidden from the address bar.", "<form action=\"/join\" method=\"___\">", &["post", "get", "hide"], 0, "post keeps data out of the address bar."),
            ]),
        d("Input Types", r##"## One element, many fields
<input> changes completely depending on its type attribute. Picking the right type gives visitors the right keyboard on phones and free checks in the browser.

    <input type="text">      one line of text
    <input type="email">     must look like an email
    <input type="password">  hides what is typed
    <input type="number">    numbers, with up/down arrows
    <input type="date">      a date picker
    <input type="color">     a colour picker
    <input type="range">     a slider

## Why it matters
- On a phone, type="email" shows a keyboard with @, and type="number" shows a number pad.
- type="email" stops the form from sending "pix" without an @.
- type="password" shows dots so nobody can read over your shoulder.

## Extra attributes per type
Numbers and ranges accept min, max and step:

    <input type="number" name="age" min="1" max="120">

## If the browser does not know a type
Unknown or very new types fall back to a plain text box. So it is always safe to use the most specific type you can."##,
            &["The type attribute decides what kind of field an <input> is.", "Use email, password, number and date for the right keyboard and checks.", "number and range accept min, max and step.", "Unknown types fall back to text, so use the most specific one."],
            vec![
                quiz("Which type hides the characters as you type?", &["hidden", "password", "secret"], 1, "type=\"password\" shows dots."),
                quiz("Why use type=\"email\" instead of text?", &["It looks prettier", "Phones show an @ keyboard and the browser checks the format", "It sends faster"], 1, "The right type gives the right keyboard and checks."),
                fill("Make a slider from 0 to 10.", "<input type=\"___\" min=\"0\" max=\"10\">", &["range", "slider", "number-line"], 0, "type=\"range\" is a slider."),
            ]),
        d("Labels", r##"## Every field needs a label
A label is the visible text that says what a field is for, like "Hero name". Without labels, a form is a row of mystery boxes, and screen readers cannot tell users what to type.

## Connecting a label to its field
Give the input an id, and point the label's for attribute at that id:

    <label for="hero">Hero name</label>
    <input type="text" id="hero" name="hero">

Now the browser knows they belong together.

## Bonus: bigger click area
Clicking a connected label puts the cursor in its field, and clicking the label of a checkbox ticks it. This helps everyone, especially on small touch screens.

## The wrapping shortcut
You can also put the input inside the label. Then no for or id is needed:

    <label>Hero name <input type="text" name="hero"></label>

## Labels versus placeholders
A placeholder disappears as soon as you start typing, so people forget what the field was for. Always use a real label, and only use a placeholder for an example such as "e.g. pix@guild.com".

## name, id and for
These three are easy to mix up: name is for sending data, id identifies the element on the page, and for links a label to an id."##,
            &["Every field needs a visible <label>.", "Connect them with for=\"x\" on the label and id=\"x\" on the input.", "Clicking a connected label focuses or ticks its field.", "Placeholders are examples, not replacements for labels."],
            vec![
                quiz("The label has for=\"mail\". What must the input have?", &["name=\"mail\"", "id=\"mail\"", "for=\"mail\""], 1, "for points at an id."),
                quiz("Why is a placeholder not enough?", &["It disappears while typing", "It is too big", "Screen readers shout it"], 0, "Once you type, the hint is gone."),
                fill("Connect the label.", "<label ___=\"pet\">Pet</label>\n<input id=\"pet\">", &["for", "name", "id"], 0, "for on the label matches the input's id."),
            ]),
        d("Checkboxes and Radios", r##"## Picking from options
When visitors choose from options instead of typing, use checkboxes or radio buttons.

## Checkboxes: pick any number
Each checkbox is independent. Visitors can tick none, one or all of them.

    <input type="checkbox" id="map" name="gear" value="map">
    <label for="map">Map</label>

## Radio buttons: pick exactly one
Radio buttons work as a group: choosing one un-chooses the others, like the buttons on an old radio. The group is formed by giving them all the SAME name.

    <input type="radio" id="w" name="class" value="warrior">
    <label for="w">Warrior</label>
    <input type="radio" id="m" name="class" value="mage">
    <label for="m">Mage</label>

## value is what gets sent
A ticked box sends its name and value, such as class=mage. If you forget value, the browser sends the word "on", which is not helpful.

## Ticked from the start
Add the boolean attribute checked to pre-select an option.

## Which one to use?
Ask: "Can the visitor choose more than one?" Yes → checkboxes. No → radios. For a single yes/no (like "I agree to the rules"), use one checkbox."##,
            &["Checkboxes allow any number of choices.", "Radio buttons allow exactly one choice per group.", "Radios with the same name form one group.", "value is what gets sent; checked pre-selects an option."],
            vec![
                quiz("What makes several radio buttons one group?", &["The same id", "The same name", "The same value"], 1, "Same name = one group."),
                quiz("'Choose your favourite foods (any number).' Which input?", &["Radio buttons", "Checkboxes", "A password field"], 1, "More than one choice means checkboxes."),
                fill("Tick this option from the start.", "<input type=\"checkbox\" name=\"news\" ___>", &["checked", "ticked", "on"], 0, "checked pre-selects it."),
            ]),
        d("Text Areas and Dropdowns", r##"## Longer text: <textarea>
<input type="text"> holds a single line. For messages, bios or feedback, use <textarea>, which allows many lines. Unlike <input>, it has a closing tag.

    <label for="bio">Your story</label>
    <textarea id="bio" name="bio" rows="4"></textarea>

rows sets how many lines are visible. Any text between the tags becomes the starting content, so keep it empty unless you want default text.

## Dropdowns: <select>
A dropdown lets visitors choose one option from a list that stays folded up until clicked. It saves space when there are many choices.

    <label for="realm">Realm</label>
    <select id="realm" name="realm">
      <option value="north">North Keep</option>
      <option value="south">South Shore</option>
    </select>

- <select> is the box; each <option> is a choice.
- value is what gets sent; the text between the tags is what people see.
- Add selected to an option to pre-choose it.

## Dropdown or radios?
With 2 to 5 options, radio buttons are better because people can see every choice at once. With many options (countries, years), a dropdown keeps the page tidy."##,
            &["<textarea> is for multi-line text and has a closing tag.", "rows sets the visible height of a textarea.", "<select> holds <option>s and makes a dropdown.", "An option's value is sent; its text is shown."],
            vec![
                quiz("Which field suits a long message?", &["<input type=\"text\">", "<textarea>", "<select>"], 1, "<textarea> allows many lines."),
                quiz("For an <option>, what is sent with the form?", &["Its text", "Its value", "Its position"], 1, "value is sent."),
                fill("Add a choice to the dropdown.", "<select name=\"pet\">\n  <___ value=\"cat\">Cat</option>\n</select>", &["option", "item", "choice"], 0, "Choices are <option>s."),
            ]),
        d("Validation", r##"## Catching mistakes before sending
Validation means checking that the answers make sense before the form is sent. HTML can do a lot of this for free, without any JavaScript.

## Built-in checks
- required: the field cannot be empty.
- minlength / maxlength: limit the number of characters.
- min / max: limit numbers and dates.
- type="email" and type="url" check the format.

    <input type="text" name="hero" required minlength="3">
    <input type="number" name="age" min="10" max="99">

When something is wrong, the browser stops the form and shows a message next to the field.

## pattern
For special formats, pattern takes a rule (a regular expression). For example, pattern="[A-Z]{3}" means exactly three capital letters. Patterns are powerful but hard to read, so explain the rule to the user with a label or hint.

## Help people succeed
- Mark required fields clearly, for example "Name (required)".
- Say what format you expect before people make a mistake.

## An important warning
Browser checks are for convenience. Anyone can switch them off, so a real website must check the data again on the server."##,
            &["required stops a field from being left empty.", "minlength/maxlength limit text; min/max limit numbers.", "type=\"email\" and pattern check formats.", "Browser checks help users, but servers must check again."],
            vec![
                quiz("Which attribute makes a field mandatory?", &["needed", "required", "must"], 1, "required blocks empty fields."),
                quiz("Why must a server check the data again?", &["Browser checks can be switched off", "Servers are faster", "HTML checks only work offline"], 0, "Never trust only the browser."),
                fill("A name of at least 3 letters.", "<input name=\"hero\" ___=\"3\">", &["minlength", "min", "length"], 0, "minlength counts characters."),
            ]),
        d("Project: Tavern Sign-Up", r##"## Build a complete form
The Pixel Tavern needs a sign-up sheet for its quest board. You will build one complete form with the Forms chapter skills.

## Plan the fields first
- A heading, so people know what the form is for.
- Hero name: a text box that can't be left empty, with a label.
- Class: a dropdown with the choices Knight and Mage.
- A button that sends the form.

## Build it
    <form>
      <h2>Join the Tavern</h2>
      <label for="nm">Name</label>
      <input type="text" id="nm" required>
      <select>
        <option>Knight</option>
        <option>Mage</option>
      </select>
      <input type="submit" value="Sign up">
    </form>

## How the parts connect
- for="nm" on the label matches id="nm" on the input. Clicking the word Name now puts the cursor in the box, and screen readers read the label aloud.
- required stops the form from being sent while the name is empty.
- A <select> shows its <option>s as a dropdown: perfect when there are several choices and exactly one may be picked.
- type="submit" turns the input into the button that sends everything.

## Final checklist
- Every field has a label connected with for and id.
- Required fields are marked as required.
- The button's value says what will happen: Sign up."##,
            &["Plan every field and its type before writing code.", "Connect each label to its field with for and id.", "required stops empty fields from being sent.", "A <select> dropdown lets people pick one choice."],
            vec![
                quiz("What does for=\"nm\" on the label connect to?", &["The input with id=\"nm\"", "The form's name", "The submit button"], 0, "for matches the id of the field it labels."),
                fill("Add the Mage choice to the dropdown.", r##"<select>
  <option>Knight</option>
  <___>Mage</option>
</select>"##, &["option", "select", "li"], 0, "Each choice in a dropdown is an <option>."),
                quiz("Why a dropdown for the class?", &["The hero picks exactly one class from a list", "Dropdowns are required in forms", "So they can type any class"], 0, "A <select> shows a list and keeps one choice."),
            ]),
        d("Why Semantics?", r##"## Meaningful tags
"Semantic" means "about meaning". Semantic HTML uses elements that describe what content is: <nav> for navigation, <article> for a story, <footer> for the bottom of the page.

## The same page, two ways
You could build a whole page out of <div>s:

    <div class="top">...</div>
    <div class="menu">...</div>

or with semantic elements:

    <header>...</header>
    <nav>...</nav>

Both can look identical after CSS. The difference is who can understand them.

## Who benefits
- Screen readers: a blind visitor can jump straight to the <nav> or <main> content.
- Search engines: they understand which part is the main article and rank pages better.
- Other developers (and future you): <footer> is clearer than <div class="ft">.
- Browsers: reader modes and smart features rely on semantic tags.

## A simple rule
Before reaching for a <div>, ask: "Is there an element that says what this is?" Headings, lists, links, buttons, forms and the layout elements in this chapter cover most of a page. Use <div> for pure styling boxes only."##,
            &["Semantic elements describe what content is.", "They help screen readers, search engines and other developers.", "Divs and semantic elements can look the same; they differ in meaning.", "Reach for a meaningful element before using a div."],
            vec![
                quiz("What does 'semantic' mean?", &["About colour", "About meaning", "About speed"], 1, "Semantic elements carry meaning."),
                quiz("Who benefits most when a page uses <nav> and <main>?", &["Screen-reader users", "Nobody", "Only the designer"], 0, "They can jump straight to the right part."),
                fill("A meaningful element for site navigation:", "<___>\n  <a href=\"/\">Home</a>\n</nav>", &["nav", "div", "menu-box"], 0, "<nav> marks navigation."),
            ]),
        d("Header and Nav", r##"## <header>
<header> holds the introduction to a page or a section. At the top of a site it usually contains the logo, the site name and the main menu.

    <header>
      <h1>Pixel Guild</h1>
      <nav>
        <a href="/">Home</a>
        <a href="/quests">Quests</a>
      </nav>
    </header>

A page can have more than one header: an <article> can have its own header with its title and author.

## <nav>
<nav> wraps a group of MAIN navigation links: the site menu, a table of contents, page numbers. Not every group of links needs a <nav>. A few links in a footer or inside a paragraph are just links.

## Menus as lists
Many developers put the links in a list, because a menu IS a list of choices:

    <nav>
      <ul>
        <li><a href="/">Home</a></li>
      </ul>
    </nav>

Screen readers then announce "list, 4 items", which helps people know how long the menu is.

## Common mix-up
<header> is NOT the same as <head>. <head> is the invisible information part of the skeleton; <header> is a visible area of the page body."##,
            &["<header> introduces a page or a section (logo, title, menu).", "<nav> wraps main navigation links.", "Menus are often lists of links inside <nav>.", "<header> is visible content; <head> is page information."],
            vec![
                quiz("What is the difference between <head> and <header>?", &["None", "<head> is page info; <header> is visible content", "<header> goes inside <title>"], 1, "Different jobs, similar names."),
                quiz("Should every group of links be in a <nav>?", &["Yes", "No, only main navigation", "Only in footers"], 1, "<nav> is for major navigation blocks."),
                fill("Wrap the top of the page.", "<___>\n  <h1>Pixel Guild</h1>\n</header>", &["header", "head", "top"], 0, "<header> is the visible intro area."),
            ]),
        d("Main and Footer", r##"## <main>
<main> holds the main content of the page: the part that is unique to this page, not repeated on every page like the menu or footer.

    <body>
      <header>...</header>
      <main>
        <h2>Today's quests</h2>
        ...
      </main>
      <footer>...</footer>
    </body>

Rules for <main>:
- Use only ONE visible <main> per page.
- Do not put it inside <header>, <nav>, <article> or <footer>.

Screen readers offer a "skip to main content" jump that uses this element, saving visitors from hearing the menu on every page.

## <footer>
<footer> holds closing information: copyright, contact details, links to rules or privacy pages. Like <header>, it can belong to the whole page or to a section or article (for example an article's author and date).

    <footer>
      <p>&copy; 2026 Pixel Guild</p>
    </footer>

## The classic page layout
Most pages follow the same order: header, nav, main and footer. Learn this shape and you can read almost any web page's code."##,
            &["<main> holds the content unique to this page.", "Use only one <main>, never inside header, nav or footer.", "<footer> holds closing info: copyright, contact, links.", "Typical layout: header, nav, main, footer."],
            vec![
                quiz("How many visible <main> elements should a page have?", &["One", "One per section", "None"], 0, "Exactly one main area."),
                quiz("Where does the copyright line usually go?", &["<main>", "<footer>", "<nav>"], 1, "Closing info belongs in the footer."),
                fill("Wrap the page's unique content.", "<___>\n  <h2>Quests</h2>\n</main>", &["main", "body", "content"], 0, "<main> holds the main content."),
            ]),
        d("Article and Section", r##"## <article>: stands on its own
An <article> is content that would still make sense if you cut it out and put it somewhere else: a blog post, a news story, a forum post, a product card, a comment.

    <article>
      <h2>The Slime Uprising</h2>
      <p>Last night, slimes...</p>
    </article>

A test: could you share it as its own link or print it alone? Then it is an article.

## <section>: a themed part
A <section> groups content about one topic inside a page or article, and it normally starts with a heading.

    <article>
      <h2>Potion Guide</h2>
      <section>
        <h3>Healing potions</h3>
      </section>
      <section>
        <h3>Mana potions</h3>
      </section>
    </article>

## Section or div?
If the group has a heading and a clear topic, use <section>. If you just need a box for styling, use <div>.

## They can nest both ways
An article can contain sections (chapters of a post), and a section can contain articles (a "Latest news" section full of stories)."##,
            &["<article> is self-contained content (post, story, card).", "<section> groups one topic and usually starts with a heading.", "No heading and just for styling? Use a <div>.", "Articles can hold sections and sections can hold articles."],
            vec![
                quiz("A single blog post should be a...", &["<section>", "<article>", "<div>"], 1, "It stands on its own."),
                quiz("A group needs a box only for styling and has no heading. Use:", &["<section>", "<div>", "<article>"], 1, "Div is the neutral box."),
                fill("A themed part of the guide:", "<___>\n  <h3>Mana potions</h3>\n</section>", &["section", "article", "part"], 0, "<section> groups one topic."),
            ]),
        d("Figures", r##"## Pictures with captions
When an image, diagram, chart or code example comes with a caption, wrap it in <figure> and put the caption in <figcaption>.

    <figure>
      <img src="map.png" alt="Map of the northern realms">
      <figcaption>The northern realms in the Age of Frost.</figcaption>
    </figure>

The <figcaption> can go first or last inside the figure. Browsers connect the caption to the content, which helps screen readers.

## alt versus figcaption
They do different jobs:
- alt replaces the image for people who cannot see it (describe what is shown).
- figcaption is visible to everyone and adds context (explain why it matters, credit the artist).

You usually need both.

## Not only images
A figure can hold anything referenced by the text: a code sample, a poem, a table, a video. The idea is "this could be moved to an appendix and the text would still make sense".

## <time>
While you are adding meaning: <time datetime="2026-05-01">May Day</time> marks a date in a way computers can read, which is useful in articles and event pages."##,
            &["<figure> wraps an image, chart or code example that goes with a caption.", "<figcaption> is the visible caption, first or last inside the figure.", "alt describes the image; figcaption adds context. Use both.", "<time datetime=\"...\"> marks dates for computers."],
            vec![
                quiz("Where does the caption go?", &["In alt", "In <figcaption> inside the <figure>", "In <title>"], 1, "figcaption sits inside the figure."),
                quiz("Do you still need alt when there is a figcaption?", &["No", "Yes, they do different jobs", "Only for videos"], 1, "alt replaces the image; the caption adds context."),
                fill("Caption the picture.", "<figure>\n  <img src=\"orb.png\" alt=\"Glowing orb\">\n  <___>The Orb of Code</figcaption>\n</figure>", &["figcaption", "caption", "label"], 0, "Captions use <figcaption>."),
            ]),
        d("Final Project: Adventure Blog", r##"## Put the whole course together
Your final HTML project is Ada's adventure blog, built with real semantic structure: every part of the page says what it is.

## The outline
- A <header> at the top with the blog's name and a <nav> menu.
- A <main> area holding the blog posts.
- Each post is an <article> with its own heading.
- A <footer> at the bottom.

## Build it
    <header>
      <h1>Ada's Adventure Log</h1>
      <nav><a href="#day1">Day 1</a></nav>
    </header>
    <main>
      <article id="day1">
        <h2>Day 1: The Cave</h2>
        <p>I found a <b>glowing gem</b>.</p>
      </article>
    </main>
    <footer>Thanks for reading!</footer>

## Why semantic tags?
Divs would look the same, but these tags tell browsers, search engines and screen readers what each part is. A screen-reader user can jump straight to the <main> content or the <nav> menu.

## The jump link
The link's href="#day1" points to the element with id="day1". Clicking Day 1 scrolls straight to that article: a menu that works without a second page.

## Review like a pro
Read your code from top to bottom: does each element say what its content is? Are all tags closed and indented? If yes, you have finished the HTML course. Next stop: CSS, to make it beautiful."##,
            &["Header + nav at the top, main with articles, footer at the bottom.", "Each blog post is a self-contained <article>.", "href=\"#id\" jumps to the element with that id.", "Semantic tags help screen readers and search engines."],
            vec![
                quiz("Where does the Day 1 link take you?", &["To the article with id=\"day1\"", "To a new page called day1", "To the top of the page"], 0, "#day1 jumps to the element with that id."),
                fill("Put the posts in the page's main area.", r##"<___>
  <article id="day1">...</article>
</main>"##, &["main", "body", "section"], 0, "<main> holds the page's main content."),
                quiz("Which tag holds the closing message at the bottom?", &["<footer>", "<header>", "<nav>"], 0, "The footer closes the page."),
            ]),
    ]
}
