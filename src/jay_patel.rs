use ansi_term::Colour::Fixed;
use ansi_term::Style;
use std::fmt::Display;

#[derive(Default)]
pub struct RandomJayPatel {
    ending: bool,
    middle: bool,
    bold: bool,
    dimmed: bool,
    italic: bool,
    underline: bool,
    blink: bool,
    reverse: bool,
    strikethrough: bool,
    colored: bool,
    color: u8,
}

pub fn randomize_jay(mut seed: u64) -> RandomJayPatel {
    let ending = read_bit(&mut seed, 1);
    let middle = read_bit(&mut seed, 1);
    let bold = read_bit(&mut seed, 2);
    let dimmed = read_bit(&mut seed, 1);
    let italic = read_bit(&mut seed, 1);
    let underline = read_bit(&mut seed, 1);
    let blink = read_bit(&mut seed, 3);
    let reverse = read_bit(&mut seed, 2);
    let strikethrough = read_bit(&mut seed, 3);
    //let colored = read_bit(&mut seed, 0);
    let colored = true;
    //Limits Colors to non white or black from the Fixed Library
    let color = (((seed & 0b11111111) % 215) + 16) as u8;

    RandomJayPatel {
        ending,
        middle,
        bold,
        dimmed,
        italic,
        underline,
        blink,
        reverse,
        strikethrough,
        colored,
        color,
    }
}

impl Display for RandomJayPatel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(apply_filters(self).as_str())
    }
}

fn apply_filters(jay: &RandomJayPatel) -> String {
    let mut s: String = (if jay.middle {
        "Jay Sanjay Patel"
    } else {
        "Jay Patel"
    })
    .to_string();

    if jay.reverse {
        s = Style::new().reverse().paint(s).to_string();
    }
    if jay.bold {
        s = Style::new().bold().paint(s).to_string();
    }
    if jay.underline {
        s = Style::new().underline().paint(s).to_string();
    }
    if jay.blink {
        s = Style::new().blink().paint(s).to_string();
    }
    if jay.dimmed {
        s = Style::new().dimmed().paint(s).to_string();
    }
    if jay.italic {
        s = Style::new().italic().paint(s).to_string();
    }
    if jay.strikethrough {
        s = Style::new().strikethrough().paint(s).to_string();
    }

    //Color
    if jay.colored {
        s = Fixed(jay.color).paint(s).to_string();
    }
    if jay.ending {
        s += "\n";
    }

    s
}

//Amount represents the number of bits that must be 1 for a true response
fn read_bit(x: &mut u64, amount: u64) -> bool {
    let mut mask = 0;
    for _ in 0..amount {
        mask += 1;
        mask = mask << 1;
    }
    let res = *x & mask == mask;
    *x = *x >> 1;
    res
}
