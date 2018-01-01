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
            .required(true)
            .index(1))
        .get_matches();

    let mut file = File::open(matches.value_of("file").unwrap())?;
    
    let mut total = 0;
    let mut len = 0;
    for byte in file.bytes() {
        total += byte?;
        len += 1;
    }
    
    // Prevent dividing by zero by making output 0 is the total is 0.
    let output = if len == 0 { 0 } else { total / len };
    
    Ok(())
}
