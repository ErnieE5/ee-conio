//!
//!

#![cfg_attr(feature = "doc-images",
cfg_attr(all(),
doc = ::embed_doc_image::embed_image!("hello_world", "../screenshots/hello_world.png"),
doc = ::embed_doc_image::embed_image!("vivid_red", "../screenshots/vivid_red_and_friends.png"),

))]
#![cfg_attr(
not(feature = "doc-images"),
doc = "**Doc images not enabled**. Compile with feature `doc-images` and Rust version >= 1.54 \
           to enable."
)]
//!
/*!
Library for "more intuitive"[^sub] [ANSI escape sequences][ansi] in
console output.

# Quick Start

```rust
use ee_conio::cprintln;
cprintln!("~[c51 C0]Hello~[c7], ~[c227]World~[c197]!");
```
![hello_world]


# Why?
`\u{1b}[38;2;247;13;26m`[^oof] is one way to change the foreground to
[`Vivid Red`](https://en.wikipedia.org/wiki/List_of_colors_(alphabetical)).
Other more compact red variants such as `\x1b[38;5;196m` or `\x1b[31m`
are hard to decipher as well[^sub].

This library makes adding escapes to output easier to reconcile[^sub].  The
examples above can be automatically inserted into static literals with this
library. Each call with the `cprintln!` macro below will emit a line of text
in red[^modern] with a black background.
```rust
use ee_conio::cprintln;
cprintln!("~[C0 #'Vivid Red']This is Vivid Red.        ");
cprintln!("~[C0 #F70D1A     ]This is also Vivid Red.   ");
cprintln!("~[C0 c196        ]8bit red color.           ");
cprintln!("~[C0 x31         ]4bit red color.           ");
```
![vivid_red][vivid_red]

During __compile__, this gets (effectively) expanded to:
```rust
println!("\u{1b}[48;5;0m\u{1b}[38;2;247;13;26mThis is Vivid Red.        \u{1b}[0m");
println!("\u{1b}[48;5;0m\u{1b}[38;2;247;13;26mThis is also Vivid Red.   \u{1b}[0m");
println!("\u{1b}[48;5;0m\u{1b}[38;5;196m8bit red color.           \u{1b}[0m");
println!("\u{1b}[48;5;0m\u{1b}[31m4bit red color.           \u{1b}[0m");
```
This library isn't for you if the "mess" above is something you enjoy seeing or
typing.


## Verify for yourself from the ee-conio repo
[cargo-expand ![Crates.io](https://img.shields.io/crates/v/cargo-expand.svg)](https://crates.io/crates/cargo-expand/)
```bash
$ #cargo install cargo-expand
$ cargo expand --example document_screenshots
```


# Notes


`~[]` is the marker for content.  When this pattern is found, it
will be replaced. This is either generated content or nothing.

Therefore the following code will not trigger an assert:
```
use ee_conio_macro::ctransform;
let x = "";
ctransform!(
let y = "~[      ]~[]~[     ]";
assert_eq!( x, y );
assert_eq!( y, "~[ ]" );
);
let z = "";
assert_eq!( y, z );
```
[ctransform!](ctransform) transforms ANY string literal inside the macro block
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

The following code WILL NOT compile:
```{rust, eval=FALSE}
let q = ctransform!("~[{}]","");

error: '{}' does not match known keywords, names, or mnemonics
   --> examples\smorgasbord.rs:137:24
    |
137 | let q = ctransform!("~[{}]","");
    |                        ^^
```

"Behind the scenes" you can think of `ctransform!` as the engine. The following
code is functionally identical.
```
use ee_conio_macro::{cprint,ctransform};
let x = "Woo!";
ctransform!( print!("~[  ]{x}~[  ]") );
cprint!("~[  ]{x}~[  ]");
```

cprintln! is slightly different.  Because MOST codes are likely to be SGR
related, an SGR 0 is appended before the newline if any replacements are found.
This "turns off" any changes before the end of the line.

```rust
use ee_conio_macro::{cprint,cprintln};
cprintln!("~[c227 C0]Bright Yellow text on a black background!");
cprint!(  "~[c227 C0]Bright Yellow text on a black background!~[x0]\n");
```

cprintln!<br>
cprint!<br>
cformat!<br>
cwrite!<br>
cwriteln!<br>


```rust
use ee_conio_macro::cprintln;
cprintln!("~[white BLUE]White text on a blue background.");
```




# Macro/Function escape primitives.
The macros/functions supporting sequences in [ee_conio] are thin wrappers
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
(and [friends](../ee_conio_macro/index.html)).



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



[ansi]:<https://en.wikipedia.org/wiki/ANSI_escape_code>
[sgr]:<https://en.wikipedia.org/wiki/ANSI_escape_code#Select_Graphic_Rendition_parameters>
[csi]:<https://en.wikipedia.org/wiki/ANSI_escape_code#Control_Sequence_Introducer_commands>
[8bit]:<https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit>
[24bit]:<https://en.wikipedia.org/wiki/ANSI_escape_code#24-bit>
[named]:<https://en.wikipedia.org/wiki/List_of_colors_(alphabetical)>
[^sub]: This is a highly subjective statement. You may disagree.
[^modern]: ANSI/VT100 escapes have been around for a long time. Support for many color and cursor options is 'new' to many "modern" terminals.
[^oof]: Do I really need to type more?

*/



pub use ee_conio_engine::{
    ansi_escape::{bg_color_256, bg_color_rgb, csi_sequence, fg_color_256, fg_color_rgb, sgr_code},
    bg_256, bg_rgb, csi, fg_256, fg_rgb, sgr,
};

pub use ::ee_conio_macro::{cformat, cprint, cprintln, ctransform, cwrite, cwriteln};
