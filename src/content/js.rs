use super::*;
use eframe::egui::Color32;

pub fn course() -> Course {
    Course {
        name: "JavaScript", tag: "JS", slug: "javascript", color: Color32::from_rgb(0xf0, 0xa0, 0x1a), lang: "javascript", cat: "Code", level: "Beginner", hours: 20,
        desc: "Bring pages to life with JavaScript: logic, loops, arrays, functions, objects and the page itself.",
        teaches: vec!["The console, variables and data types", "Decisions with if / else and random numbers", "Loops, arrays and functions", "Objects for structured data", "Changing a web page with the DOM"],
        snippet: r##"console.log("Hello, hero!");"##,
        badge: Badge { name: "Lightning Mage", art: "scroll_lightning", desc: "Cast every JavaScript spell" },
        modules: vec![console(), variables(), conditionals(), loops(), arrays(), functions(), objects(), the_page()],
    }
}

fn console() -> Module {
    ch("The Console", "Print messages and do quick math.", vec![
        l("Hello, Console", &[
            (r##"JavaScript is the language that makes web pages DO things. It also runs on its own.

console.log() prints a message to the console, a coder's notice board."##, "scroll_lightning"),
            (r##"console.log("Hello, hero!");

Text goes inside quotes. The ; ends the statement."##, "letter"),
        ], vec![
            quiz("What does console.log() do?", &["Draws on the page", "Prints a message", "Saves a file"], 1, "It prints to the console."),
            fill("Print a message.", "console.___(\"Hello, hero!\");", &["log", "print", "say"], 0, "console.log."),
            copy("Type this and see it print.", r##"console.log("Hello, hero!");"##, "Your first spell!"),
            recall("From memory: print Ready!", r##"console.log("Ready!");"##, r##"console.log("Ready!");"##),
        ]),
        l("Strings and Numbers", &[
            (r##"Text in quotes is a STRING: "Ada".
Numbers have no quotes: 42.

console.log(2 + 3);  prints 5
console.log("2 + 3"); prints 2 + 3"##, "coins"),
            (r##"Separate several values with commas; they print with spaces between:

console.log("Level", 4);"##, "gem_green"),
        ], vec![
            quiz("console.log(\"5\" + 1) prints...", &["6", "51", "error"], 1, "A string + a number joins as text: \"51\"."),
            fill("Print the number 5.", "console.log(2 ___ 3);", &["+", "&", "."], 0, "+ adds numbers."),
            copy("Type this.", "console.log(2 + 3);", "Prints 5."),
            recall("From memory: print \"Level\" and 4 with a comma.", r##"console.log("Level", 4);"##, r##"console.log("Level", 4);"##),
        ]),
        l("Comments", &[
            (r##"Comments are notes the computer skips.

// a one-line comment

Use them to explain WHY the code does something."##, "spellbook"),
            (r##"// greet the player
console.log("Welcome!");

console.log("Hi"); // comments can follow code too"##, "book"),
        ], vec![
            quiz("What happens to a // comment?", &["It prints", "It is skipped", "It causes an error"], 1, "Comments are ignored when running."),
            fill("Start a comment.", "___ greet the player", &["//", "#", "--"], 0, "JavaScript comments start with //."),
            copy("Type this.", "// greet the player\nconsole.log(\"Welcome!\");", "Only Welcome! prints."),
            recall("From memory: print Hi, then add the comment // say hi after it.", r##"console.log("Hi"); // say hi"##, r##"console.log("Hi"); // say hi"##),
        ]),
        l("Many Lines", &[
            (r##"Code runs top to bottom, one line after another. Each console.log prints on its own line."##, "torch"),
            (r##"console.log("  *");
console.log(" ***");
console.log("*****");

Spaces inside quotes are kept, so you can draw!"##, "flame"),
        ], vec![
            quiz("In what order do statements run?", &["Bottom to top", "Top to bottom", "Random"], 1, "Top to bottom."),
            fill("Finish the tree.", "console.log(\" ***\");\nconsole.log(\"___\");", &["*****", "**", "*"], 0, "The base is widest."),
            copy("Draw the tree.", "console.log(\"  *\");\nconsole.log(\" ***\");\nconsole.log(\"*****\");", "Three lines, one tree."),
            recall("From memory: print * then ** on two lines.", "console.log(\"*\");\nconsole.log(\"**\");", "Two console.log lines."),
        ]),
        p("Project: Shop Receipt", &[
            (r##"CHECKPOINT PROJECT

The potion shop needs a receipt printer. Print a title, each item with its cost, and the total, letting JavaScript do the math."##, "money_bag"),
            (r##"console.log("=== POTION SHOP ===");
console.log("Potion x2:", 2 * 15);
console.log("Elixir x1:", 40);
console.log("Total:", 2 * 15 + 40);"##, "potion_red"),
        ], vec![
            fill("Two potions at 15 each.", "console.log(\"Potion x2:\", 2 ___ 15);", &["*", "x", "+"], 0, "* multiplies."),
            quiz("2 * 15 + 40 equals...", &["110", "70", "60"], 1, "Multiply first: 30 + 40 = 70."),
            copy("Print the receipt.", "console.log(\"=== POTION SHOP ===\");\nconsole.log(\"Potion x2:\", 2 * 15);\nconsole.log(\"Elixir x1:\", 40);\nconsole.log(\"Total:\", 2 * 15 + 40);", "Total: 70"),
            recall("From memory: print \"Total:\" and 30 + 40.", r##"console.log("Total:", 30 + 40);"##, r##"console.log("Total:", 30 + 40);"##),
        ]),
    ])
}

fn variables() -> Module {
    ch("Variables", "Store values, learn data types, do arithmetic.", vec![
        l("let and const", &[
            (r##"A VARIABLE is a labelled box that holds a value.

let hp = 10;
const name = "Ada";

let can change later; const cannot."##, "chest"),
            (r##"Use the name to read the value:

console.log(hp);  prints 10

Names can't have spaces. Use camelCase: maxHp."##, "key"),
        ], vec![
            quiz("Which can be changed later?", &["const", "let", "both"], 1, "const values stay fixed."),
            fill("Make a variable that will change.", "___ hp = 10;", &["let", "const", "var="], 0, "let."),
            copy("Type this.", "let hp = 10;\nconsole.log(hp);", "Prints 10."),
            recall("From memory: const name = \"Ada\", then print name.", "const name = \"Ada\";\nconsole.log(name);", "const + console.log."),
        ]),
        l("Data Types", &[
            (r##"Every value has a TYPE:

"Ada"   string
42      number
true    boolean (true or false)"##, "gem_green"),
            (r##"typeof tells you the type:

let alive = true;
console.log(typeof alive);  boolean"##, "crystal_blue"),
        ], vec![
            quiz("What type is true?", &["string", "number", "boolean"], 2, "true and false are booleans."),
            fill("Ask for the type.", "console.log(___ 42);", &["typeof", "type", "kind"], 0, "typeof value."),
            copy("Type this.", "let alive = true;\nconsole.log(typeof alive);", "Prints boolean."),
            recall("From memory: print typeof 42.", "console.log(typeof 42);", "number"),
        ]),
        l("Arithmetic", &[
            (r##"+  add        -  subtract
*  multiply   /  divide
%  remainder  ** power

10 % 3 is 1 (10 = 3 + 3 + 3, remainder 1)."##, "cpu"),
            (r##"let gold = 17 % 5;
console.log(gold);   2

Math happens before the value is stored."##, "coins"),
        ], vec![
            quiz("2 ** 3 is...", &["6", "8", "9"], 1, "2 x 2 x 2 = 8."),
            fill("Get the remainder.", "let gold = 17 ___ 5;", &["%", "/", "//"], 0, "% is remainder."),
            copy("Type this.", "let gold = 17 % 5;\nconsole.log(gold);", "Prints 2."),
            recall("From memory: print 2 ** 3.", "console.log(2 ** 3);", "8"),
        ]),
        l("Updating Variables", &[
            (r##"Change a let variable by assigning again:

xp = xp + 5;

Shortcuts:  xp += 5;   xp -= 2;   xp++;  (adds 1)"##, "scroll_lightning"),
            (r##"let xp = 10;
xp += 5;
xp++;
console.log(xp);   16"##, "potion_magic"),
        ], vec![
            quiz("After let n = 3; n++; n is...", &["3", "4", "5"], 1, "++ adds one."),
            fill("Add 5 to xp.", "xp ___ 5;", &["+=", "=+", "++"], 0, "+= adds and stores."),
            copy("Type this.", "let xp = 10;\nxp += 5;\nxp++;\nconsole.log(xp);", "Prints 16."),
            recall("From memory: let hp = 10, subtract 3 with -=, print hp.", "let hp = 10;\nhp -= 3;\nconsole.log(hp);", "7"),
        ]),
        l("Template Literals", &[
            (r##"Backticks ` ` make a TEMPLATE LITERAL. Put ${ } around any value to drop it into the text:

let name = "Ada";
console.log(`Hi, ${name}!`);"##, "letter"),
            (r##"You can do math inside too:

console.log(`Double: ${hp * 2}`);"##, "spellbook"),
        ], vec![
            quiz("Template literals use...", &["\"double quotes\"", "'single quotes'", "`backticks`"], 2, "Backticks."),
            fill("Insert the value.", "console.log(`Hi, ___{name}!`);", &["$", "#", "@"], 0, "${ ... }"),
            copy("Type this.", "let name = \"Ada\";\nconsole.log(`Hi, ${name}!`);", "Hi, Ada!"),
            recall("From memory: let lvl = 3, print `Level ${lvl}`.", "let lvl = 3;\nconsole.log(`Level ${lvl}`);", "Level 3"),
        ]),
        p("Project: Planet Weight", &[
            (r##"CHECKPOINT PROJECT

How much would a hero weigh on Mars? Store the weight and Mars' gravity (0.38 of Earth's), compute, and print a sentence."##, "orb_red"),
            (r##"const weight = 60;
const marsGravity = 0.38;
let marsWeight = weight * marsGravity;
console.log(`On Mars you weigh ${marsWeight} kg`);"##, "crystal_purple"),
        ], vec![
            fill("Multiply by gravity.", "let marsWeight = weight ___ marsGravity;", &["*", "+", "%"], 0, "*"),
            fill("These values never change.", "___ marsGravity = 0.38;", &["const", "let", "fix"], 0, "const."),
            copy("Compute it.", "const weight = 60;\nconst marsGravity = 0.38;\nlet marsWeight = weight * marsGravity;\nconsole.log(`On Mars you weigh ${marsWeight} kg`);", "22.8 kg!"),
            recall("From memory: const w = 50, print w * 2.", "const w = 50;\nconsole.log(w * 2);", "100"),
        ]),
    ])
}

fn conditionals() -> Module {
    ch("Conditionals", "Make decisions with if, else and random numbers.", vec![
        l("if", &[
            (r##"if runs code only when a condition is true:

if (hp < 5) {
  console.log("Drink a potion!");
}"##, "potion_red"),
            (r##"The condition goes in ( ), the code in { }.
Indent the code inside so the block is easy to see."##, "shield"),
        ], vec![
            quiz("When does the code in an if run?", &["Always", "When the condition is true", "When it is false"], 1, "Only when true."),
            fill("Check the condition.", "___ (hp < 5) {", &["if", "when", "check"], 0, "if (condition)."),
            copy("Type this.", "let hp = 3;\nif (hp < 5) {\n  console.log(\"Drink a potion!\");\n}", "3 < 5 is true, so it prints."),
            recall("From memory: if (true), print Yes.", "if (true) {\n  console.log(\"Yes\");\n}", "if (true) { ... }"),
        ]),
        l("else", &[
            (r##"else runs when the if condition is false:

if (gold >= 10) {
  console.log("Buy sword");
} else {
  console.log("Keep saving");
}"##, "coins"),
            (r##"Exactly one of the two blocks runs, never both."##, "sword"),
        ], vec![
            quiz("If the condition is false, which block runs?", &["The if block", "The else block", "Both"], 1, "else catches the false case."),
            fill("Otherwise...", "} ___ {", &["else", "or", "if"], 0, "else."),
            copy("Type this.", "let gold = 5;\nif (gold >= 10) {\n  console.log(\"Buy sword\");\n} else {\n  console.log(\"Keep saving\");\n}", "Prints Keep saving."),
            recall("From memory: if (1 > 2) print A, else print B.", "if (1 > 2) {\n  console.log(\"A\");\n} else {\n  console.log(\"B\");\n}", "Prints B."),
        ]),
        l("else if", &[
            (r##"Chain more checks with else if. JavaScript tests them in order and runs the FIRST true one:

if (temp > 25) { hot }
else if (temp > 10) { mild }
else { cold }"##, "campfire"),
            (r##"let temp = 15;
if (temp > 25) {
  console.log("Hot");
} else if (temp > 10) {
  console.log("Mild");
} else {
  console.log("Cold");
}"##, "lantern"),
        ], vec![
            quiz("With temp = 30, what prints?", &["Hot", "Mild", "Hot and Mild"], 0, "Only the first true branch runs."),
            fill("Add another check.", "} else ___ (temp > 10) {", &["if", "when", "elif"], 0, "else if."),
            copy("Type this.", "let temp = 15;\nif (temp > 25) {\n  console.log(\"Hot\");\n} else if (temp > 10) {\n  console.log(\"Mild\");\n} else {\n  console.log(\"Cold\");\n}", "Prints Mild."),
            recall("From memory: let n = 0; if n > 0 print Positive, else if n < 0 print Negative, else print Zero.", "let n = 0;\nif (n > 0) {\n  console.log(\"Positive\");\n} else if (n < 0) {\n  console.log(\"Negative\");\n} else {\n  console.log(\"Zero\");\n}", "Prints Zero."),
        ]),
        l("Comparison Operators", &[
            (r##"===  equal (same value AND type)
!==  not equal
<  >  <=  >=

5 === 5 is true. 5 === "5" is false."##, "medallion"),
            (r##"Use === rather than ==. The loose == turns "5" into 5 first, which causes surprises."##, "hourglass"),
        ], vec![
            quiz("5 === \"5\" is...", &["true", "false", "an error"], 1, "Different types: number vs string."),
            fill("Check 'not equal'.", "console.log(5 ___ 3);", &["!==", "=/=", "<>"], 0, "!== means not equal."),
            copy("Type this.", "console.log(5 === 5, 5 !== 3);", "true true"),
            recall("From memory: print 10 >= 7.", "console.log(10 >= 7);", "true"),
        ]),
        l("Logical Operators", &[
            (r##"&&  AND - both must be true
||  OR  - at least one true
!   NOT - flips true/false"##, "keyhole"),
            (r##"let hasKey = true;
let level = 3;
if (hasKey && level >= 3) {
  console.log("Door opens");
}"##, "key"),
        ], vec![
            quiz("true && false is...", &["true", "false"], 1, "AND needs both true."),
            fill("Need both conditions.", "if (hasKey ___ level >= 3) {", &["&&", "||", "and"], 0, "&& = and."),
            copy("Type this.", "let hasKey = true;\nlet level = 3;\nif (hasKey && level >= 3) {\n  console.log(\"Door opens\");\n}", "Both are true."),
            recall("From memory: print true || false.", "console.log(true || false);", "true"),
        ]),
        l("Random Numbers", &[
            (r##"Math.random() gives a random decimal from 0 up to (not including) 1.

Math.floor() rounds DOWN to a whole number."##, "orb_red"),
            (r##"A dice roll from 1 to 6:

let roll = Math.floor(Math.random() * 6) + 1;

random * 6 -> 0 to 5.99, floor -> 0 to 5, + 1 -> 1 to 6."##, "crystal_purple"),
        ], vec![
            quiz("Math.floor(4.7) is...", &["4", "5", "4.7"], 0, "floor rounds down."),
            fill("Round down.", "let roll = Math.___(Math.random() * 6) + 1;", &["floor", "round", "ceil"], 0, "floor."),
            copy("Roll a die.", "let roll = Math.floor(Math.random() * 6) + 1;\nconsole.log(roll);", "Different every run!"),
            recall("From memory: print Math.floor(4.7).", "console.log(Math.floor(4.7));", "4"),
        ]),
        p("Project: Oracle Orb", &[
            (r##"CHECKPOINT PROJECT

The Oracle Orb answers any question. Pick a random number from 0 to 2 and print a different answer for each."##, "orb_red"),
            (r##"let pick = Math.floor(Math.random() * 3);
if (pick === 0) {
  console.log("Yes, hero.");
} else if (pick === 1) {
  console.log("The stars say no.");
} else {
  console.log("Ask again later.");
}"##, "mystic_figure"),
        ], vec![
            fill("Three possible answers: 0, 1 or 2.", "let pick = Math.floor(Math.random() * ___);", &["3", "2", "4"], 0, "random * 3, then floor, gives 0-2."),
            fill("Compare strictly.", "if (pick ___ 0) {", &["===", "=", "=>"], 0, "=== compares; = assigns."),
            copy("Build the orb.", "let pick = Math.floor(Math.random() * 3);\nif (pick === 0) {\n  console.log(\"Yes, hero.\");\n} else if (pick === 1) {\n  console.log(\"The stars say no.\");\n} else {\n  console.log(\"Ask again later.\");\n}", "Run it a few times!"),
            recall("From memory: a coin flip: coin = Math.floor(Math.random() * 2); if coin === 0 print Heads, else Tails.", "let coin = Math.floor(Math.random() * 2);\nif (coin === 0) {\n  console.log(\"Heads\");\n} else {\n  console.log(\"Tails\");\n}", "Heads or Tails."),
        ]),
    ])
}

fn loops() -> Module {
    ch("Loops", "Repeat code with while and for.", vec![
        l("while Loops", &[
            (r##"while repeats a block AS LONG AS its condition stays true:

let n = 1;
while (n <= 3) {
  console.log(n);
  n++;
}"##, "hourglass"),
            (r##"Something inside must change, or the loop never ends! Here n++ moves n toward 4."##, "torch"),
        ], vec![
            quiz("What stops a while loop?", &["Its condition becoming false", "A comment", "Nothing"], 0, "It checks before each round."),
            fill("Repeat while n is 3 or less.", "___ (n <= 3) {", &["while", "if", "repeat"], 0, "while (condition)."),
            copy("Count to 3.", "let n = 1;\nwhile (n <= 3) {\n  console.log(n);\n  n++;\n}", "1, 2, 3"),
            recall("From memory: let i = 0; while i < 2 print Hi and i++.", "let i = 0;\nwhile (i < 2) {\n  console.log(\"Hi\");\n  i++;\n}", "Hi twice."),
        ]),
        l("for Loops", &[
            (r##"for packs the counter into one line:

for (let i = 0; i < 3; i++) {
  console.log("Step", i);
}"##, "map"),
            (r##"let i = 0   start
i < 3       keep going while true
i++         after each round"##, "gear"),
        ], vec![
            quiz("How many times does for (let i = 0; i < 3; i++) run?", &["2", "3", "4"], 1, "i = 0, 1, 2."),
            fill("Add one after each round.", "for (let i = 0; i < 3; ___) {", &["i++", "i+", "++"], 0, "i++."),
            copy("Type this loop.", "for (let i = 0; i < 3; i++) {\n  console.log(\"Step\", i);\n}", "Step 0, 1, 2."),
            recall("From memory: a for loop printing 1 to 5.", "for (let i = 1; i <= 5; i++) {\n  console.log(i);\n}", "Start at 1, stop after 5."),
        ]),
        l("Even and Odd", &[
            (r##"% (remainder) tells you if a number is even:

n % 2 === 0   even
n % 2 === 1   odd"##, "coins"),
            (r##"for (let i = 1; i <= 4; i++) {
  if (i % 2 === 0) {
    console.log(i, "is even");
  }
}"##, "crystal_blue"),
        ], vec![
            quiz("7 % 2 is...", &["0", "1", "3"], 1, "7 = 2 x 3 + 1."),
            fill("Check for even.", "if (i % 2 === ___) {", &["0", "1", "2"], 0, "No remainder = even."),
            copy("Find even numbers.", "for (let i = 1; i <= 4; i++) {\n  if (i % 2 === 0) {\n    console.log(i, \"is even\");\n  }\n}", "2 and 4."),
            recall("From memory: print 7 % 2.", "console.log(7 % 2);", "1"),
        ]),
        l("break and continue", &[
            (r##"break leaves the loop immediately.
continue skips to the next round."##, "axe"),
            (r##"for (let i = 1; i <= 10; i++) {
  if (i === 4) break;
  console.log(i);
}

Prints 1, 2, 3, then stops."##, "sword"),
        ], vec![
            quiz("Which skips just one round?", &["break", "continue", "stop"], 1, "continue jumps to the next round."),
            fill("Leave the loop.", "if (i === 4) ___;", &["break", "exit", "return"], 0, "break."),
            copy("Type this.", "for (let i = 1; i <= 10; i++) {\n  if (i === 4) break;\n  console.log(i);\n}", "1, 2, 3"),
            recall("From memory: loop i from 0 to 2, skip 1 with continue, print i.", "for (let i = 0; i < 3; i++) {\n  if (i === 1) continue;\n  console.log(i);\n}", "0 and 2."),
        ]),
        p("Project: Lucky Number", &[
            (r##"CHECKPOINT PROJECT

Keep rolling a 10-sided die until you hit the lucky 7. Count the tries and print how many it took."##, "gem_green"),
            (r##"let lucky = 7;
let roll = 0;
let tries = 0;
while (roll !== lucky) {
  roll = Math.floor(Math.random() * 10) + 1;
  tries++;
}
console.log(`Rolled ${lucky} after ${tries} tries`);"##, "orb_red"),
        ], vec![
            fill("Loop until it matches.", "while (roll ___ lucky) {", &["!==", "===", "<"], 0, "Keep going while NOT equal."),
            fill("Count each try.", "  ___;", &["tries++", "tries", "++"], 0, "tries++ adds one."),
            copy("Roll for luck.", "let lucky = 7;\nlet roll = 0;\nlet tries = 0;\nwhile (roll !== lucky) {\n  roll = Math.floor(Math.random() * 10) + 1;\n  tries++;\n}\nconsole.log(`Rolled ${lucky} after ${tries} tries`);", "A different count each time!"),
            recall("From memory: add 1 + 2 + 3 with a for loop into total, then print total.", "let total = 0;\nfor (let i = 1; i <= 3; i++) {\n  total += i;\n}\nconsole.log(total);", "6"),
        ]),
    ])
}

fn arrays() -> Module {
    ch("Arrays", "Keep many values in one ordered list.", vec![
        l("Making Arrays", &[
            (r##"An ARRAY holds a list of values in order, inside square brackets:

let loot = ["gem", "coin", "key"];"##, "chest"),
            (r##"Arrays can hold any values, even mixed:

let nums = [1, 2, 3];
console.log(nums);   [ 1, 2, 3 ]"##, "coins"),
        ], vec![
            quiz("Arrays use which brackets?", &["( )", "[ ]", "{ }"], 1, "Square brackets."),
            fill("Separate the items.", "let loot = [\"gem\"___ \"coin\"];", &[",", ";", " +"], 0, "Commas separate items."),
            copy("Type this.", "let loot = [\"gem\", \"coin\", \"key\"];\nconsole.log(loot);", "Your bag."),
            recall("From memory: let nums = [1, 2, 3], print nums.", "let nums = [1, 2, 3];\nconsole.log(nums);", "[ 1, 2, 3 ]"),
        ]),
        l("Indexes", &[
            (r##"Each item has a position number called an INDEX. Counting starts at 0!

let party = ["Ada", "Bo", "Cy"];
party[0] is "Ada", party[2] is "Cy"."##, "helmet_knight"),
            (r##"console.log(party[0]);
console.log(party[2]);"##, "map"),
        ], vec![
            quiz("The FIRST item is at index...", &["0", "1", "-1"], 0, "Indexes start at 0."),
            fill("Get Cy.", "console.log(party[___]);", &["2", "3", "1"], 0, "0 Ada, 1 Bo, 2 Cy."),
            copy("Type this.", "let party = [\"Ada\", \"Bo\", \"Cy\"];\nconsole.log(party[0]);\nconsole.log(party[2]);", "Ada, Cy"),
            recall("From memory: let a = [\"x\", \"y\"], print a[1].", "let a = [\"x\", \"y\"];\nconsole.log(a[1]);", "y"),
        ]),
        l("length and Updating", &[
            (r##".length tells you how many items there are.

Change an item by assigning to its index:
bag[1] = "torch";"##, "torch"),
            (r##"let bag = ["map", "rope"];
bag[1] = "torch";
console.log(bag, bag.length);"##, "chest"),
        ], vec![
            quiz("[5, 6, 7].length is...", &["2", "3", "7"], 1, "Three items."),
            fill("Count the items.", "console.log(bag.___);", &["length", "size", "count"], 0, ".length."),
            copy("Type this.", "let bag = [\"map\", \"rope\"];\nbag[1] = \"torch\";\nconsole.log(bag, bag.length);", "[ 'map', 'torch' ] 2"),
            recall("From memory: let b = [5, 6, 7], print b.length.", "let b = [5, 6, 7];\nconsole.log(b.length);", "3"),
        ]),
        l("push and pop", &[
            (r##"push() adds to the END of an array.
pop() removes the last item."##, "potion_magic"),
            (r##"let queue = ["slime"];
queue.push("goblin");
queue.pop();
console.log(queue);   [ 'slime' ]"##, "axe"),
        ], vec![
            quiz("Which adds an item to the end?", &["pop()", "push()", "add()"], 1, "push."),
            fill("Add milk to the list.", "list.___(\"milk\");", &["push", "pop", "put"], 0, "push."),
            copy("Type this.", "let queue = [\"slime\"];\nqueue.push(\"goblin\");\nqueue.pop();\nconsole.log(queue);", "Back to just slime."),
            recall("From memory: let list = []; push \"milk\"; print list.", "let list = [];\nlist.push(\"milk\");\nconsole.log(list);", "[ 'milk' ]"),
        ]),
        l("Looping Arrays", &[
            (r##"for...of visits each item:

for (const s of songs) {
  console.log(s);
}"##, "cup_silver"),
            (r##"Or use the index with a classic for loop:

for (let i = 0; i < xs.length; i++) {
  console.log(xs[i]);
}"##, "gear"),
        ], vec![
            quiz("for (const s of songs) gives s...", &["each index", "each item", "the length"], 1, "of = each value."),
            fill("Visit each song.", "for (const s ___ songs) {", &["of", "in", "from"], 0, "for...of."),
            copy("Type this.", "let songs = [\"Battle\", \"Tavern\", \"Rest\"];\nfor (const s of songs) {\n  console.log(s);\n}", "Three songs."),
            recall("From memory: let xs = [1, 2]; loop with i < xs.length and print xs[i].", "let xs = [1, 2];\nfor (let i = 0; i < xs.length; i++) {\n  console.log(xs[i]);\n}", "1, 2"),
        ]),
        l("Searching Arrays", &[
            (r##"includes(x) - true if x is in the array
indexOf(x)  - x's index, or -1 if missing"##, "keyhole"),
            (r##"let crowd = ["Bo", "Waldo", "Cy"];
console.log(crowd.indexOf("Waldo"));  1"##, "mystic_figure"),
        ], vec![
            quiz("indexOf returns what when the item is missing?", &["0", "-1", "null"], 1, "-1 means not found."),
            fill("Is the gem there?", "console.log(items.___(\"gem\"));", &["includes", "has", "find"], 0, "includes."),
            copy("Find Waldo.", "let crowd = [\"Bo\", \"Waldo\", \"Cy\"];\nconsole.log(crowd.indexOf(\"Waldo\"));", "1"),
            recall("From memory: let items = [\"gem\"]; print items.includes(\"gem\").", "let items = [\"gem\"];\nconsole.log(items.includes(\"gem\"));", "true"),
        ]),
        p("Project: Bard's Playlist", &[
            (r##"CHECKPOINT PROJECT

The bard's playlist needs a new song and a numbered printout. Use push, a loop, indexes and length."##, "cup_silver"),
            (r##"const playlist = ["Dragon Waltz", "Goblin Groove"];
playlist.push("Slime Shuffle");
for (let i = 0; i < playlist.length; i++) {
  console.log(`${i + 1}. ${playlist[i]}`);
}
console.log(`${playlist.length} songs`);"##, "banner_royal"),
        ], vec![
            fill("Add a song.", "playlist.___(\"Slime Shuffle\");", &["push", "pop", "add"], 0, "push."),
            fill("Number from 1, not 0.", "console.log(`${i ___ 1}. ${playlist[i]}`);", &["+", "-", "*"], 0, "i + 1."),
            copy("Print the playlist.", "const playlist = [\"Dragon Waltz\", \"Goblin Groove\"];\nplaylist.push(\"Slime Shuffle\");\nfor (let i = 0; i < playlist.length; i++) {\n  console.log(`${i + 1}. ${playlist[i]}`);\n}\nconsole.log(`${playlist.length} songs`);", "1. 2. 3. and 3 songs."),
            recall("From memory: const p = [\"a\"]; push \"b\"; print p.length.", "const p = [\"a\"];\np.push(\"b\");\nconsole.log(p.length);", "2"),
        ]),
    ])
}

fn functions() -> Module {
    ch("Functions", "Name a spell once, cast it many times.", vec![
        l("Defining and Calling", &[
            (r##"A FUNCTION is a named block of code you can run again and again.

function greet() {
  console.log("Hello, traveler!");
}"##, "staff"),
            (r##"Defining it does nothing yet. CALL it with its name and ( ):

greet();
greet();   runs twice"##, "scroll_lightning"),
        ], vec![
            quiz("How do you run a function named greet?", &["greet", "greet()", "run greet"], 1, "Name plus parentheses."),
            fill("Define a function.", "___ greet() {", &["function", "func", "def"], 0, "function name() { }."),
            copy("Type this.", "function greet() {\n  console.log(\"Hello, traveler!\");\n}\ngreet();", "Define, then call."),
            recall("From memory: a function cheer that prints Huzzah!, then call it.", "function cheer() {\n  console.log(\"Huzzah!\");\n}\ncheer();", "function cheer() { ... } cheer();"),
        ]),
        l("Parameters", &[
            (r##"PARAMETERS are inputs. List them in the ( ):

function greet(name) {
  console.log("Hello, " + name);
}"##, "letter"),
            (r##"Pass ARGUMENTS when calling:

greet("Ada");   Hello, Ada
greet("Bo");    Hello, Bo"##, "helmet"),
        ], vec![
            quiz("In greet(\"Ada\"), \"Ada\" is...", &["the function name", "an argument", "a comment"], 1, "Values passed in are arguments."),
            fill("Accept a name.", "function greet(___) {", &["name", "\"name\"", "()"], 0, "Parameters are plain names."),
            copy("Type this.", "function greet(name) {\n  console.log(\"Hello, \" + name);\n}\ngreet(\"Ada\");\ngreet(\"Bo\");", "One function, two greetings."),
            recall("From memory: function shout(word) prints word.toUpperCase(); call shout(\"run\").", "function shout(word) {\n  console.log(word.toUpperCase());\n}\nshout(\"run\");", "RUN"),
        ]),
        l("Return Values", &[
            (r##"return sends a value BACK to whoever called the function:

function heal(hp) {
  return hp + 10;
}"##, "potion_red"),
            (r##"console.log(heal(5));   15

The function stops at return."##, "heart"),
        ], vec![
            quiz("What does return do?", &["Prints a value", "Sends a value back", "Restarts the function"], 1, "return hands back a result."),
            fill("Send back the result.", "function heal(hp) {\n  ___ hp + 10;\n}", &["return", "give", "print"], 0, "return."),
            copy("Type this and see 15.", "function heal(hp) {\n  return hp + 10;\n}\nconsole.log(heal(5));", "5 + 10 = 15."),
            recall("From memory: function double(n) returns n * 2; print double(4).", "function double(n) {\n  return n * 2;\n}\nconsole.log(double(4));", "8"),
        ]),
        l("Arrow Functions", &[
            (r##"Arrow functions are a short way to write functions:

const square = (n) => n * n;

For one expression, the result is returned automatically."##, "bow"),
            (r##"const add = (a, b) => a + b;
console.log(add(2, 3));   5"##, "sword"),
        ], vec![
            quiz("(n) => n * n returns...", &["n", "n * n", "nothing"], 1, "One expression is returned."),
            fill("Write the arrow.", "const square = (n) ___ n * n;", &["=>", "->", "=="], 0, "=> is the arrow."),
            copy("Type this.", "const square = (n) => n * n;\nconsole.log(square(5));", "25"),
            recall("From memory: const add = (a, b) => a + b; print add(2, 3).", "const add = (a, b) => a + b;\nconsole.log(add(2, 3));", "5"),
        ]),
        l("Scope", &[
            (r##"Variables made INSIDE a function only exist inside it. That's its SCOPE.

function earn() {
  let bonus = 3;
}
bonus does not exist outside."##, "keyhole"),
            (r##"Functions can still READ variables from outside:

let gold = 5;
function earn() {
  let bonus = 3;
  return gold + bonus;
}"##, "coins"),
        ], vec![
            quiz("Can code outside a function use its inner let variables?", &["Yes", "No"], 1, "They live only inside."),
            fill("Hand back the total.", "function earn() {\n  let bonus = 3;\n  ___ gold + bonus;\n}", &["return", "let", "log"], 0, "return."),
            copy("Type this.", "let gold = 5;\nfunction earn() {\n  let bonus = 3;\n  return gold + bonus;\n}\nconsole.log(earn());", "8"),
            recall("From memory: function f() with let x = 1 that returns x; print f().", "function f() {\n  let x = 1;\n  return x;\n}\nconsole.log(f());", "1"),
        ]),
        p("Project: Palindrome Checker", &[
            (r##"CHECKPOINT PROJECT

A palindrome reads the same backwards: level, racecar. Write a function that builds the reversed word and returns whether it matches."##, "spellbook"),
            (r##"function isPalindrome(word) {
  let reversed = "";
  for (let i = word.length - 1; i >= 0; i--) {
    reversed += word[i];
  }
  return reversed === word;
}
console.log(isPalindrome("level"));
console.log(isPalindrome("hero"));"##, "scroll_fire"),
        ], vec![
            fill("Start from the last letter.", "for (let i = word.length ___ 1; i >= 0; i--) {", &["-", "+", "*"], 0, "The last index is length - 1."),
            fill("Return true or false.", "  return reversed ___ word;", &["===", "=", "!=="], 0, "=== compares."),
            copy("Write the checker.", "function isPalindrome(word) {\n  let reversed = \"\";\n  for (let i = word.length - 1; i >= 0; i--) {\n    reversed += word[i];\n  }\n  return reversed === word;\n}\nconsole.log(isPalindrome(\"level\"));\nconsole.log(isPalindrome(\"hero\"));", "true, false"),
            recall("From memory: function isEven(n) returns n % 2 === 0; print isEven(4).", "function isEven(n) {\n  return n % 2 === 0;\n}\nconsole.log(isEven(4));", "true"),
        ]),
    ])
}

fn objects() -> Module {
    ch("Objects", "Group named values into one thing.", vec![
        l("Object Literals", &[
            (r##"An OBJECT groups related values under names called KEYS:

const hero = { name: "Ada", hp: 10 };

Each key: value pair is a PROPERTY."##, "helmet_knight"),
            (r##"Objects describe things: a hero, a car, a pizza order.

console.log(hero);   { name: 'Ada', hp: 10 }"##, "chest"),
        ], vec![
            quiz("Objects use which brackets?", &["[ ]", "{ }", "( )"], 1, "Curly braces."),
            fill("Separate key and value.", "const hero = { name___ \"Ada\" };", &[":", "=", "->"], 0, "key: value."),
            copy("Type this.", "const hero = { name: \"Ada\", hp: 10 };\nconsole.log(hero);", "An object with two properties."),
            recall("From memory: const pet = { name: \"Rex\" }; print pet.", "const pet = { name: \"Rex\" };\nconsole.log(pet);", "{ name: 'Rex' }"),
        ]),
        l("Dot and Bracket Access", &[
            (r##"Read a property with a DOT:
car.color

Or with brackets and a string:
car["seats"]"##, "key"),
            (r##"const car = { color: "red", seats: 4 };
console.log(car.color);     red
console.log(car["seats"]);  4"##, "gear"),
        ], vec![
            quiz("car.color and car[\"color\"] are...", &["the same", "different", "errors"], 0, "Two ways to read one property."),
            fill("Read the color.", "console.log(car___color);", &[".", ":", "->"], 0, "Dot access."),
            copy("Type this.", "const car = { color: \"red\", seats: 4 };\nconsole.log(car.color);\nconsole.log(car[\"seats\"]);", "red, 4"),
            recall("From memory: const p = { x: 1 }; print p.x.", "const p = { x: 1 };\nconsole.log(p.x);", "1"),
        ]),
        l("Changing Objects", &[
            (r##"Assign to change a property, or to add a new one:

pizza.size = "small";
pizza.cheese = true;"##, "potion_magic"),
            (r##"Even a const object can change its properties. const only stops you replacing the whole object."##, "shield"),
        ], vec![
            quiz("Can you change properties of a const object?", &["Yes", "No"], 0, "Yes, const locks the variable, not the contents."),
            fill("Add a new property.", "pizza.___ = true;", &["cheese", "\"cheese\"", "[cheese]"], 0, "Just name it with a dot."),
            copy("Type this.", "const pizza = { size: \"large\" };\npizza.size = \"small\";\npizza.cheese = true;\nconsole.log(pizza);", "{ size: 'small', cheese: true }"),
            recall("From memory: const o = { hp: 5 }; set o.hp to 9; print o.hp.", "const o = { hp: 5 };\no.hp = 9;\nconsole.log(o.hp);", "9"),
        ]),
        l("Nested Data", &[
            (r##"Properties can hold arrays or other objects. Spread a big object over several lines:

const monster = {
  name: "Slime",
  moves: ["bounce", "ooze"]
};"##, "mystic_figure"),
            (r##"Chain the access:

monster.moves[1]   "ooze"
monster.moves.length   2"##, "potion_red"),
        ], vec![
            quiz("monster.moves[0] is...", &["\"Slime\"", "\"bounce\"", "\"ooze\""], 1, "The first move."),
            fill("Get the second move.", "console.log(monster.moves[___]);", &["1", "2", "0"], 0, "Index 1 is second."),
            copy("Type this.", "const monster = {\n  name: \"Slime\",\n  moves: [\"bounce\", \"ooze\"]\n};\nconsole.log(monster.moves[1]);", "ooze"),
            recall("From memory: const m = { tags: [\"a\", \"b\"] }; print m.tags.length.", "const m = { tags: [\"a\", \"b\"] };\nconsole.log(m.tags.length);", "2"),
        ]),
        l("Looping Objects", &[
            (r##"for...in visits each KEY of an object:

for (const key in stats) {
  console.log(key, stats[key]);
}"##, "scroll_lightning"),
            (r##"Object.keys(obj) gives an array of the keys:
Object.keys({ a: 1 })   [ 'a' ]"##, "map"),
        ], vec![
            quiz("for (const k in obj) gives k...", &["each value", "each key", "the length"], 1, "in = keys."),
            fill("Loop over the keys.", "for (const key ___ stats) {", &["in", "of", "from"], 0, "for...in for objects."),
            copy("Type this.", "const stats = { str: 5, dex: 7 };\nfor (const key in stats) {\n  console.log(key, stats[key]);\n}", "str 5, dex 7"),
            recall("From memory: const s = { a: 1 }; print Object.keys(s).", "const s = { a: 1 };\nconsole.log(Object.keys(s));", "[ 'a' ]"),
        ]),
        p("Project: Monster Card", &[
            (r##"CHECKPOINT PROJECT

Describe a monster as an object, power it up, then print its card with template literals and a loop over its moves."##, "mystic_figure"),
            (r##"const monster = {
  name: "Sparkmouse",
  type: "Electric",
  hp: 35,
  moves: ["Zap", "Dash"]
};
monster.hp += 10;
console.log(`${monster.name} (${monster.type})`);
console.log(`HP: ${monster.hp}`);
for (const move of monster.moves) {
  console.log(`- ${move}`);
}"##, "scroll_lightning"),
        ], vec![
            fill("Power up by 10.", "monster.hp ___ 10;", &["+=", "=+", "++"], 0, "+= adds."),
            fill("Loop over the moves array.", "for (const move ___ monster.moves) {", &["of", "in", "at"], 0, "for...of for arrays."),
            copy("Print the card.", "const monster = {\n  name: \"Sparkmouse\",\n  type: \"Electric\",\n  hp: 35,\n  moves: [\"Zap\", \"Dash\"]\n};\nmonster.hp += 10;\nconsole.log(`${monster.name} (${monster.type})`);\nconsole.log(`HP: ${monster.hp}`);\nfor (const move of monster.moves) {\n  console.log(`- ${move}`);\n}", "A complete monster card!"),
            recall("From memory: const m = { name: \"Bat\", hp: 8 }; print m.name and m.hp.", "const m = { name: \"Bat\", hp: 8 };\nconsole.log(m.name, m.hp);", "Bat 8"),
        ]),
    ])
}

fn the_page() -> Module {
    ch("The Page (DOM)", "HTML + CSS + JS together: change a page with code.", vec![
        l("Finding Elements", &[
            (r##"The browser turns HTML into the DOM (Document Object Model): a tree of objects JavaScript can change.

The page itself is the object called document."##, "map"),
            (r##"Find an element by its id:

const title = document.getElementById("title");

Pixel Learn simulates a page so you can see each change."##, "keyhole"),
        ], vec![
            quiz("document.getElementById(\"title\") finds...", &["every h1", "the element with id=\"title\"", "the page title bar"], 1, "It matches the id."),
            fill("Find by id.", "document.___(\"title\");", &["getElementById", "getId", "find"], 0, "getElementById."),
            copy("Type this.", "const title = document.getElementById(\"title\");\nconsole.log(title);", "An element object."),
            recall("From memory: find the element with id menu.", "document.getElementById(\"menu\");", "getElementById(\"menu\")"),
        ]),
        l("Changing Text", &[
            (r##".textContent reads or sets the text inside an element:

title.textContent = "Quest Board";

The page updates instantly."##, "scroll_fire"),
            (r##"You can do it in one line too:

document.getElementById("msg").textContent = "Hi";"##, "letter"),
        ], vec![
            quiz("Which property sets an element's text?", &["text", "textContent", "value"], 1, "textContent."),
            fill("Change the words.", "title.___ = \"Quest Board\";", &["textContent", "innerText()", "words"], 0, "textContent."),
            copy("Type this.", "const title = document.getElementById(\"title\");\ntitle.textContent = \"Quest Board\";", "The heading changes."),
            recall("From memory: set the text of #msg to Hi in one line.", "document.getElementById(\"msg\").textContent = \"Hi\";", "getElementById(\"msg\").textContent = \"Hi\""),
        ]),
        l("Changing Styles", &[
            (r##".style changes CSS from JavaScript:

box.style.color = "black";

CSS names with dashes become camelCase: background-color -> backgroundColor."##, "potion_magic"),
            (r##"const box = document.getElementById("box");
box.style.backgroundColor = "gold";
box.style.color = "black";"##, "crystal_purple"),
        ], vec![
            quiz("background-color in JavaScript is written...", &["background-color", "backgroundColor", "BackgroundColor"], 1, "camelCase."),
            fill("Set the text color.", "box.style.___ = \"black\";", &["color", "Color", "text-color"], 0, "style.color."),
            copy("Type this.", "const box = document.getElementById(\"box\");\nbox.style.backgroundColor = \"gold\";\nbox.style.color = \"black\";", "A golden box."),
            recall("From memory: make #t red with style.color in one line.", "document.getElementById(\"t\").style.color = \"red\";", "style.color = \"red\""),
        ]),
        l("Events", &[
            (r##"An EVENT is something that happens: a click, a key press.

addEventListener runs a function when the event happens."##, "sword"),
            (r##"const btn = document.getElementById("attack");
btn.addEventListener("click", () => {
  console.log("You swing your sword!");
});

Pixel Learn simulates one click so you see the result."##, "axe"),
        ], vec![
            quiz("addEventListener(\"click\", fn) runs fn...", &["right away, once", "every time it's clicked", "never"], 1, "On each click."),
            fill("Listen for clicks.", "btn.addEventListener(\"___\", () => {", &["click", "onclick", "press"], 0, "The event name is \"click\"."),
            copy("Type this.", "const btn = document.getElementById(\"attack\");\nbtn.addEventListener(\"click\", () => {\n  console.log(\"You swing your sword!\");\n});", "Click to attack!"),
            recall("From memory: get #b, and on click alert(\"Hi\").", "const b = document.getElementById(\"b\");\nb.addEventListener(\"click\", () => {\n  alert(\"Hi\");\n});", "addEventListener + alert."),
        ]),
        l("querySelector and innerHTML", &[
            (r##"document.querySelector() finds the first element matching ANY CSS selector:

document.querySelector("#loot")
document.querySelector(".hp")"##, "gear"),
            (r##".innerHTML sets HTML inside an element:

list.innerHTML = "<li>Gem</li>";"##, "chest"),
        ], vec![
            quiz("querySelector(\".hp\") finds...", &["the first element with class hp", "every element", "the id hp"], 0, "Dot = class, just like CSS."),
            fill("Put HTML inside.", "list.___ = \"<li>Gem</li>\";", &["innerHTML", "textContent", "html"], 0, "innerHTML parses tags."),
            copy("Type this.", "const list = document.querySelector(\"#loot\");\nlist.innerHTML = \"<li>Gem</li>\";", "A new list item."),
            recall("From memory: set the text of .hp to \"10\" with querySelector.", "document.querySelector(\".hp\").textContent = \"10\";", "querySelector(\".hp\").textContent"),
        ]),
        p("Final Project: Mood Ring", &[
            (r##"FINAL PROJECT

The Mood Ring glows a random color with a matching mood. Pick a random index and change the ring's background and text."##, "amulet"),
            (r##"const ring = document.getElementById("ring");
const moods = ["calm", "happy", "fierce"];
const colors = ["blue", "gold", "red"];
let pick = Math.floor(Math.random() * moods.length);
ring.style.backgroundColor = colors[pick];
ring.textContent = `You feel ${moods[pick]}`;"##, "crystal_blue"),
        ], vec![
            fill("A random index for the array.", "let pick = Math.floor(Math.random() * moods.___);", &["length", "size", "count"], 0, ".length."),
            fill("Change the glow.", "ring.style.___ = colors[pick];", &["backgroundColor", "background-color", "bg"], 0, "camelCase in JS."),
            copy("Build the ring.", "const ring = document.getElementById(\"ring\");\nconst moods = [\"calm\", \"happy\", \"fierce\"];\nconst colors = [\"blue\", \"gold\", \"red\"];\nlet pick = Math.floor(Math.random() * moods.length);\nring.style.backgroundColor = colors[pick];\nring.textContent = `You feel ${moods[pick]}`;", "HTML + CSS + JS: the trifecta!"),
            recall("From memory: get #ring into r and set r.style.color to blue.", "const r = document.getElementById(\"ring\");\nr.style.color = \"blue\";", "style.color = \"blue\""),
        ]),
    ])
}
