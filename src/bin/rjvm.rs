extern crate clap;
extern crate env_logger;
extern crate jvm;

use clap::{App, Arg};
use jvm::classfile::ClassFile;
use jvm::utils::print::Print;
use std::default::Default;
use std::fs::File;
use std::path::Path;

fn main() {
    env_logger::init().unwrap();

    let options = vec![
        Arg::with_name("CLASSPATH")
            .short("c")
            .long("classpath")
            .takes_value(true),
    ];

    let matches = App::new("rjvm")
        .version("0.1.0")
        .author("KokaKiwi <kokakiwi@kokakiwi.net>")
        .args(options)
        .arg(Arg::with_name("CLASS")
            .required(true)
            .index(1))
        .get_matches();

    let class = matches.value_of("CLASS").unwrap();

    let path = Path::new(class).with_extension("class");
    println!("Opening: {}", path.display());

    let cf = {
        let mut file = File::open(path).unwrap();
        ClassFile::read(&mut file).unwrap()
    };

    let mut printer = Default::default();
    cf.dump(&mut printer).unwrap();
}
