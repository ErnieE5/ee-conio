#[derive(Debug, Clone)]
pub struct ParseError {
    pub origin: String,
    pub msg: String,
    pub start: usize,
    pub end: usize,
}

impl ParseError {
    pub fn new(origin: &str, msg: String, s: usize, e: usize) -> ParseError {
        ParseError {
            origin: origin.to_string(),
            msg: msg.clone(),
            start: s,
            end: e,
        }
    }
    pub fn wrap(&self, origin: &str, msg: String, s: usize, e: usize) -> ParseError {
        ParseError {
            origin: format!("{} -> {}", self.origin, origin),
            msg,
            start: s,
            end: e,
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "the Frack has Attacked...")
    }
}

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
