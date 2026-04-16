/*!
Compile time macros that replace "human understandable" mnemonic shortcuts
with messy [ANSI escape codes] in literals during compilation.

All of the `c` prefixed macros modify only `"String Literals"` within the body of the macro.

This library is exposed by ee-conio.
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
