#[macro_use] extern crate clap;
extern crate env_logger;
extern crate jvm;

use jvm::classfile::ClassFile;
use std::fs::File;
use std::path::Path;

fn main() {
    env_logger::init().unwrap();

    let matches = clap_app!(rjvm =>
        (author: "KokaKiwi <kokakiwi@kokakiwi.net>")
        (version: crate_version!())

        (@arg CLASSPATH: -c --classpath +takes_value)
        (@arg CLASS: +required)
    ).get_matches();

    let class = matches.value_of("CLASS").unwrap();

    let path = Path::new(class).with_extension("class");
    println!("Opening: {}", path.display());

    let cf = {
        let mut file = File::open(path).unwrap();
        ClassFile::read(&mut file).unwrap()
    };

    cf.dump();
}
