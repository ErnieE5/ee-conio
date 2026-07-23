/*!
Library for more intuitive[^sub] [ANSI escape sequences][ansi] in
console output.

__The documentation is a WIP while the library is in Alpha.__

# Quick Start

```rust
use ee_conio::cprintln;
cprintln!("~[c51 C0]Hello~[c7], ~[c227]World~[c197]!");
```
![hello_world](https://raw.githubusercontent.com/ErnieE5/ee-conio/refs/heads/main/screenshots/hello_world.png)


# Why?
`\u{1b}[38;2;247;13;26m`[^oof] is one way to change the foreground to
[`Vivid Red`](https://en.wikipedia.org/wiki/List_of_colors_(alphabetical)).
Other more compact red variants such as `\x1b[38;5;196m` or `\x1b[31m`
are hard to decipher as well[^sub].

This library makes adding escapes to output easier to reconcile[^sub].  The
examples above can be automatically inserted into __static literals__ with this
library. Each use of `cprintln!` macro below will emit a line of text
in red with a black background[^modern].
```rust
use ee_conio::cprintln;
cprintln!("~[C0 #'Vivid Red']This is Vivid Red.        ");
cprintln!("~[C0 #F70D1A     ]This is also Vivid Red.   ");
cprintln!("~[C0 c196        ]8bit red color.           ");
cprintln!("~[C0 x31         ]4bit red color.           ");
```
![vivid_red](https://raw.githubusercontent.com/ErnieE5/ee-conio/refs/heads/main/screenshots/vivid_red_and_friends.png)

During __compile__, this gets (effectively) expanded to:
```rust
println!("\u{1b}[48;5;0m\u{1b}[38;2;247;13;26mThis is Vivid Red.        \u{1b}[0m");
println!("\u{1b}[48;5;0m\u{1b}[38;2;247;13;26mThis is also Vivid Red.   \u{1b}[0m");
println!("\u{1b}[48;5;0m\u{1b}[38;5;196m8bit red color.           \u{1b}[0m");
println!("\u{1b}[48;5;0m\u{1b}[31m4bit red color.           \u{1b}[0m");
```
This library isn't for you if the "mess" above is something you enjoy seeing or
typing.


## Verify for yourself: [<img alt="github" style="vertical-align:middle" src="https://img.shields.io/badge/github-ErnieE5/ee--conio-2B60DE?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/ErnieE5/ee-conio)

```bash
#cargo install cargo-expand
#cargo run --example document_screenshots
#cd ee-conio/ee-conio
cargo expand --example document_screenshots
```


# ~[]

`~[]` is the marker for content.  When this pattern is found, it
will be replaced. This is either generated content or nothing.

Therefore the following code __will not__ trigger an assert:
```
use ee_conio::ctransform;
let x = "";
ctransform!(
let y = "~[      ]~[]~[     ]";
assert_eq!( x, y );
assert_eq!( y, "~[ ]" );
);
let z = "";
assert_eq!( y, z );
```
[ctransform!](ctransform) transforms __ANY__ string literal inside the macro block
leaving all other code as is.  `y` and `z` above are in the same scope after the
macro is finished.

After the macro runs this is how the code is left:
```
let x = "";
let y = "";
assert_eq!( x, y );
assert_eq!( y, "" );
let z = "";
assert_eq!( y, z );
```

The following code __will not__ compile.  This block is a `compile_fail`
doctest, so `cargo test` asserts that it is still rejected:
```compile_fail
use ee_conio::ctransform;
let q = ctransform!("~[{}]");
```
and is rejected like this:
```text
error: '{}' does not match known keywords, names, or mnemonics
 --> src\main.rs:4:28
  |
4 |     let q = ctransform!("~[{}]");
  |                            ^^
```
Please note that the accurate identification of the exact location above
currently requires a nightly build[^see].

# ctransform and friends

"Behind the scenes" you can think of `ctransform!` as the engine. The following
code is functionally identical for the last two lines.
```
use ee_conio::{cprint,ctransform};
let x = "Woo!";
ctransform!( print!("~[  ]{x}~[  ]") );
cprint!("~[  ]{x}~[  ]");
```

`cprintln!` is slightly different.  Because MOST codes are likely to be SGR
related, an SGR 0 is appended before the newline if any replacements are found.
This "turns off" any changes before the end of the line.

```rust
ee_conio::cprintln!("~[c227 C0]Bright Yellow text on a black background!");
ee_conio::cprint!(  "~[c227 C0]Bright Yellow text on a black background!~[x0]\n");
```
![bright_yellow](https://raw.githubusercontent.com/ErnieE5/ee-conio/refs/heads/main/screenshots/bright_yellow_black_bg.png)

cprintln!<br>
cprint!<br>
cformat!<br>
cwrite!<br>
cwriteln!<br>


```rust
use ee_conio::cprintln;
cprintln!("~[white BLUE]White text on a blue background.");
```




# Macro/Function escape primitives.
The macros/functions supporting sequences in [ee_conio](crate) are thin wrappers
for [ANSI Escape Sequences][ansi]. They help simplify creation, but are not
much of an abstraction beyond that.

| Macro[^m]              | Function[^f]                 | Mnemonic  | Expansion                    | Description                |
| ------                 | --------                     | --------  |--------------                | -----------                |
| [esc!]\()              |                              |           |`"\u{1b}"`                    | Escape Literal             |
| [csi!]\("K")           | [csi_sequence]\("K")         | XK        |`"\u{1b}[K"`                  | Control Sequence Introducer|
| [sgr!]\(0)             | [sgr_code]\(0)               | x0        |`"\u{1b}[0m"`                 | Select Graphic Rendition   |
| [fg_256!]\(227)        | [fg_color_256]\(227)         | c227      |`"\u{1b}[38;5;227m"`          | Foreground 8bit color      |
| [bg_256!]\(196)        | [bg_color_256]\(196)         | C196      |`"\u{1b}[48;5;196m"`          | Background 8bit color      |
| [fg_rgb!]\(255,0,255)  | [fg_color_rgb]\(255,0,255)   | #FF00FF   |`"\u{1b}[38;2;255;0;255m"`    | Foreground 24bit RGB       |
| [bg_rgb!]\(0,0,0)      | [fg_color_rgb]\(0,0,0)       | $000000   |`"\u{1b}[48;2;0;0;0m"`        | Background 24bit RGB       |
[^m]:Macros "cook" into `&'static str`.
[^f]:Functions return a String.


Most of how I use this library is with mnemonic expansions in `cprintln!`
(and [friends](../ee_conio/index.html)).



# Mnemonics
Mnemonics are shorthand for escape sequences that help hide the syntax, but
do VERY little to hide requirement for understanding what they are and how they
should be used.

| Mnemonic      | values             | Example(s)                   | Description |
| -             | -                  | -                         | -           |
| `x{code}`     | 0..108             | `x0`                      | [Select Graphic Rendition](`sgr`) |
| `c{code}`     | 0..256             | `c227`                    | [8bit][8bit] Foreground color   |
| `C{code}`     | 0..256             | `C196`                    | [8bit][8bit] Background color   |
| `#XXXXXX`     | 6 Digit Hex Value  | `#39FF14`                 | [24bit][24bit] Foreground color   |
| `$XXXXXX`     | 6 Digit Hex Value  | `$87421F`                 | [24bit][24bit] Background color   |
| `#'Literal'`  | String Literal   | `#'Neon Tangerine'`       | [Named][named] Foreground color   |
| `$'Literal'`  | String Literal   | `$'Psychedelic Purple'`   | [Named][named] Background color   |
| `X{seq_str}`  | String Literal   | `X2J`,`XK`,`X5;5H`        | [CSI][csi] Clear Screen, Clear to End of Line, Position cursor row 5 column 5|

# Keywords
Keywords take it once step further and try to encode the intent.  An example of
a useful keyword is:
 `~[underline]` == `x4`  == `\x1b[4m`
 `~[under_off]` == `x24` == `\x1b[24m`

# Known limitations

__Colors cannot be turned off at run time.__ The escapes are baked into the
string literals while the program is being compiled, so by the time it runs
there is nothing left to strip. [`NO_COLOR`][no_color], `CLICOLOR`, a
`--color=never` flag, and "is stdout actually a terminal" checks are __not__
implemented, and the macros as they stand cannot honor them. If your output
may be redirected to a file or piped into another program, this library will
put escape sequences there. If that matters for your use, a run time styling
crate is the better fit today.

## Stripping escapes downstream

Filter the output instead. Note that the usual one line recipe found on the
internet matches __CSI only__ and will leave some of this library's output
behind, because [`~[dhtop]`](#keywords) and friends emit `ESC #n`, which is
not a CSI sequence.

Everything `ee_conio` can emit is one of three shapes:

| Shape                              | Emitted by                                  |
| ---------------------------------- | ------------------------------------------- |
| `ESC [` params intermediates final | every color, [`sgr!`], [`csi!`], `~[x0]`, `~[X2J]`, … |
| `ESC #` digit                      | `~[dhtop]` `~[dhbot]` `~[swsh]` `~[dwsh]`    |
| a lone `ESC`                       | [`esc!`] used on its own                     |

Recipes that cover all three:

```bash
# perl -- the most portable of these
your_program | perl -pe 's{\e\[[0-9:;<=>?]*[ -/]*[@-~]}{}g; s{\e#[0-9]}{}g; s{\e}{}g'

# GNU sed
your_program | sed -E 's,\x1B\[[0-9:;<=>?]*[ -/]*[@-~],,g; s,\x1B#[0-9],,g; s,\x1B,,g'
```

```powershell
# PowerShell 7 (`e is ESC)
your_program | ForEach-Object {
    $_ -replace "`e\[[0-9:;<=>?]*[ -/]*[@-~]", '' -replace "`e#[0-9]", '' -replace "`e", ''
}
```

`ansifilter` and `ansi2txt` are ready made alternatives, though it is worth
checking that whichever you pick handles `ESC #n` and not just CSI.

Two things these deliberately do __not__ do. A lone `ESC` from [`esc!`] loses
only the escape byte itself, since the library has no idea what you meant to
compose after it. And the emoji keywords (`~[heart]`, `~[poo]`, …) are
ordinary characters rather than escapes, so they survive stripping — which is
usually what you want in a log, but is worth knowing.

__Exact error locations require a nightly compiler.__ Parse errors always
carry a correct message and offset, but placing the underline on the offending
characters relies on [`proc_macro2::Literal::subspan`], which yields nothing
on stable. On stable the error is attributed to the whole string literal
instead[^see].

__The API is unstable.__ This is an alpha. Names and behavior may change in
any release; see the [CHANGELOG][changelog].

[no_color]:<https://no-color.org/>
[changelog]:<https://github.com/ErnieE5/ee-conio/blob/main/CHANGELOG.md>
[`proc_macro2::Literal::subspan`]:<https://docs.rs/proc-macro2/latest/proc_macro2/struct.Literal.html#method.subspan>



[ansi]:<https://en.wikipedia.org/wiki/ANSI_escape_code>
[sgr]:<https://en.wikipedia.org/wiki/ANSI_escape_code#Select_Graphic_Rendition_parameters>
[csi]:<https://en.wikipedia.org/wiki/ANSI_escape_code#Control_Sequence_Introducer_commands>
[8bit]:<https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit>
[24bit]:<https://en.wikipedia.org/wiki/ANSI_escape_code#24-bit>
[named]:<https://en.wikipedia.org/wiki/List_of_colors_(alphabetical)>
[^sub]: This is a highly subjective statement. You may disagree.
[^modern]: ANSI/VT100 escapes have been around for very long time. Support for many color and cursor options is 'new' to many "modern" terminals.
[^oof]: Do I really need to type more?
[^see]: 4/2026 [proc-macro2::subspan](https://docs.rs/proc-macro2/latest/proc_macro2/struct.Literal.html#method.subspan)

*/

pub use ee_conio_engine::{
    ansi_escape::{bg_color_256, bg_color_rgb, csi_sequence, fg_color_256, fg_color_rgb, sgr_code},
    bg_256, bg_rgb, csi, esc, fg_256, fg_rgb, sgr,
};

pub use ::ee_conio_macro::{cformat, cprint, cprintln, ctransform, cwrite, cwriteln};
