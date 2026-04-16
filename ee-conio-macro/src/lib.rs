/*!
Compile time macros that replace "human understandable" mnemonic shortcuts
with messy [ANSI escape codes] in literals during compilation.

All of the `c` prefixed macros modify only `"String Literals"` within the body of the macro.

# Quick Start

```rust
use ee_conio_macro::cprintln;

cprintln!("~[c51]Hello~[x0], ~[c227]World~[c197]!");
```

# Why?
`\u{1b}[38;2;247;13;26m`[^oof] is one way to change the foreground to
[`Vivid Red`](https://en.wikipedia.org/wiki/List_of_colors_(alphabetical)).
Other more compact red variants such as `\x1b[38;5;196m` or `\x1b[1m`
are hard to decipher as well[^sub].

This library makes adding escapes to output easier to reconcile[^sub].  The
examples above can be automatically inserted into static literals with this
library. Each call with the `cprintln!` macro below will emit a line of text
in red[^modern].
```rust
use ee_conio_macro::cprintln;
cprintln!("~[#'Vivid Red']This is Vivid Red.");
cprintln!("~[#F70D1A     ]This is also Vivid Red.");
cprintln!("~[c196        ]8bit red color.");
cprintln!("~[x1          ]4bit red color.");
```
During compile, this gets expanded to:
```rust
println!("\u{1b}[38;2;247;13;26mThis is Vivid Red.\u{1b}[0m");
println!("\u{1b}[38;2;247;13;26mThis is also Vivid Red.\u{1b}[0m");
println!("\u{1b}[38;5;196m8bit red color.\u{1b}[0m");
println!("\u{1b}[1m4bit red color.\u{1b}[0m");
```
This library isn't for you if the "mess" above is something you enjoy seeing or
typing.




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


```
use ee_conio_macro::cprintln;
cprintln!("~[white BLUE]White text on a blue background.");
```

[ANSI escape codes]: https://en.wikipedia.org/wiki/ANSI_escape_code
*/

#[doc(hidden)]
mod eeimpl;

#[doc(hidden)]
use proc_macro::TokenStream;

#[doc(hidden)]
use syn::parse_macro_input as pmi;

#[doc(hidden)]
use eeimpl::{EndWith, remap_token_stream};

/// ```
/// use ee_conio_macro::cprintln;
///
/// cprintln!("~[white BLUE]White text on a blue background.");
/// ```
#[proc_macro]
pub fn cprintln(input: TokenStream) -> TokenStream {
    remap_token_stream(Some("println"), pmi!(input), EndWith::Reset)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

/// ```
/// use ee_conio_macro::cprint;
///
/// cprint!("~[underline]Hey~[under_off] now.");
/// ```
#[proc_macro]
pub fn cprint(input: TokenStream) -> TokenStream {
    remap_token_stream(Some("print"), pmi!(input), EndWith::Nothing)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

/// ```
/// use ee_conio_macro::cformat;
///
/// let t = cformat!("~[c227]Some yellow foreground text.~[x0]");
/// ```
#[proc_macro]
pub fn cformat(input: TokenStream) -> TokenStream {
    remap_token_stream(Some("format"), pmi!(input), EndWith::Nothing)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

/// ```
/// use std::io::{self, Write};
/// use ee_conio_macro::cwrite;
///
/// cwrite!(io::stdout(),"~[c227]Hello~[c7], ~[c51]{}~[c196]!~[x0]","World");
/// ```
#[proc_macro]
pub fn cwrite(input: TokenStream) -> TokenStream {
    remap_token_stream(Some("write"), pmi!(input), EndWith::Nothing)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

/// ```
/// use std::io::{self, Write};
/// use ee_conio_macro::cwriteln;
///
/// cwriteln!(io::stdout(),"~[c227]Hello~[c7], {}~[c196]!~[x0]","~[c51]World");
/// ```
#[proc_macro]
pub fn cwriteln(input: TokenStream) -> TokenStream {
    remap_token_stream(Some("writeln"), pmi!(input), EndWith::Reset)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

/// ```
/// use ee_conio_macro::{cformat,ctransform};
///
/// ctransform!(
/// let u = "~[underline]";
/// let t = "~[dhtop]";
/// let b = "~[dhbot]";
/// let e = "~[                                      ]";
///);
/// assert_eq!( u, cformat!("~[underline]") );
/// assert_eq!( t, cformat!("~[dhtop]") );
/// assert_eq!( b, cformat!("~[dhbot]") );
/// assert_eq!( e, "" );
/// ```
#[proc_macro]
pub fn ctransform(input: TokenStream) -> TokenStream {
    remap_token_stream(None, pmi!(input), EndWith::Nothing)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
