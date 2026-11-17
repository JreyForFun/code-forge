//! Python: nine chapters from Hello World to modules. Topic order follows the common beginner path
//! (as in Codédex's Python course); all wording, examples and projects are original.
use super::*;
use eframe::egui::Color32;

pub fn course() -> Course {
    Course {
        name: "Python", tag: "PY", slug: "python", color: Color32::from_rgb(0x2f, 0x6f, 0xd6), lang: "python", cat: "Code", level: "Beginner", hours: 24,
        desc: "Learn to program with Python: variables, decisions, loops, lists, functions, dictionaries, classes and modules.",
        teaches: vec!["Printing, variables and maths", "Decisions with if / elif / else", "while and for loops", "Lists and dictionaries", "Functions, classes and objects", "Using modules like math and random"],
        snippet: r#"print("Hello, hero!")"#,
        badge: Badge { name: "Serpent Sage", art: "trophy", desc: "Finished every Python quest" },
        modules: vec![hello(), variables(), control(), loops(), lists(), functions(), dicts(), classes(), modules()],
    }
}

fn hello() -> Module {
    ch("Hello World", "Your first lines of Python: printing text, comments and special characters.", vec![
        l("Printing", &[
            (r#"print() shows text on the screen.

print("Hello, World!")

The text goes inside the brackets, between quotes."#, "scroll_fire"),
            (r#"Each print() starts a new line:

print("Ready")
print("Set")
print("Go!")"#, "torch"),
        ], vec![
            quiz("What does print() do?", &["Shows something on the screen", "Saves a file", "Draws a picture"], 0, "print() writes output."),
            quiz("Which line is correct Python?", &["print(\"Hi\")", "Print(\"Hi\")", "print \"Hi\""], 0, "Lowercase print with brackets."),
            fill("Show a message.", r#"___("Hello, World!")"#, &["print", "echo", "say"], 0, "print() writes to the screen."),
            fill("Close the text.", r#"print("Hello___"#, &["\")", ")", "'"], 0, "Close the quote, then the bracket."),
            copy("Type this line.", r#"print("Hello, World!")"#, "print(\"text\")"),
            recall("From memory: print Hi.", r#"print("Hi")"#, r#"print("Hi")"#),
        ]).with(r##"## Your first program
A program is a list of instructions for the computer. Python reads them from top to bottom, one line at a time. The most useful first instruction is print(), which shows something on the screen.

    print("Hello, World!")

- print is the name of a built-in function: a ready-made command.
- The round brackets hold what you want to show.
- The quotes mark the start and end of the text. Text in quotes is called a string.

## Order matters
    print("Ready")
    print("Set")
    print("Go!")

prints three lines in exactly that order. Each print() starts on a new line.

## Common mistakes
- Python is case sensitive: Print() and PRINT() do not exist.
- Every opening bracket needs a closing one, and every quote needs a partner.
- Forgetting the quotes, print(Hello), makes Python look for a variable called Hello and fail.

When something goes wrong, Python shows an error message. Read it calmly: it usually names the line and the problem."##,
        &["print() shows output on the screen.", "Text in quotes is a string.", "Python runs lines top to bottom.", "Names are case sensitive: print, not Print."]),
        l("Strings and Comments", &[
            (r#"A string is text in quotes. Double "..." or single '...' both work, as long as both ends match.

A line starting with # is a comment: Python ignores it."#, "book"),
            (r#"# draw a tower
print("*")
print("**")
print("***")"#, "chest"),
        ], vec![
            quiz("Which is a valid string?", &["'Hello'", "Hello", "(Hello)"], 0, "Strings need quotes."),
            quiz("What does Python do with a comment?", &["Runs it", "Ignores it", "Prints it"], 1, "Comments are notes for people."),
            fill("Write a comment.", "___ this is a note", &["#", "//", "--"], 0, "Python comments start with #."),
            fill("Print two stars.", r#"print("___")"#, &["**", "2", "*+*"], 0, "Whatever is in the quotes is printed."),
            copy("Type this pattern.", "print(\"*\")\nprint(\"**\")", "Two prints, two lines."),
            recall("From memory: print A then B on two lines.", "print(\"A\")\nprint(\"B\")", "Two prints."),
        ]).with(r##"## Strings
A string is any text wrapped in quotes. Python accepts double quotes or single quotes:

    print("Code Forge")
    print('Code Forge')

Both print the same thing. Pick one style and stay consistent, but use the other kind when your text contains a quote:

    print("It's dangerous to go alone!")

## Drawing with text
Because each print() is a new line, you can draw simple pictures with characters. This is called ASCII art:

    print("  /\\")
    print(" /  \\")
    print("/____\\")

## Comments
A comment starts with # and runs to the end of the line. Python skips it entirely.

    # Draw the castle tower
    print("[]")  # the top

Use comments to explain WHY you wrote something, to label sections, or to switch a line off while testing (put # in front of it).

## Joining strings
The + sign glues strings together: print("Dragon" + "fly") shows Dragonfly. Note that it adds no space; include one yourself if you need it."##,
        &["Strings use matching single or double quotes.", "Use the other quote type to put a quote inside text.", "# starts a comment that Python ignores.", "+ joins strings without adding spaces."]),
        l("Escape Characters", &[
            (r#"A backslash \ starts an escape character:

\n  new line
\t  tab
\"  a quote inside quotes"#, "scroll_lightning"),
            (r#"print("Line one\nLine two")

prints two lines from one print()."#, "letter"),
        ], vec![
            quiz("What does \\n do inside a string?", &["Starts a new line", "Prints the letter n twice", "Ends the program"], 0, "\\n is a newline."),
            quiz("Which escape inserts a tab?", &["\\t", "\\b", "\\tab"], 0, "\\t is a tab."),
            fill("Break the text into two lines.", r#"print("Hi___there")"#, &["\\n", "/n", "\\l"], 0, "\\n starts a new line."),
            fill("Put a quote inside the string.", r#"print("She said \"run\___")"#, &["\"", "'", "n"], 0, "\\\" is a quote character."),
            copy("Type this.", r#"print("Name:\tPix")"#, "\\t adds a tab."),
            recall("From memory: print Up and Down on two lines with one print().", r#"print("Up\nDown")"#, r#"print("Up\nDown")"#),
        ]).with(r##"## Special characters in strings
Some characters are hard to type inside a string: a new line, a tab, or the same quote that surrounds the text. Python solves this with escape characters: a backslash \ followed by a letter.

    \n   new line
    \t   tab (a wide space, good for lining up columns)
    \"   a double quote inside "..."
    \'   a single quote inside '...'
    \\   a real backslash

## Examples
    print("Line one\nLine two")
    print("Name:\tPix")
    print("He said \"hello\"")

The first prints two lines, the second lines up "Pix" after a tab, the third shows quotes around hello.

## Why the backslash?
The backslash tells Python: "the next character is special, do not treat it normally". That is why a real backslash needs two: \\.

## Common mistake
Using a forward slash /n instead of \n prints the characters /n literally. The escape character is always the backslash, the one that leans left at the top.

## Triple quotes
For long text, Python also allows triple quotes """...""", which can span several lines without any \n."##,
        &["A backslash starts an escape character.", "\\n is a new line and \\t is a tab.", "\\\" puts a quote inside a string; \\\\ is a real backslash.", "Use the backslash \\, not the slash /."]),
        p("Project: Snail Mail", &[
            (r#"Write a letter to a friend in the next village, using several prints, a blank line and escape characters."#, "letter"),
            (r#"print("Dear Rook,\n")
print("The slimes are back.")
print("\t- Pix")"#, "scroll_fire"),
        ], vec![
            quiz("How do you add an empty line?", &["print() or \\n", "#", "print(\"\\t\")"], 0, "An empty print() or \\n gives a blank line."),
            quiz("Which line indents the signature?", &["print(\"\\t- Pix\")", "print(\"- Pix\")", "# - Pix"], 0, "\\t adds a tab."),
            fill("Start the letter with a greeting and a blank line.", r#"print("Dear Rook,___")"#, &["\\n", "\\t", "/n"], 0, "\\n adds the line break."),
            fill("Add a note for yourself that Python ignores.", "___ signature goes last", &["#", "//", "\\n"], 0, "Comments start with #."),
            copy("Type the closing line.", r#"print("Your friend,\n\tPix")"#, "A newline, then a tab."),
            recall("From memory: print Hello Rook! followed by a new line and See you soon.", r#"print("Hello Rook!\nSee you soon.")"#, "One print with \\n."),
        ]).with(r##"## Your first real program
In this project you write a short letter that Python prints neatly. It uses everything from the Hello World chapter: print(), strings, comments and escape characters.

## Plan the letter
- A greeting: Dear Rook,
- A blank line.
- Two or three lines of news.
- A sign-off, indented with a tab.

    # Letter to Rook
    print("Dear Rook,\n")
    print("The slimes are back in the cellar.")
    print("Bring your biggest broom!")
    print()
    print("Your friend,\n\tPix")

## Things to notice
- print() with nothing inside prints an empty line.
- \n inside the text adds a line break at that spot.
- \t pushes the name to the right like a real signature.
- The comment at the top is a label for you; it is not printed.

## Check your work
Run it and compare the output with your plan. If quotes or brackets are missing, Python points to the line. Fix the first error first: later errors are often caused by it."##,
        &["Plan the output before writing code.", "print() alone prints a blank line.", "Combine \\n and \\t to lay out text.", "Fix the first error first."]),
    ])
}

fn variables() -> Module {
    ch("Variables", "Store values, do maths, convert between types and build text with f-strings.", vec![
        l("Variables", &[
            (r#"A variable stores a value under a name. No keyword needed.

lives = 3
name = "Pix""#, "potion_red"),
            (r#"Use the name to get the value back:

print(name)
print(lives)

= means "store", not "equals"."#, "crystal_blue"),
        ], vec![
            quiz("What does = do in Python?", &["Stores a value in a variable", "Checks if two things are equal", "Adds numbers"], 0, "= is assignment."),
            quiz("What does print(\"lives\") show?", &["The value of lives", "The word lives", "An error"], 1, "Quotes make it text."),
            fill("Store 3 in lives.", "lives ___ 3", &["=", "==", ":"], 0, "One = stores a value."),
            fill("Show the value of name.", "name = \"Pix\"\nprint(___)", &["name", "\"name\"", "Name"], 0, "No quotes: use the variable."),
            copy("Type this.", "gold = 10\nprint(gold)", "Store, then print."),
            recall("From memory: store \"Pix\" in hero, then print hero.", "hero = \"Pix\"\nprint(hero)", "hero = \"Pix\" then print(hero)."),
        ]).with(r##"## Boxes with labels
A variable is like a labelled box. You put a value in it and later use the label to get the value back.

    lives = 3
    name = "Pix"
    print(name)
    print(lives)

Read lives = 3 as "store 3 in lives". The = sign means store, not "equals" like in maths.

## Changing a variable
A variable can be given a new value at any time. The old value is replaced:

    gold = 10
    gold = 15
    print(gold)   # 15

You can also use the old value to compute the new one: gold = gold + 5.

## Naming rules
- Use letters, digits and underscores, but do not start with a digit.
- No spaces: write max_health (Python style uses underscores, called snake_case).
- Names are case sensitive: Gold and gold are different variables.
- Choose names that say what the value is: lives is better than x.

## Common mistakes
- Using a variable before giving it a value causes a NameError.
- print("gold") prints the word gold; print(gold) prints the value."##,
        &["A variable stores a value under a name.", "= means store, not equals.", "Variables can be given new values at any time.", "Use snake_case names that describe the value."]),
        l("Data Types", &[
            (r#"Values have types:

int     whole numbers: 7
float   decimals: 2.5
str     text: "Pix"
bool    True or False"#, "gem_green"),
            (r#"type() tells you a value's type:

print(type(7))      # int
print(type("7"))    # str"#, "cpu"),
        ], vec![
            quiz("What type is 2.5?", &["int", "float", "str"], 1, "Decimals are floats."),
            quiz("What type is \"7\"?", &["int", "str", "bool"], 1, "Anything in quotes is a string."),
            fill("Find the type of the value.", "print(___(True))", &["type", "kind", "class"], 0, "type() gives the type."),
            fill("A boolean value:", "is_open = ___", &["True", "\"True\"", "true"], 0, "Python booleans are True and False, capitalised."),
            copy("Type this.", "print(type(3.5))", "3.5 is a float."),
            recall("From memory: store False in has_key and print its type.", "has_key = False\nprint(type(has_key))", "bool."),
        ]).with(r##"## Every value has a type
Python sorts values into types. The type decides what you can do with the value.

    int     whole numbers         7, -3, 1000
    float   numbers with a point  2.5, 0.1, 3.0
    str     text in quotes        "Pix", '42'
    bool    yes or no             True, False

## Checking a type
    print(type(7))       # int
    print(type(7.0))     # float
    print(type("7"))     # str
    print(type(True))    # bool

## Why types matter
The same symbol can do different things for different types:

    print(7 + 3)        # 10
    print("7" + "3")    # 73

With numbers, + adds. With strings, + joins. And mixing them, "Level " + 3, causes an error, because Python will not guess whether you meant maths or text. The next lessons show how to convert between types.

## Booleans
True and False must start with a capital letter. They answer yes/no questions such as "is the door open?" and power the if statements in the Control Flow chapter."##,
        &["Basic types: int, float, str and bool.", "type(value) tells you the type.", "+ adds numbers but joins strings.", "Booleans are True and False with capitals."]),
        l("Math", &[
            (r#"Python is a calculator:

+  add      -  subtract
*  multiply /  divide
** power    %  remainder"#, "coins"),
            (r#"gold = 10
gold = gold + 5
print(gold)   # 15"#, "coin_gold"),
        ], vec![
            quiz("What is 7 % 3?", &["1", "2", "2.33"], 0, "% gives the remainder: 7 = 2*3 + 1."),
            quiz("What is 2 ** 3?", &["6", "8", "5"], 1, "** is power: 2*2*2."),
            fill("Multiply.", "print(6 ___ 7)", &["*", "x", "^"], 0, "* multiplies."),
            fill("Add 5 to gold.", "gold = 10\ngold = gold ___ 5\nprint(gold)", &["+", "-", "="], 0, "Take gold, add 5, store it."),
            copy("Type this.", "print(10 // 3)", "// divides and drops the decimals."),
            recall("From memory: store 7 in a, then print a times a.", "a = 7\nprint(a * a)", "a * a = 49."),
        ]).with(r##"## Python as a calculator
    +    add              5 + 2   is 7
    -    subtract         5 - 2   is 3
    *    multiply         5 * 2   is 10
    /    divide           5 / 2   is 2.5
    //   whole division   5 // 2  is 2
    %    remainder        5 % 2   is 1
    **   power            5 ** 2  is 25

## Division
/ always gives a float, even 4 / 2 gives 2.0. // throws away the decimal part, which is handy for counting whole groups: how many full teams of 4 can 10 players make? 10 // 4 is 2.

## The remainder %
% gives what is left over. It tells even from odd (n % 2 is 0 for even numbers) and helps with wrap-around, like turning 25 hours into 1 o'clock.

## Order of operations
** first, then * / // %, then + -. Brackets change the order: (2 + 3) * 4 is 20.

## Updating variables
    gold = 10
    gold = gold + 5   # 15
    gold += 5         # shortcut: 20

The shortcuts +=, -=, *= and /= update a variable using its current value."##,
        &["+ - * / do the usual maths; ** is power.", "/ gives a float; // drops the decimals.", "% gives the remainder (n % 2 tells even from odd).", "+= and -= update a variable using its old value."]),
        l("Type Conversion", &[
            (r#"Convert between types:

int("5")     -> 5
float("2.5") -> 2.5
str(10)      -> "10""#, "cauldron"),
            (r#"input() always gives text:

age = int(input("Age? "))

int() turns the text into a number."#, "potion_magic"),
        ], vec![
            quiz("What type does input() give?", &["int", "str", "float"], 1, "input() always returns text."),
            quiz("What does \"Level \" + 3 do?", &["Prints Level 3", "Causes an error", "Prints Level"], 1, "You can't join text and a number; use str(3)."),
            fill("Turn the text into a whole number.", "n = ___(\"42\")", &["int", "str", "num"], 0, "int() converts to a whole number."),
            fill("Join text and a number safely.", "print(\"Level \" + ___(3))", &["str", "int", "text"], 0, "str() converts to text."),
            copy("Type this.", "print(int(\"7\") + 3)", "7 + 3 = 10."),
            recall("From memory: print \"Gold: \" joined with str(50).", "print(\"Gold: \" + str(50))", "str() before joining."),
        ]).with(r##"## Changing a value's type
Sometimes you have the right value in the wrong type. Python has functions to convert:

    int("5")       # 5      text to whole number
    float("2.5")   # 2.5    text to decimal
    str(10)        # "10"   number to text
    int(3.9)       # 3      float to int (drops the decimals)

## Joining text and numbers
    gold = 50
    print("Gold: " + str(gold))

Without str(), Python raises a TypeError, because + cannot mix text and numbers.

## Reading input
input() asks the user to type something and returns it as text, ALWAYS, even if they type digits:

    age = input("How old are you? ")
    age = int(age)
    print(age + 1)

You can do both steps in one line: age = int(input("Age? ")). (In Code Forge, input() returns empty text, so the lessons give the values directly.)

## When conversion fails
int("hello") is an error: there is no number in that text. Programs that read input often check it first, for example with "42".isdigit(), which is True only when every character is a digit."##,
        &["int(), float() and str() convert between types.", "Use str() before joining a number to text.", "input() always returns text; convert it with int() for maths.", "int(\"hello\") fails; check with .isdigit() first."]),
        l("f-Strings", &[
            (r#"Put f before the quotes and {variables} inside:

name = "Pix"
print(f"Hello, {name}!")"#, "scroll_lightning"),
            (r#"Any expression works inside { }:

print(f"Next level in {100 - 40} XP")"#, "spellbook"),
        ], vec![
            quiz("Which string fills in the value of name?", &["f\"Hi {name}\"", "\"Hi {name}\"", "\"Hi \" name"], 0, "Only f-strings fill in { }."),
            quiz("Do you need str() inside an f-string?", &["Yes", "No"], 1, "f-strings convert values for you."),
            fill("Make it an f-string.", "name = \"Pix\"\nprint(___\"Hi {name}\")", &["f", "s", "$"], 0, "Put f before the quotes."),
            fill("Insert the value of hp.", "hp = 30\nprint(f\"HP: ___\")", &["{hp}", "(hp)", "hp"], 0, "Curly braces insert the value."),
            copy("Type this.", "gold = 5\nprint(f\"You have {gold} gold\")", "f-string with {gold}."),
            recall("From memory: store \"Rook\" in name and print Welcome, Rook! using an f-string.", "name = \"Rook\"\nprint(f\"Welcome, {name}!\")", "f\"Welcome, {name}!\"."),
        ]).with(r##"## Building text the easy way
Joining with + and str() gets messy:

    print("Hero " + name + " has " + str(hp) + " HP")

An f-string (formatted string) is cleaner. Put the letter f right before the opening quote and write values inside curly braces:

    print(f"Hero {name} has {hp} HP")

Python replaces each {...} with the value, converting numbers to text automatically.

## Expressions inside braces
Anything that produces a value can go inside:

    print(f"Total: {price * amount} gold")
    print(f"Next level in {100 - xp} XP")
    print(f"Shout: {name.upper()}")

## Rounding inside f-strings
round() keeps numbers tidy: f"{round(3.14159, 2)}" gives 3.14.

## Common mistakes
- Forgetting the f: "Hi {name}" prints the braces literally.
- Putting the f inside the quotes: "f Hi..." is just text.
- To show a real curly brace, double it: f"{{ }}"."##,
        &["An f-string starts with f before the quotes.", "{expression} inserts a value, converted to text automatically.", "Any expression works inside the braces.", "Without the f, the braces are printed literally."]),
        p("Project: Temperature Converter", &[
            (r#"Convert a temperature from Celsius to Fahrenheit:

F = C * 9 / 5 + 32"#, "flame"),
            (r#"celsius = 25
fahrenheit = celsius * 9 / 5 + 32
print(f"{celsius}C is {fahrenheit}F")"#, "hourglass"),
        ], vec![
            quiz("What is 100 Celsius in Fahrenheit (100 * 9 / 5 + 32)?", &["212", "180", "132"], 0, "180 + 32 = 212."),
            quiz("Why store the formula result in a variable?", &["To reuse and print it clearly", "Variables are faster", "It is required"], 0, "Named results make code readable."),
            fill("Apply the formula.", "f = c * 9 / 5 ___ 32", &["+", "-", "*"], 0, "Multiply, divide, then add 32."),
            fill("Print the result with an f-string.", "f = 77\nprint(___\"{f}F\")", &["f", "str", "%"], 0, "f-strings start with f."),
            copy("Type the converter.", "c = 30\nf = c * 9 / 5 + 32\nprint(f\"{c}C = {f}F\")", "Formula plus f-string."),
            recall("From memory: store 0 in c, compute f with c * 9 / 5 + 32, and print f.", "c = 0\nf = c * 9 / 5 + 32\nprint(f)", "0C is 32F."),
        ]).with(r##"## A useful little tool
Travellers from the northern realms measure temperature in Fahrenheit; your village uses Celsius. Your converter fixes the confusion. It combines variables, maths, type conversion and f-strings.

## The formula
    fahrenheit = celsius * 9 / 5 + 32

Python follows the usual order: multiply and divide from left to right, then add 32.

## Build it
    celsius = 25
    fahrenheit = celsius * 9 / 5 + 32
    print(f"{celsius}C is {fahrenheit}F")

## Make it nicer
- Round the result: round(fahrenheit, 1).
- Go the other way too: celsius = (fahrenheit - 32) * 5 / 9. The brackets matter here.
- In a real program you would read the temperature with float(input("Celsius? ")).

## Check
Test values you know: 0C should be 32F and 100C should be 212F. Testing with known answers is how programmers make sure a formula is right."##,
        &["Turn a formula into code with variables.", "Brackets control the order of operations.", "Print results with f-strings and round().", "Test with values whose answers you already know."]),
    ])
}

fn control() -> Module {
    ch("Control Flow", "Make decisions with if, else and elif, and combine conditions.", vec![
        l("If Statements", &[
            (r#"if runs code only when a condition is True:

if hp < 10:
    print("Drink a potion!")

The colon and the indent are required."#, "shield"),
            (r#"Comparisons:

==  equal     !=  not equal
>   bigger    <   smaller
>=  at least  <=  at most"#, "sword"),
        ], vec![
            quiz("What marks the code that belongs to an if?", &["Indentation", "Curly braces", "A semicolon"], 0, "Python uses indentation for blocks."),
            quiz("Which checks equality?", &["=", "==", "==="], 1, "== compares; = stores."),
            fill("Finish the if line.", "if hp < 10___\n    print(\"Low HP\")", &[":", ";", ""], 0, "if lines end with a colon."),
            fill("Check if gold is at least 5.", "if gold ___ 5:\n    print(\"Buy\")", &[">=", "=>", "=="], 0, ">= means at least."),
            copy("Type this.", "hp = 4\nif hp < 10:\n    print(\"Low HP\")", "Colon, then an indented line."),
            recall("From memory: set gold to 7; if gold > 5 print Rich.", "gold = 7\nif gold > 5:\n    print(\"Rich\")", "if with an indented print."),
        ]).with(r##"## Making decisions
Programs often need to choose what to do. if runs a block of code only when a condition is True:

    hp = 4
    if hp < 10:
        print("Drink a potion!")
    print("Onward!")

- The condition (hp < 10) is a question with a True or False answer.
- The line ends with a colon.
- The block underneath is indented (4 spaces is the standard). That indentation is how Python knows which lines belong to the if.
- "Onward!" is not indented, so it always runs.

## Comparison operators
    ==   equal to            !=   not equal to
    >    greater than        <    less than
    >=   greater or equal    <=   less or equal

## The = trap
= stores a value; == compares. if gold = 5: is an error. Read == out loud as "is equal to".

## Indentation errors
Mixing up indentation is the most common Python error for beginners. Every line in a block must be indented by the same amount, and the block must be indented more than the if line."##,
        &["if condition: runs the indented block only when the condition is True.", "The line ends with a colon and the block is indented.", "Comparisons: == != > < >= <=.", "= stores; == compares."]),
        l("Else", &[
            (r#"else runs when the condition is False:

if gold >= 10:
    print("Buy the sword")
else:
    print("Keep saving")"#, "coin_gold"),
            (r#"else lines up with its if and has no condition of its own."#, "key"),
        ], vec![
            quiz("When does the else block run?", &["When the if condition is False", "Always", "Never"], 0, "else catches the other case."),
            quiz("Does else have a condition?", &["Yes", "No"], 1, "else: has no condition."),
            fill("Add the other path.", "if win:\n    print(\"Yay\")\n___:\n    print(\"Again\")", &["else", "elif", "otherwise"], 0, "else catches everything else."),
            fill("Line the else up correctly.", "if a > 1:\n    print(\"big\")\n___\n    print(\"small\")", &["else:", "    else:", "else"], 0, "else lines up with if and ends with a colon."),
            copy("Type this.", "n = 3\nif n > 5:\n    print(\"Big\")\nelse:\n    print(\"Small\")", "if / else."),
            recall("From memory: set age to 12; if age >= 13 print Teen, else print Kid.", "age = 12\nif age >= 13:\n    print(\"Teen\")\nelse:\n    print(\"Kid\")", "if / else."),
        ]).with(r##"## Two paths
if alone either runs a block or skips it. Adding else gives a second path for when the condition is False:

    gold = 6
    if gold >= 10:
        print("You buy the sword.")
    else:
        print("Keep saving.")

Exactly one of the two blocks runs, never both and never neither.

## Rules for else
- else has no condition: it simply catches every case the if did not.
- It lines up exactly under its if (same indentation).
- It ends with a colon, and its block is indented.

## Thinking in pairs
Many questions have two outcomes: right or wrong answer, day or night, win or lose. Whenever you write "if this, do that, otherwise do something else", you need if / else.

## Choosing a value
A common pattern sets a variable differently in each branch and uses it afterwards:

    if score >= 50:
        result = "Pass"
    else:
        result = "Try again"
    print(result)"##,
        &["else runs when the if condition is False.", "Exactly one of if / else runs.", "else has no condition and lines up under its if.", "Set a variable in each branch, then use it after."]),
        l("Elif", &[
            (r#"elif adds more choices:

if score >= 90:
    print("Gold")
elif score >= 70:
    print("Silver")
else:
    print("Bronze")"#, "medallion"),
            (r#"Python checks from the top. The FIRST true condition wins; the rest are skipped."#, "crown"),
        ], vec![
            quiz("score = 95. With checks >= 90 then >= 70, what prints?", &["Gold only", "Gold and Silver", "Silver only"], 0, "The first true branch wins."),
            quiz("What is elif short for?", &["else if", "elif is a name", "else finally"], 0, "elif = else if."),
            fill("Add a middle choice.", "if t > 30:\n    print(\"Hot\")\n___ t > 15:\n    print(\"Warm\")", &["elif", "else if", "if"], 0, "Python uses elif."),
            fill("Check the highest grade first.", "if score >= ___:\n    print(\"A\")\nelif score >= 70:\n    print(\"B\")", &["90", "50", "70"], 0, "Highest threshold first."),
            copy("Type this.", "n = 0\nif n > 0:\n    print(\"Positive\")\nelif n < 0:\n    print(\"Negative\")\nelse:\n    print(\"Zero\")", "if / elif / else."),
            recall("From memory: set hp to 5; if hp > 20 print Fine, elif hp > 0 print Hurt, else print Down.", "hp = 5\nif hp > 20:\n    print(\"Fine\")\nelif hp > 0:\n    print(\"Hurt\")\nelse:\n    print(\"Down\")", "Three branches."),
        ]).with(r##"## More than two choices
elif (short for "else if") adds extra conditions between if and else:

    score = 75
    if score >= 90:
        print("Gold medal")
    elif score >= 70:
        print("Silver medal")
    elif score >= 50:
        print("Bronze medal")
    else:
        print("Keep training")

## First match wins
Python checks the conditions from top to bottom and runs only the FIRST block whose condition is True. With 75, the first check fails, the second matches, and the rest are skipped, even though 75 >= 50 is also true.

## Order matters
If you put score >= 50 first, a score of 95 would get a bronze medal, because that check matches first. Put the strictest (highest) conditions first.

## else is optional
Without a final else, nothing happens when no condition matches. With else, you always have a default.

## Indentation
if, every elif and else line up in the same column; their blocks are indented one level deeper."##,
        &["elif adds more conditions between if and else.", "Conditions are checked top to bottom; the first True one wins.", "Put the strictest conditions first.", "A final else is a safe default."]),
        l("Logical Operators", &[
            (r#"Combine conditions with words:

and  both must be True
or   at least one True
not  flips True/False"#, "keyhole"),
            (r#"if has_key and door_locked:
    print("Unlock it")
if not is_alive:
    print("Game over")"#, "key"),
        ], vec![
            quiz("True and False gives...", &["True", "False"], 1, "and needs both."),
            quiz("False or True gives...", &["True", "False"], 0, "or needs at least one."),
            fill("Both must be true.", "if age >= 13 ___ age <= 19:\n    print(\"Teen\")", &["and", "&&", "or"], 0, "Python uses the word and."),
            fill("Flip the boolean.", "if ___ is_open:\n    print(\"Closed\")", &["not", "!", "no"], 0, "Python uses not."),
            copy("Type this.", "a = 15\nif a > 10 and a < 20:\n    print(\"Teen number\")", "and joins two checks."),
            recall("From memory: set day to \"Sat\"; if day == \"Sat\" or day == \"Sun\" print Weekend.", "day = \"Sat\"\nif day == \"Sat\" or day == \"Sun\":\n    print(\"Weekend\")", "or joins two checks."),
        ]).with(r##"## Combining conditions
Python uses plain English words to combine True/False values:

- and: True only if BOTH sides are True.
- or: True if AT LEAST ONE side is True.
- not: flips True to False and False to True.

    if has_key and door_locked:
        print("Unlock the door")
    if is_weekend or is_holiday:
        print("The shop is closed")
    if not is_alive:
        print("Game over")

## Ranges
To check a number is between two values:

    if age >= 13 and age <= 19:
        print("Teenager")

Python also allows the maths-style shortcut 13 <= age <= 19, but the and version works in every language.

## A common mistake
    if day == "Sat" or "Sun":     # wrong!

This is always True, because "Sun" on its own counts as True. Repeat the comparison on both sides: day == "Sat" or day == "Sun".

## Truthiness
0, empty text "" and empty lists [] count as False; most other values count as True. So if name: means "if name is not empty"."##,
        &["and needs both sides True; or needs at least one.", "not flips a boolean.", "Check ranges with two comparisons joined by and.", "Repeat the comparison on both sides of or."]),
        p("Project: Grade Calculator", &[
            (r#"Turn a test score into a letter grade:

90+ A, 80+ B, 70+ C, 60+ D, below F"#, "book"),
            (r#"score = 84
if score >= 90:
    grade = "A"
elif score >= 80:
    grade = "B"
...
print(f"Grade: {grade}")"#, "medallion"),
        ], vec![
            quiz("Score 84: which grade?", &["A", "B", "C"], 1, "84 is not >= 90 but is >= 80."),
            quiz("Why is the score >= 90 check first?", &["Otherwise high scores would match a lower grade first", "It is shorter", "Python requires it"], 0, "First match wins."),
            fill("Add the C grade.", "elif score ___ 70:\n    grade = \"C\"", &[">=", "<=", "=="], 0, "At least 70."),
            fill("Catch every other score.", "___:\n    grade = \"F\"", &["else", "elif", "if"], 0, "else is the default."),
            copy("Type this.", "score = 91\nif score >= 90:\n    grade = \"A\"\nelse:\n    grade = \"B\"\nprint(grade)", "Set grade in each branch."),
            recall("From memory: set score to 65; if score >= 60 set grade to \"Pass\", else \"Fail\"; print grade.", "score = 65\nif score >= 60:\n    grade = \"Pass\"\nelse:\n    grade = \"Fail\"\nprint(grade)", "Choose a value, then print."),
        ]).with(r##"## Scores to grades
The guild academy needs a program that turns test scores into letter grades. This project uses if, elif and else in the right order.

## The rules
    90 and above   A
    80 to 89       B
    70 to 79       C
    60 to 69       D
    below 60       F

## Build it
    score = 84
    if score >= 90:
        grade = "A"
    elif score >= 80:
        grade = "B"
    elif score >= 70:
        grade = "C"
    elif score >= 60:
        grade = "D"
    else:
        grade = "F"
    print(f"Score {score}: grade {grade}")

## Why it works
Because the checks go from the highest to the lowest and the first match wins, we never need to write "score >= 80 and score < 90". By the time Python reaches the B check, it already knows the score is below 90.

## Test the edges
Try 90, 89, 60 and 59. Boundaries are where bugs hide: is 90 an A? Is 60 a D?"##,
        &["Order elif checks from highest to lowest.", "The first match wins, so ranges need only one comparison.", "Set a variable in each branch and print it once.", "Test boundary values like 90 and 89."]),
    ])
}

fn loops() -> Module {
    ch("Loops", "Repeat code with while and for, count with range(), and steer loops with break and continue.", vec![
        l("While Loops", &[
            (r#"while repeats while a condition is True:

count = 3
while count > 0:
    print(count)
    count -= 1"#, "hourglass"),
            (r#"Something inside must change, or the loop never ends!"#, "flame"),
        ], vec![
            quiz("When does a while loop stop?", &["When its condition becomes False", "After 10 rounds", "Never"], 0, "It runs while the condition is True."),
            quiz("What causes an endless loop?", &["Nothing ever makes the condition False", "Using print", "Using numbers"], 0, "Change something each round."),
            fill("Repeat while n is below 3.", "n = 0\n___ n < 3:\n    n += 1", &["while", "for", "if"], 0, "while repeats."),
            fill("Count down.", "c = 3\nwhile c > 0:\n    print(c)\n    c ___ 1", &["-=", "+=", "="], 0, "c -= 1 moves towards 0."),
            copy("Type this.", "n = 1\nwhile n <= 3:\n    print(n)\n    n += 1", "Prints 1, 2, 3."),
            recall("From memory: start hp at 3 and while hp > 0 print hp, then subtract 1.", "hp = 3\nwhile hp > 0:\n    print(hp)\n    hp -= 1", "Count down to 1."),
        ]).with(r##"## Repeating code
A loop runs the same block again and again. while repeats as long as its condition is True:

    count = 3
    while count > 0:
        print(count)
        count -= 1
    print("Lift off!")

## Step by step
1. Check the condition. If it is False, skip to the line after the loop.
2. Run the indented block.
3. Go back to step 1.

Here the block runs with count = 3, 2 and 1. Then count is 0, the condition is False, and the loop ends.

## The endless loop
If nothing inside the loop ever makes the condition False, it runs forever. Always ask: "what changes each time round, and will it eventually stop the loop?" (Code Forge stops runaway loops, but real programs can freeze.)

## When to use while
Use while when you cannot know in advance how many repeats you need: keep asking until the password is right, keep rolling until you get a six, keep fighting until one side wins.

## while True
while True: loops forever on purpose; you leave it with break (later in this chapter)."##,
        &["while condition: repeats the block while the condition is True.", "The condition is checked before every round.", "Something must change, or the loop never ends.", "Use while when the number of repeats is unknown."]),
        l("For Loops", &[
            (r#"for goes through every item of a list or string:

for hero in ["Pix", "Rook"]:
    print(hero)"#, "map"),
            (r#"for letter in "Hi":
    print(letter)

prints H, then i."#, "scroll_fire"),
        ], vec![
            quiz("How many times does the body run for a list of 3 items?", &["Once", "3 times", "Forever"], 1, "Once per item."),
            quiz("What does for ch in \"abc\" go through?", &["Each letter", "The whole word once", "Nothing"], 0, "Strings are sequences of characters."),
            fill("Loop over the list.", "___ x in [1, 2, 3]:\n    print(x)", &["for", "while", "each"], 0, "for ... in ..."),
            fill("Finish the loop header.", "for item ___ bag:\n    print(item)", &["in", "of", ":"], 0, "for item in bag."),
            copy("Type this.", "for food in [\"pie\", \"ale\"]:\n    print(food)", "One line per item."),
            recall("From memory: loop over [\"N\", \"S\"] and print each item.", "for d in [\"N\", \"S\"]:\n    print(d)", "for d in list: print(d)."),
        ]).with(r##"## Going through a sequence
A for loop takes each item of a sequence in turn and runs the block once per item:

    party = ["Pix", "Rook", "Ada"]
    for hero in party:
        print(f"{hero} joins the quest!")

Read it as: "for each hero in party, do this".

## The loop variable
hero is a variable created by the loop. In each round it holds the next item. You can call it anything, but a name that describes ONE item (hero, item, letter) makes code easy to read.

## Strings are sequences too
    for letter in "Pix":
        print(letter)

prints P, i and x on separate lines.

## for versus while
- for: you have a collection (or a known count) to go through.
- while: you repeat until something happens.

## Totals
A loop plus a variable created BEFORE the loop builds totals:

    total = 0
    for price in [5, 7, 3]:
        total += price
    print(total)   # 15"##,
        &["for item in sequence: runs the block once per item.", "The loop variable holds the current item.", "Strings can be looped letter by letter.", "Keep totals in a variable created before the loop."]),
        l("Range", &[
            (r#"range() makes a run of numbers:

range(5)       0 1 2 3 4
range(1, 4)    1 2 3
range(0, 10, 2) 0 2 4 6 8"#, "gear"),
            (r#"for i in range(3):
    print("Swing!")

repeats exactly 3 times."#, "sword"),
        ], vec![
            quiz("What numbers does range(4) give?", &["1 2 3 4", "0 1 2 3", "0 1 2 3 4"], 1, "It starts at 0 and stops before 4."),
            quiz("range(2, 5) gives...", &["2 3 4", "2 3 4 5", "3 4 5"], 0, "The end number is not included."),
            fill("Repeat 5 times.", "for i in ___(5):\n    print(\"Hi\")", &["range", "repeat", "times"], 0, "range(5) has 5 numbers."),
            fill("Count by twos.", "for i in range(0, 10, ___):\n    print(i)", &["2", "10", "0"], 0, "The third number is the step."),
            copy("Type this.", "for i in range(1, 4):\n    print(i)", "Prints 1, 2, 3."),
            recall("From memory: print the numbers 0 to 2 using range.", "for i in range(3):\n    print(i)", "range(3) is 0, 1, 2."),
        ]).with(r##"## Counting with range()
To repeat something a set number of times, loop over range():

    for i in range(3):
        print("Swing!")

range() produces numbers on demand:

    range(5)          0, 1, 2, 3, 4
    range(1, 6)       1, 2, 3, 4, 5
    range(0, 10, 2)   0, 2, 4, 6, 8
    range(5, 0, -1)   5, 4, 3, 2, 1

## The stop number is not included
range(1, 6) stops BEFORE 6. This feels odd at first, but it means range(5) has exactly 5 numbers, and range(a, b) has b - a numbers.

## Using the number
The loop variable holds the current number:

    for level in range(1, 4):
        print(f"Level {level}")

## Times tables
    for n in range(1, 11):
        print(f"7 x {n} = {7 * n}")

## Off-by-one errors
Running one time too many or too few is the classic loop bug. Check the first and last values: range(1, 10) ends at 9, not 10."##,
        &["range(n) gives 0 up to n - 1.", "range(start, stop) stops before stop.", "A third number sets the step, even negative.", "Check the first and last values to avoid off-by-one errors."]),
        l("Break and Continue", &[
            (r#"break leaves the loop now.
continue skips to the next round."#, "axe"),
            (r#"for n in range(1, 10):
    if n == 4:
        break
    print(n)

prints 1, 2, 3."#, "hammer"),
        ], vec![
            quiz("What does continue do?", &["Ends the loop", "Skips the rest of this round", "Restarts the program"], 1, "It jumps to the next round."),
            quiz("for n in range(5): if n == 2: continue; print(n). What prints?", &["0 1 3 4", "0 1", "2"], 0, "2 is skipped."),
            fill("Leave the loop when found.", "for x in bag:\n    if x == \"key\":\n        ___", &["break", "stop", "exit"], 0, "break leaves the loop."),
            fill("Skip odd numbers.", "for n in range(6):\n    if n % 2 == 1:\n        ___\n    print(n)", &["continue", "break", "pass"], 0, "continue skips this round."),
            copy("Type this.", "for n in range(5):\n    if n == 3:\n        break\n    print(n)", "Prints 0, 1, 2."),
            recall("From memory: loop n over range(4); continue when n == 1; print n.", "for n in range(4):\n    if n == 1:\n        continue\n    print(n)", "Prints 0, 2, 3."),
        ]).with(r##"## Steering a loop from inside
## break: leave now
break ends the loop immediately, even if there are items left or the condition is still True. Python continues with the first line after the loop.

    for item in ["rope", "key", "map"]:
        if item == "key":
            print("Found the key!")
            break
        print(f"Not {item}...")

## continue: skip this round
continue skips the rest of the current round and jumps straight to the next one.

    for n in range(1, 6):
        if n == 3:
            continue
        print(n)        # 1 2 4 5

## while True with break
A common pattern is a loop that runs until something happens inside it:

    tries = 0
    while True:
        tries += 1
        if tries == 3:
            break
    print(f"Done after {tries} tries")

## Use them wisely
break and continue are great for searching and filtering, but many of them in one loop make code hard to follow. If a loop is getting complicated, try a clearer condition instead.

## pass
pass does nothing at all; it is a placeholder where Python needs a block but you have nothing to write yet."##,
        &["break leaves the loop immediately.", "continue skips to the next round.", "while True plus break loops until something happens.", "pass is an empty placeholder block."]),
        p("Project: FizzBuzz", &[
            (r#"Count from 1 to 15. For multiples of 3 print Fizz, of 5 print Buzz, of both print FizzBuzz."#, "orb_red"),
            (r#"for n in range(1, 16):
    if n % 15 == 0:
        print("FizzBuzz")
    elif n % 3 == 0:
        print("Fizz")
    ..."#, "staff"),
        ], vec![
            quiz("What should 15 print?", &["Fizz", "Buzz", "FizzBuzz"], 2, "15 is a multiple of 3 and 5."),
            quiz("Why check n % 15 first?", &["Otherwise 15 would print Fizz", "15 is the biggest", "It is shorter"], 0, "First match wins."),
            fill("Test for multiples of 3.", "if n % 3 ___ 0:\n    print(\"Fizz\")", &["==", "=", "!="], 0, "No remainder means a multiple."),
            fill("Count from 1 to 15.", "for n in range(1, ___):\n    print(n)", &["16", "15", "14"], 0, "The stop number is not included."),
            copy("Type this.", "for n in range(1, 4):\n    if n % 3 == 0:\n        print(\"Fizz\")\n    else:\n        print(n)", "Prints 1, 2, Fizz."),
            recall("From memory: loop n from 1 to 5; if n % 5 == 0 print Buzz, else print n.", "for n in range(1, 6):\n    if n % 5 == 0:\n        print(\"Buzz\")\n    else:\n        print(n)", "Prints 1 2 3 4 Buzz."),
        ]).with(r##"## A famous puzzle
FizzBuzz is a classic programming exercise, sometimes used in job interviews. It combines loops, the remainder operator and if / elif / else.

## The rules
Count from 1 to 15:
- Multiples of both 3 and 5: print FizzBuzz.
- Multiples of 3: print Fizz.
- Multiples of 5: print Buzz.
- Otherwise: print the number.

## The solution
    for n in range(1, 16):
        if n % 15 == 0:
            print("FizzBuzz")
        elif n % 3 == 0:
            print("Fizz")
        elif n % 5 == 0:
            print("Buzz")
        else:
            print(n)

## Why check 15 first?
15 is a multiple of 3, so if the Fizz check came first, it would win and FizzBuzz would never print. A number divisible by both 3 and 5 is divisible by 15, so that check goes on top.

## Check
The first fifteen lines should be: 1, 2, Fizz, 4, Buzz, Fizz, 7, 8, Fizz, Buzz, 11, Fizz, 13, 14, FizzBuzz."##,
        &["% tells you whether a number divides evenly.", "Put the most specific check (15) first.", "range(1, 16) counts from 1 to 15.", "Compare your output with the expected list."]),
    ])
}

fn lists() -> Module {
    ch("Lists", "Store many values in order, read them by position, change them and loop over them.", vec![
        l("Lists", &[
            (r#"A list holds many values in order:

bag = ["sword", "potion", "map"]
print(bag)
print(len(bag))   # 3"#, "chest"),
            (r#"Lists can hold any type, even mixed:

scores = [90, 75, 100]
empty = []"#, "coins"),
        ], vec![
            quiz("Which makes a list?", &["[\"a\", \"b\"]", "(\"a\" \"b\")", "{a, b}"], 0, "Lists use square brackets."),
            quiz("What does len([4, 5, 6]) give?", &["3", "6", "15"], 0, "len counts items."),
            fill("Make an empty list.", "loot = ___", &["[]", "()", "\"\""], 0, "[] is an empty list."),
            fill("Count the items.", "print(___(bag))", &["len", "count", "size"], 0, "len() counts items."),
            copy("Type this.", "bag = [\"rope\", \"torch\"]\nprint(len(bag))", "Two items."),
            recall("From memory: make a list nums with 1, 2, 3 and print it.", "nums = [1, 2, 3]\nprint(nums)", "Square brackets, commas."),
        ]).with(r##"## Many values in one variable
A list stores several values, in order, in one variable:

    bag = ["sword", "potion", "map"]
    scores = [90, 75, 100]
    empty = []

Write the items between square brackets, separated by commas.

## Why lists?
Separate variables item1, item2, item3 do not scale. A list can grow and shrink, you can loop over it with one for loop, and Python has many tools for sorting and searching it.

## How many items?
len() counts the items:

    print(len(bag))   # 3

len() works on strings too: len("Pix") is 3.

## Is it in the list?
The in operator answers yes or no:

    if "map" in bag:
        print("You won't get lost.")

## Mixed lists
A list can hold different types, even other lists, but most lists hold one kind of thing: all names, all scores. That makes them easier to work with."##,
        &["A list stores values in order inside [ ].", "Items are separated by commas; [] is empty.", "len() counts the items.", "x in my_list checks if x is there."]),
        l("Indexes and Slices", &[
            (r#"Items are numbered from 0:

bag = ["sword", "potion", "map"]
bag[0]   "sword"
bag[-1]  "map" (the last)"#, "key"),
            (r#"A slice takes part of a list:

bag[0:2]   ["sword", "potion"]
bag[1:]    ["potion", "map"]"#, "map"),
        ], vec![
            quiz("bag = [\"a\", \"b\", \"c\"]. What is bag[1]?", &["a", "b", "c"], 1, "Index 1 is the second item."),
            quiz("What does bag[-1] give?", &["The first item", "The last item", "An error"], 1, "Negative indexes count from the end."),
            fill("Get the first item.", "print(bag[___])", &["0", "1", "first"], 0, "Counting starts at 0."),
            fill("Take the first two items.", "print(bag[0:___])", &["2", "1", "3"], 0, "The end of a slice is not included."),
            copy("Type this.", "bag = [\"a\", \"b\", \"c\"]\nprint(bag[1:])", "Everything from index 1."),
            recall("From memory: make nums = [5, 6, 7] and print the last item with a negative index.", "nums = [5, 6, 7]\nprint(nums[-1])", "nums[-1] is 7."),
        ]).with(r##"## Positions start at 0
Each item has an index: its position, counting from 0.

    bag = ["sword", "potion", "map"]
    print(bag[0])   # sword
    print(bag[2])   # map

The index means "how many steps from the start", so the first item is 0 steps away. The last index is always len(bag) - 1.

## Negative indexes
Negative numbers count from the end: bag[-1] is the last item, bag[-2] the one before it. Handy when you do not know the length.

## Index errors
bag[3] on a 3-item list is an IndexError: there is no fourth item.

## Slices
A slice copies part of a list with [start:stop]. Like range(), the stop is NOT included:

    bag[0:2]   # ["sword", "potion"]
    bag[1:]    # from index 1 to the end
    bag[:2]    # from the start up to index 2

## Strings slice too
    word = "dragon"
    print(word[0:3])   # dra
    print(word[-1])    # n"##,
        &["Indexes start at 0; the last is len - 1.", "Negative indexes count from the end: [-1] is the last.", "list[start:stop] slices; stop is not included.", "Strings can be indexed and sliced too."]),
        l("Changing Lists", &[
            (r#"Lists can change:

bag.append("rope")    add to the end
bag.insert(0, "gem")  add at a position
bag.remove("map")     remove by value
bag.pop()             remove the last"#, "hammer"),
            (r#"Replace an item by index:

bag[0] = "shield""#, "shield"),
        ], vec![
            quiz("Which adds to the END of a list?", &["append", "insert", "pop"], 0, "append adds at the end."),
            quiz("What does pop() return?", &["The removed item", "The new length", "Nothing"], 0, "pop gives back what it removed."),
            fill("Add rope to the bag.", "bag = []\nbag.___(\"rope\")", &["append", "add", "push"], 0, "Python lists use append."),
            fill("Replace the first item.", "bag = [\"a\", \"b\"]\nbag[___] = \"z\"", &["0", "1", "-2.5"], 0, "Index 0 is the first item."),
            copy("Type this.", "bag = [\"map\"]\nbag.append(\"rope\")\nprint(bag)", "append adds to the end."),
            recall("From memory: make bag = [\"a\", \"b\"], remove \"a\", then print bag.", "bag = [\"a\", \"b\"]\nbag.remove(\"a\")\nprint(bag)", "remove deletes by value."),
        ]).with(r##"## Lists can change
Unlike strings, lists can be changed after they are made.

## Adding
    bag = ["map"]
    bag.append("rope")       # ["map", "rope"]
    bag.insert(0, "gem")     # ["gem", "map", "rope"]

append adds to the end; insert(index, item) puts an item at a position and shifts the rest along.

## Removing
    bag.remove("map")        # removes the first "map"
    last = bag.pop()         # removes and returns the last item
    first = bag.pop(0)       # removes and returns index 0
    del bag[0]               # deletes by index

remove() causes an error if the value is not there, so check with in first if you are unsure.

## Replacing
    bag[0] = "shield"

## Sorting and reversing
    scores.sort()            # smallest to largest (A to Z for text)
    scores.reverse()         # flip the order

These change the list itself and return nothing, so write scores.sort(), not scores = scores.sort().

## Methods
append, insert, remove, pop and sort are methods: functions that belong to the list, called with a dot."##,
        &["append adds to the end; insert(i, x) adds at a position.", "remove(x) deletes by value; pop() removes and returns an item.", "list[i] = x replaces an item.", "sort() and reverse() change the list itself."]),
        l("Looping Lists", &[
            (r#"Loop over a list with for:

for item in bag:
    print(item)"#, "torch"),
            (r#"Build totals:

total = 0
for s in [3, 4, 5]:
    total += s
print(total)   # 12"#, "coins"),
        ], vec![
            quiz("Where should the total variable be created?", &["Before the loop", "Inside the loop", "After the loop"], 0, "Inside, it would reset each round."),
            quiz("What does sum([1, 2, 3]) give?", &["6", "3", "123"], 0, "sum adds the numbers."),
            fill("Loop over every score.", "for s ___ scores:\n    print(s)", &["in", "of", "from"], 0, "for s in scores."),
            fill("Add each price to total.", "total = 0\nfor p in [5, 7]:\n    total ___ p\nprint(total)", &["+=", "=+", "=="], 0, "+= adds to total."),
            copy("Type this.", "for i, item in enumerate([\"a\", \"b\"]):\n    print(i, item)", "enumerate gives the index too."),
            recall("From memory: loop over [2, 4] and print each number times 10.", "for n in [2, 4]:\n    print(n * 10)", "Prints 20 and 40."),
        ]).with(r##"## Doing something with every item
    party = ["Pix", "Rook", "Ada"]
    for hero in party:
        print(f"{hero} is ready")

## Totals, counts and maximums
Create a variable before the loop and update it inside:

    scores = [70, 95, 80]
    total = 0
    for s in scores:
        total += s
    print(total / len(scores))   # the average

Python also has shortcuts: sum(scores), max(scores) and min(scores).

## Knowing the position
enumerate() gives each item together with its index:

    for i, hero in enumerate(party):
        print(f"{i + 1}. {hero}")

The loop unpacks two values each round: i and hero.

## Filtering into a new list
    high = []
    for s in scores:
        if s >= 80:
            high.append(s)

## Do not change a list while looping over it
Removing items from the list you are looping over makes Python skip items. Build a new list instead, as above."##,
        &["for item in list visits every item.", "Totals live in a variable created before the loop.", "sum(), max() and min() are handy shortcuts.", "enumerate() gives the index and the item."]),
        p("Project: Shopping List", &[
            (r#"Build a market list: add items, remove what you bought, and print a numbered list."#, "money_bag"),
            (r#"shopping = ["bread", "apples"]
shopping.append("cheese")
shopping.remove("bread")
for i, item in enumerate(shopping):
    print(f"{i + 1}. {item}")"#, "letter"),
        ], vec![
            quiz("Why print i + 1?", &["People count from 1, indexes from 0", "To skip an item", "It is required"], 0, "Friendly numbering starts at 1."),
            quiz("What does len(shopping) tell you?", &["How many items are left", "The first item", "The total price"], 0, "len counts items."),
            fill("Add milk.", "shopping.___(\"milk\")", &["append", "insert", "pop"], 0, "append adds to the end."),
            fill("Check before removing.", "if \"eggs\" ___ shopping:\n    shopping.remove(\"eggs\")", &["in", "of", "=="], 0, "in checks membership."),
            copy("Type this.", "items = [\"pie\"]\nitems.append(\"ale\")\nprint(len(items))", "Two items now."),
            recall("From memory: make items = [\"bread\"], append \"jam\", and print items.", "items = [\"bread\"]\nitems.append(\"jam\")\nprint(items)", "append, then print."),
        ]).with(r##"## A list that changes
The innkeeper needs a shopping list for market day. Your program builds it, changes it, and prints it neatly.

## Plan
1. Start with a few items.
2. Add new items with append (and one at the front with insert).
3. Remove an item you already have, checking it is there first.
4. Print the number of items and a numbered list.

    shopping = ["bread", "apples"]
    shopping.append("cheese")
    shopping.insert(0, "salt")
    if "bread" in shopping:
        shopping.remove("bread")
    print(f"{len(shopping)} things to buy:")
    for i, item in enumerate(shopping):
        print(f"{i + 1}. {item}")

## Extra ideas
- Sort the list alphabetically before printing.
- Keep a second list, bought, and move items from one to the other with pop and append.

## Check
Does the numbering start at 1? Does removing a missing item crash, or did you check with in first?"##,
        &["append and insert add items; remove deletes them.", "Check with in before removing.", "enumerate plus i + 1 gives friendly numbering.", "len() reports how many items remain."]),
    ])
}

fn functions() -> Module {
    ch("Functions", "Package code into reusable functions with parameters, return values and defaults.", vec![
        l("Functions", &[
            (r#"A function is a named, reusable block:

def greet():
    print("Welcome!")

greet()
greet()"#, "spellbook"),
            (r#"def defines it. greet() calls it. Nothing runs until you call it."#, "hat_wizard"),
        ], vec![
            quiz("Which keyword defines a function?", &["def", "function", "func"], 0, "Python uses def."),
            quiz("When does the code inside a function run?", &["When it is defined", "When it is called", "Never"], 1, "Calling runs it."),
            fill("Define a function.", "___ hello():\n    print(\"Hi\")", &["def", "fun", "define"], 0, "def name():"),
            fill("Call the function.", "def hello():\n    print(\"Hi\")\nhello___", &["()", ":", "[]"], 0, "Brackets call it."),
            copy("Type this.", "def roar():\n    print(\"ROAR\")\nroar()", "Define, then call."),
            recall("From memory: define cheer() that prints Hooray, then call it.", "def cheer():\n    print(\"Hooray\")\ncheer()", "def then call."),
        ]).with(r##"## Reusable blocks of code
A function is a named block of code you can run whenever you like. Write it once, use it many times.

    def greet():
        print("Welcome, traveller!")

    greet()
    greet()

## Defining and calling
- def greet(): defines the function. The code inside is indented. Nothing runs yet: you have only written the recipe.
- greet() calls the function: Python jumps into it, runs the block, then comes back. The brackets are what make it run.

## Why use functions?
- No repetition: the same code lives in one place.
- Easy fixes: change the function and every call is updated.
- Readable programs: names like open_shop() explain what is happening.

## Naming
Function names follow the same rules as variables. Python style uses lowercase words with underscores: roll_dice, show_menu.

## Define before you call
Python runs top to bottom, so a function must be defined before the line that calls it runs.

## You already use functions
print(), len(), int() and range() are functions that come with Python."##,
        &["def name(): defines a function; the body is indented.", "name() calls it; nothing runs until it is called.", "Functions remove repetition and make code readable.", "Define a function before calling it."]),
        l("Parameters", &[
            (r#"Parameters let you pass values in:

def greet(name):
    print(f"Hi, {name}!")

greet("Pix")
greet("Rook")"#, "staff"),
            (r#"Several parameters, separated by commas:

def attack(who, damage):
    print(f"{who} deals {damage}")"#, "sword"),
        ], vec![
            quiz("In greet(\"Pix\"), what is \"Pix\"?", &["An argument", "A function", "A loop"], 0, "Values passed in are arguments."),
            quiz("def f(a, b) is called f(1, 2). What is b?", &["1", "2", "None"], 1, "Matched by position."),
            fill("Add a parameter.", "def heal(___):\n    print(amount)", &["amount", "\"amount\"", "10"], 0, "Parameters are names."),
            fill("Pass the argument.", "def greet(name):\n    print(name)\ngreet(___)", &["\"Ada\"", "name", "def"], 0, "Pass a value in the call."),
            copy("Type this.", "def double(n):\n    print(n * 2)\ndouble(4)", "Prints 8."),
            recall("From memory: define shout(word) that prints word.upper(), then call shout(\"hi\").", "def shout(word):\n    print(word.upper())\nshout(\"hi\")", "Prints HI."),
        ]).with(r##"## Giving a function information
Parameters are variables listed in the function's brackets. Arguments are the values you pass when calling it.

    def greet(name):
        print(f"Welcome, {name}!")

    greet("Pix")    # Welcome, Pix!
    greet("Rook")   # Welcome, Rook!

Each call gives name a different value, so one function handles every hero.

## Several parameters
Separate them with commas. Arguments are matched by position: the first argument goes to the first parameter, and so on.

    def attack(attacker, target, damage):
        print(f"{attacker} hits {target} for {damage}!")

    attack("Pix", "Slime", 5)

## Keyword arguments
You can also name the arguments, so the order no longer matters:

    attack(target="Slime", damage=5, attacker="Pix")

## The wrong number of arguments
Calling attack("Pix") is an error: Python says the function needs more values. The next lessons show how to give parameters default values.

## Parameters are local
A parameter only exists inside its function. Outside, the name means nothing (or something different)."##,
        &["Parameters are names in the definition; arguments are values in the call.", "Arguments are matched to parameters by position.", "Keyword arguments (name=value) can be in any order.", "Parameters exist only inside the function."]),
        l("Return Values", &[
            (r#"return sends a value back:

def double(n):
    return n * 2

x = double(5)
print(x)   # 10"#, "chalice"),
            (r#"return ends the function. A function without return gives None."#, "cup_silver"),
        ], vec![
            quiz("What does a function without return give back?", &["0", "None", "An error"], 1, "No return means None."),
            quiz("What happens after return runs?", &["The function ends", "The next line inside still runs", "The program stops"], 0, "return ends the function."),
            fill("Send the result back.", "def triple(n):\n    ___ n * 3", &["return", "print", "give"], 0, "return sends it back."),
            fill("Store the result.", "def add(a, b):\n    return a + b\ntotal = ___(2, 3)\nprint(total)", &["add", "return", "def"], 0, "Call it and store the value."),
            copy("Type this.", "def square(n):\n    return n * n\nprint(square(6))", "Prints 36."),
            recall("From memory: define half(n) that returns n / 2, then print half(10).", "def half(n):\n    return n / 2\nprint(half(10))", "Prints 5.0."),
        ]).with(r##"## Getting an answer back
print() shows a value to a person. return hands a value back to the code that called the function, so the program can keep using it.

    def double(n):
        return n * 2

    x = double(5)          # x is 10
    print(double(x) + 1)   # 21

## return ends the function
As soon as return runs, the function stops. This is useful for early exits:

    def check(hp):
        if hp <= 0:
            return "Defeated"
        return "Standing"

## No return means None
A function that never returns gives back None, Python's "nothing" value. A common bug is a function that prints its answer instead of returning it; then x = f() stores None.

## Print versus return
- print: for people to read.
- return: for the program to use (store, compare, add up, pass to another function).

Functions that return are more flexible, because the caller decides what to do with the result.

## Returning booleans
Functions that answer yes/no questions often return True or False:

    def can_buy(gold, price):
        return gold >= price"##,
        &["return sends a value back to the caller.", "return ends the function immediately.", "A function with no return gives None.", "print is for people; return is for the program."]),
        l("Default Values and Scope", &[
            (r#"Give a parameter a default:

def heal(hp, amount=10):
    return hp + amount

heal(5)      15
heal(5, 1)   6"#, "potion_red"),
            (r#"Variables made inside a function are local: they vanish when it ends."#, "lantern"),
        ], vec![
            quiz("def heal(hp, amount=10). What is heal(5)?", &["5", "15", "Error"], 1, "amount uses its default 10."),
            quiz("Can code outside a function use a variable created inside it?", &["Yes", "No"], 1, "It is local to the function."),
            fill("Give amount a default of 10.", "def heal(hp, amount___10):\n    return hp + amount", &["=", ":", "=="], 0, "Defaults use =."),
            fill("Use the default.", "def greet(name=\"hero\"):\n    print(name)\ngreet(___)", &["", "name", "None"], 0, "Call with no argument to use the default."),
            copy("Type this.", "def power(n, exp=2):\n    return n ** exp\nprint(power(3))", "Prints 9."),
            recall("From memory: define greet(name=\"friend\") that prints name, then call greet().", "def greet(name=\"friend\"):\n    print(name)\ngreet()", "Prints friend."),
        ]).with(r##"## Default values
A parameter can have a default value, used when the caller leaves it out:

    def heal(hp, amount=10):
        return hp + amount

    print(heal(5))      # 15, amount is 10
    print(heal(5, 1))   # 6

Parameters with defaults must come AFTER the ones without: def heal(amount=10, hp) is an error.

## Scope: where variables live
Variables created inside a function are local. They exist only while the function runs:

    def roll():
        result = 4
        return result

    roll()
    print(result)   # error: result is not defined

This is a good thing: each function has its own private workspace, so names inside one function never clash with names elsewhere.

## Reading outer variables
A function can READ a variable created outside it (a global):

    bonus = 5
    def total(score):
        return score + bonus

But relying on globals makes code harder to follow. Passing values as parameters and returning results keeps functions independent and easy to test."##,
        &["param=value gives a parameter a default.", "Defaults go after parameters without defaults.", "Variables created inside a function are local.", "Prefer parameters and return values over globals."]),
        p("Project: Tip Calculator", &[
            (r#"Write a function that adds a tip to a tavern bill and splits it between friends."#, "coins"),
            (r#"def split_bill(total, people, tip=10):
    with_tip = total + total * tip / 100
    return with_tip / people"#, "money_bag"),
        ], vec![
            quiz("Why give tip a default?", &["Most bills use the usual tip, so callers can skip it", "Defaults are faster", "Python requires it"], 0, "Defaults cover the common case."),
            quiz("split_bill(40, 4) with a 10% tip gives...", &["11", "10", "44"], 0, "44 split 4 ways is 11."),
            fill("Return each person's share.", "def share(total, people):\n    ___ total / people", &["return", "print", "give"], 0, "Return the value."),
            fill("Add the tip percentage.", "def with_tip(total, tip=10):\n    return total + total * tip / ___", &["100", "10", "1"], 0, "A percentage is divided by 100."),
            copy("Type this.", "def share(total, people):\n    return total / people\nprint(share(30, 3))", "Prints 10.0."),
            recall("From memory: define add_tip(total) that returns total * 1.1, then print add_tip(50).", "def add_tip(total):\n    return total * 1.1\nprint(add_tip(50))", "Prints about 55."),
        ]).with(r##"## A handy function
Splitting a bill after a feast is always a puzzle. Your function does it in one call, using parameters, a default value and a return value.

## Plan
1. Take the bill total, the number of people, and a tip percentage (default 10).
2. Add the tip: total + total * tip / 100.
3. Divide by the number of people.
4. Return the share, rounded to 2 decimals.

    def split_bill(total, people, tip=10):
        with_tip = total + total * tip / 100
        return round(with_tip / people, 2)

    print(split_bill(40, 4))       # 11.0
    print(split_bill(40, 4, 20))   # 12.0

## Why return instead of print?
Because the caller can now use the share: compare it to how much gold each person has, add it to a running total, or print it in a nicer message.

## Guard against mistakes
What if people is 0? Dividing by zero is an error. A careful version checks first:

    if people <= 0:
        return 0"##,
        &["Combine parameters, defaults and return in one function.", "Percentages: multiply by the percent, divide by 100.", "Return results so callers can reuse them.", "Guard against bad input like dividing by zero."]),
    ])
}

fn dicts() -> Module {
    ch("Dictionaries", "Store values by name with key-value pairs, change them and loop over them.", vec![
        l("Dictionaries", &[
            (r#"A dictionary stores values by key:

hero = {"name": "Pix", "hp": 30}
print(hero["name"])"#, "book"),
            (r#"Keys are usually strings. Each key: value pair is separated by commas."#, "key"),
        ], vec![
            quiz("Which makes a dictionary?", &["{\"hp\": 30}", "[\"hp\", 30]", "(\"hp\" = 30)"], 0, "Curly braces with key: value."),
            quiz("How do you read the value for \"hp\"?", &["hero[\"hp\"]", "hero.hp()", "hero(hp)"], 0, "Square brackets with the key."),
            fill("Separate key and value.", "stats = {\"mp\"___ 12}", &[":", "=", ","], 0, "key: value."),
            fill("Read the name.", "hero = {\"name\": \"Pix\"}\nprint(hero[___])", &["\"name\"", "name", "0"], 0, "Use the key in quotes."),
            copy("Type this.", "pet = {\"type\": \"cat\", \"age\": 3}\nprint(pet[\"type\"])", "Prints cat."),
            recall("From memory: make item = {\"name\": \"Sword\"} and print item[\"name\"].", "item = {\"name\": \"Sword\"}\nprint(item[\"name\"])", "Prints Sword."),
        ]).with(r##"## Values with names
A list finds items by position. A dictionary finds values by name. Each entry is a key: value pair:

    hero = {
        "name": "Pix",
        "hp": 30,
        "class": "Wizard",
    }
    print(hero["name"])   # Pix

- Curly braces hold the dictionary.
- Each key is followed by a colon and its value.
- Pairs are separated by commas.

## Real-world dictionaries
A real dictionary maps words to meanings; a Python dictionary maps keys to values. Phone books, game settings, a character's stats and translations are all natural dictionaries.

## Keys
Keys are usually strings and must be unique: writing the same key twice keeps only the last value. Looking up a key that does not exist is a KeyError.

## get() for safety
    print(hero.get("pet"))           # None, no error
    print(hero.get("pet", "none"))   # a default value

## Checking for a key
    if "hp" in hero:
        print("Has hit points")

in checks the keys, not the values."##,
        &["A dictionary stores key: value pairs in { }.", "dict[key] reads a value; a missing key is an error.", "get(key, default) reads safely.", "key in dict checks whether a key exists."]),
        l("Changing Dictionaries", &[
            (r#"Add or change by key:

hero["hp"] = 25
hero["pet"] = "owl"

Remove:
del hero["pet"]"#, "potion_magic"),
            (r#"pop() removes and returns a value:

gold = bag.pop("gold")"#, "money_bag"),
        ], vec![
            quiz("hero has no \"pet\" key. What does hero[\"pet\"] = \"owl\" do?", &["Adds the key", "Error", "Nothing"], 0, "Assignment adds missing keys."),
            quiz("Which removes a key?", &["del d[\"k\"]", "d.remove(\"k\")", "d[\"k\"] = None"], 0, "del removes the pair."),
            fill("Change hp to 25.", "hero = {\"hp\": 30}\nhero[\"hp\"] ___ 25", &["=", ":", "=="], 0, "Assign with =."),
            fill("Delete the pet key.", "hero = {\"pet\": \"owl\"}\n___ hero[\"pet\"]", &["del", "remove", "pop"], 0, "del removes a key."),
            copy("Type this.", "bag = {\"gold\": 5}\nbag[\"gold\"] += 10\nprint(bag)", "Prints {'gold': 15}."),
            recall("From memory: make stats = {}, set stats[\"hp\"] to 30 and print stats.", "stats = {}\nstats[\"hp\"] = 30\nprint(stats)", "Prints {'hp': 30}."),
        ]).with(r##"## Dictionaries can change
## Adding and updating
Assigning to a key creates it if it is new, or replaces the value if it already exists:

    hero = {"name": "Pix", "hp": 30}
    hero["hp"] = 25          # update
    hero["pet"] = "owl"      # add
    hero["hp"] -= 5          # update using the old value

## Removing
    del hero["pet"]          # delete a key
    hp = hero.pop("hp")      # delete and get the value back

Both cause an error if the key is missing; pop("key", default) avoids that.

## Merging
update() copies all pairs from another dictionary:

    hero.update({"level": 2, "gold": 50})

## Counting with a dictionary
Dictionaries make great tallies:

    votes = {}
    for v in ["cat", "owl", "cat"]:
        votes[v] = votes.get(v, 0) + 1
    print(votes)   # {'cat': 2, 'owl': 1}

get(v, 0) returns 0 the first time a key is seen, so the count starts from zero."##,
        &["dict[key] = value adds or updates a pair.", "del dict[key] removes a pair; pop(key) also returns the value.", "update() merges another dictionary in.", "dict.get(key, 0) + 1 builds a tally."]),
        l("Looping Dictionaries", &[
            (r#"Loop over keys:

for key in stats:
    print(key, stats[key])"#, "scroll_fire"),
            (r#"Or over pairs:

for key, value in stats.items():
    print(f"{key}: {value}")"#, "spellbook"),
        ], vec![
            quiz("What does for k in d go through?", &["The keys", "The values", "Nothing"], 0, "Looping a dict gives its keys."),
            quiz("What does d.items() give?", &["Key-value pairs", "Only keys", "Only values"], 0, "Pairs of key and value."),
            fill("Loop over the pairs.", "for k, v in stats.___():\n    print(k, v)", &["items", "pairs", "keys"], 0, ".items() gives pairs."),
            fill("Loop over the values only.", "for v in stats.___():\n    print(v)", &["values", "items", "keys"], 0, ".values() gives values."),
            copy("Type this.", "stats = {\"hp\": 30, \"mp\": 12}\nfor k, v in stats.items():\n    print(k, v)", "Two lines."),
            recall("From memory: loop over prices = {\"pie\": 3} with .items() and print each key and value.", "prices = {\"pie\": 3}\nfor k, v in prices.items():\n    print(k, v)", "Prints pie 3."),
        ]).with(r##"## Going through every entry
## Keys
Looping over a dictionary directly gives its keys:

    stats = {"hp": 30, "mp": 12, "speed": 7}
    for key in stats:
        print(key, stats[key])

## Values
    for value in stats.values():
        print(value)

## Both at once
items() gives each key together with its value, and the loop unpacks them into two variables:

    for name, value in stats.items():
        print(f"{name}: {value}")

This is the most common way to loop over a dictionary.

## Totals and searches
    total = 0
    for v in stats.values():
        total += v

    for name, value in stats.items():
        if value > 20:
            print(f"{name} is high")

## Order
Dictionaries remember the order in which keys were added, so loops go through them in that order.

## keys() and len()
stats.keys() gives the keys, and len(stats) tells you how many pairs there are."##,
        &["for key in d loops over the keys.", "d.values() gives the values.", "for k, v in d.items() gives both.", "len(d) counts the pairs."]),
        p("Project: Inventory", &[
            (r#"Track a hero's inventory as item: count pairs, add loot, use items and print a report."#, "chest"),
            (r#"inv = {"potion": 2, "arrow": 10}
inv["potion"] -= 1
inv["gem"] = 1
for item, n in inv.items():
    print(f"{item}: {n}")"#, "gem_green"),
        ], vec![
            quiz("Why use a dictionary for an inventory?", &["Each item name maps to its count", "Dictionaries are sorted", "Lists can't hold numbers"], 0, "Look up counts by name."),
            quiz("How do you add 5 arrows when arrow may be missing?", &["inv[\"arrow\"] = inv.get(\"arrow\", 0) + 5", "inv.arrow += 5", "inv[\"arrow\"] + 5"], 0, "get with a default handles missing keys."),
            fill("Use one potion.", "inv = {\"potion\": 2}\ninv[\"potion\"] ___ 1", &["-=", "+=", "="], 0, "-= subtracts."),
            fill("Print each item and count.", "for item, n in inv.___():\n    print(item, n)", &["items", "values", "keys"], 0, ".items() gives pairs."),
            copy("Type this.", "inv = {\"arrow\": 10}\ninv[\"arrow\"] -= 3\nprint(inv[\"arrow\"])", "Prints 7."),
            recall("From memory: make inv = {\"gem\": 1}, add 1 to inv[\"gem\"], print inv.", "inv = {\"gem\": 1}\ninv[\"gem\"] += 1\nprint(inv)", "Prints {'gem': 2}."),
        ]).with(r##"## A real inventory
Every adventure game has an inventory. A dictionary is perfect: item names are keys, counts are values.

## Plan
1. Start with a few items.
2. Pick up loot: add to existing counts, or create new items.
3. Use an item: subtract one, and remove it when the count reaches zero.
4. Print a tidy report.

    inv = {"potion": 2, "arrow": 10}

    def pick_up(item, n):
        inv[item] = inv.get(item, 0) + n

    def use(item):
        if inv.get(item, 0) > 0:
            inv[item] -= 1
            if inv[item] == 0:
                del inv[item]

    pick_up("gem", 1)
    use("potion")
    for item, count in inv.items():
        print(f"{item}: {count}")

## Things to notice
- get(item, 0) makes new items start at zero.
- Using an item you do not have does nothing instead of crashing.
- The functions change the dictionary created outside them, because dictionaries are shared, not copied."##,
        &["Map item names to counts in a dictionary.", "get(key, 0) handles items you don't have yet.", "Remove keys when a count reaches zero.", "Functions can update a dictionary passed to or shared with them."]),
    ])
}

fn classes() -> Module {
    ch("Classes and Objects", "Design your own types with classes, give objects attributes and methods.", vec![
        l("Classes", &[
            (r#"A class is a blueprint. Objects are things built from it.

class Slime:
    hp = 10

s = Slime()
print(s.hp)"#, "cauldron"),
            (r#"Each object has its own copy of the attributes:

a = Slime()
b = Slime()"#, "orb_red"),
        ], vec![
            quiz("What is a class?", &["A blueprint for making objects", "A kind of loop", "A list of numbers"], 0, "Classes describe objects."),
            quiz("How do you make an object from class Slime?", &["Slime()", "new Slime", "Slime.make"], 0, "Call the class like a function."),
            fill("Define a class.", "___ Dragon:\n    hp = 100", &["class", "def", "object"], 0, "class Name:"),
            fill("Read the attribute.", "class Dragon:\n    hp = 100\nd = Dragon()\nprint(d___hp)", &[".", ":", "->"], 0, "object.attribute."),
            copy("Type this.", "class Bat:\n    wings = 2\nb = Bat()\nprint(b.wings)", "Prints 2."),
            recall("From memory: class Cat with lives = 9; make c = Cat(); print c.lives.", "class Cat:\n    lives = 9\nc = Cat()\nprint(c.lives)", "Prints 9."),
        ]).with(r##"## Making your own types
Python has built-in types like str, list and dict. A class lets you create your own type, such as Hero, Slime or BankAccount.

## Blueprint and objects
A class is a blueprint; an object (also called an instance) is a thing built from it. One Slime class can produce many slimes.

    class Slime:
        hp = 10
        color = "green"

    s = Slime()
    print(s.hp)       # 10
    print(s.color)    # green

- class Slime: starts the class. Class names use CapitalWords by convention.
- The indented lines inside are its attributes: data each object has.
- Slime() creates a new object, like calling a function.

## Each object is separate
    a = Slime()
    b = Slime()
    a.hp = 3
    print(a.hp, b.hp)   # 3 10

Changing one slime does not change the other.

## type()
type(s) tells you the object's class: Slime.

## Why classes?
Classes keep data and the actions on that data together. Big programs, from games to websites, are organised around classes."##,
        &["A class is a blueprint; objects are built from it.", "class Name: defines it; Name() creates an object.", "Attributes are read with object.attribute.", "Each object has its own separate attributes."]),
        l("The __init__ Method", &[
            (r#"__init__ sets up each new object:

class Hero:
    def __init__(self, name):
        self.name = name

h = Hero("Pix")
print(h.name)"#, "helmet"),
            (r#"self means "this object". self.name = name stores the argument on the object."#, "helmet_knight"),
        ], vec![
            quiz("When does __init__ run?", &["When an object is created", "When the program ends", "Only when called by name"], 0, "Hero(...) runs __init__."),
            quiz("What does self refer to?", &["The object being created or used", "The class name", "The previous object"], 0, "self is this object."),
            fill("Name the setup method.", "class Hero:\n    def ___(self, name):\n        self.name = name", &["__init__", "init", "setup"], 0, "Two underscores on each side."),
            fill("Store the name on the object.", "def __init__(self, name):\n    ___.name = name", &["self", "this", "Hero"], 0, "self.attribute = value."),
            copy("Type this.", "class Pet:\n    def __init__(self, kind):\n        self.kind = kind\np = Pet(\"owl\")\nprint(p.kind)", "Prints owl."),
            recall("From memory: class Item with __init__(self, name) storing self.name; make Item(\"Key\") and print its name.", "class Item:\n    def __init__(self, name):\n        self.name = name\ni = Item(\"Key\")\nprint(i.name)", "Prints Key."),
        ]).with(r##"## Setting up each object
Most objects need their own starting values: every hero has a different name. The special method __init__ runs automatically every time an object is created.

    class Hero:
        def __init__(self, name, hp=30):
            self.name = name
            self.hp = hp

    pix = Hero("Pix")
    rook = Hero("Rook", 45)
    print(pix.name, pix.hp)     # Pix 30
    print(rook.name, rook.hp)   # Rook 45

## What is self?
self is the object being set up. self.name = name means "store the name argument on THIS object". Python passes self automatically: you write Hero("Pix"), not Hero(self, "Pix").

## The double underscores
__init__ has two underscores before and after. Methods with this shape are "special methods" that Python calls for you. Typing _init_ with single underscores is a common bug: the method never runs.

## Attributes versus parameters
name (the parameter) disappears when __init__ ends. self.name (the attribute) stays on the object for as long as the object exists.

## Defaults
Parameters of __init__ can have defaults, like hp=30 above, so common values do not need to be typed every time."##,
        &["__init__ runs automatically when an object is created.", "self is the object being set up or used.", "self.x = value stores an attribute on the object.", "Use two underscores on each side: __init__."]),
        l("Methods", &[
            (r#"Methods are functions inside a class:

class Hero:
    def __init__(self, name):
        self.name = name
    def greet(self):
        return f"I am {self.name}""#, "hat_wizard"),
            (r#"h = Hero("Pix")
print(h.greet())

Methods can change the object: self.hp -= 5"#, "staff"),
        ], vec![
            quiz("What is the first parameter of every method?", &["self", "this", "obj"], 0, "Methods take self first."),
            quiz("How do you call greet on hero h?", &["h.greet()", "greet(h)()", "Hero.greet"], 0, "object.method()."),
            fill("Define a method.", "class Bat:\n    ___ fly(self):\n        return \"flap\"", &["def", "method", "fun"], 0, "Methods are defined with def."),
            fill("Change the object's hp.", "def hit(self, dmg):\n    self.hp ___ dmg", &["-=", "=-", "=="], 0, "-= subtracts."),
            copy("Type this.", "class Orc:\n    def __init__(self):\n        self.hp = 20\n    def hit(self):\n        self.hp -= 5\no = Orc()\no.hit()\nprint(o.hp)", "Prints 15."),
            recall("From memory: class Dog with a method bark(self) returning \"Woof\"; print Dog().bark().", "class Dog:\n    def bark(self):\n        return \"Woof\"\nprint(Dog().bark())", "Prints Woof."),
        ]).with(r##"## Objects that can do things
Attributes are what an object HAS. Methods are what it can DO. A method is a function defined inside a class:

    class Hero:
        def __init__(self, name):
            self.name = name
            self.hp = 30

        def greet(self):
            return f"I am {self.name}!"

        def take_hit(self, damage):
            self.hp -= damage
            if self.hp <= 0:
                print(f"{self.name} falls!")

    pix = Hero("Pix")
    print(pix.greet())
    pix.take_hit(12)
    print(pix.hp)   # 18

## self again
Every method's first parameter is self, the object the method was called on. pix.take_hit(12) runs take_hit with self = pix and damage = 12.

## Methods change their own object
Because self is the object itself, self.hp -= damage changes pix's hit points and nobody else's.

## Methods calling methods
Inside a class, a method can call another with self.other_method().

## Why this matters
Keeping the data (hp) and the actions (take_hit) together means the rules live in one place: every hero takes damage the same way."##,
        &["Methods are functions defined inside a class.", "Every method takes self as its first parameter.", "Call them with object.method(arguments).", "Methods can read and change the object's attributes."]),
        p("Project: Bank Account", &[
            (r#"Build a BankAccount class for the guild bank: deposit, withdraw (if there is enough gold) and show the balance."#, "coin_gold"),
            (r#"class BankAccount:
    def __init__(self, owner, balance=0):
        self.owner = owner
        self.balance = balance"#, "money_bag"),
        ], vec![
            quiz("Where should the withdraw rule (enough gold?) live?", &["Inside the withdraw method", "In every place that withdraws", "In __init__"], 0, "Rules belong with the data."),
            quiz("Why give balance a default of 0?", &["New accounts start empty", "Defaults are required", "To hide it"], 0, "Common starting value."),
            fill("Add money to the balance.", "def deposit(self, amount):\n    self.balance ___ amount", &["+=", "-=", "="], 0, "+= adds."),
            fill("Only withdraw when there is enough.", "def withdraw(self, amount):\n    if amount ___ self.balance:\n        self.balance -= amount", &["<=", ">=", "=="], 0, "The amount must be at most the balance."),
            copy("Type this.", "class Account:\n    def __init__(self):\n        self.balance = 0\n    def deposit(self, n):\n        self.balance += n\na = Account()\na.deposit(50)\nprint(a.balance)", "Prints 50."),
            recall("From memory: class Wallet with __init__ setting self.gold = 10 and a method spend(self, n) that subtracts n; spend 3 and print gold.", "class Wallet:\n    def __init__(self):\n        self.gold = 10\n    def spend(self, n):\n        self.gold -= n\nw = Wallet()\nw.spend(3)\nprint(w.gold)", "Prints 7."),
        ]).with(r##"## A class with rules
The guild bank wants a program for its accounts. Each account has an owner and a balance, and money can only be withdrawn if there is enough.

## Build it
    class BankAccount:
        def __init__(self, owner, balance=0):
            self.owner = owner
            self.balance = balance

        def deposit(self, amount):
            self.balance += amount

        def withdraw(self, amount):
            if amount <= self.balance:
                self.balance -= amount
                return True
            print("Not enough gold!")
            return False

        def report(self):
            return f"{self.owner}: {self.balance} gold"

    acct = BankAccount("Pix")
    acct.deposit(100)
    acct.withdraw(30)
    acct.withdraw(500)
    print(acct.report())   # Pix: 70 gold

## What you practised
- __init__ with a default value.
- Methods that change the object's state.
- A rule (no overdrawing) kept inside the class, so it can never be skipped.
- Returning True/False so the caller knows whether it worked.

## Level up
Create two accounts and write a transfer that withdraws from one and deposits into the other only if the withdrawal succeeded."##,
        &["Keep data and the rules for changing it in one class.", "Methods can refuse invalid actions.", "Return True/False to report success.", "Several objects of one class each keep their own state."]),
    ])
}

fn modules() -> Module {
    ch("Modules", "Use Python's ready-made toolboxes: import math and random.", vec![
        l("Importing Modules", &[
            (r#"A module is a toolbox of ready-made code. import loads it:

import math
print(math.sqrt(16))"#, "gear"),
            (r#"Or take just one tool:

from math import sqrt
print(sqrt(16))"#, "hammer"),
        ], vec![
            quiz("What does import math do?", &["Loads the math module", "Does maths", "Deletes math"], 0, "import loads a module."),
            quiz("After import math, how do you call sqrt?", &["math.sqrt(9)", "sqrt.math(9)", "math(sqrt, 9)"], 0, "module.function()."),
            fill("Load the module.", "___ math\nprint(math.sqrt(9))", &["import", "include", "use"], 0, "import module."),
            fill("Take only sqrt.", "from math ___ sqrt\nprint(sqrt(9))", &["import", "use", "get"], 0, "from module import name."),
            copy("Type this.", "import math\nprint(math.sqrt(25))", "Prints 5."),
            recall("From memory: from math import sqrt, then print sqrt(49).", "from math import sqrt\nprint(sqrt(49))", "Prints 7."),
        ]).with(r##"## Toolboxes of code
Python comes with hundreds of modules: ready-made collections of functions for maths, dates, random numbers, files, the internet and more. You load one with import.

    import math
    print(math.sqrt(16))   # 4.0

After import math, everything in the module is reached with a dot: math.sqrt, math.pi, math.floor.

## Importing just what you need
    from math import sqrt, pi
    print(sqrt(16))
    print(pi)

Now you can use the names directly, without math. in front.

## Which style?
- import math keeps it clear where each name comes from.
- from math import sqrt is shorter when you use one function a lot.

## Where imports go
Put imports at the very top of your file, so readers can see at a glance which tools the program uses.

## Forgetting the import
Using math.sqrt without import math gives an error saying math is not defined. If you see that for a module name, check the top of your file.

## Beyond the standard library
Thousands more modules are written by other programmers (for games, data and websites) and can be installed with a tool called pip."##,
        &["A module is a toolbox of ready-made code.", "import math, then use math.name.", "from math import sqrt lets you use sqrt directly.", "Put imports at the top of the file."]),
        l("The math Module", &[
            (r#"math has maths tools:

math.sqrt(9)    3
math.floor(2.7) 2
math.ceil(2.1)  3
math.pi         3.14159..."#, "cpu"),
            (r#"Area of a circle:

import math
r = 3
print(math.pi * r ** 2)"#, "orb_red"),
        ], vec![
            quiz("What does math.ceil(2.1) give?", &["2", "3", "2.1"], 1, "ceil rounds up."),
            quiz("What does math.floor(2.9) give?", &["2", "3", "2.9"], 0, "floor rounds down."),
            fill("Round down.", "import math\nprint(math.___(4.8))", &["floor", "down", "ceil"], 0, "floor rounds down."),
            fill("Use pi.", "import math\nprint(math.___ * 2)", &["pi", "PI", "Pi"], 0, "math.pi in lowercase."),
            copy("Type this.", "import math\nprint(math.ceil(7 / 2))", "Prints 4."),
            recall("From memory: import math and print math.floor(9.99).", "import math\nprint(math.floor(9.99))", "Prints 9."),
        ]).with(r##"## Maths tools
The math module gives you functions and constants beyond + - * /.

    import math
    print(math.sqrt(81))    # 9.0    square root
    print(math.floor(2.7))  # 2      round down
    print(math.ceil(2.1))   # 3      round up
    print(math.pi)          # 3.14159...
    print(math.pow(2, 5))   # 32.0

## floor and ceil
- floor always goes DOWN to the whole number below.
- ceil (ceiling) always goes UP.
- round() goes to the NEAREST whole number.

ceil is perfect for "how many do I need": 10 players in boats of 4 need math.ceil(10 / 4) = 3 boats.

## pi
math.pi is the ratio of a circle's circumference to its diameter. Circle formulas:

    area = math.pi * r ** 2
    circumference = 2 * math.pi * r

## Square roots
math.sqrt(x) gives the number that, multiplied by itself, makes x. It is used in distance calculations: the distance between two points is math.sqrt(dx ** 2 + dy ** 2). sqrt of a negative number is an error."##,
        &["math.sqrt gives square roots.", "math.floor rounds down; math.ceil rounds up.", "math.pi is about 3.14159.", "ceil answers 'how many do I need' questions."]),
        l("The random Module", &[
            (r#"random adds chance:

import random
random.randint(1, 6)   a dice roll
random.choice(list)    a random item"#, "orb_red"),
            (r#"loot = ["gold", "gem", "potion"]
print(random.choice(loot))"#, "chest"),
        ], vec![
            quiz("Can random.randint(1, 6) return 6?", &["Yes", "No"], 0, "randint includes both ends."),
            quiz("What does random.choice([\"a\", \"b\"]) return?", &["One random item", "Both items", "A number"], 0, "It picks one item."),
            fill("Roll a dice.", "import random\nprint(random.___(1, 6))", &["randint", "roll", "range"], 0, "randint(a, b)."),
            fill("Pick a random item.", "import random\nprint(random.___([\"x\", \"y\"]))", &["choice", "pick", "item"], 0, "choice(list)."),
            copy("Type this.", "import random\nroll = random.randint(1, 20)\nprint(roll >= 1)", "Always True."),
            recall("From memory: import random and print random.choice([\"heads\", \"tails\"]).", "import random\nprint(random.choice([\"heads\", \"tails\"]))", "A coin flip."),
        ]).with(r##"## Adding chance
Games need luck: dice, loot drops, critical hits. The random module provides it.

    import random
    print(random.randint(1, 6))        # 1 to 6, both included
    print(random.choice(["gold", "gem", "potion"]))
    print(random.random())             # a decimal from 0 up to 1

## randint includes both ends
Unlike range(), random.randint(1, 6) CAN return 6. That makes it exactly like a dice.

## Chances and percentages
To do something 25% of the time:

    if random.random() < 0.25:
        print("Critical hit!")

## Random but testable
Random code gives different results each run, which makes testing tricky. Run it many times and check that every outcome can happen and that nothing impossible happens. For example, check that a roll is always between 1 and 6.

## Choosing fairly
random.choice gives every item the same chance. To make something rarer, put it in the list fewer times than the others, or use a random number with thresholds:

    r = random.randint(1, 100)
    if r <= 5:
        print("Legendary!")"##,
        &["random.randint(a, b) includes both a and b.", "random.choice(list) picks one item fairly.", "random.random() < 0.25 happens about 25% of the time.", "Test random code by running it many times."]),
        p("Final Project: Magic 8-Ball", &[
            (r#"Build a fortune teller: store answers in a list, pick one at random, and print the question with its answer."#, "mystic_figure"),
            (r#"import random
answers = ["Yes", "No", "Maybe"]
print(random.choice(answers))"#, "crystal_purple"),
        ], vec![
            quiz("Which module picks the answer?", &["random", "math", "time"], 0, "Chance comes from random."),
            quiz("Why keep answers in a list?", &["Easy to add answers and pick one with choice", "Lists are required", "To sort them"], 0, "random.choice works on lists."),
            fill("Load the module.", "___ random\nprint(random.choice([\"Yes\"]))", &["import", "from", "load"], 0, "import random."),
            fill("Pick one answer.", "import random\nanswers = [\"Yes\", \"No\"]\nprint(random.___(answers))", &["choice", "randint", "pick"], 0, "choice picks an item."),
            copy("Type this.", "import random\nanswers = [\"Yes\", \"No\", \"Ask again\"]\nprint(random.choice(answers))", "A random answer."),
            recall("From memory: import random, make answers = [\"Yes\", \"No\"], print a random choice.", "import random\nanswers = [\"Yes\", \"No\"]\nprint(random.choice(answers))", "random.choice(answers)."),
        ]).with(r##"## The oracle of the guild
Your final Python project is a Magic 8-Ball that answers any yes/no question. It brings together modules, lists, functions and f-strings.

## Build it
    import random

    answers = [
        "It is certain.",
        "The stars say no.",
        "Ask again after supper.",
        "Signs point to yes.",
        "Very doubtful.",
    ]

    def ask(question):
        answer = random.choice(answers)
        return f"{question} -> {answer}"

    print(ask("Will I find the dragon?"))
    print(ask("Is the tavern open?"))

## What you combined
- import random for chance.
- A list of answers that is easy to extend.
- A function with a parameter and a return value.
- An f-string for the final message.

## Level up
- Make rare answers: add common answers twice so the rare one appears less often.
- Count how often each answer appears in 100 asks using a dictionary.
- Wrap it in a class Oracle with a method ask.

You have finished the Python course. Every program you write from now on uses these same building blocks."##,
        &["import random and use random.choice for the answer.", "Keep the answers in a list so they are easy to change.", "A function with a parameter and return makes it reusable.", "Everything you learned works together in one program."]),
    ])
}
