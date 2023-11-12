use std::fs::File;
use readelf::elf;
use clap::{App, Arg};

fn usage() {
    println!("Usage: easy-readelf [OPTION]... FILE");
    println!("OPTIONS:");
    println!("\t-h(--header)\tTo read the elf header");
    println!("\t-s(--sections)\tTo read the elf section table");
    println!("\t-l(--segments)\tTo read the elf program header table");
}

fn main() {
    let matches = App::new("easy-readelf")
        .version("0.1.0")
        .author("TurtleRuss")
        .about("A simple readelf implementation")
        .args(&[
            Arg::with_name("header")
                .short('h')
                .long("header")
                .help("To read the elf header")
                .required(false)
                .takes_value(true),
            Arg::with_name("sections")
                .short('s')
                .long("sections")
                .help("To read the elf section table")
                .required(false)
                .takes_value(true),
            Arg::with_name("segments")
                .short('l')
                .long("segments")
                .help("To read the elf program header table")
                .required(false)
                .takes_value(true),
        ]).get_matches();
    if matches.is_present("header") {
        let mut elf = File::open(matches.value_of("header").unwrap()).expect("Failed to open file");
        let header = elf::header::parse_header(&mut elf).unwrap();
        println!("{}",header);
    }else if matches.is_present("sections") {
        let mut elf = File::open(matches.value_of("sections").unwrap()).expect("Failed to open file");
        elf::section::parse_sections(&mut elf).
            expect("Failed to parse sections")
            .iter()
            .for_each(|section| println!("{}",section));
    }else if matches.is_present("segments") {
        let mut elf = File::open(matches.value_of("segments").unwrap()).expect("Failed to open file");
        elf::segment::parse_segments(&mut elf).
            expect("Failed to parse segments").
            iter().
            for_each(|segment| println!("{}",segment));
    }else {
        usage();
    }
}
