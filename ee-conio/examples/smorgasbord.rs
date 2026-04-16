mod common;
use common::*;

use ee_conio::{
    bg_256, bg_color_256, cformat, cprint, cprintln, csi, csi_sequence, ctransform, fg_256,
    fg_color_256, sgr, sgr_code,
};

#[rustfmt::skip]
pub fn main() {
    cprintln!("~[cls]");
    header!("smorgasbord");

    cprint!  ("~[c51    ]forground ");
    cprint!  ("~[c:51   ]forground ");
    cprintln!("~[fore:51]forground");

    cprint!  ("~[C164    ]background ");
    cprint!  ("~[C:164   ]background ");
    cprintln!("~[back:164]background");

    cprint!(  "~[C51         c234]both ");
    cprint!(  "~[C:51       c:234]both ");
    cprintln!("~[back:51 fore:234]both");

    cprintln!("~[black] black ~[red] red ~[green] green ~[yellow] yellow ~[blue] blue ~[magenta] magenta ~[cyan] cyan ~[white] white ");
    cprintln!("~[BLACK] BLACK ~[RED] RED ~[GREEN] GREEN ~[YELLOW] YELLOW ~[BLUE] BLUE ~[MAGENTA] MAGENTA ~[CYAN] CYAN ~[WHITE] WHITE ");
    cprintln!("~[x30  ] black ~[x31] red ~[x32  ] green ~[x33   ] yellow ~[x34 ] blue ~[x35    ] magenta ~[x36 ] cyan ~[x37  ] white ");
    cprintln!("~[x40  ] BLACK ~[x41] RED ~[x42  ] GREEN ~[x43   ] YELLOW ~[x44 ] BLUE ~[x45    ] MAGENTA ~[x46 ] CYAN ~[x47  ] WHITE ");

    let b = cformat!("{:^15}", "Corvette");
    cprintln!("~[white $'Candy Apple Red']{b}~[x0 $'Strawberry Red']{b}~[x0 $'Barn Red']{b}~[x0 $'Sizzling Red']{b}");
    cprintln!("~[      #'Candy Apple Red']{b}~[x0 #'Strawberry Red']{b}~[x0 #'Barn Red']{b}~[x0 #'Sizzling Red']{b}");

    let b = cformat!("{:^15}", "Rain");
    cprintln!("~[white $'Vibrant Purple']{b}~[x0 $'Purple Heart']{b}~[x0 $'Psychedelic Purple']{b}~[x0 $'Palatinate Purple']{b}");
    cprintln!("~[      #'Vibrant Purple']{b}~[x0 #'Purple Heart']{b}~[x0 #'Psychedelic Purple']{b}~[x0 #'Palatinate Purple']{b}");

    let b = cformat!("{:^15}", "Beret");
    cprintln!("~[white $'Raspberry']{b}~[x0 $'Vivid Raspberry']{b}~[x0 $'French Raspberry']{b}~[x0 $'Dark Raspberry']{b}");
    cprintln!("~[      #'Raspberry']{b}~[x0 #'Vivid Raspberry']{b}~[x0 #'French Raspberry']{b}~[x0 #'Dark Raspberry']{b}");

    cprintln!("~[underline]underline ~[x0 overline]overline ~[x0 strike]strike ~[x0 blink]blink");
    cprintln!("");
    cprintln!("~[BLUE yellow underline overline strike blink]underline+overline+strike+blink");
    cprintln!("~[MAGENTA white underline overline]underline+overline");


    // Line clearing
    cprintln!("~[x0]Some text |-~[$A70D2A c255 clrln  ]-|~[c227 BLUE] Clear line   ");
    cprintln!("~[x0]Some text |-~[$A70D2A c255 clrbol ]-|~[c227 BLUE] Clear to BOL ");
    cprintln!("~[x0]Some text |-~[$A70D2A c255 clreol ]-|~[c227 BLUE] Clear to EOL ");


    cprintln!("~[#FFFFFF $EE5511]White text with Orange Background~[clreol]");
    cprintln!("~[$EE3333 #FFFFFF]Orange Background with White text~[X5D clrbol X10C clreol x0](Hi!)");
    cprintln!("~[white BLUE]Before cursor move~[x0]~[red WHITE CPL]After cursor movement~[x0]\n\n");

    let m = "black text, yellow background";
    // Runtime
    print!  ("{}{}{m}{}{}:",bg_color_256(227),fg_color_256(16),sgr_code(0),csi_sequence("K"));
    println!("{:?}{:?}{m:?}{:?}{:?}",bg_color_256(227),fg_color_256(16),sgr_code(0),csi_sequence("K"));
    // Compiled
    print!  ("{}{}{m}{}{}:",bg_256!(227),fg_256!(16),sgr!(0),csi!("K"));
    println!("{:?}{:?}{m:?}{:?}{:?}",bg_256!(227),fg_256!(16),sgr!(0),csi!("K"));
    // Compiled transforms
    let o = cformat!("~[C227 c16]{m}~[x0 clreol]");
    // let o = format!( ctransform!("~[C227 c16]{m}~[x0 XK]"),m=m);
    print!  ("{o}:");
    println!("{o:?}");

    // The usage of cprintln/ctransform below is equivalent
    println!(ctransform!("~[#AAF0D1]Magic Man~[c255] by ~[heart x0]"));
    ctransform!(println!("~[#AAF0D1]Magic Man~[c255] by ~[heart x0]"));
    cprintln!(           "~[#AAF0D1]Magic Man~[c255] by ~[heart   ]" );

    cprintln!("~[hide]😸~[:) YELLOW clreol GREEN blue heart]Hello, ~[peach]y ~[reveal]this is ~[poo]💩.~[x0] (keep it 💯) ");
    cprintln!();

    cprint!("~[$'Dark Sienna' c227]Showing the impact of CSI codes on this vs the following lines \n");
    cprintln!("Text on this line is still the same. No tokens are processed so nothing changes.");
    cprintln!("SGR reset is sent ~[italic_on]before~[italic_off] the newline at end because of the SGR codes (italic on/off).");
    cprintln!("This line is back to normal after the reset from line above.");
    cprintln!();
}
