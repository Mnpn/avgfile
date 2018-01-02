#[macro_use]
extern crate clap;
use clap::{App, Arg};
use std::io::{Error, Read};
use std::fs::File;

fn main() {
    // If any error would occur in inner_main(), print the error.'
    if let Err(err) = inner_main() {
        eprintln!("{}", err);
    }
}

fn inner_main() -> Result<(), Error> {
    // clap app creation, with macros that read project information from Cargo.toml.
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(Arg::with_name("file")
            .help("The file to read.")
            .required(true) // Make argument required.
            .index(1))
        // Create arguments
        .arg(Arg::with_name("total")
            .help("Calculate the total amount of bytes in file.")
            .short("t")
            .long("total"))
        .arg(Arg::with_name("search")
            .help("Search for number of occurences of specified value.")
            .short("s")
            .long("search")
            .takes_value(true)) // Make --search take an argument to search for.
        .arg(Arg::with_name("unique")
            .help("List amount of unique bytes.")
            .short("u")
            .long("unique"))
        .get_matches();

    // Define variables.
    // Open file from argument.
    let file_name = matches.value_of("file").unwrap();
    let file = File::open(&file_name)?;

    let search = value_t!(matches, "search", u8).ok();
    let is_unique  = matches.is_present("unique");
    let is_total   = matches.is_present("total");
    let is_average = search.is_none() && !is_unique;

    let mut total: usize = 0;
    let mut len: usize = 0;
    let mut occurences: usize = 0;
    let mut unique = Vec::new();

    for byte in file.bytes() {
        let byte = byte?;

        // Get the average
        if is_average {
            total += byte as usize;
            len += 1;
        }
        // Search for occurences of byte in file.
        if Some(byte) == search {
            occurences += 1;
        }
        // Search for unique bytes.
        if is_unique {
            if let Err(index) = unique.binary_search(&byte) {
                unique.insert(index, byte)
            }
        }
    }

    if is_total {
        // Print total sum of all bytes in the file.
        println!("Total sum of all bytes in {} in {}.", file_name, total);
    } else if let Some(search) = search {
        // Print number of occurences in the file.
        println!("Found {} bytes matching {} in {}.", occurences, search, file_name);
    } else if is_unique {
        println!("Found {} unique bytes in {}.", unique.len(), file_name);
    } else {
        // Prevent dividing by zero by making output 0 is the total is 0.
        let output = if len == 0 { 0 } else { total / len };

        // Print the output.
        println!("Average byte of {} is {}.", file_name, output);
    }

    // Everything is alright! Well done, code!
    Ok(())
}
