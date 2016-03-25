extern crate image;
extern crate getopts;

use std::env;
use std::path::Path;
use std::hash::SipHasher;
use std::hash::Hasher;
use std::iter::Iterator;

use getopts::Options;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn hash_image(path: &str) -> u64 {
    let img = image::open(&Path::new(path)).unwrap();

    img.resize(4, 4, image::FilterType::Lanczos3);

    let mut hasher = SipHasher::new();

    for pixel in img.to_rgb().pixels() {
        hasher.write(&pixel.data);
    }

    return hasher.finish();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();

    opts.optflag("h", "help", "print this help menu");
    opts.optflag("v", "verbose", "Verbose output. Prints image paths along with hashes.");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let verbose = matches.opt_present("v");

    let paths = matches.free;
    if paths.is_empty() {
        print_usage(&program, opts);
        return;
    }

    let maxlen = match paths.iter().map(|p| p.len()).max() {
        Some(v) => { v }
        None => { panic!("No values for maxlen") }
    };

    for path in paths {
        let hash = hash_image(&*path);

        match verbose {
            true  => { println!("{:2$} {:x}", path, hash, maxlen) }
            false => { println!("{:x}", hash) }
        }
    }
}
