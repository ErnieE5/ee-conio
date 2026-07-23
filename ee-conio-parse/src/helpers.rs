use std::ops::Range;

/// Something inside a `~[...]` marker could not be resolved.
///
/// Carries the byte range of the offending characters relative to the string
/// literal being transformed, which is what lets the proc macro underline them
/// rather than the whole literal.
///
/// The fields are private on purpose: this type is part of a published API and
/// the representation is not something to freeze.  Use [`msg`](Self::msg) and
/// [`span`](Self::span).
#[derive(Debug, Clone)]
pub struct ParseError {
    /// Breadcrumb trail of the functions this error passed through, built up
    /// by [`wrap`](Self::wrap).  Debugging aid for this crate only, so it is
    /// private and shows up solely in the `Debug` output.
    origin: String,
    msg: String,
    start: usize,
    end: usize,
}

impl ParseError {
    pub(crate) fn new(origin: &str, msg: String, s: usize, e: usize) -> ParseError {
        ParseError {
            origin: origin.to_string(),
            msg,
            start: s,
            end: e,
        }
    }

    /// Re-base onto a new byte range, noting where it passed through.  The
    /// message is carried over unchanged.
    pub(crate) fn wrap(&self, origin: &str, s: usize, e: usize) -> ParseError {
        ParseError {
            origin: format!("{} -> {}", self.origin, origin),
            msg: self.msg.clone(),
            start: s,
            end: e,
        }
    }

    /// What went wrong.  Same text as the [`Display`](std::fmt::Display) impl.
    pub fn msg(&self) -> &str {
        &self.msg
    }

    /// Byte range of the offending characters within the literal.
    pub fn span(&self) -> Range<usize> {
        self.start..self.end
    }

    /// Byte offset of the first offending character.
    pub fn start(&self) -> usize {
        self.start
    }

    /// Byte offset one past the last offending character.
    pub fn end(&self) -> usize {
        self.end
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for ParseError {}

pub fn r_g_b_from_string(s: &str, p: &str, o: &str) -> Result<(u32, u32, u32), ParseError> {
    if s.len() != 7 {
        let msg = format!("got value '{s}' expected six hex digits following '{p}'");
        return Err(ParseError::new(o, msg, 0, s.len() - 1));
    }
    if s[0..1] != *p {
        let msg = format!("got value '{s}' expected prefix of '{p}'");
        return Err(ParseError::new(o, msg, 0, 1));
    }

    let Ok(r) = u32::from_str_radix(&s[1..3], 16) else {
        let msg = format!("'{s}' is invalid at '{p}!!XXXX'");
        return Err(ParseError::new(o, msg, 1, 2));
    };
    let Ok(g) = u32::from_str_radix(&s[3..5], 16) else {
        let msg = format!("'{s}' is invalid at '{p}XX!!XX'");
        return Err(ParseError::new(o, msg, 3, 4));
    };
    let Ok(b) = u32::from_str_radix(&s[5..7], 16) else {
        let msg = format!("'{s}' is invalid at '{p}XXXX!!'");
        return Err(ParseError::new(o, msg, 5, 6));
    };

    Ok((r, g, b))
}
