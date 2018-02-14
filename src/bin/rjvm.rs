#[macro_use] extern crate clap;
extern crate env_logger;
extern crate jvm;

use jvm::classfile::Classfile;
use std::fs::File;
use std::path::Path;

fn main() {
    env_logger::init();

    let matches = clap::App::new("rjvm")
        .author(crate_authors!())
        .version(crate_version!())
        .arg(clap::Arg::with_name("CLASSPATH")
             .short("c").long("classpath")
             .takes_value(true))
        .arg(clap::Arg::with_name("CLASS")
             .required(true))
        .get_matches();

    let class = matches.value_of("CLASS").unwrap();

    let path = Path::new(class).with_extension("class");
    println!("Opening: {}", path.display());

    let cf = {
        let mut file = File::open(path).unwrap();
        Classfile::read(&mut file).unwrap()
    };

    cf.dump();
}
