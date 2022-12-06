#![forbid(unsafe_code)]

/**
 * prompt user for password to check
 * read password.txt into memory
 * loop through passwords in password.txt
 * if user_password exists, print warning
 */


mod user_input;

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let user_password: String = user_input::get_user_input("Enter password:");
    let filename: String = user_input::get_user_input("Enter filename:");

    let file: File = File::open(filename)?;
    let reader: BufReader<File> = BufReader::new(file);

    for line in reader.lines() {
        let line: String = line.unwrap();
        if line == user_password {
            println!("Password file contains {}!", user_password);
            break;
        }
    }

    Ok(())
}
