#[macro_use]
extern crate clap;
use clap::App;
use std::io::Error;

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
        .get_matches();
    Ok(())
}
