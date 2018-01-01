#[macro_use]
extern crate clap;
use clap::{App, Arg};
use std::io::Error;
use std::fs::File;

fn main() {
	if let Err(err) = inner_main() {
        eprintln!("An error occured: {}", err);
    }
}

fn inner_main() -> Result<(), Error> {
	let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(Arg::with_name("file")
            .help("The file to read.")
            .index(1))
        .get_matches();

    let mut file = File::open(matches.value_of("file").unwrap())?;
    Ok(())
}
