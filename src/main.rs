use std::io::{self, BufReader, prelude::*};

use cuelsp::handle_message;

fn main() -> std::io::Result<()> {
    // Read from the input stream from stdin (from IDE)
    let stdin = io::stdin();

    let mut reader = BufReader::new(stdin);
    let mut last_line = String::new();

    // main reading loop
    for l in reader.lines() {
        // TODO make this better
        let l = l.expect("Could Not Read StdIn Line (main.rs)");

        // find end of header
        if l == r#"\r\n"# {
            // TODO HERE
            // Unpack size of bytes, try read from Bufreader of byte buffer size (All Json), decode message
            // update l to null again... continue
            println!("INSIDE RN");
            continue;
        }
        println!("LAST LINE CLONING UNDER");
        last_line = l.clone();
    }

    Ok(())
}
