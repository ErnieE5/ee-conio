#![doc(html_playground_url = "https://play.rust-lang.org/")]
/*!
Library for "more intuitive"[^sub] [ANSI escape sequences][ansi] in
console output.

This library is in two parts. This part is mostly the "run time" module. The
other is "compile time."



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
