use cuelsp::lsp_server::rpc::decode_message;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader, prelude::*};

fn main() -> Result<(), Box<dyn Error>> {
    // Read from the input stream from stdin (from IDE)
    let stdin = io::stdin();

    let mut reader = BufReader::new(stdin);
    let mut last_line = String::new();
    let mut log_file = File::create("log.txt")?;
    // main reading loop
    // use loop so that reader is not used as iter..
    // add buffer to line, let _
    loop {
        let mut line = String::default();
        let _ = reader.read_line(&mut line);
        if line.trim().is_empty() {
            // TODO HERE
            // Unpack size of bytes, try read from Bufreader of byte buffer size (All Json), decode message
            // update l to null again... continue
            // get ContentLength
            // strip prefix, take resulting leftover, and then parse usize (remove CRLF)
            if let Some(rest) = last_line.strip_prefix("Content-Length: ") {
                let length: usize = rest.trim().parse()?;

                // TODO try better error handling
                // legnth needs to parse only digis... NOT \r\n at end of Content-Length line
                // clear buffer
                let mut raw_message = vec![0u8; length];
                reader.read_exact(&mut raw_message)?;
                let decoded_message = decode_message(&raw_message);
                let meth = decoded_message?.method;
                log_file.write(format!("Method {meth} found").as_bytes())?;
            }

            last_line.clear();
            continue;
        }
        last_line = line.clone();
    }
}
