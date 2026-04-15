use ee_conio_macro::ctransform;

struct BoxSpec<T> {
    ul: T,
    um: T,
    ur: T,
    ml: T,
    mr: T,
    ll: T,
    lm: T,
    lr: T,
}

#[allow(dead_code)]
static BS1: BoxSpec<char> = BoxSpec {
    ul: '╭',
    um: '─',
    ur: '╮',
    ml: '│',
    mr: '│',
    ll: '╰',
    lm: '─',
    lr: '╯',
};

ctransform!(
    #[allow(dead_code)]
    static BS2: BoxSpec<&str> = BoxSpec {
        ul: "~[] ",
        um: "~[underline] ~[under_off]",
        ur: "~[] ",
        ml: "~[] ",
        mr: "~[] ",
        ll: "~[] ",
        lm: "~[overline] ~[over_off]",
        lr: "~[] ",
    };
);

static B: &str = ctransform!("~[x0 $'Maastricht Blue' #'Canary Yellow']");
static C: &str = ctransform!("~[x0 $'Maastricht Blue' #'Bright Cyan'  ]");
static E: &str = ctransform!("~[x0 XK]\n");

pub fn header_impl(s: String) {
    //! [WIP] idea for a more generic helper
    let BoxSpec {
        ul,
        um,
        ur,
        ml,
        mr,
        ll,
        lm,
        lr,
    } = BS1;

    let (pl, pr) = (1, 1); // Pad left, right
    let tl = String::from(um).repeat(s.len() + (pl + pr));
    let bl = String::from(lm).repeat(s.len() + (pl + pr));
    let pl = " ".repeat(pl);
    let pr = " ".repeat(pr);

    print!("{B}{ul}{tl             }{ur}{E}");
    print!("{B}{ml}{pl}{C}{s}{B}{pr}{mr}{E}");
    print!("{B}{ll}{bl             }{lr}{E}");
}

#[macro_export]
macro_rules! header {
    ($($arg:tt)*) => { header_impl( format!($($arg)*) ); }
}
