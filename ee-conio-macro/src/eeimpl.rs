#[doc(hidden)]
use proc_macro2::{
    Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree,
    TokenTree::{Group as TTGroup, Ident as TTIdent, Literal as TTLiteral, Punct as TTPunct},
};

#[doc(hidden)]
use syn::{Error, LitStr, Result};

#[doc(hidden)]
use ee_conio::{find_replacement_patterns, sgr};

#[doc(hidden)]
#[derive(PartialEq)]
pub enum EndWith {
    Nothing,
    Reset,
}

// TODO
// replace when stable
// procmacro2_semver_exempt
// RUSTFLAGS="--cfg procmacro2_semver_exempt"
// let mut literal:String = l.str_value().expect("unable to extract raw data from Literal");

use rustc_literal_escaper::unescape_str;

fn unescape_literal(l: &Literal) -> String {
    let lit = LitStr::new(&l.to_string(), l.span()).value();
    let mut buf = String::with_capacity(lit.len());
    unescape_str(&lit, |_, c| {
        if let Ok(c) = c {
            buf.push(c)
        }
    });
    buf
}

// Recursive call to handle any Literal that contains our "magic sauce" that
// needs replacing.
fn process_stream(input: TokenStream, at_end: fn(String) -> String) -> Result<Vec<TokenTree>> {
    let mut tokens: Vec<TokenTree> = vec![];

    for input_token in input {
        match input_token {
            TTLiteral(ref l) => {
                let mut literal = unescape_literal(l);

                // For every ~[...] collect the sequences to replace
                let replacement_patterns = match find_replacement_patterns(&literal) {
                    Ok(r) => r,
                    Err(e) => {
                        let ss = match l.subspan(e.start + 1..e.end + 1) {
                            Some(i) => i,
                            None => l.span(),
                        };
                        return Err(Error::new(ss, e.msg.to_string()));
                    }
                };

                if !replacement_patterns.is_empty() {
                    for (v, r) in replacement_patterns {
                        literal = literal.replace(v.as_str(), r.join("").as_str());
                    }

                    if tokens.is_empty() {
                        literal = at_end(literal);
                    }

                    // Create new Literal with the modifications
                    tokens.push(TTLiteral(Literal::string(literal.as_str())));
                } else {
                    // Just keep on keeping on, we didn't change anything
                    tokens.push(TTLiteral(l.clone()));
                }
            }

            TTPunct(p) => {
                tokens.push(TTPunct(p));
            }

            TTGroup(g) => {
                fn noop(x: String) -> String {
                    x
                }

                // Recurse to look for other stuff we can mess with
                match process_stream(g.stream(), noop) {
                    Ok(r) => {
                        tokens.push(TTGroup(Group::new(g.delimiter(), r.into_iter().collect())));
                    }
                    Err(e) => {
                        return Err(Error::new(e.span(), format!("{}", e)));
                    }
                }
            }

            TTIdent(i) => {
                tokens.push(TTIdent(i));
            }
        }
    }

    Ok(tokens)
}

/// Takes the TokenStream from within a macro that "looks like" a format type
/// of output and modifies any literals with "special" sequences
/// that tell this method to emit ANSI escape sequences.
pub fn remap_token_stream(
    wrap: Option<&str>,
    input: TokenStream,
    at_end: EndWith,
) -> Result<TokenStream> {
    // If any CGI codes are added an SGR reset is appended
    // to the end of the format string.
    fn doit(x: String) -> String {
        x + sgr!(0)
    }
    // Just keep on keeping on..
    fn noop(x: String) -> String {
        x
    }

    let term_first_literal = match at_end {
        EndWith::Reset => doit,
        EndWith::Nothing => noop,
    };

    // "Cook" any of the sub tokens and then
    // gather all of them into a new stream to pass
    // back as if we didn't do anything.
    match process_stream(input, term_first_literal) {
        Ok(r) => Ok(match wrap {
            Some(w) => {
                vec![
                    TTIdent(Ident::new(w, Span::call_site())),
                    TTPunct(Punct::new('!', Spacing::Alone)),
                    TTGroup(Group::new(Delimiter::Parenthesis, r.into_iter().collect())),
                ]
            }
            None => r,
        }
        .into_iter()
        .collect()),

        Err(e) => Err(e),
    }
}
