//! JavaScript: the full explanation, key points and extra practice for every lesson.
use super::*;

pub fn deep() -> Vec<Deep> {
    vec![
        // ---------------- The Console
        d("Hello, Console", r##"## What JavaScript does
HTML builds a page and CSS styles it. JavaScript makes it DO things: react to clicks, calculate scores, change text, run games. It is a real programming language that runs inside every web browser.

## Your first instruction
    console.log("Hello, hero!");

- console is a built-in tool for developers.
- .log() is a command on it that prints a message.
- The message goes between the round brackets.
- The text is in quotes, which makes it a string.
- The semicolon marks the end of the instruction, like a full stop.

## Where does it appear?
In a browser, press F12 and open the Console tab: that is where console.log writes. Players never see it, so it is the programmer's notebook for checking what the code is doing. In Code Forge, the output appears right under your code.

## Programs run in order
    console.log("Ready");
    console.log("Set");
    console.log("Go!");

prints three lines, top to bottom. Computers follow instructions exactly and in order.

## Common mistakes
JavaScript is case sensitive: Console.Log does not exist. Also check that every opening bracket and quote has a closing partner."##,
            &["JavaScript makes pages interactive and runs in every browser.", "console.log(...) prints a message for the developer.", "Text in quotes is a string.", "Instructions run top to bottom; names are case sensitive."],
            vec![
                quiz("Who normally sees console.log messages?", &["Every visitor", "The developer, in the console", "Nobody"], 1, "It is the programmer's notebook."),
                quiz("Which line is correct?", &["Console.Log(\"Hi\");", "console.log(\"Hi\");", "console.log(Hi\");"], 1, "Lowercase, matching quotes and brackets."),
                fill("Print Ready.", "console.___(\"Ready\");", &["log", "print", "write"], 0, "console.log prints."),
            ]),
        d("Strings and Numbers", r##"## Two kinds of values
The values you print can be strings (text) or numbers.

    console.log("Gold coins");   // a string
    console.log(42);             // a number

Strings go in quotes; numbers do not. You can use double quotes "..." or single quotes '...', as long as both ends match.

## Numbers do maths
    console.log(10 + 5);    // 15
    console.log(10 - 3);    // 7
    console.log(4 * 2);     // 8
    console.log(9 / 3);     // 3

* means multiply and / means divide.

## The quote trap
"10" + "5" is NOT 15. With quotes, they are strings, and + joins strings together:

    console.log("10" + "5");   // 105

And mixing a string with a number also joins: "Level " + 3 gives "Level 3". That is useful for messages, but surprising when you wanted maths.

## Quotes inside strings
To put a quote inside a string, use the other kind outside: "It's a trap!" or 'He said "run"'.

## Rule of thumb
If you want to calculate, write numbers without quotes. If you want to show words, use quotes."##,
            &["Strings are text in quotes; numbers have no quotes.", "+ - * / do maths with numbers.", "+ with strings joins them: \"10\" + \"5\" is \"105\".", "Use the other quote type to put quotes inside a string."],
            vec![
                quiz("What does console.log(\"10\" + \"5\") print?", &["15", "105", "Error"], 1, "Strings are joined, not added."),
                quiz("Which symbol multiplies?", &["x", "*", "^"], 1, "* is multiply."),
                fill("Print the number 12, not text.", "console.log(___);", &["12", "\"12\"", "twelve"], 0, "Numbers have no quotes."),
            ]),
        d("Comments", r##"## Notes in your code
Comments are notes for humans. JavaScript ignores them completely.

    // This is a single-line comment
    console.log("Hi"); // a comment can follow code

    /* This is a
       multi-line comment */

- // comments out the rest of the line.
- /* ... */ can span many lines.

## What to write in comments
Good comments explain WHY, not what. The code already says what it does:

    // bad: add 1 to lives
    // good: the first win of the day gives a bonus life

Use comments to label sections, explain a tricky rule, or leave a TODO note.

## Switching code off
Putting // in front of a line "comments it out": the line stays in the file but does not run. This is a handy way to test what happens without a line, without deleting it.

    // console.log("debug info");

## Do not over-comment
A comment on every line makes code harder to read. Clear names and short functions often explain themselves. Aim for a few comments in the places where a reader might get stuck."##,
            &["// comments out the rest of a line.", "/* ... */ comments out several lines.", "Good comments explain why, not what.", "Comment out a line to switch it off while testing."],
            vec![
                quiz("Which comment can span several lines?", &["// ...", "/* ... */", "<!-- ... -->"], 1, "/* */ spans lines in JavaScript."),
                quiz("What does a good comment explain?", &["Why the code does something", "Every single line", "The colour of the code"], 0, "The code shows what; comments say why."),
                fill("Switch this line off.", "___ console.log(\"debug\");", &["//", "#", "--"], 0, "// comments out the line."),
            ]),
        d("Many Lines", r##"## Programs are lists of steps
Real programs are many instructions, run one after another from top to bottom. Each instruction is called a statement and usually ends with a semicolon.

    console.log("Welcome to the shop!");
    console.log("Potion: 5 gold");
    console.log("Sword: 20 gold");
    console.log("Total: " + (5 + 20) + " gold");

## Order matters
If you swap two lines, the output swaps too. When something prints in the wrong place, follow the code line by line like the computer does.

## Brackets in maths
JavaScript follows the usual maths rules: * and / happen before + and -. Brackets go first:

    console.log(2 + 3 * 4);     // 14
    console.log((2 + 3) * 4);   // 20

In the shop example, (5 + 20) is in brackets so it is added as numbers BEFORE being joined to the text. Without the brackets you would get "Total: 520 gold".

## Errors stop the program
If one line has an error, the lines after it usually do not run. Read the error message: it says what went wrong and often on which line. Fix the first error first; later ones are often caused by it."##,
            &["A program is statements run top to bottom.", "* and / happen before + and -; brackets go first.", "Use brackets to do maths before joining numbers to text.", "An error usually stops the lines after it; fix the first error first."],
            vec![
                quiz("What does console.log(2 + 3 * 4) print?", &["20", "14", "234"], 1, "Multiplication first: 2 + 12."),
                quiz("Why write \"Total: \" + (5 + 20)?", &["To add 5 and 20 before joining", "For decoration", "Brackets are required"], 0, "Without brackets you get 520."),
                fill("Calculate (2 + 3) times 4.", "console.log(___ * 4);", &["(2 + 3)", "2 + 3", "[2 + 3]"], 0, "Brackets go first."),
            ]),
        d("Project: Shop Receipt", r##"## Print a receipt
The potion shop needs a receipt printer. Your program prints a title, a line for each item with its cost, and the total, and JavaScript does all the maths.

## Plan
- A title line.
- Two potions at 15 gold each: let JavaScript multiply.
- One elixir at 40 gold.
- The total, calculated from the same numbers.

## Build it
    console.log("=== POTION SHOP ===");
    console.log("Potion x2:", 2 * 15);
    console.log("Elixir x1:", 40);
    console.log("Total:", 2 * 15 + 40);

It prints:

    === POTION SHOP ===
    Potion x2: 30
    Elixir x1: 40
    Total: 70

## Commas in console.log
console.log("Total:", 70) prints both values with a space between them. Because the number is a separate value, JavaScript works out 2 * 15 + 40 first (multiplication before addition: 30 + 40) and then prints the result.

## Why let JavaScript add it up?
If a price changes, the total stays correct. Doing maths by hand and typing the answer causes mistakes.

## Check
- Do all the strings have matching quotes?
- Does every line end with a semicolon?"##,
            &["Build output line by line, top to bottom.", "Let the program calculate totals instead of typing them.", "console.log(a, b) prints several values separated by spaces.", "* happens before +, so 2 * 15 + 40 is 70."],
            vec![
                quiz("Why calculate the total in code?", &["It stays correct when prices change", "It looks nicer", "It is required"], 0, "Code does the maths reliably."),
                fill("Print the label and the number as two values.", r##"console.log("Elixir x1:"___ 40);"##, &[",", ";", "."], 0, "A comma separates the values console.log prints."),
                quiz("What does console.log(\"Potion x2:\", 2 * 15) print?", &["Potion x2: 30", "Potion x2: 2 * 15", "Potion x2:30 gold"], 0, "The maths runs first, then both values print with a space."),
            ]),

        // ---------------- Variables
        d("let and const", r##"## Boxes with names
A variable is a named box that stores a value so you can use it later.

    let gold = 10;
    const heroName = "Pix";

- let creates a variable whose value can change later.
- const creates a constant: once set, it cannot be given a new value.
- = means "store this value", not "is equal to".

## Using a variable
Write its name without quotes:

    console.log(gold);          // 10
    console.log(heroName);      // Pix
    console.log("heroName");    // heroName (just text!)

## Changing it
    gold = 15;     // fine, gold was made with let
    heroName = "Max";   // error: heroName is a const

## Which one to use?
Start with const. Switch to let only when you know the value must change (a score, a counter). This makes code easier to trust, because readers know which values stay fixed.

## Naming rules
- Names can use letters, digits, _ and $, but cannot start with a digit.
- No spaces: use camelCase like maxHealth.
- Names are case sensitive: gold and Gold are different.
- Choose descriptive names: lives, not x."##,
            &["Variables store values under a name.", "let can be changed later; const cannot.", "= stores a value; use the name without quotes.", "Use camelCase and descriptive names; prefer const."],
            vec![
                quiz("Which keyword makes a value that cannot be reassigned?", &["let", "const", "var"], 1, "const = constant."),
                quiz("What does console.log(\"gold\") print?", &["The value of gold", "The word gold", "Nothing"], 1, "Quotes make it text."),
                fill("Make a changeable score.", "___ score = 0;", &["let", "const", "set"], 0, "let allows changes."),
            ]),
        d("Data Types", r##"## Every value has a type
JavaScript values come in a few basic types:

- string: text in quotes, "Pix".
- number: whole or decimal, 42 or 3.5.
- boolean: true or false (no quotes).
- undefined: a variable with no value yet.
- null: deliberately "nothing".

    let name = "Pix";       // string
    let hp = 30;            // number
    let isAlive = true;     // boolean
    let weapon;             // undefined

## typeof
The typeof operator tells you a value's type:

    console.log(typeof hp);       // number
    console.log(typeof name);     // string
    console.log(typeof isAlive);  // boolean

## Why types matter
The same symbol does different things for different types: 2 + 2 is 4, but "2" + "2" is "22". Many bugs come from a number accidentally being a string (for example, text typed into a form is always a string). When a result looks strange, check the types with typeof.

## Booleans
Booleans answer yes/no questions: Is the door open? Has the player won? You will use them all the time with if statements in the next chapter."##,
            &["Basic types: string, number, boolean, undefined, null.", "typeof value tells you the type.", "Types change behaviour: \"2\" + \"2\" is \"22\".", "Booleans (true/false) answer yes/no questions."],
            vec![
                quiz("What is the type of true?", &["string", "boolean", "number"], 1, "true and false are booleans."),
                quiz("A variable declared with no value is...", &["null", "undefined", "0"], 1, "No value yet = undefined."),
                fill("Find out the type of hp.", "console.log(___ hp);", &["typeof", "type", "kind"], 0, "typeof gives the type."),
            ]),
        d("Arithmetic", r##"## Maths operators
    +   add
    -   subtract
    *   multiply
    /   divide
    %   remainder (modulo)
    **  power

    let damage = 12 * 2;    // 24
    let half = 9 / 2;       // 4.5
    let left = 10 % 3;      // 1

## The remainder operator %
% gives what is left over after dividing. 10 % 3 is 1, because 3 goes into 10 three times with 1 left. It is surprisingly useful:
- n % 2 is 0 for even numbers and 1 for odd numbers.
- It can wrap things around, like turning hour 25 into hour 1.

## Power
2 ** 3 means 2 × 2 × 2 = 8.

## Order of operations
** first, then * / %, then + -. Use brackets whenever the order is not obvious:

    let total = (price + tax) * amount;

## Whole numbers
Division can give decimals. Math.floor() rounds down and Math.round() rounds to the nearest whole number:

    console.log(Math.floor(9 / 2));   // 4

## Dividing by zero
In JavaScript, 5 / 0 gives Infinity instead of crashing. That rarely means what you wanted, so check before dividing."##,
            &["+ - * / do the usual maths; ** is power.", "% gives the remainder: 10 % 3 is 1.", "n % 2 tells even (0) from odd (1).", "Use brackets for clarity; Math.floor and Math.round make whole numbers."],
            vec![
                quiz("What is 10 % 4?", &["2", "2.5", "6"], 0, "4 goes in twice, 2 left over."),
                quiz("How can you test if n is even?", &["n % 2 === 0", "n / 2 === 0", "n * 2 === 0"], 0, "Even numbers leave no remainder."),
                fill("Round 4.5 down to 4.", "console.log(Math.___(4.5));", &["floor", "down", "cut"], 0, "Math.floor rounds down."),
            ]),
        d("Updating Variables", r##"## Changing values over time
Games are full of values that change: health goes down, gold goes up, the turn counter ticks. With let you can reassign a variable:

    let hp = 30;
    hp = hp - 5;     // hp is now 25

Read it right to left: take hp (30), subtract 5, store the result back in hp.

## Shortcuts
    hp -= 5;     // same as hp = hp - 5
    gold += 10;  // gold = gold + 10
    power *= 2;  // power = power * 2
    turns++;     // add 1
    lives--;     // subtract 1

## Strings too
+= also adds text to the end of a string:

    let log = "Start";
    log += " -> Cave";     // "Start -> Cave"

## Why not const?
A const cannot be reassigned, so hp -= 5 on a const causes an error. Use let for any value you will update.

## Tracing
When you are unsure what a variable holds, follow the code line by line and write the value after each step, or add console.log(hp) after each change. This is called tracing and it is how programmers find bugs."##,
            &["Reassign a let variable with =.", "+=, -=, *= update using the current value.", "++ adds 1 and -- subtracts 1.", "Trace values line by line to understand code."],
            vec![
                quiz("let gold = 5; gold += 3; What is gold?", &["3", "8", "53"], 1, "5 + 3 = 8."),
                quiz("What does lives-- do?", &["Subtracts 1", "Deletes lives", "Adds 1"], 0, "-- subtracts one."),
                fill("Double the power.", "power ___ 2;", &["*=", "+=", "=*"], 0, "*= multiplies in place."),
            ]),
        d("Template Literals", r##"## Building text with values
Joining strings with + gets messy fast:

    console.log("Hero " + name + " has " + hp + " HP");

Template literals are cleaner. Use backticks (`) instead of quotes and put values inside ${ }:

    console.log(`Hero ${name} has ${hp} HP`);

## Anything goes inside ${ }
You can put any expression there, not only variable names:

    console.log(`Total: ${price * amount} gold`);
    console.log(`Next level in ${100 - xp} XP`);

## Where is the backtick?
On most keyboards it is the key above Tab, left of 1. Regular quotes "..." do NOT read ${ }: they print it literally.

## Multiple lines
Backtick strings can span several lines, which is handy for longer messages.

## Spaces count
Everything between the backticks is kept exactly, including spaces. `${a}${b}` has no space between the values; `${a} ${b}` has one."##,
            &["Template literals use backticks: `...`.", "${expression} inserts a value into the text.", "Any expression works inside ${ }.", "Normal quotes do not fill in ${ }."],
            vec![
                quiz("Which string fills in the value of name?", &["\"Hi ${name}\"", "`Hi ${name}`", "'Hi ${name}'"], 1, "Only backticks fill in ${ }."),
                quiz("What can go inside ${ }?", &["Only variable names", "Any expression, like price * 2", "Only numbers"], 1, "Any expression works."),
                fill("Insert the hp value.", "let msg = `HP: ___`;", &["${hp}", "{hp}", "$hp"], 0, "${hp} inserts the value."),
            ]),
        d("Project: Planet Weight", r##"## How much would a hero weigh on Mars?
Gravity is weaker on Mars, so a hero weighs less there: about 0.38 times their Earth weight. In this project you store the numbers in variables, calculate the Mars weight and print a sentence.

## Plan
1. Store the hero's Earth weight: 60 kg.
2. Store Mars' gravity factor: 0.38.
3. Multiply them and keep the result.
4. Print a sentence with the result inside.

## Build it
    const weight = 60;
    const marsGravity = 0.38;
    let marsWeight = weight * marsGravity;
    console.log(`On Mars you weigh ${marsWeight} kg`);

It prints: On Mars you weigh 22.8 kg

## const or let?
weight and marsGravity never change while the program runs, so const protects them from accidental changes. Change the weight once at the top and the result updates by itself.

## Template literals
Backticks make a template literal: ${marsWeight} drops the variable's value straight into the text, with no + signs needed.

## Check
- Are values that never change declared with const?
- Does the message use backticks, not quotes?"##,
            &["Store fixed values in const.", "Calculate results from variables, not typed numbers.", "Backticks and ${ } put values inside text.", "Change a value once at the top and every result follows."],
            vec![
                quiz("What does the program print?", &["On Mars you weigh 22.8 kg", "On Mars you weigh ${marsWeight} kg", "On Mars you weigh 60 kg"], 0, "60 * 0.38 is 22.8, placed into the text by ${ }."),
                fill("Put the result inside the message.", r##"console.log(`On Mars you weigh ${___} kg`);"##, &["marsWeight", "weight", "marsGravity"], 0, "The variable holding the result goes inside ${ }."),
                quiz("Which line prints the weight using a template literal?", &["console.log(`Mars: ${w} kg`);", "console.log(\"Mars: ${w} kg\");", "console.log(Mars: w kg);"], 0, "Only backticks turn ${ } into a value."),
            ]),

        // ---------------- Conditionals
        d("if", r##"## Making decisions
Programs often need to choose: only open the door if the player has the key. That is what if does.

    let hasKey = true;
    if (hasKey) {
      console.log("The door opens.");
    }

- The condition goes in round brackets.
- The code to run goes in curly braces { }, called a block.
- If the condition is true, the block runs. If it is false, the block is skipped and the program continues after it.

## Conditions are true or false
Any comparison gives a boolean:

    if (gold >= 10) {
      console.log("You can buy a potion.");
    }

## Indentation
Indent the code inside the block by two spaces. JavaScript does not need it, but it shows at a glance which lines belong to the if.

## Truthy and falsy
Non-boolean values also work as conditions. 0, "" (empty string), null and undefined count as false ("falsy"); almost everything else counts as true ("truthy"). So if (name) means "if name is not empty".

## Common mistake
if (gold = 10) uses a single = which STORES 10 instead of comparing. To compare, use === (next lessons)."##,
            &["if (condition) { ... } runs the block only when the condition is true.", "Conditions produce true or false.", "Indent the code inside the block.", "0, \"\", null and undefined count as false."],
            vec![
                quiz("What happens when the if condition is false?", &["The program crashes", "The block is skipped", "The block runs twice"], 1, "False means skip the block."),
                quiz("Which value counts as false in a condition?", &["\"hi\"", "0", "5"], 1, "0 is falsy."),
                fill("Run the block only when hp is above 0.", "___ (hp > 0) {\n  console.log(\"Alive\");\n}", &["if", "when", "check"], 0, "if starts a decision."),
            ]),
        d("else", r##"## Two paths
if runs code when a condition is true. else gives the other path: code that runs when it is false.

    if (gold >= 10) {
      console.log("You buy the potion.");
    } else {
      console.log("Not enough gold.");
    }

Exactly ONE of the two blocks runs, never both and never neither.

## else has no condition
else never has brackets after it. It simply catches every case the if did not.

## Think in pairs
Many decisions are yes/no:
- Is the answer right? Show "Correct" or "Try again".
- Is it night? Show the moon or the sun.
- Is the player old enough? Allow or block.

## Watch the braces
The else goes right after the closing brace of the if block. A common bug is an extra semicolon or a missing } that disconnects them.

## Variables set in both paths
A clean pattern is to choose a value in each branch:

    let message;
    if (score >= 50) {
      message = "Pass";
    } else {
      message = "Try again";
    }
    console.log(message);"##,
            &["else runs when the if condition is false.", "Exactly one of the if or else blocks runs.", "else never has a condition.", "Use if/else for yes-or-no decisions."],
            vec![
                quiz("Can both the if block and the else block run?", &["Yes", "No, exactly one runs"], 1, "One or the other."),
                quiz("What goes after else?", &["A condition in brackets", "A block in braces", "A semicolon"], 1, "else has no condition."),
                fill("Add the other path.", "if (win) {\n  console.log(\"Victory\");\n} ___ {\n  console.log(\"Defeat\");\n}", &["else", "otherwise", "elif"], 0, "else catches the other case."),
            ]),
        d("else if", r##"## More than two choices
Some decisions have several outcomes. else if adds extra conditions between the if and the else:

    if (score >= 90) {
      console.log("Gold medal");
    } else if (score >= 70) {
      console.log("Silver medal");
    } else if (score >= 50) {
      console.log("Bronze medal");
    } else {
      console.log("Keep training");
    }

## Top to bottom, first match wins
JavaScript checks each condition in order and runs the FIRST block whose condition is true. Then it skips the rest. With score = 95, only "Gold medal" prints, even though 95 is also >= 70.

## Order matters
If you put score >= 50 first, a score of 95 would get Bronze, because that check matches first. Put the most specific (or highest) conditions first.

## The final else is optional
Without an else, nothing happens when no condition matches. With it, you have a safe default.

## Python note
Other languages write elif or elsif. In JavaScript it is two words: else if."##,
            &["else if adds more conditions between if and else.", "Conditions are checked top to bottom; the first true one wins.", "Put the most specific conditions first.", "A final else catches everything else."],
            vec![
                quiz("score = 95. With the checks >= 90, >= 70, >= 50, which blocks run?", &["All three", "Only the first", "Only the last"], 1, "First match wins."),
                quiz("Why put score >= 90 before score >= 50?", &["Higher checks must come first or they never match", "It is faster", "No reason"], 0, "Otherwise 95 would match >= 50 first."),
                fill("Add another condition.", "if (hp > 20) {\n  console.log(\"Healthy\");\n} ___ (hp > 0) {\n  console.log(\"Hurt\");\n}", &["else if", "elif", "if else"], 0, "In JavaScript: else if."),
            ]),
        d("Comparison Operators", r##"## Comparing values
Comparisons produce true or false:

    ===   equal (same value AND same type)
    !==   not equal
    >     greater than
    <     less than
    >=    greater than or equal
    <=    less than or equal

    console.log(5 > 3);        // true
    console.log(5 === 5);      // true
    console.log(5 === "5");    // false (number vs string)

## Three equals signs
- = stores a value.
- === compares strictly: value and type must match.
- == compares loosely and converts types first, so 5 == "5" is true. This causes confusing bugs, so always use === and !==.

## Comparing strings
"apple" === "apple" is true. Comparisons are case sensitive: "Pix" === "pix" is false.

## Edges
A score of exactly 50 is NOT > 50, but it IS >= 50. Many bugs live on the edge, so think about what should happen exactly at the boundary."##,
            &["Comparisons give true or false.", "Use === and !== (strict); avoid == and !=.", "= stores, === compares.", "Watch boundaries: 50 > 50 is false, 50 >= 50 is true."],
            vec![
                quiz("What does 5 === \"5\" give?", &["true", "false", "Error"], 1, "Different types, so strictly not equal."),
                quiz("score is exactly 50. Which is true?", &["score > 50", "score >= 50", "score < 50"], 1, ">= includes the boundary."),
                fill("Check that the name is not Pix.", "if (name ___ \"Pix\") { }", &["!==", "=!", "<>"], 0, "!== means strictly not equal."),
            ]),
        d("Logical Operators", r##"## Combining conditions
Sometimes one condition is not enough. Logical operators combine them:

    &&   AND: true only if BOTH sides are true
    ||   OR: true if AT LEAST ONE side is true
    !    NOT: flips true to false and false to true

    if (hasKey && doorIsLocked) {
      console.log("Unlock the door");
    }
    if (isWeekend || isHoliday) {
      console.log("The shop is closed");
    }
    if (!isAlive) {
      console.log("Game over");
    }

## Truth tables in words
- true && true is true; any false makes && false.
- false || false is false; any true makes || true.

## Ranges
To check a number is between two values, use && with two comparisons:

    if (age >= 13 && age <= 19) {
      console.log("Teenager");
    }

You cannot write 13 <= age <= 19 like in maths; it does something else.

## Short-circuiting
JavaScript stops as soon as it knows the answer. In a && b, if a is false, b is never checked. This can protect you: if (player && player.hp > 0) avoids an error when player is missing."##,
            &["&& is true only when both sides are true.", "|| is true when at least one side is true.", "! flips a boolean.", "Check ranges with two comparisons joined by &&."],
            vec![
                quiz("true && false gives...", &["true", "false"], 1, "AND needs both to be true."),
                quiz("false || true gives...", &["true", "false"], 0, "OR needs at least one true."),
                fill("Age between 13 and 19.", "if (age >= 13 ___ age <= 19) { }", &["&&", "||", "and"], 0, "Both must be true: &&."),
            ]),
        d("Random Numbers", r##"## Rolling dice
Games need chance: dice rolls, loot drops, critical hits. Math.random() gives a random decimal from 0 up to (but not including) 1, such as 0.4721.

## From decimal to dice
To get a whole number from 1 to 6:

    let roll = Math.floor(Math.random() * 6) + 1;

Step by step:
1. Math.random() → 0 to 0.999...
2. × 6 → 0 to 5.999...
3. Math.floor → 0, 1, 2, 3, 4 or 5
4. + 1 → 1 to 6

## The general recipe
A whole number from 1 to N: Math.floor(Math.random() * N) + 1.
From 0 to N - 1: leave out the + 1, which is perfect for picking a list position.

## Chances
To do something 25% of the time:

    if (Math.random() < 0.25) {
      console.log("Critical hit!");
    }

## Testing random code
Random results change every run, so run your code several times and check every outcome can happen, including the smallest and largest values."##,
            &["Math.random() gives a decimal from 0 up to (not including) 1.", "Math.floor(Math.random() * N) + 1 gives 1 to N.", "Math.random() < 0.25 is true about 25% of the time.", "Run random code several times to test every outcome."],
            vec![
                quiz("Can Math.random() return exactly 1?", &["Yes", "No"], 1, "It goes up to, but not including, 1."),
                quiz("Math.floor(Math.random() * 6) without + 1 gives...", &["1 to 6", "0 to 5", "0 to 6"], 1, "The + 1 shifts 0-5 to 1-6."),
                fill("A 10% chance.", "if (Math.random() < ___) { }", &["0.1", "10", "0.01"], 0, "0.1 is 10%."),
            ]),
        d("Project: Oracle Orb", r##"## Build a magic orb
The Oracle Orb answers any question with a random prophecy. You combine a random number with an if / else if / else chain.

## Plan
1. Pick a random whole number: 0, 1 or 2.
2. Turn each number into a different answer.
3. Print the answer.

## Build it
    let pick = Math.floor(Math.random() * 3);
    if (pick === 0) {
      console.log("Yes, hero.");
    } else if (pick === 1) {
      console.log("The stars say no.");
    } else {
      console.log("Ask again later.");
    }

## How the random number works
Math.random() gives a decimal from 0 up to (but not including) 1. Times 3, that is anywhere from 0 to just under 3. Math.floor drops the decimals, leaving 0, 1 or 2: three answers, all equally likely.

## Why end with else?
By the time JavaScript reaches the last branch, pick can only be 2. A plain else catches it, and it guarantees the orb always answers, even if you change the numbers later.

## Check
Run it several times: do you see all three answers?"##,
            &["Math.floor(Math.random() * 3) gives 0, 1 or 2.", "if / else if / else runs exactly one branch.", "=== compares; = assigns.", "A final else is a safe default."],
            vec![
                quiz("If pick is 1, what does the orb say?", &["The stars say no.", "Yes, hero.", "Ask again later."], 0, "pick === 1 matches the else if branch."),
                fill("Check the second answer only when the first didn't match.", r##"} ___ (pick === 1) {"##, &["else if", "if", "elif"], 0, "else if is checked only when the branches above were false."),
                quiz("Why does the last branch use else with no condition?", &["pick can only be 2 by then, and the orb always answers", "else is faster", "else if is not allowed at the end"], 0, "else is the safe default."),
            ]),

        // ---------------- Loops
        d("while Loops", r##"## Repeating code
Loops repeat code so you do not have to copy it. A while loop repeats as long as its condition is true:

    let hp = 3;
    while (hp > 0) {
      console.log(`HP: ${hp}`);
      hp--;
    }
    console.log("Rest at the inn.");

## How it runs
1. Check the condition. If false, skip to after the loop.
2. Run the block.
3. Go back to step 1.

With hp = 3, the block runs for 3, 2 and 1, then hp is 0, the condition is false and the loop ends.

## The infinite loop
If nothing inside the loop ever makes the condition false, it runs forever and the page freezes. Always make sure something changes: here, hp-- moves hp towards 0. (Code Forge stops runaway loops for you, but real browsers may hang.)

## When to use while
Use while when you do not know in advance how many times to repeat: keep rolling until you get a 6, keep asking until the answer is valid, keep fighting until someone wins."##,
            &["while (condition) repeats a block as long as the condition is true.", "The condition is checked before every repetition.", "Something inside must eventually make it false, or it loops forever.", "Use while when the number of repeats is unknown."],
            vec![
                quiz("When is the while condition checked?", &["Once, at the start", "Before every repetition", "Only at the end"], 1, "It is re-checked each time round."),
                quiz("What causes an infinite loop?", &["The condition never becomes false", "Using console.log", "Using let"], 0, "Something must change the condition."),
                fill("Keep going while n is below 5.", "___ (n < 5) {\n  n++;\n}", &["while", "loop", "repeat"], 0, "while repeats while true."),
            ]),
        d("for Loops", r##"## Counting loops
When you know how many times to repeat, a for loop keeps the counter in one tidy line:

    for (let i = 1; i <= 3; i++) {
      console.log(`Round ${i}`);
    }

## The three parts
Inside the brackets, separated by semicolons:
1. Start: let i = 1 runs once, before the loop.
2. Condition: i <= 3 is checked before each round.
3. Step: i++ runs after each round.

This prints Round 1, Round 2, Round 3.

## Counting from zero
Programmers often count from 0 because list positions start at 0:

    for (let i = 0; i < 5; i++) { ... }   // 0,1,2,3,4: five rounds

## Other steps
    for (let i = 10; i > 0; i--) { ... }      // countdown
    for (let i = 0; i <= 100; i += 10) { ... } // 0,10,20...

## Off-by-one errors
The most common loop bug is running one time too many or too few. Check the start, the condition (< or <=?) and the last value. Trace the first and last rounds by hand to be sure."##,
            &["for (start; condition; step) { ... } is a counting loop.", "The start runs once; the condition is checked each round; the step runs after each round.", "i < 5 from 0 gives five rounds: 0 to 4.", "Check the first and last rounds to avoid off-by-one errors."],
            vec![
                quiz("for (let i = 0; i < 3; i++) runs how many times?", &["2", "3", "4"], 1, "0, 1, 2."),
                quiz("When does the step (i++) run?", &["Before the first round", "After each round", "Only at the end"], 1, "After each round."),
                fill("Count down from 5.", "for (let i = 5; i > 0; ___) { }", &["i--", "i++", "i - 1"], 0, "i-- counts down."),
            ]),
        d("Even and Odd", r##"## Loops plus decisions
Loops become powerful when combined with if. A classic example: go through numbers and treat even and odd ones differently.

    for (let i = 1; i <= 6; i++) {
      if (i % 2 === 0) {
        console.log(`${i} is even`);
      } else {
        console.log(`${i} is odd`);
      }
    }

## Why % 2?
Even numbers divide by 2 with nothing left over, so i % 2 is 0. Odd numbers leave 1.

## Other patterns with %
- i % 3 === 0 is true for every third number (3, 6, 9...).
- i % 10 === 0 picks round tens.

## Counting and totals
You can also collect results while looping:

    let evens = 0;
    for (let i = 1; i <= 10; i++) {
      if (i % 2 === 0) {
        evens++;
      }
    }
    console.log(evens);   // 5

A variable created BEFORE the loop keeps its value across rounds. A variable created INSIDE the loop starts fresh each round, so totals must live outside."##,
            &["Combine loops with if to treat items differently.", "i % 2 === 0 means i is even.", "i % 3 === 0 is true for every third number.", "Keep totals in a variable created before the loop."],
            vec![
                quiz("Which numbers make i % 3 === 0 true?", &["1, 2, 3", "3, 6, 9", "2, 4, 6"], 1, "Multiples of 3 leave no remainder."),
                quiz("Where should a running total be declared?", &["Inside the loop", "Before the loop", "After the loop"], 1, "Inside, it would reset every round."),
                fill("Test for odd numbers.", "if (i % 2 ___ 1) { }", &["===", "=", "!"], 0, "Odd numbers leave 1."),
            ]),
        d("break and continue", r##"## Controlling a loop from inside
Sometimes you want to stop a loop early or skip one round.

## break: stop now
break exits the loop immediately. Code after the loop continues.

    for (let i = 1; i <= 100; i++) {
      if (i * i > 50) {
        break;
      }
      console.log(i);
    }

This prints 1 to 7 and stops at 8, because 64 > 50. Use break when you have found what you were looking for.

## continue: skip to the next round
continue skips the rest of the current round and jumps to the next one.

    for (let i = 1; i <= 5; i++) {
      if (i === 3) {
        continue;
      }
      console.log(i);
    }

This prints 1, 2, 4, 5: round 3 was skipped.

## Use them carefully
A loop with many breaks and continues becomes hard to follow. Often a clearer condition does the same job. They shine when searching (break when found) and filtering (continue past items you do not want).

## They only affect the nearest loop
In loops inside loops, break and continue act on the innermost loop."##,
            &["break exits the loop immediately.", "continue skips to the next round.", "Use break when you found what you were looking for.", "They affect only the innermost loop."],
            vec![
                quiz("What does continue do?", &["Stops the loop", "Skips the rest of this round", "Restarts the program"], 1, "It jumps to the next round."),
                quiz("for i 1..5, continue when i === 2. What prints?", &["1 3 4 5", "1", "2"], 0, "Round 2 is skipped."),
                fill("Stop the loop when the key is found.", "if (item === \"key\") {\n  ___;\n}", &["break", "stop", "exit"], 0, "break leaves the loop."),
            ]),
        d("Project: Lucky Number", r##"## Roll for luck
A 10-sided die rolls until it lands on the lucky number 7. Nobody knows how many rolls that will take, so you need a loop that keeps going until it happens, and a counter for the tries.

## Plan
1. Store the lucky number, the current roll and a try counter.
2. While the roll is not the lucky number, roll again and count the try.
3. Print how many tries it took.

## Build it
    let lucky = 7;
    let roll = 0;
    let tries = 0;
    while (roll !== lucky) {
      roll = Math.floor(Math.random() * 10) + 1;
      tries++;
    }
    console.log(`Rolled a ${lucky}! Tries: ${tries}`);

## Why a while loop?
A for loop is great when you know how many times to repeat. Here you don't: it could take 1 roll or 30. while repeats as long as its condition is true, which is exactly what this needs.

## Small details that matter
- roll starts at 0, which is never lucky, so the loop always runs at least once.
- Math.floor(Math.random() * 10) gives 0 to 9; + 1 shifts it to 1 to 10, like a real die.
- tries is declared before the loop, so it keeps counting across every round.

## Check
Run it several times: the number of tries changes every time, but the loop always stops on a 7."##,
            &["while repeats as long as its condition is true.", "Use while when you don't know how many repeats you need.", "Declare counters before the loop.", "Math.floor(Math.random() * 10) + 1 rolls 1 to 10."],
            vec![
                quiz("Why does roll start at 0?", &["0 is never lucky, so the loop runs at least once", "Dice start at 0", "while needs a 0"], 0, "If roll started at 7, the loop would never run."),
                fill("Make the die roll 1 to 10 instead of 0 to 9.", r##"roll = Math.floor(Math.random() * 10) ___ 1;"##, &["+", "-", "*"], 0, "Adding 1 shifts 0-9 up to 1-10."),
                quiz("What is the smallest number of tries possible?", &["1", "0", "7"], 0, "The very first roll can be a 7."),
            ]),

        // ---------------- Arrays
        d("Making Arrays", r##"## Lists of values
An array stores many values in one variable, in order. Write the values between square brackets, separated by commas:

    let inventory = ["sword", "shield", "potion"];
    let scores = [90, 75, 100];

Arrays can hold any type, even a mix, but usually hold items of the same kind.

## Printing an array
    console.log(inventory);

shows all the items. Arrays are perfect for anything that comes as a list: party members, high scores, quest steps, shop items.

## Empty arrays
An empty list is just []:

    let loot = [];

You can add items later (lesson "push and pop").

## const arrays can still change
    const party = ["Pix"];

const means the variable always points to the SAME array, but the items inside can still be added or changed. You just cannot replace the whole array with a new one.

## Why not separate variables?
item1, item2, item3... does not scale. With an array you can loop over every item with one loop, count them with .length, and sort or search them with built-in tools."##,
            &["An array is an ordered list in square brackets.", "Items are separated by commas and can be any type.", "[] is an empty array.", "A const array can change its items but not be replaced."],
            vec![
                quiz("Which creates an array?", &["{\"a\", \"b\"}", "[\"a\", \"b\"]", "(\"a\", \"b\")"], 1, "Arrays use square brackets."),
                quiz("Can you add items to a const array?", &["Yes", "No"], 0, "const stops replacing, not changing items."),
                fill("Make an empty loot list.", "let loot = ___;", &["[]", "{}", "\"\""], 0, "[] is an empty array."),
            ]),
        d("Indexes", r##"## Positions start at 0
Each item in an array has a position number called its index. The first item is at index 0, not 1.

    let party = ["Pix", "Rook", "Ada"];
    console.log(party[0]);   // Pix
    console.log(party[2]);   // Ada

## Why zero?
The index means "how many steps from the start". The first item is 0 steps away. It feels odd at first, then becomes natural. Almost every programming language does this.

## The last item
The last index is always length - 1. For three items, the last index is 2.

    console.log(party[party.length - 1]);   // Ada

## Going too far
party[5] does not crash: it gives undefined, because there is nothing there. If you see undefined unexpectedly, check your index.

## Changing an item
Use the index on the left of =:

    party[1] = "Max";   // ["Pix", "Max", "Ada"]

## Strings have indexes too
"hero"[0] is "h". Text works like a list of characters."##,
            &["Indexes start at 0.", "array[i] reads the item at index i.", "The last item is at array.length - 1.", "A missing index gives undefined; array[i] = x changes an item."],
            vec![
                quiz("let a = [\"x\", \"y\", \"z\"]; What is a[1]?", &["x", "y", "z"], 1, "Index 1 is the second item."),
                quiz("What do you get for an index past the end?", &["An error", "undefined", "The last item"], 1, "Nothing there = undefined."),
                fill("Get the first item.", "console.log(party[___]);", &["0", "1", "first"], 0, "Counting starts at 0."),
            ]),
        d("length and Updating", r##"## How many items?
.length tells you how many items an array has:

    let bag = ["rope", "torch", "map"];
    console.log(bag.length);   // 3

It updates automatically as the array grows and shrinks.

## length versus last index
Three items have indexes 0, 1, 2. So length is always one more than the last index. bag[bag.length] is undefined; bag[bag.length - 1] is the last item.

## Changing items
    bag[0] = "lantern";    // replace the first item

## Checking for empty
    if (bag.length === 0) {
      console.log("Your bag is empty.");
    }

## length in loops
length makes loops work for any size of list:

    for (let i = 0; i < bag.length; i++) {
      console.log(bag[i]);
    }

Notice the < (not <=): i stops at length - 1, the last index. Using <= would read one item past the end and print undefined.

## Strings have length too
"Pixel".length is 5. It counts characters, including spaces."##,
            &[".length is the number of items.", "The last index is length - 1.", "array[i] = value replaces an item.", "Loop with i < array.length to visit every item."],
            vec![
                quiz("An array has 4 items. What is its last index?", &["4", "3", "5"], 1, "length - 1."),
                quiz("Why use i < bag.length instead of <=?", &["<= reads one past the end", "< is faster", "No reason"], 0, "The last index is length - 1."),
                fill("Check for an empty bag.", "if (bag.___ === 0) { }", &["length", "size", "count"], 0, ".length counts items."),
            ]),
        d("push and pop", r##"## Adding and removing at the end
Arrays have built-in methods (functions attached to them):

    let stack = ["sword"];
    stack.push("shield");   // ["sword", "shield"]
    stack.push("potion");   // ["sword", "shield", "potion"]
    let last = stack.pop(); // removes "potion" and gives it back

- push(item) adds to the END and returns the new length.
- pop() removes the LAST item and returns it.

## At the start
- unshift(item) adds to the START.
- shift() removes the FIRST item and returns it.

## A stack of plates
push and pop together work like a stack of plates: you add to the top and take from the top. The last thing in is the first thing out. Undo buttons work this way.

## A queue
push plus shift works like a queue at the tavern: join at the back, leave from the front.

## Methods change the array
These methods change the original array; they do not make a copy. After stack.pop(), the array is one item shorter.

## Popping an empty array
pop() on [] returns undefined. Check length first if that matters."##,
            &["push adds to the end; pop removes from the end.", "unshift adds to the start; shift removes from the start.", "pop and shift return the removed item.", "These methods change the original array."],
            vec![
                quiz("Which method removes the last item?", &["push", "pop", "shift"], 1, "pop takes from the end."),
                quiz("What does pop() return?", &["The removed item", "The new length", "Nothing"], 0, "It gives back what it removed."),
                fill("Add a gem to the start of the list.", "loot.___(\"gem\");", &["unshift", "push", "pop"], 0, "unshift adds to the start."),
            ]),
        d("Looping Arrays", r##"## Visiting every item
The point of a list is often to do something with every item. Three common ways:

## Classic for loop
    let party = ["Pix", "Rook", "Ada"];
    for (let i = 0; i < party.length; i++) {
      console.log(`${i + 1}. ${party[i]}`);
    }

Use this when you need the index (for numbering, or comparing neighbours).

## for...of
    for (const hero of party) {
      console.log(hero);
    }

Shorter and clearer when you only need each item, not its position. Read it as "for each hero of the party".

## forEach
    party.forEach((hero) => {
      console.log(hero);
    });

forEach calls a function for each item. You will understand it fully after the Functions chapter.

## Totals and maximums
    let scores = [70, 95, 80];
    let total = 0;
    for (const s of scores) {
      total += s;
    }
    console.log(total / scores.length);   // the average

The pattern "start with a value before the loop, update it inside" works for totals, counts, maximums and building new lists."##,
            &["A for loop with an index visits every item and knows its position.", "for (const item of list) is simpler when you only need the item.", "forEach runs a function for each item.", "Keep totals in a variable before the loop."],
            vec![
                quiz("Which loop is simplest when you only need each item, not its index?", &["for...of", "while", "for with i"], 0, "for...of gives each item directly."),
                quiz("How do you compute an average?", &["total / list.length", "total * length", "list[0]"], 0, "Sum divided by count."),
                fill("Loop over every hero.", "for (const hero ___ party) { }", &["of", "in", "from"], 0, "for...of iterates items."),
            ]),
        d("Searching Arrays", r##"## Finding things in lists
Arrays have methods to answer common questions:

- includes(x): is x in the list? Gives true or false.
- indexOf(x): where is x? Gives its index, or -1 if it is not there.

    let bag = ["rope", "torch", "map"];
    console.log(bag.includes("map"));    // true
    console.log(bag.indexOf("torch"));   // 1
    console.log(bag.indexOf("sword"));   // -1

## Why -1?
-1 can never be a real index, so it clearly means "not found". Always check for it:

    if (bag.indexOf("key") === -1) {
      console.log("You need a key.");
    }

## Joining and slicing
- join(", ") turns the array into one string: "rope, torch, map".
- slice(0, 2) copies part of the array (from index 0 up to, not including, 2).

## Sorting and reversing
- sort() puts items in order (alphabetical for strings).
- reverse() flips the order.
Both change the original array.

## Searching by hand
You can always write the search yourself with a loop and break. Built-in methods are shorter and less error-prone, so prefer them."##,
            &["includes(x) answers yes/no: is x in the list?", "indexOf(x) gives the position, or -1 if not found.", "join(sep) makes a string; slice copies part of the list.", "sort() and reverse() change the original array."],
            vec![
                quiz("indexOf returns what when the item is missing?", &["0", "-1", "undefined"], 1, "-1 means not found."),
                quiz("Which method answers true or false?", &["includes", "indexOf", "join"], 0, "includes gives a boolean."),
                fill("Turn the list into one string separated by commas.", "let text = bag.___(\", \");", &["join", "concat", "string"], 0, "join glues items into text."),
            ]),
        d("Project: Bard's Playlist", r##"## Manage a playlist
The tavern bard keeps a playlist. Your program adds a new song to the end, prints a numbered list, and says how many songs there are.

## Plan
1. Start with two songs in an array.
2. Add "Slime Shuffle" to the end.
3. Loop over every index and print a numbered line.
4. Print the total number of songs.

## Build it
    const playlist = ["Dragon Waltz", "Goblin Groove"];
    playlist.push("Slime Shuffle");
    for (let i = 0; i < playlist.length; i++) {
      console.log(`${i + 1}. ${playlist[i]}`);
    }
    console.log(`${playlist.length} songs`);

It prints:

    1. Dragon Waltz
    2. Goblin Groove
    3. Slime Shuffle
    3 songs

## Indexes and people
Arrays count from 0, people count from 1. playlist[0] is the first song, so the loop prints i + 1 as the number and playlist[i] as the song.

## const and push
const stops the name playlist from being pointed at a different array, but the array itself can still change. That is why push works on a const array.

## Check
Does the numbering start at 1? Does the loop stop after the last song (i < playlist.length, not <=)?"##,
            &["push adds an item to the end of an array.", "Indexes start at 0; number items for people with i + 1.", "Loop with i < array.length to visit every item.", "A const array can still change its contents."],
            vec![
                quiz("How many songs does playlist.length give after the push?", &["3", "2", "4"], 0, "Two songs plus the pushed one."),
                fill("Stop the loop after the last song.", r##"for (let i = 0; i ___ playlist.length; i++) {"##, &["<", "<=", ">"], 0, "With <=, i would reach 3, and there is no playlist[3]."),
                quiz("Why can a const array still get a new song?", &["const stops reassigning the name, not changing the contents", "push ignores const", "It can't: this is an error"], 0, "The array changes; the name still points at the same array."),
            ]),

        // ---------------- Functions
        d("Defining and Calling", r##"## Reusable spells
A function is a named block of code you can run whenever you want. Write it once, use it many times.

    function greet() {
      console.log("Welcome, traveller!");
    }

    greet();
    greet();

## Two steps
1. Defining: function name() { ... } creates the function. Nothing runs yet; it is like writing a spell in your book.
2. Calling: name() runs it. Each call runs the whole block. The brackets are what make it run.

## Why functions?
- No repetition: the same code lives in one place.
- Easier fixes: change the function once and every call is updated.
- Readable programs: names like openShop() explain what happens.

## Naming
Function names are usually verbs in camelCase: showMenu, rollDice, healHero.

## A common mistake
Writing greet without brackets does NOT run the function; it just refers to it. Always add () to call it.

## Built-in functions
You have already used functions others wrote: console.log() and Math.random() are functions too."##,
            &["A function is a named, reusable block of code.", "function name() { ... } defines it; name() calls it.", "Defining does not run the code; calling does.", "Use verb names like rollDice; console.log is a function too."],
            vec![
                quiz("What runs the code inside a function?", &["Defining it", "Calling it with ()", "Saving the file"], 1, "A call runs it."),
                quiz("What does greet (without brackets) do?", &["Runs greet", "Refers to the function without running it", "Deletes it"], 1, "Brackets are needed to call."),
                fill("Define a function named heal.", "___ heal() {\n  console.log(\"+10 HP\");\n}", &["function", "def", "func"], 0, "JavaScript uses the function keyword."),
            ]),
        d("Parameters", r##"## Giving a function information
A function becomes much more useful when you can give it values to work with. Those values are called parameters (in the definition) and arguments (in the call).

    function greet(name) {
      console.log(`Welcome, ${name}!`);
    }

    greet("Pix");    // Welcome, Pix!
    greet("Rook");   // Welcome, Rook!

name is a parameter: a variable that only exists inside the function and holds whatever was passed in.

## Several parameters
Separate them with commas. They are matched by position:

    function attack(attacker, target, damage) {
      console.log(`${attacker} hits ${target} for ${damage}`);
    }
    attack("Pix", "Slime", 5);

## Missing arguments
If you pass fewer arguments than parameters, the missing ones are undefined. You can give a default:

    function heal(amount = 10) { ... }

heal() uses 10; heal(25) uses 25.

## Parameters are local
A parameter can share a name with a variable outside the function without affecting it. Each call gets its own fresh copy."##,
            &["Parameters are the names in the definition; arguments are the values passed in.", "Arguments are matched to parameters by position.", "Missing arguments are undefined unless you give a default.", "Parameters exist only inside the function."],
            vec![
                quiz("In greet(\"Pix\"), what is \"Pix\"?", &["A parameter", "An argument", "A function"], 1, "Values passed in are arguments."),
                quiz("function f(a, b) is called f(1). What is b?", &["0", "undefined", "1"], 1, "Missing arguments are undefined."),
                fill("Give amount a default of 10.", "function heal(amount ___ 10) { }", &["=", ":", "=="], 0, "Defaults use =."),
            ]),
        d("Return Values", r##"## Getting an answer back
console.log only SHOWS a value. return SENDS a value back to the code that called the function, so it can be stored or used.

    function double(n) {
      return n * 2;
    }

    let x = double(5);         // x is 10
    console.log(double(x));    // 20

## return ends the function
As soon as return runs, the function stops. Code after it in the same block never runs.

    function check(hp) {
      if (hp <= 0) {
        return "Defeated";
      }
      return "Still standing";
    }

## No return means undefined
A function without return gives back undefined. That is why let r = console.log("hi") stores undefined.

## Printing versus returning
- Printing is for people to read.
- Returning is for the program to use.

A function that returns is more flexible: the caller decides whether to print the result, add it up, or compare it.

## Functions calling functions
Return values let you chain functions: total(price(item), tax()). Each function does one small job and hands its result to the next."##,
            &["return sends a value back to the caller.", "return immediately ends the function.", "Without return, a function gives undefined.", "Print for people; return for the program."],
            vec![
                quiz("What does a function without return give back?", &["0", "undefined", "An error"], 1, "No return = undefined."),
                quiz("What happens to code after return in the same block?", &["It runs later", "It never runs", "It runs first"], 1, "return ends the function."),
                fill("Send the result back.", "function triple(n) {\n  ___ n * 3;\n}", &["return", "console.log", "give"], 0, "return sends the value back."),
            ]),
        d("Arrow Functions", r##"## A shorter way to write functions
Arrow functions are a compact syntax that is everywhere in modern JavaScript:

    const double = (n) => {
      return n * 2;
    };

## Even shorter
If the body is a single expression, you can drop the braces and the word return. The value is returned automatically:

    const double = (n) => n * 2;
    const greet = (name) => `Hi, ${name}!`;

With exactly one parameter, the brackets around it are optional: n => n * 2. With zero or several parameters, keep them: () => ..., (a, b) => ...

## Stored in variables
An arrow function is a value stored in a const. You call it the same way: double(4).

## Where they shine
Arrow functions are perfect for small helper functions passed to other functions:

    [1, 2, 3].forEach((n) => console.log(n * 10));

## Careful with braces
With braces, you MUST write return, or the function gives undefined:

    const bad = (n) => { n * 2 };   // returns undefined
    const good = (n) => n * 2;      // returns n * 2"##,
            &["const f = (a, b) => { ... } is an arrow function.", "A single-expression body returns automatically: (n) => n * 2.", "One parameter may skip the brackets; zero or many need them.", "With braces, you must write return."],
            vec![
                quiz("What does (n) => { n * 2 } return?", &["n * 2", "undefined", "An error"], 1, "With braces you need return."),
                quiz("Which is a valid arrow function with no parameters?", &["=> 5", "() => 5", "[] => 5"], 1, "Empty brackets for no parameters."),
                fill("Write a one-line arrow that adds 1.", "const inc = (n) ___ n + 1;", &["=>", "->", "=="], 0, "The arrow is =>."),
            ]),
        d("Scope", r##"## Where can a variable be used?
Scope is the region of code where a variable exists.

## Block scope
Variables made with let and const live inside the nearest { } block. Outside it, they do not exist:

    if (true) {
      let secret = "hidden";
    }
    console.log(secret);   // error: secret is not defined

## Function scope
Variables created inside a function, including its parameters, exist only during that call:

    function roll() {
      let result = 4;
      return result;
    }
    // result does not exist out here

## Global scope
Variables created outside every function and block are global: any code can see them. That is convenient but risky in big programs, because anything can change them. Keep globals few.

## Inner sees outer
Code inside a block CAN see variables from outside it:

    let gold = 10;
    function show() {
      console.log(gold);   // 10
    }

## Shadowing
If an inner variable has the same name as an outer one, the inner one hides ("shadows") it inside its block. This is legal but confusing, so choose different names."##,
            &["Scope is where a variable exists.", "let and const live inside their { } block.", "Function variables and parameters exist only during the call.", "Inner code can see outer variables, not the other way round."],
            vec![
                quiz("A let variable is created inside an if block. Can code after the block use it?", &["Yes", "No"], 1, "It only lives inside the block."),
                quiz("Can a function read a variable created before it, outside?", &["Yes", "No"], 0, "Inner code sees outer variables."),
                quiz("Why keep global variables few?", &["Any code can change them, causing bugs", "They are slower to type", "They cannot hold numbers"], 0, "Shared state is risky."),
            ]),
        d("Project: Palindrome Checker", r##"## Words that read the same backwards
A palindrome reads the same forwards and backwards: "level", "racecar", "noon". Your function builds the word backwards, letter by letter, and returns whether it matches.

## Plan
1. Write a function isPalindrome(word).
2. Start with an empty string, reversed.
3. Loop from the last letter down to the first, adding each one to reversed.
4. Return whether reversed equals the original word.

## Build it
    function isPalindrome(word) {
      let reversed = "";
      for (let i = word.length - 1; i >= 0; i--) {
        reversed += word[i];
      }
      return reversed === word;
    }
    console.log(isPalindrome("level"));
    console.log(isPalindrome("hero"));

It prints true, then false.

## Walking backwards
The last letter of a word is at index word.length - 1 (for "hero", that is 3). The loop starts there, keeps going while i >= 0, and counts down with i--. For "hero", reversed grows: "o", "or", "ore", "oreh".

## Why return a boolean?
Returning true or false (instead of printing) lets other code use the answer, for example in an if statement.

## Level up
Capital letters count as different characters, so "Level" gives false. Try making the word lowercase first with word.toLowerCase()."##,
            &["Break a problem into small steps before coding.", "The last index is word.length - 1.", "Count down with i-- to walk a string backwards.", "Return booleans so other code can use the answer."],
            vec![
                quiz("For \"hero\", what is reversed at the end?", &["\"oreh\"", "\"hero\"", "\"ore\""], 0, "The letters are added from the last to the first."),
                fill("Add the current letter to the end of reversed.", r##"reversed ___ word[i];"##, &["+=", "=", "-="], 0, "+= joins the letter onto the text built so far."),
                quiz("Why does the function return instead of using console.log?", &["So other code can use the answer", "return prints faster", "Functions can't use console.log"], 0, "A returned value can be stored, printed or tested."),
            ]),

        // ---------------- Objects
        d("Object Literals", r##"## Grouping related data
An array is a list of things. An object describes ONE thing with named properties:

    const hero = {
      name: "Pix",
      level: 3,
      isWizard: true,
    };

- Curly braces hold the object.
- Each property is key: value.
- Properties are separated by commas.

## Why objects?
Separate variables like heroName, heroLevel, heroHp drift apart. An object keeps everything about the hero together, so you can pass the whole hero to a function in one go.

## Keys and values
Keys are names (usually camelCase, no quotes needed). Values can be anything: strings, numbers, booleans, arrays, even other objects or functions.

    const quest = {
      title: "Slime Cave",
      reward: 50,
      steps: ["enter", "fight", "loot"],
    };

## Arrays of objects
Real data is often a list of objects, like a party of heroes or a shop full of items:

    const shop = [
      { item: "Potion", price: 5 },
      { item: "Sword", price: 20 },
    ];"##,
            &["An object groups related data as key: value properties.", "Objects use curly braces; properties are separated by commas.", "Values can be any type, including arrays and objects.", "Lists of objects model real data like shops and parties."],
            vec![
                quiz("Which creates an object?", &["[name: \"Pix\"]", "{ name: \"Pix\" }", "(name = \"Pix\")"], 1, "Objects use braces with key: value."),
                quiz("What separates properties?", &["Semicolons", "Commas", "Spaces"], 1, "Commas between properties."),
                fill("Separate the key from its value.", "const potion = { heal___ 20 };", &[":", "=", "=>"], 0, "key: value."),
            ]),
        d("Dot and Bracket Access", r##"## Reading properties
Use a dot and the property name:

    const hero = { name: "Pix", level: 3 };
    console.log(hero.name);    // Pix
    console.log(hero.level);   // 3

## Bracket notation
You can also use square brackets with the key as a string:

    console.log(hero["name"]);   // Pix

## When brackets are needed
- The key is stored in a variable:

    let stat = "level";
    console.log(hero[stat]);   // 3

  hero.stat would look for a property literally called "stat".
- The key has spaces or special characters: item["max hp"].

## Missing properties
Reading a property that does not exist gives undefined (not an error). But reading a property OF undefined is an error: hero.pet.name crashes if hero has no pet.

## Which to use?
Use dots by default because they are shorter and easier to read. Use brackets when the key comes from a variable or is not a simple name."##,
            &["object.key reads a property with dot notation.", "object[\"key\"] reads it with bracket notation.", "Use brackets when the key is in a variable.", "A missing property is undefined."],
            vec![
                quiz("let k = \"hp\"; Which reads the hp property?", &["hero.k", "hero[k]", "hero.\"k\""], 1, "Brackets use the variable's value."),
                quiz("What do you get for a property that does not exist?", &["An error", "undefined", "null"], 1, "Missing = undefined."),
                fill("Read the name with dot notation.", "console.log(hero___name);", &[".", ":", "->"], 0, "object.key."),
            ]),
        d("Changing Objects", r##"## Updating properties
Assign with = using dot or bracket notation:

    const hero = { name: "Pix", hp: 30 };
    hero.hp = 25;          // change
    hero.hp -= 5;          // update using the old value
    hero.weapon = "staff"; // add a new property

If the property exists, it is changed. If not, it is created.

## Deleting
    delete hero.weapon;

## const objects
Just like arrays, a const object can have its properties changed. const only stops you from replacing the whole object with hero = {...}.

## Objects are shared
When you assign an object to another variable, both names point to the SAME object:

    const a = { hp: 10 };
    const b = a;
    b.hp = 0;
    console.log(a.hp);   // 0

This also happens when you pass an object to a function: the function can change it. That is powerful (a heal(hero) function can update the hero), but surprising if you expected a copy.

## Counting with objects
Objects make great tallies: votes[choice] = (votes[choice] || 0) + 1 counts how often each choice appears."##,
            &["object.key = value changes or adds a property.", "delete object.key removes a property.", "A const object's properties can still change.", "Assigning an object shares it; it does not copy it."],
            vec![
                quiz("hero has no gold property. What does hero.gold = 5 do?", &["Error", "Adds the property", "Nothing"], 1, "Assignment creates missing properties."),
                quiz("const b = a; b.hp = 0. What is a.hp?", &["Unchanged", "0", "undefined"], 1, "Both names point to the same object."),
                fill("Remove the weapon property.", "___ hero.weapon;", &["delete", "remove", "drop"], 0, "delete removes a property."),
            ]),
        d("Nested Data", r##"## Objects inside objects
Real data has layers. A hero has stats; stats have numbers:

    const hero = {
      name: "Pix",
      stats: { hp: 30, mp: 12 },
      bag: ["rope", "torch"],
    };

## Reading deeper
Chain the accessors from left to right:

    console.log(hero.stats.hp);   // 30
    console.log(hero.bag[1]);     // torch

Read hero.stats.hp as "the hero's stats' hp".

## Arrays of objects
    const party = [
      { name: "Pix", hp: 30 },
      { name: "Rook", hp: 45 },
    ];
    console.log(party[1].name);   // Rook

First pick the item by index, then the property by name.

## Looping nested data
    for (const member of party) {
      console.log(`${member.name}: ${member.hp} HP`);
    }

## Avoid crashes
If a middle part is missing, the chain breaks: hero.pet.name fails when there is no pet. The optional chaining operator ?. stops safely and gives undefined instead: hero.pet?.name.

## Data from the internet
Data from web services (weather, games, maps) arrives as nested objects and arrays called JSON. Everything in this lesson is how you read it."##,
            &["Objects and arrays can be nested inside each other.", "Chain accessors left to right: hero.stats.hp, party[1].name.", "Loop over arrays of objects and read each property.", "?. avoids crashes when a middle part is missing."],
            vec![
                quiz("How do you read Rook's hp from party (Rook is index 1)?", &["party.hp[1]", "party[1].hp", "party.Rook.hp"], 1, "Index first, then property."),
                quiz("What does hero.pet?.name give when there is no pet?", &["An error", "undefined", "\"\""], 1, "Optional chaining stops safely."),
                fill("Read the mp inside stats.", "console.log(hero.stats___mp);", &[".", "[", ":"], 0, "Chain with another dot."),
            ]),
        d("Looping Objects", r##"## Going through every property
Sometimes you do not know the keys in advance, or you want to show them all, such as every stat of a monster.

## for...in
for...in loops over an object's keys:

    const stats = { hp: 30, mp: 12, speed: 7 };
    for (const key in stats) {
      console.log(`${key}: ${stats[key]}`);
    }

Note the brackets: stats[key], because the key is in a variable.

## Object.keys and Object.values
These turn an object into arrays you can loop over with for...of or use with array methods:

    console.log(Object.keys(stats));     // ["hp", "mp", "speed"]
    console.log(Object.values(stats));   // [30, 12, 7]

Object.keys(stats).length tells you how many properties there are.

## of versus in
- for...of goes through the VALUES of an array (or other lists).
- for...in goes through the KEYS of an object.
Mixing them up is a common mistake; for arrays, prefer for...of.

## Totals over objects
    let total = 0;
    for (const v of Object.values(stats)) {
      total += v;
    }"##,
            &["for (const key in obj) loops over an object's keys.", "Use obj[key] inside the loop to read each value.", "Object.keys() and Object.values() give arrays.", "for...of is for array values; for...in is for object keys."],
            vec![
                quiz("Inside for (const key in stats), how do you read the value?", &["stats.key", "stats[key]", "key.value"], 1, "The key is in a variable, so use brackets."),
                quiz("What does Object.values(stats) return?", &["An array of the values", "The keys", "A number"], 0, "An array of values."),
                fill("Loop over the object's keys.", "for (const key ___ stats) { }", &["in", "of", "from"], 0, "for...in loops over keys."),
            ]),
        d("Project: Monster Card", r##"## A bestiary entry
Describe a monster as an object, power it up, then print its card with template literals and a loop over its moves.

## Plan
1. Create an object with a name, a type, hp and an array of moves.
2. Add 10 to its hp.
3. Print the name with the type, then the hp.
4. Loop over the moves and print each one.

## Build it
    const monster = {
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
    }

It prints:

    Sparkmouse (Electric)
    HP: 45
    - Zap
    - Dash

## What you practised
- An object groups facts about one thing as name: value pairs.
- Dot notation reads and changes a property: monster.hp += 10.
- An array can live inside an object (moves), and for...of visits each item.

## Level up
Put several monsters in an array and loop over it to print every card."##,
            &["An object groups facts about one thing as name: value pairs.", "Dot notation reads and changes properties.", "Objects can hold arrays, like a list of moves.", "for...of visits every item in an array."],
            vec![
                quiz("After monster.hp += 10, what is monster.hp?", &["45", "35", "10"], 0, "35 + 10 = 45."),
                fill("Print the monster's type in brackets.", r##"console.log(`${monster.name} (${monster.___})`);"##, &["type", "kind", "class"], 0, "The property is called type."),
                quiz("What does the loop print first?", &["- Zap", "- Dash", "Sparkmouse"], 0, "for...of starts at the first move."),
            ]),

        // ---------------- The Page (DOM)
        d("Finding Elements", r##"## The DOM
When a browser loads HTML, it builds a live model of the page called the DOM (Document Object Model). Every element becomes an object JavaScript can read and change. The whole page is available as document.

## getElementById
The most direct way to grab one element is by its id:

    <h1 id="title">Guild Hall</h1>

    const title = document.getElementById("title");

Now title is the heading element, and you can change it (next lessons).

## Note the details
- getElementById is case sensitive: capital E, B and I.
- You pass the id WITHOUT a # (unlike CSS).
- If no element has that id, you get null, and using it causes an error. When something "does nothing", check the id spelling in both files.

## Store it once
Finding an element takes work, so store it in a const and reuse the variable instead of searching again and again.

## Where the script goes
A script that runs before the HTML has loaded cannot find elements yet. That is why scripts are usually placed at the END of the body, or use the defer attribute: <script src="app.js" defer></script>."##,
            &["The DOM is the browser's live model of the page.", "document.getElementById(\"id\") finds one element (no #).", "A missing id gives null; check spellings in both files.", "Run scripts after the HTML loads (end of body or defer)."],
            vec![
                quiz("What does getElementById return when nothing matches?", &["undefined", "null", "An empty element"], 1, "No match = null."),
                quiz("Do you write a # in getElementById(\"...\")?", &["Yes", "No"], 1, "Only the id name."),
                fill("Find the element with id score.", "const s = document.___(\"score\");", &["getElementById", "getById", "find"], 0, "document.getElementById."),
            ]),
        d("Changing Text", r##"## Rewriting the page
Once you have an element, change its text with textContent:

    const title = document.getElementById("title");
    title.textContent = "Welcome back, Pix!";

The heading on the page updates instantly, without reloading.

## Reading text
textContent also reads the current text:

    console.log(title.textContent);

## Numbers from the page
Text from the page is always a string. To do maths, convert it first:

    const scoreEl = document.getElementById("score");
    let score = Number(scoreEl.textContent);
    score += 10;
    scoreEl.textContent = score;

## textContent versus innerHTML
- textContent treats everything as plain text. "<b>hi</b>" appears literally, with the angle brackets.
- innerHTML reads the string as HTML and builds real elements.

Use textContent for normal text: it is faster and safe. innerHTML with text typed by users is dangerous, because someone could inject their own HTML or scripts.

## Inputs are different
For form fields, use .value instead: document.getElementById("name").value gives what the user typed."##,
            &["element.textContent = \"...\" changes an element's text.", "textContent also reads the current text, always as a string.", "Convert with Number() before doing maths.", "Use .value for form fields; prefer textContent over innerHTML."],
            vec![
                quiz("What type is textContent?", &["Always a string", "A number if it looks like one", "A boolean"], 0, "Page text is a string."),
                quiz("Which property gives what a user typed in an input?", &["textContent", "value", "innerText"], 1, "Inputs use .value."),
                fill("Change the heading's text.", "title.___ = \"Victory!\";", &["textContent", "text", "content"], 0, "textContent sets text."),
            ]),
        d("Changing Styles", r##"## CSS from JavaScript
Every element has a style property for inline CSS:

    const box = document.getElementById("box");
    box.style.backgroundColor = "gold";
    box.style.display = "none";

## camelCase names
CSS property names with hyphens become camelCase in JavaScript:
- background-color → backgroundColor
- font-size → fontSize
- border-radius → borderRadius

Values are strings, and sizes need units: box.style.width = "200px".

## Classes are usually better
Instead of setting many styles one by one, define a class in CSS and switch it on and off with classList:

    .hidden { display: none; }

    box.classList.add("hidden");
    box.classList.remove("hidden");
    box.classList.toggle("hidden");   // on if off, off if on

This keeps the look in CSS (where designers expect it) and the behaviour in JavaScript.

## Showing and hiding
Toggling a class like "hidden" or "open" is how menus, pop-ups and accordions work on almost every site."##,
            &["element.style.property = \"value\" sets inline CSS.", "Hyphenated CSS names become camelCase: backgroundColor.", "Sizes are strings with units, like \"200px\".", "Prefer classList.add/remove/toggle with CSS classes."],
            vec![
                quiz("How is font-size written in JavaScript?", &["font-size", "fontSize", "FontSize"], 1, "Hyphens become camelCase."),
                quiz("Which switches a class on if it is off and off if it is on?", &["classList.add", "classList.toggle", "classList.remove"], 1, "toggle flips it."),
                fill("Hide the box using a class.", "box.classList.___(\"hidden\");", &["add", "push", "set"], 0, "classList.add adds a class."),
            ]),
        d("Events", r##"## Reacting to the user
An event is something that happens on the page: a click, a key press, the mouse moving, a form being sent. JavaScript can listen for events and run a function when they happen.

    const btn = document.getElementById("attack");
    btn.addEventListener("click", function () {
      console.log("You swing your sword!");
    });

## The parts
- addEventListener attaches a listener to the element.
- "click" is the event type.
- The function is the handler: it runs every time the event happens, not right away.

## Common events
- click: the element is clicked or tapped.
- input: the text in a field changes.
- keydown: a key is pressed.
- submit: a form is sent.
- mouseover: the mouse moves onto the element.

## The event object
The handler receives information about the event:

    btn.addEventListener("click", (event) => {
      console.log(event.type);   // click
    });

## Do not call the handler yourself
addEventListener("click", attack()) runs attack immediately and passes its result. Pass the function itself: addEventListener("click", attack)."##,
            &["Events are things that happen: click, input, keydown, submit.", "element.addEventListener(\"click\", fn) runs fn on each click.", "The handler runs later, when the event happens.", "Pass the function itself, not fn()."],
            vec![
                quiz("When does the handler function run?", &["Immediately", "Every time the event happens", "Once, after loading"], 1, "It waits for the event."),
                quiz("Which event fires when text in a field changes?", &["click", "input", "submit"], 1, "input fires on each change."),
                fill("Listen for clicks.", "btn.addEventListener(\"___\", attack);", &["click", "onclick", "press"], 0, "The event type is \"click\"."),
            ]),
        d("querySelector and innerHTML", r##"## Finding with CSS selectors
document.querySelector finds the FIRST element that matches any CSS selector:

    const title = document.querySelector("#title");    // by id
    const firstCard = document.querySelector(".card"); // by class
    const navLink = document.querySelector("nav a");   // descendant

Here you DO use # and . because it is a CSS selector.

## querySelectorAll
querySelectorAll finds ALL matches and gives a list you can loop over:

    const cards = document.querySelectorAll(".card");
    for (const card of cards) {
      card.classList.add("shiny");
    }

## innerHTML: building HTML
innerHTML replaces an element's content with HTML built from a string:

    const list = document.querySelector("#loot");
    list.innerHTML = "<li>Gold</li><li>Gem</li>";

Combined with a loop, this can build a whole list from an array.

## Safety first
Never put text typed by users into innerHTML: they could inject their own HTML or scripts. For user text, use textContent, or create elements with document.createElement.

## Which finder?
getElementById is fine for single ids. querySelector is more flexible because it accepts any selector you already know from CSS."##,
            &["querySelector(selector) finds the first match using CSS selectors.", "querySelectorAll finds every match as a list.", "innerHTML replaces content with HTML from a string.", "Never put user input into innerHTML."],
            vec![
                quiz("Which finds every element with class card?", &["querySelector(\".card\")", "querySelectorAll(\".card\")", "getElementById(\"card\")"], 1, "All matches: querySelectorAll."),
                quiz("Why avoid putting user text into innerHTML?", &["It could inject HTML or scripts", "It is slower", "It removes CSS"], 0, "It is a security risk."),
                fill("Find the first link inside nav.", "document.querySelector(\"___\");", &["nav a", "#nav.a", "nav>>a"], 0, "A descendant selector, like CSS."),
            ]),
        d("Final Project: Mood Ring", r##"## An interactive mood ring
Your final JavaScript project brings everything together: a ring on the page that glows a random colour, with a matching mood, every time the player presses Spin.

## The HTML
    <div id="ring"></div>
    <button id="spin">Spin</button>

## The JavaScript
    const ring = document.getElementById("ring");
    const moods = ["calm", "happy", "fierce"];
    const colors = ["teal", "gold", "crimson"];
    function setMood(pick) {
      ring.style.backgroundColor = colors[pick];
      ring.textContent = `You feel ${moods[pick]}`;
    }
    const btn = document.getElementById("spin");
    btn.addEventListener("click", () => {
      setMood(Math.floor(Math.random() * moods.length));
    });

## How it fits together
- The two arrays line up: moods[1] is "happy" and colors[1] is "gold", so one index picks a matching pair.
- setMood(pick) does the changing in one place: the ring's background colour and its text.
- addEventListener waits for a click and then runs the arrow function, which picks a random index from 0 to moods.length - 1.

## What you combined
Finding elements, arrays, a function with a parameter, changing styles and text, random numbers and an event listener: the whole course in eleven lines.

## Level up
Add a second button that always picks "calm" by calling setMood(0). You have finished the JavaScript course!"##,
            &["Store page elements in constants and reuse them.", "Matching arrays let one index pick a pair of values.", "Put shared behaviour in a function with parameters.", "addEventListener runs code when the player clicks."],
            vec![
                quiz("Why are the changes inside a setMood function?", &["The changing happens in one place, and any button can reuse it", "Functions are required for events", "It makes the ring round"], 0, "Call setMood from anywhere."),
                fill("Run the code when the button is clicked.", r##"btn.addEventListener("___", () => {"##, &["click", "press", "tap"], 0, "The event name is \"click\"."),
                quiz("If the random index is 2, what does the ring show?", &["You feel fierce, in crimson", "You feel happy, in gold", "You feel calm, in teal"], 0, "moods[2] is fierce and colors[2] is crimson."),
            ]),
    ]
}
