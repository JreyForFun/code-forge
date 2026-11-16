//! CSS: the full explanation, key points and extra practice for every lesson.
use super::*;

pub fn deep() -> Vec<Deep> {
    vec![
        d("What Is CSS?", r##"## The paint for your page
HTML builds the structure of a page; CSS (Cascading Style Sheets) decides how it looks: colours, fonts, spacing, layout. Keeping the two apart means you can restyle a whole site without touching its content.

## A rule has three parts
    h1 {
      color: gold;
    }

- The selector (h1) picks WHICH elements to style.
- The property (color) is WHAT to change.
- The value (gold) is HOW to change it.

Properties and values sit inside curly braces. Each pair is called a declaration, ends with a semicolon, and has a colon between property and value.

## Many declarations
    p {
      color: white;
      font-size: 18px;
    }

## Common mistakes
A missing semicolon makes the NEXT line fail too, because the browser reads both lines as one broken declaration. A missing closing brace can break every rule after it. When a style "does nothing", check the punctuation first.

## Why "cascading"?
When several rules target the same element, CSS has a set of rules for who wins. You will learn them step by step; for now, remember that a later rule beats an earlier one of the same strength."##,
            &["CSS controls how HTML looks.", "A rule = selector { property: value; }.", "Each declaration ends with a semicolon.", "When rules conflict, the later one of equal strength wins."],
            vec![
                quiz("In p { color: red; }, what is 'color'?", &["The selector", "The property", "The value"], 1, "The property is what you change."),
                quiz("What separates a property from its value?", &["A colon", "A semicolon", "An equals sign"], 0, "property: value;"),
                fill("End the declaration.", "h1 { color: gold___ }", &[";", ":", ","], 0, "Declarations end with a semicolon."),
            ]),
        d("Linking CSS", r##"## Three places to write CSS
1. Inline, in a style attribute:

    <p style="color: red;">Hot!</p>

2. Internal, in a <style> element inside <head>:

    <style>
      p { color: red; }
    </style>

3. External, in a separate .css file linked from <head>:

    <link rel="stylesheet" href="style.css">

## Why external wins
An external file can be shared by every page of your site. Change one line in style.css and every page updates. It also keeps your HTML short and readable. Professionals use external stylesheets for almost everything.

## The link element
- rel="stylesheet" says what the linked file is.
- href is the path to the file, just like a link or image path.
- It goes inside <head> and is empty (no closing tag).

## When inline is useful
Inline styles beat other rules and only affect one element, which makes them hard to manage. Use them rarely, for quick tests or styles set by JavaScript.

## Common mistake
If your styles do not show up, check that the file name in href matches exactly (including capital letters) and that the .css file is in the folder you think it is."##,
            &["CSS can be inline, internal (<style>) or external (.css file).", "External stylesheets are shared by many pages and keep HTML clean.", "Link them with <link rel=\"stylesheet\" href=\"style.css\"> in <head>.", "Inline styles are hard to manage; use them rarely."],
            vec![
                quiz("Which way of adding CSS can style every page of a site at once?", &["Inline styles", "An external stylesheet", "A <style> tag on one page"], 1, "One external file, many pages."),
                quiz("Where does the <link> to a stylesheet go?", &["In <head>", "At the end of <body>", "Inside <style>"], 0, "Page information lives in the head."),
                fill("Link the stylesheet.", "<link rel=\"___\" href=\"style.css\">", &["stylesheet", "css", "style"], 0, "rel=\"stylesheet\"."),
            ]),
        d("Type Selectors", r##"## Selecting by element name
The simplest selector is the element's name. It is called a type selector, and it styles EVERY element of that type on the page.

    p { color: white; }
    h2 { color: gold; }
    li { font-size: 18px; }

## The universal selector
* selects every element. It is often used to reset spacing:

    * { margin: 0; }

## Inheritance
Some properties, like color and font-family, are inherited: children take them from their parents. Style the body and almost all text follows:

    body {
      color: white;
      font-family: sans-serif;
    }

Box-related properties such as border, padding and background are NOT inherited, otherwise every child would get its own border.

## When type selectors are too broad
Styling p affects every paragraph. Usually you want to style only some of them, like the warning paragraphs. That is what classes are for, in the next lesson. A good habit: set calm defaults with type selectors (body, h1, a), then pick out special elements with classes."##,
            &["A type selector (p, h1, li) styles every element of that type.", "* selects every element.", "color and font-family are inherited by children; borders and padding are not.", "Set defaults with type selectors, then use classes for special cases."],
            vec![
                quiz("Which selector styles every element on the page?", &["*", "all", "body"], 0, "* is the universal selector."),
                quiz("You set color on body. What happens to the paragraphs inside?", &["They inherit the colour", "Nothing", "They turn black"], 0, "color is inherited."),
                fill("Make every list item bigger.", "___ { font-size: 20px; }", &["li", ".li", "#li"], 0, "Type selectors are plain element names."),
            ]),
        d("Class and ID Selectors", r##"## Targeting specific elements
To style only some elements, give them a class or id in HTML and select it in CSS.

- A class selector starts with a dot: .warning
- An id selector starts with a hash: #title

    <p class="warning">Trap ahead!</p>
    <h1 id="title">Guild Hall</h1>

    .warning { color: red; }
    #title { color: gold; }

## Combining
You can be more precise by joining a type and a class with no space: p.warning means "paragraphs that have the class warning". An element with two classes can be selected with both: .card.rare.

## Specificity: who wins?
When rules clash, the more specific selector wins, no matter the order:
- id selectors are the strongest of the three,
- then classes,
- then type selectors.

So #title beats .big, and .big beats h1. Because ids are so strong, they are hard to override later. Most developers style with classes and keep ids for links and JavaScript.

## Naming tip
Name classes by purpose (.quest-card, .error) rather than appearance (.red-text). If the design changes to blue, .error still makes sense."##,
            &[".name selects a class; #name selects an id.", "p.warning means paragraphs with the class warning.", "Specificity: id beats class, class beats type.", "Prefer classes for styling; keep ids for links and scripts."],
            vec![
                quiz("Which selector is the most specific?", &["h1", ".title", "#title"], 2, "ids are the most specific of the three."),
                quiz("What does p.note select?", &["All p and all .note elements", "Paragraphs with the class note", "A p inside .note"], 1, "No space means 'both at once'."),
                fill("Select the class 'rare'.", "___rare { color: purple; }", &[".", "#", "*"], 0, "Classes start with a dot."),
            ]),
        d("Grouping and Nesting", r##"## Grouping: one rule, many selectors
If several selectors need the same style, list them with commas instead of repeating the rule:

    h1, h2, h3 {
      font-family: Georgia, serif;
    }

Read the comma as "and also".

## Descendant selectors: a space
A space between selectors means "inside". This selects only links that are somewhere inside a nav:

    nav a { color: gold; }

Links elsewhere on the page are not affected.

## Child selector: >
> is stricter: direct children only, not grandchildren.

    ul > li { list-style: square; }

## Pseudo-classes
A colon adds a state: a:hover styles a link while the mouse is over it, and li:first-child picks the first item of each list.

    a:hover { color: white; }

## Read selectors right to left
For .menu li a, start at the right: "links, inside list items, inside .menu". Reading this way makes long selectors much easier to understand.

## Common mistake
.card .title (with a space) means a .title INSIDE a .card. .card.title (no space) means ONE element with both classes. One space changes everything."##,
            &["Commas group selectors that share a style.", "A space means 'inside' (descendant).", "> means direct child only.", ":hover and :first-child style states and positions."],
            vec![
                quiz("What does nav a select?", &["All nav and all a elements", "Links inside a nav", "A nav inside a link"], 1, "A space means 'inside'."),
                quiz("What is the difference between '.card .title' and '.card.title'?", &["None", "The first is a .title inside .card; the second is one element with both classes", "The second is invalid"], 1, "The space matters."),
                fill("Style links while the mouse is over them.", "a___ { color: white; }", &[":hover", ".hover", "#hover"], 0, "Pseudo-classes start with a colon."),
            ]),
        d("Project: Bard Concert Flyer", r##"## Style a flyer with selectors only
The bard is playing at the tavern and needs a flyer. The HTML is ready: a title, a date, a list of songs and a "special guest" note. Your job is to style it using everything from the Selectors chapter.

## Plan
- Set calm defaults on body: text colour and font.
- Make the h1 title stand out with its own colour.
- Group h2 and h3 to share a heading font.
- Style the song list items with a descendant selector: .songs li.
- Give the special guest paragraph a class and colour it.
- Add a hover colour to the ticket link.

## Example
    body { color: white; font-family: sans-serif; }
    h1 { color: gold; }
    h2, h3 { font-family: Georgia, serif; }
    .songs li { color: lightblue; }
    .guest { color: orange; }
    a:hover { color: white; }

## Check
- Did you use a class for the special paragraph instead of an id?
- Does .songs li only affect the song list, not every list on the page?
- Are all declarations closed with semicolons and all rules with braces?"##,
            &["Start with defaults on body, then style specific parts.", "Group selectors with commas to avoid repeating rules.", "Use descendant selectors to style only one area.", "Add :hover for interactive feedback."],
            vec![
                quiz("Which selector styles only the song list items?", &["li", ".songs li", "songs > *"], 1, "A descendant selector limits the area."),
                fill("Share one rule between h2 and h3.", "h2___ h3 { font-family: Georgia; }", &[",", " ", ">"], 0, "Commas group selectors."),
                quiz("The guest paragraph should be styled with...", &["a class", "an inline style", "the universal selector"], 0, "Classes are the reusable way."),
            ]),
        d("Colors", r##"## Ways to write a colour
CSS accepts colours in several formats. They all describe the same colours, so pick whichever is clearest.

- Names: red, gold, teal, white. There are about 140 named colours.
- Hex codes: #ff0000. Two digits each for red, green and blue, from 00 (none) to ff (full).
- RGB: rgb(255, 0, 0). The same three channels as numbers from 0 to 255.
- RGBA: rgba(0, 0, 0, 0.5). A fourth number from 0 to 1 sets transparency.

    h1 { color: #facc15; }
    .shadow { background-color: rgba(0, 0, 0, 0.5); }

## color versus background-color
color changes the TEXT colour. background-color fills the element's box. Mixing them up is very common.

## Reading hex codes
#000000 is black (no light) and #ffffff is white (full light). #ff0000 is pure red. A short form exists: #f00 is the same as #ff0000.

## Contrast matters
Light grey text on a white background is hard to read, especially for people with low vision. Aim for strong contrast between text and background: dark on light or light on dark."##,
            &["Colours can be names, hex (#ff0000), rgb() or rgba().", "In hex and rgb, the three parts are red, green and blue.", "The fourth rgba value sets transparency from 0 to 1.", "color = text, background-color = box. Keep contrast high."],
            vec![
                quiz("Which property colours the TEXT?", &["background-color", "color", "text-fill"], 1, "color is for text."),
                quiz("What colour is #000000?", &["White", "Black", "Red"], 1, "All channels at zero = no light = black."),
                fill("Make a half-transparent black background.", "background-color: rgba(0, 0, 0, ___);", &["0.5", "50", "half"], 0, "Alpha goes from 0 to 1."),
            ]),
        d("Units", r##"## Sizes need units
Most sizes in CSS need a number AND a unit: 16px, 2em, 50%. Writing just 16 does not work (except for 0, which needs no unit).

## Absolute: px
px (pixels) is a fixed size. 16px is always 16px. It is simple and predictable, great for borders and small details.

## Relative units
- % is relative to the parent: width: 50% takes half the parent's width.
- em is relative to the element's own font size: if the font is 20px, 2em is 40px.
- rem is relative to the root (html) font size, usually 16px, so 1.5rem is 24px.
- vw and vh are 1% of the browser window's width and height.

    h1 { font-size: 2rem; }
    .panel { width: 80%; }
    .hero { height: 100vh; }

## Why relative units?
Relative units adapt. Many people increase their browser's default text size to read comfortably. Fonts in rem grow with that setting; fonts in px do not. Percentages let layouts fit phones and wide screens.

## A simple strategy
Use rem for font sizes, % or flexbox for widths, and px for borders and thin lines."##,
            &["Sizes need a unit, except 0.", "px is fixed; %, em, rem, vw and vh are relative.", "rem follows the root font size, so text respects user settings.", "Use rem for text, % for widths and px for borders."],
            vec![
                quiz("The root font size is 16px. How big is 2rem?", &["16px", "32px", "2px"], 1, "2 × 16px = 32px."),
                quiz("width: 50% is half of...", &["The screen", "The parent element", "The font size"], 1, "% is relative to the parent."),
                fill("Make the section fill the window's height.", "height: 100___;", &["vh", "vw", "px"], 0, "vh = viewport height."),
            ]),
        d("Fonts", r##"## Choosing a typeface
font-family sets the typeface. Always give a list, ending with a generic family, because the visitor may not have your first choice installed:

    body { font-family: "Trebuchet MS", Arial, sans-serif; }

The browser uses the first font it has. Names with spaces need quotes.

## Generic families
- serif: letters with little feet (Georgia, Times).
- sans-serif: clean letters without feet (Arial, Verdana).
- monospace: every letter the same width, like code.
- cursive and fantasy: decorative, use sparingly.

## Size and weight
- font-size: 18px or 1.2rem sets the size.
- font-weight: bold or a number (400 normal, 700 bold) sets thickness.
- font-style: italic slants the letters.

    h1 {
      font-family: Georgia, serif;
      font-size: 2.5rem;
      font-weight: 700;
    }

## Web fonts
Services like Google Fonts give you a <link> to put in <head>. After that, you can use the font's name in font-family like any other.

## Keep it readable
Two font families per site is plenty: one for headings, one for body text. Body text around 16 to 18px is comfortable for most readers."##,
            &["font-family takes a list ending with a generic family.", "Generic families: serif, sans-serif, monospace.", "font-size, font-weight and font-style control size, thickness and slant.", "Use one or two families and readable body sizes."],
            vec![
                quiz("Why end a font-family list with sans-serif?", &["It is required", "As a fallback if other fonts are missing", "It makes text bold"], 1, "The generic family always exists."),
                quiz("Which font-weight is bold?", &["100", "400", "700"], 2, "700 is bold; 400 is normal."),
                fill("Use a font where every letter has the same width.", "code { font-family: ___; }", &["monospace", "serif", "fixed"], 0, "monospace letters share one width."),
            ]),
        d("Text Styling", r##"## Arranging text
- text-align moves text left, right, center or justify (stretched to both edges).
- line-height sets the space between lines. A unitless value like 1.6 means 1.6 times the font size and is the easiest to work with.
- letter-spacing adds space between letters, nice for small headings.
- text-transform: uppercase changes letters to capitals without changing the HTML.

    p {
      line-height: 1.6;
      text-align: left;
    }
    .label {
      text-transform: uppercase;
      letter-spacing: 2px;
    }

## Decoration
text-decoration adds or removes lines: underline, line-through, or none. Removing the underline from links is common, but then make links clearly different another way (colour, bold) so people can still find them.

    a { text-decoration: none; color: gold; }

## Shadows
text-shadow: 2px 2px 0 black; draws a shadow 2px right and 2px down. Pixel-style games love a hard black shadow.

## Readability
Long lines are tiring to read. Comfortable body text has a line-height around 1.5 and lines of 50 to 75 characters. Centred text is fine for short headings but hard to read in long paragraphs."##,
            &["text-align positions text; line-height spaces the lines.", "letter-spacing and text-transform style headings and labels.", "text-decoration adds or removes underlines; keep links recognisable.", "Use line-height around 1.5 and avoid centring long paragraphs."],
            vec![
                quiz("Which property sets the space between lines?", &["letter-spacing", "line-height", "text-align"], 1, "line-height controls line spacing."),
                quiz("You remove link underlines. What should you also do?", &["Nothing", "Make links stand out another way", "Make all text underlined"], 1, "Links must still be recognisable."),
                fill("Show a label in capitals without changing the HTML.", ".label { text-transform: ___; }", &["uppercase", "caps", "bold"], 0, "text-transform: uppercase."),
            ]),
        d("Backgrounds", r##"## Filling the box
Every element is a box, and its background can be a colour, an image or a gradient.

    .panel { background-color: #1e293b; }

## Background images
    .hero {
      background-image: url("castle.png");
      background-size: cover;
      background-position: center;
      background-repeat: no-repeat;
    }

- url() points at the image file.
- background-size: cover scales the image to fill the box completely (parts may be cut off). contain fits the whole image inside (gaps may appear).
- background-position chooses which part stays visible.
- background-repeat: no-repeat stops the image from tiling. By default, small images repeat like wallpaper, which is great for patterns.

## Gradients
A gradient is a smooth blend between colours, made by CSS with no image file:

    .sky { background-image: linear-gradient(navy, purple); }

## Keep text readable
Text on top of a busy picture is hard to read. Add a dark overlay, a background colour behind the text, or a text shadow. Always set a background-color too, so there is something sensible to show while the image loads or if it fails."##,
            &["background-color fills the element's box.", "background-image: url(...) adds a picture; it repeats by default.", "background-size: cover fills the box; contain fits the whole image.", "linear-gradient() blends colours without an image file."],
            vec![
                quiz("Which value makes a background image fill the whole box?", &["cover", "contain", "repeat"], 0, "cover fills, possibly cropping."),
                quiz("Why also set a background-color under an image?", &["It is required", "It shows while the image loads or if it fails", "It makes the image sharper"], 1, "A safe fallback keeps text readable."),
                fill("Stop the image from tiling.", "background-repeat: ___;", &["no-repeat", "none", "once"], 0, "no-repeat shows it once."),
            ]),
        d("Shorthands", r##"## Saying more with less
Some properties come in families, and CSS lets you set the whole family in one line called a shorthand.

## margin and padding
Values go clockwise from the top: top, right, bottom, left.

    margin: 10px 20px 10px 20px;

Shorter forms follow a pattern:
- margin: 10px; → all four sides.
- margin: 10px 20px; → top and bottom 10px, left and right 20px.
- margin: 10px 20px 5px; → top 10, sides 20, bottom 5.

## border
    border: 2px solid gold;

sets width, style and colour at once. The style (solid, dashed, dotted) is required, otherwise no border appears.

## font and background
    font: italic 700 18px Georgia, serif;
    background: #000 url("stars.png") repeat;

## A trap
A shorthand resets every part you leave out to its default. If you write background-image first and background: black after it, the image disappears, because the shorthand reset it. Put shorthands first, then the single properties that adjust them.

## Remember the clock
"Top, right, bottom, left" follows a clock from 12. Some people remember it as TRouBLe."##,
            &["Shorthands set a family of properties in one line.", "margin/padding values go clockwise: top, right, bottom, left.", "border: width style colour; the style is required.", "A shorthand resets the parts you leave out."],
            vec![
                quiz("margin: 10px 20px; sets what on the left?", &["10px", "20px", "0"], 1, "Two values: vertical then horizontal."),
                quiz("Why might border: 2px gold; show no border?", &["Gold is not a colour", "The style (like solid) is missing", "Borders need px twice"], 1, "A border needs a style."),
                fill("Set a 3px dashed red border.", "border: 3px ___ red;", &["dashed", "dash", "lines"], 0, "dashed is a border style."),
            ]),
        d("Project: Royal Invitation", r##"## Style an invitation to the royal ball
This project brings the Properties chapter together: colours, units, fonts, text styling, backgrounds and shorthands.

## The look to aim for
- A dark night-sky background with a gradient.
- A centred card in a parchment colour.
- A fancy serif title in gold with a shadow.
- Comfortable body text with good line height.
- Details (date, place) in small capitals with letter spacing.

## Example
    body {
      background: linear-gradient(#0b1228, #3b1e5a);
      font-family: Georgia, serif;
    }
    .card {
      background-color: #f0e2b6;
      color: #2a1c12;
      padding: 2rem;
    }
    h1 {
      color: goldenrod;
      text-shadow: 2px 2px 0 #000;
      text-align: center;
    }
    .details {
      text-transform: uppercase;
      letter-spacing: 2px;
    }

## Check
- Is the text easy to read against its background?
- Did you use rem or % where sizes should adapt?
- Did your shorthands come before the single properties that adjust them?"##,
            &["Combine colour, font, text and background properties into one design.", "Use gradients for backgrounds without image files.", "Check text contrast against every background.", "Use relative units where sizes should adapt."],
            vec![
                quiz("Which property adds the title's shadow?", &["box-shadow", "text-shadow", "shadow"], 1, "text-shadow is for text."),
                fill("Space out the letters of the details.", ".details { ___: 2px; }", &["letter-spacing", "word-gap", "spacing"], 0, "letter-spacing adds space between letters."),
                quiz("Dark brown text on a parchment card has...", &["good contrast", "no contrast", "too much shadow"], 0, "Dark on light reads well."),
            ]),
        d("Every Element Is a Box", r##"## The box model
To the browser, every element is a rectangle made of four layers, from inside to outside:

1. Content: the text or image itself, sized by width and height.
2. Padding: space between the content and the border, inside the box. It shows the background colour.
3. Border: a line around the padding.
4. Margin: space outside the border that pushes other elements away. It is always transparent.

    .card {
      width: 200px;
      padding: 16px;
      border: 4px solid gold;
      margin: 20px;
    }

## How big is the box?
By default, width means the CONTENT only. So the card above is actually 200 + 16 + 16 + 4 + 4 = 240px wide on screen, plus 20px of margin on each side. This surprise causes many layout bugs; the Box Sizing lesson shows the fix.

## See it for yourself
Every browser has developer tools (press F12). Hover over an element and you will see its content, padding, border and margin highlighted in different colours. This is the best way to debug spacing.

## Padding or margin?
Padding = space inside, between content and border (like the stuffing in a cushion). Margin = space outside, between this box and its neighbours."##,
            &["Every element is a box: content, padding, border, margin.", "Padding is inside the border and shows the background; margin is outside and transparent.", "By default, width sets only the content width.", "Use the browser's developer tools (F12) to see boxes."],
            vec![
                quiz("Which layer is outside the border?", &["Padding", "Content", "Margin"], 2, "Margin pushes neighbours away."),
                quiz("width: 100px, padding: 10px, border: 5px. How wide is the box on screen (default sizing)?", &["100px", "130px", "115px"], 1, "100 + 10 + 10 + 5 + 5 = 130px."),
                fill("Add space INSIDE the border.", ".card { ___: 16px; }", &["padding", "margin", "gap"], 0, "Padding is inside."),
            ]),
        d("Borders and Corners", r##"## Borders
A border needs three things: a width, a style and a colour.

    .card { border: 3px solid gold; }

Styles include solid, dashed, dotted, double and none. You can also style one side at a time:

    .tab { border-bottom: 4px solid gold; }

## Rounded corners
border-radius rounds the corners. A small value softens them; 50% on a square turns it into a circle.

    .button { border-radius: 8px; }
    .avatar { border-radius: 50%; }

You can round each corner separately: border-radius: 10px 0 10px 0;

## Outlines and shadows
- outline looks like a border but does not take up space. Browsers use it to show keyboard focus, so do not remove it without a replacement.
- box-shadow adds a shadow to the whole box:

    .card { box-shadow: 0 4px 0 black; }

The values are: right offset, down offset, blur and colour. A blur of 0 makes a hard pixel-style shadow.

## Pixel-art tip
Square corners and hard shadows (blur 0) give a retro game look; rounded corners and soft shadows look modern."##,
            &["A border needs width, style and colour.", "border-bottom and friends style a single side.", "border-radius rounds corners; 50% makes a circle.", "box-shadow adds a shadow; outline shows focus, so keep it."],
            vec![
                quiz("How do you turn a square image into a circle?", &["border-radius: 50%", "border: circle", "shape: round"], 0, "50% radius on a square = circle."),
                quiz("Why should you be careful removing outline?", &["It shows keyboard focus", "It is slow", "It removes the border too"], 0, "Keyboard users rely on the focus outline."),
                fill("Underline only the bottom of the tab.", ".tab { ___: 4px solid gold; }", &["border-bottom", "border-down", "underline"], 0, "border-bottom styles one side."),
            ]),
        d("Padding", r##"## Space inside the box
Padding is the space between an element's content and its border. It makes boxes feel comfortable instead of cramped, and it is part of the clickable area of buttons and links.

    .button { padding: 12px 24px; }

## One side at a time
    padding-top: 8px;
    padding-right: 16px;
    padding-bottom: 8px;
    padding-left: 16px;

Or with the clockwise shorthand you learned: padding: 8px 16px;

## Padding shows the background
Padding is inside the box, so the background colour fills it. That is why a button's colour extends around the text.

## Padding grows the box
With default sizing, padding is added on top of the width. A 200px wide box with padding: 20px becomes 240px wide. The Box Sizing lesson fixes this.

## Why padding helps people
Bigger padding on buttons and links makes them easier to tap on phones. A common guideline is a tappable area of at least 44 by 44 pixels.

## No negative padding
Padding cannot be negative. If you need to pull things closer, adjust margins instead."##,
            &["Padding is space between content and border.", "padding-top/right/bottom/left or the clockwise shorthand set it.", "The background colour fills the padding.", "Padding adds to the box size and makes buttons easier to tap."],
            vec![
                quiz("Does the background colour show in the padding?", &["Yes", "No", "Only with a border"], 0, "Padding is inside the box."),
                quiz("Can padding be negative?", &["Yes", "No"], 1, "Only margins can be negative."),
                fill("Add 10px of padding only on the left.", "___: 10px;", &["padding-left", "padding-start", "left-padding"], 0, "padding-left."),
            ]),
        d("Margin", r##"## Space between boxes
Margin is the transparent space OUTSIDE an element's border. It pushes neighbouring elements away.

    .card { margin-bottom: 20px; }

Like padding, it has four sides and a clockwise shorthand.

## Centring a block
Give a block element a width and set its left and right margins to auto. The browser splits the leftover space equally, which centres it:

    .page {
      width: 600px;
      margin: 0 auto;
    }

## Margin collapsing
When two vertical margins touch, they do not add up. The bigger one wins. If one paragraph has margin-bottom: 20px and the next has margin-top: 30px, the gap is 30px, not 50px. This only happens with top and bottom margins, never left and right.

## Negative margins
Margins can be negative to pull an element closer or make it overlap. Use them sparingly, because they make layouts harder to follow.

## Browser defaults
Browsers give some elements a default margin: <body> has about 8px, and headings and paragraphs have space above and below. Many stylesheets start with * { margin: 0; } and add spacing back deliberately."##,
            &["Margin is transparent space outside the border.", "margin: 0 auto centres a block that has a width.", "Touching vertical margins collapse: the bigger one wins.", "Browsers add default margins; many sites reset them."],
            vec![
                quiz("margin-bottom: 20px meets margin-top: 30px. How big is the gap?", &["50px", "30px", "20px"], 1, "Vertical margins collapse to the larger."),
                quiz("How do you centre a 600px block horizontally?", &["margin: 0 auto", "text-align: center", "padding: auto"], 0, "auto side margins share the space."),
                fill("Remove all default margins.", "* { margin: ___; }", &["0", "none", "auto"], 0, "margin: 0."),
            ]),
        d("Box Sizing", r##"## The width surprise
By default, width sets only the content area. Padding and border are added outside it. So:

    .box { width: 200px; padding: 20px; border: 5px solid; }

takes 250px of space. That makes layouts hard: two "50%" boxes with padding no longer fit side by side.

## The fix: border-box
    .box { box-sizing: border-box; }

Now width includes the padding and the border. The box above is exactly 200px wide; the content shrinks to make room.

## Use it everywhere
Almost every modern stylesheet starts with this rule so that all boxes behave predictably:

    * {
      box-sizing: border-box;
    }

## The two values
- content-box (the default): width = content only.
- border-box: width = content + padding + border.

Margin is never included in either, because it is outside the box.

## When it matters most
Percent widths, grids of cards, and form fields next to each other. Whenever things "should fit but wrap to the next line", check box-sizing first."##,
            &["By default (content-box), padding and border are added to the width.", "box-sizing: border-box includes padding and border in the width.", "Most stylesheets apply border-box to every element with *.", "Margin is never part of the width."],
            vec![
                quiz("With border-box, width: 200px and padding: 20px, how wide is the box?", &["200px", "240px", "160px"], 0, "border-box includes the padding."),
                quiz("Is margin included in border-box width?", &["Yes", "No"], 1, "Margin is outside the box."),
                fill("Make every element use the predictable sizing.", "* { box-sizing: ___; }", &["border-box", "content-box", "fit"], 0, "border-box."),
            ]),
        d("Project: Library Card", r##"## Build a library card with the box model
The guild library needs membership cards. Each card is a box with a photo, a name and some details, and it must look neat and evenly spaced.

## Plan
- Use box-sizing: border-box for everything.
- The card: fixed width, padding inside, a thick border, rounded corners and a hard shadow.
- Centre the card on the page with margin: 0 auto.
- The photo: a circle using border-radius: 50%.
- Details separated by a dashed line: border-top on the details section.

## Example
    * { box-sizing: border-box; }
    .card {
      width: 320px;
      margin: 40px auto;
      padding: 20px;
      border: 4px solid #8b5a2b;
      border-radius: 8px;
      box-shadow: 0 6px 0 #000;
    }
    .photo { width: 96px; border-radius: 50%; }
    .details { border-top: 2px dashed #8b5a2b; padding-top: 12px; }

## Check with developer tools
Press F12, hover the card and confirm the content, padding, border and margin are what you expect. Is the card exactly 320px wide thanks to border-box?"##,
            &["Start with box-sizing: border-box.", "Use padding inside, borders around and margin outside.", "Centre fixed-width blocks with margin: 0 auto.", "Check the box layers with the developer tools."],
            vec![
                quiz("Which line centres the card?", &["margin: 40px auto;", "padding: auto;", "text-align: center;"], 0, "auto side margins centre a block."),
                fill("Separate the details with a dashed line on top.", ".details { ___: 2px dashed brown; }", &["border-top", "outline", "line-top"], 0, "border-top draws one side."),
                quiz("With border-box, how wide is a card with width: 320px and padding: 20px?", &["320px", "360px", "280px"], 0, "Padding is included."),
            ]),
        d("Block and Inline", r##"## The display property
Every element has a display type that decides how it sits next to others.

## block
Block elements start on a new line and stretch to fill the width: div, p, h1, ul, section. You can set their width, height, margin and padding freely.

## inline
Inline elements sit inside a line of text, side by side: span, a, strong, em. Their width and height cannot be set, and top/bottom margins do not push other lines away.

## inline-block
The best of both: it sits in a line like inline, but accepts width, height and all margins like block. Great for buttons and badges in a row.

    .badge {
      display: inline-block;
      width: 80px;
      padding: 4px;
    }

## none
display: none removes the element completely, as if it were not in the HTML. (visibility: hidden hides it but keeps its space.)

## Changing the display
You can switch any element: make links block so they fill a menu, or list items inline-block so they sit in a row. The meaning stays the same; only the layout changes. Later, display: flex will give you far more control."##,
            &["block: new line, full width, all sizes allowed.", "inline: flows in text; width and height are ignored.", "inline-block: in a line, but sizes and margins work.", "display: none removes the element; visibility: hidden keeps its space."],
            vec![
                quiz("Which display lets elements sit in a row AND accept a width?", &["inline", "inline-block", "none"], 1, "inline-block combines both."),
                quiz("What is the difference between display: none and visibility: hidden?", &["None", "none removes the space; hidden keeps it", "hidden removes the space"], 1, "Hidden still takes up room."),
                fill("Make the links fill the menu's width.", "nav a { display: ___; }", &["block", "inline", "hidden"], 0, "Block elements fill the width."),
            ]),
        d("Position Relative", r##"## Normal flow
By default, elements are laid out in "normal flow": blocks stack downwards and inline elements flow like text. The position property lets you step out of that.

## position: relative
A relatively positioned element stays in the normal flow, but you can nudge it from where it would have been, using top, right, bottom and left:

    .lifted {
      position: relative;
      top: -4px;
      left: 10px;
    }

top: -4px moves it UP by 4px (towards the top), left: 10px moves it RIGHT by 10px (away from the left). The space it originally took is kept, so other elements do not move.

## Reading offsets
Think of each offset as a push from that side: left: 10px pushes from the left (so it moves right). This trips up everyone at first.

## The real superpower
Relative positioning is most often used with no offsets at all, to make the element an anchor (a "positioned parent") for absolutely positioned children. That is the next lesson.

## static
The default value is static, which means "normal flow, offsets ignored"."##,
            &["Normal flow stacks blocks and flows inline content.", "position: relative nudges an element but keeps its original space.", "top/left/right/bottom push from that side (left: 10px moves right).", "relative is often used as an anchor for absolute children."],
            vec![
                quiz("position: relative; left: 20px moves the element...", &["Left", "Right", "Up"], 1, "It is pushed from the left side."),
                quiz("Does a relatively moved element leave a gap where it was?", &["Yes, its space is kept", "No"], 0, "Relative keeps its place in the flow."),
                fill("Make the element movable but keep its space.", ".gem { position: ___; top: -4px; }", &["relative", "static", "fixed"], 0, "relative keeps the space."),
            ]),
        d("Position Absolute", r##"## Leaving the flow
position: absolute removes an element from the normal flow. Other elements act as if it were not there, and it is placed exactly where you say.

## Relative to what?
An absolute element is positioned relative to its nearest positioned ancestor (one whose position is not static). If there is none, it uses the whole page. That is why we usually give the parent position: relative.

    .card { position: relative; }
    .card .badge {
      position: absolute;
      top: 8px;
      right: 8px;
    }

The badge now sits in the card's top-right corner, wherever the card is on the page.

## Typical uses
- Badges and labels in a corner ("NEW", a notification dot).
- Close buttons on pop-ups.
- Text on top of an image.

## Watch out
Because absolute elements leave the flow, the parent does not grow to fit them, and they can overlap other content. Use absolute positioning for small decorations, not for laying out a whole page; flexbox is better for that.

## Stretching
Setting top: 0; right: 0; bottom: 0; left: 0; on an absolute element makes it cover its positioned parent completely, perfect for overlays."##,
            &["position: absolute removes the element from the flow.", "It is placed relative to the nearest positioned ancestor.", "Give the parent position: relative to use it as the anchor.", "Use absolute for small overlays and badges, not whole layouts."],
            vec![
                quiz("An absolute badge should sit in its card's corner. What does the card need?", &["position: relative", "display: none", "margin: auto"], 0, "The card becomes the anchor."),
                quiz("Does the parent grow to fit an absolute child?", &["Yes", "No"], 1, "Absolute children leave the flow."),
                fill("Place the badge 8px from the top.", ".badge { position: absolute; ___: 8px; }", &["top", "up", "margin-top"], 0, "top offsets from the anchor's top edge."),
            ]),
        d("Fixed and Sticky", r##"## position: fixed
A fixed element is positioned relative to the browser window and stays there even when the page scrolls. Like absolute, it leaves the normal flow.

    .help-button {
      position: fixed;
      bottom: 20px;
      right: 20px;
    }

Typical uses: chat buttons, cookie banners, a top bar that never moves. Because it covers content, leave room for it (for example padding on the body) so it does not hide important text.

## position: sticky
Sticky is a mix of relative and fixed. The element scrolls normally until it reaches the offset you set, then sticks there while its parent is still on screen.

    .chapter-title {
      position: sticky;
      top: 0;
    }

Sticky needs an offset (usually top). It sticks only inside its parent: when the parent scrolls away, the sticky element goes with it. This is perfect for section headings in long lists.

## Choosing
- Always visible on screen, no matter what? fixed.
- Visible while its section is on screen? sticky.
- Nudged a little? relative. Placed inside a parent? absolute."##,
            &["fixed elements stay in the same spot of the window while scrolling.", "sticky elements scroll until they reach their offset, then stick.", "sticky needs an offset like top: 0 and stays inside its parent.", "Leave room so fixed elements do not cover content."],
            vec![
                quiz("A chat button should stay in the corner while the page scrolls. Use:", &["fixed", "relative", "static"], 0, "fixed stays in place on screen."),
                quiz("What does sticky need to work?", &["An offset such as top: 0", "A border", "display: none"], 0, "Without an offset it never sticks."),
                fill("Keep the heading at the top while its section scrolls.", "h2 { position: ___; top: 0; }", &["sticky", "fixed", "absolute"], 0, "sticky sticks within its section."),
            ]),
        d("Stacking with z-index", r##"## Which element is on top?
When positioned elements overlap, the browser has to decide which one to draw in front. By default, elements later in the HTML are drawn on top of earlier ones.

## z-index
z-index changes the order. Think of the z axis as coming out of the screen towards you: higher numbers are closer to you.

    .popup { position: absolute; z-index: 10; }
    .map   { position: relative; z-index: 1; }

The popup (10) appears over the map (1).

## It only works on positioned elements
z-index does nothing on an element with position: static. Give it relative, absolute, fixed or sticky first. (Flex and grid children are an exception.)

## Stacking contexts
A child can never escape its parent's level. If a parent has z-index: 1 and another box has z-index: 2, nothing inside the first parent can appear above the second box, even with z-index: 9999. When z-index "does not work", check the parents.

## Keep a simple scale
Huge numbers like 99999 cause wars later. Many teams use a small scale: 1 for raised cards, 10 for dropdowns, 100 for pop-ups, 1000 for full-screen overlays."##,
            &["Later elements are drawn on top by default.", "z-index puts higher numbers in front.", "z-index only works on positioned elements.", "A child cannot rise above its parent's stacking level."],
            vec![
                quiz("Which appears in front: z-index 5 or z-index 50?", &["5", "50", "Whichever is first in the HTML"], 1, "Higher z-index = closer to you."),
                quiz("z-index has no effect. What is the most likely cause?", &["The element is position: static", "The number is too small", "The colour is wrong"], 0, "z-index needs a positioned element."),
                fill("Bring the popup to the front.", ".popup { position: absolute; ___: 100; }", &["z-index", "layer", "z-order"], 0, "z-index sets the stacking order."),
            ]),
        d("Project: Art Gallery", r##"## Hang pictures in a gallery
The castle gallery shows paintings with name plates, a "NEW" ribbon on the latest one, a sticky gallery title and a fixed "Buy tickets" button. This project uses every Layout skill.

## Plan
- Paintings are inline-block frames so they sit in rows.
- Each frame is position: relative so its ribbon can be placed absolutely in a corner.
- The ribbon uses z-index to sit above the painting.
- The room title is sticky at the top.
- The ticket button is fixed at the bottom right.

## Example
    .frame {
      display: inline-block;
      position: relative;
      margin: 12px;
      border: 8px solid #8b5a2b;
    }
    .ribbon {
      position: absolute;
      top: -6px;
      right: -6px;
      z-index: 2;
    }
    .room-title { position: sticky; top: 0; }
    .tickets { position: fixed; bottom: 20px; right: 20px; }

## Check
Scroll the page: the title should stick while its room is visible, and the ticket button should never move. Does the ribbon stay in its frame's corner when the window is resized?"##,
            &["inline-block places frames in rows.", "A relative parent anchors an absolute ribbon.", "z-index keeps decorations on top.", "sticky for section titles, fixed for always-visible buttons."],
            vec![
                quiz("Why is each frame position: relative?", &["To anchor its absolute ribbon", "To hide it", "To make it fixed"], 0, "Relative parents anchor absolute children."),
                fill("Keep the ticket button on screen while scrolling.", ".tickets { position: ___; bottom: 20px; }", &["fixed", "relative", "static"], 0, "fixed stays put."),
                quiz("Which property keeps the ribbon above the painting?", &["z-index", "display", "float"], 0, "z-index controls stacking."),
            ]),
        d("Display Flex", r##"## Flexbox: layout made easy
Flexbox is the modern way to place items in a row or column, space them out and line them up. You turn it on for the PARENT (the flex container), and its direct children become flex items.

    .menu { display: flex; }

    <nav class="menu">
      <a>Home</a> <a>Quests</a> <a>Shop</a>
    </nav>

The links now sit side by side in a row.

## gap
gap adds space between items without margins on each one:

    .menu { display: flex; gap: 16px; }

## Main axis and cross axis
Flexbox works along two directions:
- the main axis: the direction items flow (a row by default, left to right),
- the cross axis: across it (top to bottom for a row).

The next lessons use these words a lot: justify-content works on the main axis, align-items on the cross axis.

## Only direct children
display: flex affects the container's children, not grandchildren. A grandchild can become flexible only if its own parent is also a flex container.

## Why flexbox?
Before flexbox, centring things and making equal columns needed tricks. Now a few properties do it, and the layout adapts to any screen size."##,
            &["display: flex on a parent turns its children into flex items.", "Items sit in a row by default.", "gap adds space between items.", "The main axis is the flow direction; the cross axis runs across it."],
            vec![
                quiz("Where do you write display: flex?", &["On the parent container", "On every child", "On the body only"], 0, "The container controls its items."),
                quiz("Which property adds space between flex items?", &["gap", "space", "padding-between"], 0, "gap spaces items evenly."),
                fill("Put the menu links in a row.", ".menu { display: ___; }", &["flex", "row", "inline"], 0, "display: flex."),
            ]),
        d("Direction and Wrap", r##"## flex-direction
flex-direction sets the main axis:
- row (default): left to right.
- column: top to bottom.
- row-reverse and column-reverse: the same, but backwards.

    .sidebar { display: flex; flex-direction: column; }

When the direction is column, the main axis runs DOWN. So justify-content now moves things vertically and align-items horizontally. Keep that in mind when things move the "wrong" way.

## flex-wrap
By default, flex items all squeeze onto one line, even if they get very thin. flex-wrap: wrap lets them move onto new lines when there is not enough room:

    .cards {
      display: flex;
      flex-wrap: wrap;
      gap: 12px;
    }

This is the key to responsive card grids: on a wide screen you see four cards per row, on a phone just one or two, with no extra code.

## flex-flow
The shorthand flex-flow: row wrap; sets both at once.

## Order in the HTML
Reversing with CSS does not change the HTML order, which screen readers and keyboard users follow. Use reverse directions for visuals only, never to fix the content order."##,
            &["flex-direction: row (default) or column sets the main axis.", "In a column, justify-content works vertically.", "flex-wrap: wrap lets items flow onto new lines.", "Reversing visually does not change the HTML order."],
            vec![
                quiz("Items keep shrinking on one line. Which property lets them move to a new line?", &["flex-wrap: wrap", "flex-direction: column", "gap"], 0, "Wrapping allows new lines."),
                quiz("With flex-direction: column, justify-content moves items...", &["Horizontally", "Vertically", "It stops working"], 1, "The main axis now runs down."),
                fill("Stack the sidebar items vertically.", ".sidebar { display: flex; flex-direction: ___; }", &["column", "vertical", "down"], 0, "column stacks top to bottom."),
            ]),
        d("Justify Content", r##"## Spacing along the main axis
justify-content decides how flex items share the free space along the main axis (horizontally in a row).

- flex-start (default): packed at the start.
- flex-end: packed at the end.
- center: grouped in the middle.
- space-between: first item at the start, last at the end, equal gaps between.
- space-around: equal space around each item (half-size gaps at the edges).
- space-evenly: exactly equal gaps everywhere, including the edges.

    .navbar {
      display: flex;
      justify-content: space-between;
    }

A classic use: a logo on the left and a menu on the right with space-between.

## Only free space
justify-content only works when there is leftover space. If the items already fill the row, nothing visible changes.

## Pushing one item away
A neat trick: margin-left: auto on one item pushes it (and everything after it) to the far end. Handy for a "Log in" button at the end of a menu.

## Remember
justify = main axis. In a column layout, that means up and down."##,
            &["justify-content shares free space along the main axis.", "Values: flex-start, flex-end, center, space-between, space-around, space-evenly.", "It has no effect when items fill the line.", "margin-left: auto pushes an item to the end."],
            vec![
                quiz("Logo on the left, menu on the right. Which value?", &["center", "space-between", "flex-start"], 1, "space-between pushes them apart."),
                quiz("Which value makes every gap equal, including the edges?", &["space-evenly", "space-around", "space-between"], 0, "space-evenly."),
                fill("Group the items in the middle of the row.", ".row { display: flex; justify-content: ___; }", &["center", "middle", "align"], 0, "center groups them in the middle."),
            ]),
        d("Align Items", r##"## Lining up across the axis
align-items lines items up along the CROSS axis (vertically in a row).

- stretch (default): items stretch to fill the container's height.
- flex-start: aligned to the top.
- flex-end: aligned to the bottom.
- center: centred vertically.
- baseline: their text sits on the same line.

    .toolbar {
      display: flex;
      align-items: center;
    }

An icon and a label of different heights now line up neatly in the middle.

## Perfect centring
For years, centring something both ways was a famous CSS puzzle. With flexbox it takes three lines:

    .center-me {
      display: flex;
      justify-content: center;
      align-items: center;
    }

(The container needs a height, for example height: 100vh, so there is space to centre in.)

## One item only
align-self overrides align-items for a single item: align-self: flex-end; moves just that item to the bottom.

## Equal-height cards
Because the default is stretch, cards in a flex row automatically get the same height, even with different amounts of text. This is often exactly what you want."##,
            &["align-items aligns items along the cross axis.", "Values: stretch (default), flex-start, flex-end, center, baseline.", "justify-content + align-items: center centres both ways.", "align-self changes the alignment of one item."],
            vec![
                quiz("Which two properties centre something horizontally AND vertically in a row?", &["justify-content and align-items", "text-align and margin", "gap and wrap"], 0, "Main axis + cross axis."),
                quiz("Why do flex cards get equal heights by default?", &["align-items defaults to stretch", "flex-wrap", "They have borders"], 0, "stretch fills the height."),
                fill("Move only this item to the bottom.", ".last { align-___: flex-end; }", &["self", "items", "content"], 0, "align-self works on one item."),
            ]),
        d("Flexible Sizing", r##"## How items grow and shrink
Flex items can grow to fill space or shrink to fit. Three properties control this, and the flex shorthand sets them all.

## flex-grow
How much of the leftover space an item takes. 0 (default) means "do not grow". If one item has flex-grow: 1 and the others 0, it takes all the extra space. If two items have 1 and 2, the second gets twice as much extra.

    .search { flex-grow: 1; }

A search box that stretches while the buttons stay small.

## flex-shrink
How much an item gives up when space runs out. The default 1 means everyone shrinks equally; 0 means "never shrink" (good for logos and icons).

## flex-basis
The starting size before growing or shrinking, like a width: flex-basis: 200px.

## The flex shorthand
    .card { flex: 1; }

flex: 1 means "grow 1, shrink 1, start from 0", so all items with flex: 1 become equal columns. flex: 0 0 200px means "exactly 200px, never grow or shrink".

## Responsive cards
Combine flex-wrap: wrap with flex: 1 1 200px: each card wants 200px, grows to fill the row, and wraps when there is no room."##,
            &["flex-grow shares leftover space between items.", "flex-shrink decides who shrinks; 0 means never shrink.", "flex-basis is the starting size.", "flex: 1 makes equal columns; flex: 1 1 200px makes wrapping cards."],
            vec![
                quiz("Two items have flex-grow 1 and 2. Who gets more extra space?", &["The first", "The second", "They get the same"], 1, "2 gets twice as much as 1."),
                quiz("A logo should never get smaller. Use:", &["flex-shrink: 0", "flex-grow: 0", "flex-basis: 0"], 0, "flex-shrink: 0 stops shrinking."),
                fill("Let the search box take the leftover space.", ".search { flex-___: 1; }", &["grow", "shrink", "basis"], 0, "flex-grow takes extra space."),
            ]),
        d("Final Project: Monster Cards", r##"## A responsive bestiary
Your final CSS project is a page of monster cards: a header bar, a wrapping grid of cards, and nicely aligned content inside each card. It should look good on a phone and on a wide screen.

## Plan
- Header: a flex row with the title on the left and a menu on the right (space-between, align-items: center).
- Card grid: a flex container with flex-wrap: wrap and gap.
- Each card: flex: 1 1 220px so cards fill rows and wrap, a border, rounded corners and a hard shadow.
- Inside a card: flex-direction: column, with the "Fight" button pushed to the bottom with margin-top: auto.

## Example
    .grid { display: flex; flex-wrap: wrap; gap: 16px; }
    .card {
      flex: 1 1 220px;
      display: flex;
      flex-direction: column;
      padding: 16px;
      border: 4px solid #334155;
      border-radius: 6px;
    }
    .card .fight { margin-top: auto; }

## Final review
Resize the window: do the cards wrap without breaking? Are buttons lined up at the bottom of every card even when text lengths differ? Is the text readable on every background? You have now finished the CSS course."##,
            &["Use flex rows for headers and wrapping flex grids for cards.", "flex: 1 1 220px makes cards fill and wrap.", "A column flex card with margin-top: auto pins a button to the bottom.", "Test by resizing the window."],
            vec![
                quiz("How do the cards wrap onto new rows?", &["flex-wrap: wrap on the grid", "position: absolute", "z-index"], 0, "Wrapping moves cards to new lines."),
                fill("Push the Fight button to the bottom of the card.", ".card .fight { margin-top: ___; }", &["auto", "100%", "bottom"], 0, "An auto margin takes the free space."),
                quiz("Which justify-content puts the title left and menu right?", &["space-between", "center", "flex-end"], 0, "space-between pushes them apart."),
            ]),
    ]
}
