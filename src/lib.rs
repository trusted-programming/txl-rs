extern crate reqwest;
use std::{
    env,
    io::{Error, ErrorKind},
    path::Path,
    process::{Command, Stdio},
};

use flate2::read::GzDecoder;
use tar::Archive;

/// .
///
/// # Errors
///
/// This function will return an error if the txl command is not found.
pub fn txl(args: Vec<String>) -> Result<String, Error> {
    let mut cmd = "txl";
    let mut my_args = args.clone();
    if Path::new("txl10.8b.linux64/bin/txl").exists() {
        cmd = "txl10.8b.linux64/bin/txl";
        my_args.push("-i".to_string());
        my_args.push("txl10.8b.linux64/lib/Rust".to_string());
    }
    // println!("{cmd}");
    if let Ok(command) = Command::new(cmd)
        .args(&my_args)
        .stdout(Stdio::piped())
        .spawn()
    {
        if let Ok(output) = command.wait_with_output() {
            match String::from_utf8(output.stdout).ok() {
                Some(s) => Ok(s),
                None => Err(Error::new(ErrorKind::Other, "output is not UTF8")),
            }
        } else {
            println!("Cannot run txl {:?}", args);
            Err(Error::new(
                ErrorKind::Other,
                format!("Cannot run `txl {:?}`", args),
            ))
        }
    } else {
        println!("txl not found, downlading...");
        if let Ok(resp) =
            reqwest::blocking::get("http://www.txl.ca/download/13483-txl10.8b.linux64.tar.gz")
        {
            // println!("{:?}", resp);
            if let Ok(bytes) = resp.bytes() {
                let tar = GzDecoder::new(&bytes[..]);
                let mut archive = Archive::new(tar);
                archive.unpack(".")?;
                if let Ok(path) = env::var("PATH") {
                    env::set_var(
                        "PATH",
                        format!("{:?}/txl10.8b.linux64:{path}", env::current_dir()),
                    );
                }
                if let Ok(resp) =
                    reqwest::blocking::get("http://www.txl.ca/examples/Grammars/Rust/Rust.tar.gz")
                {
                    if let Ok(bytes) = resp.bytes() {
                        let tar = GzDecoder::new(&bytes[..]);
                        let mut archive = Archive::new(tar);
                        archive.unpack("txl10.8b.linux64/lib")?;
                    }
                }
                txl(args)
            } else {
                Err(Error::new(ErrorKind::Other, "Bytes conversion error"))
            }
        } else {
            Err(Error::new(ErrorKind::Other, "Command `txl` not found"))
        }
    }
}
