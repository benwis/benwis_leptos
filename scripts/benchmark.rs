#!/usr/bin/env -S cargo -Zscript

//! ```cargo
//! [dependencies]
//! clap = { version = "4.2", features = ["derive"] }
//! ```

use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{Command, Stdio};

fn main() -> Result<(), Error> {
    let build_server = Command::new("cargo")
        .args(["build", "--no-default-features", "--features=ssr"])
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output"));

    let reader = BufReader::new(build_server);

    //reader.lines().for_each(|line| println!("{}", line));
    Ok(())
}
