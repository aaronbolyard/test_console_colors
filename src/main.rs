#[macro_use] extern crate enum_primitive;
extern crate getopts;
extern crate num;
extern crate rand;

use getopts::Options;
use num::FromPrimitive;
use rand::{thread_rng, Rng};
use std::env;
use std::iter;

enum_from_primitive! {
    enum Color {
        Black = 30,
        Red = 31,
        Green = 32,
        Yellow = 33,
        Blue = 34,
        Magenta = 35,
        Cyan = 36,
        White = 37
    }
}

const COLOR_RANGE_START: usize = 30;
const COLOR_RANGE_END: usize = 38;
const COLOR_COUNT: usize = 8;

enum_from_primitive! {
    enum Mode {
        Bright,
        Normal
    }
}

const MODE_COUNT: usize = 2;

fn set_text_color(color: Color, mode: Mode) {
    print!("\x1B[");

    match mode {
        Mode::Bright => print!("1;"),
        Mode::Normal => print!("0;")
    }

    print!("{0}m", color as u32);
}

fn reset_text_color() {
    print!("\x1B[0m");
}

const UTILITY_WORD_COUNT: usize = 64;
const UTILITY_BLOCK_TEXT: &'static str = "\u{2591}\u{2592}\u{2593}\u{2588}";
const UTILITY_LOREM_IPSUM_TEXT: &'static str = "Lorem ipsum dolor sit amet, \
                                                consectetur adipiscing elit. \
                                                Nam tempor cursus libero, nec \
                                                porta mauris auctor eget. \
                                                Curabitur arcu mauris, egestas \
                                                euismod neque non, semper \
                                                interdum elit. Vestibulum \
                                                tempor, nisi id blandit \
                                                laoreet, velit arcu laoreet \
                                                mauris, vitae porta sem justo \
                                                a purus. Ut eleifend suscipit \
                                                dui, sit amet faucibus mauris \
                                                placerat eu. Ut imperdiet \
                                                massa nec justo fermentum \
                                                lobortis. Sed hendrerit et \
                                                nibh.";

fn print_help(program: String, options: Options) {
    let brief = format!("Usage: {} [options]", program);

    println!("{}", options.usage(&brief));
}

fn main() {
    let args: Vec<String> =env::args().collect();
    let program = args[0].clone();

    let mut options = Options::new();
    options.optopt("o", "output", "Output data source; default 'ipsum'.", "<ipsum|block>");
    options.optopt("m", "mode", "Color generation mode; default 'cycle'.", "<cycle|random|cycle-random>");
    options.optflag("h", "help", "Print help.");

    let matches = match options.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string())
    };

    if matches.opt_present("h") {
        print_help(program, options);

        return;
    }

    let output_data_source = matches.opt_str("o").unwrap_or(String::from("ipsum"));
    let color_generation_mode = matches.opt_str("m").unwrap_or(String::from("cycle"));

    let output_data: Vec<&str> = match &*output_data_source {
        "ipsum" => UTILITY_LOREM_IPSUM_TEXT.split(' ').collect(),
        "block" => iter::repeat(UTILITY_BLOCK_TEXT).take(UTILITY_WORD_COUNT).collect(),
        _ => panic!("unexpected output data source: {}", output_data_source)
    };

    let mut rng = rand::thread_rng();

    let colors: Vec<usize> = match &*color_generation_mode {
        "cycle" => {
            (COLOR_RANGE_START..COLOR_RANGE_END)
                .cycle()
                .take(UTILITY_WORD_COUNT)
                .collect()
        },
        "random" => {
            rng.gen_iter::<usize>()
                .take(UTILITY_WORD_COUNT)
                .map(|x| x % COLOR_COUNT + COLOR_RANGE_START)
                .collect()
        },
        "cycle-random" => {
            let random = rng.gen_iter::<usize>()
                                 .map(|x| x % COLOR_COUNT + COLOR_RANGE_START);

            (COLOR_RANGE_START..COLOR_RANGE_END)
                .cycle()
                .take(COLOR_COUNT * 2)
                .chain(random)
                .take(UTILITY_WORD_COUNT)
                .collect()
        },
        _ => panic!("unexpected color generation mode: {}", color_generation_mode)
    };

    let modes: Vec<usize> = match &*color_generation_mode {
        "cycle" => {
            let normal = iter::repeat(Mode::Normal as usize).take(COLOR_COUNT);
            let bright = iter::repeat(Mode::Bright as usize).take(COLOR_COUNT);

            normal.chain(bright).cycle().take(UTILITY_WORD_COUNT).collect()
        },
        "random" => {
            rng.gen_iter::<usize>()
                .take(UTILITY_WORD_COUNT)
                .map(|x| x % MODE_COUNT)
                .collect()
        },
        "cycle-random" => {
            let normal = iter::repeat(Mode::Normal as usize).take(COLOR_COUNT);
            let bright = iter::repeat(Mode::Bright as usize).take(COLOR_COUNT);
            let random = rng.gen_iter::<usize>()
                                 .map(|x| x % MODE_COUNT);

            normal.chain(bright)
                .cycle()
                .take(COLOR_COUNT * 2)
                .chain(random)
                .take(UTILITY_WORD_COUNT)
                .collect()
        },
        _ => panic!("unexpected color generation mode: {}", color_generation_mode)
    };

    for value in output_data.iter().enumerate() {
        //reset_text_color();
        //println!("(index={}, color={}, mode={}) ", value.0, colors[value.0], modes[value.0]);

        let color = Color::from_usize(colors[value.0]).unwrap();
        let mode = Mode::from_usize(modes[value.0]).unwrap();

        set_text_color(color, mode);
        print!("{} ", value.1);
    }

    reset_text_color();
    println!("\n\nDone. How does it look?");
}
