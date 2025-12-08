use std::io::{self, BufReader, prelude::*};

use cuelsp::lsp_server::rpc::decode_message;

fn main() -> std::io::Result<()> {
    // Read from the input stream from stdin (from IDE)
    let stdin = io::stdin();

    let mut reader = BufReader::new(stdin);
    let mut line = String::new();
    let mut last_line = String::new();
    let mut raw_message: Vec<u8> = vec![];
    // main reading loop
    // use loop so that reader is not used as iter..
    // add buffer to line, let _
    loop {
        line.clear();
        let _ = reader.read_line(&mut line);

        if line.trim().is_empty() {
            // TODO HERE
            // Unpack size of bytes, try read from Bufreader of byte buffer size (All Json), decode message
            // update l to null again... continue
            // get ContentLength
            // strip prefix, take resulting leftover, and then parse usize (remove CRLF)
            if let Some(rest) = last_line.strip_prefix("Content-Length: ") {
                let length: usize = rest.trim().parse().expect("Could Not Parse Length");

                // TODO try better error handling
                // legnth needs to parse only digis... NOT \r\n at end of Content-Length line
                // clear buffer
                let mut raw_message = vec![0u8; length];
                reader.read_exact(&mut raw_message)?;

                let decoded_message = decode_message(&raw_message);
                dbg!(&decoded_message);
            }

            line.clear();
            last_line.clear();
            continue;
        }
        last_line = line.clone();
    }
    /*
    for l in reader.lines() {
        // TODO make this better
        let l = l.expect("Could Not Read StdIn Line (main.rs)");

        // find end of header
        if l == r#"\r\n"# {
            // TODO HERE
            // Unpack size of bytes, try read from Bufreader of byte buffer size (All Json), decode message
            // update l to null again... continue
            // get ContentLength
            let (_, length) = last_line.split_at("Content-Length: ".len());
            // TODO try better error handling
            let length: usize = length.parse().unwrap();

            let message_to_decode = reader.consume(length);
            println!("INSIDE RN");
            continue;
        }
        println!("LAST LINE CLONING UNDER");
        last_line = l.clone();
    }*/

    Ok(())
}
