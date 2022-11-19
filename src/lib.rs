extern crate reqwest;
use std::{
    env,
    io::{Error, ErrorKind},
    path::Path,
    process::{Command, Stdio},
};

use flate2::read::GzDecoder;
use tar::Archive;

#[cfg(target_os = "macos")]
const FOLDER: &str = "txl10.8b.macosx64";
#[cfg(target_os = "macos")]
const URL: &str = "http://txl.ca/download/11206-txl10.8b.macosx64.tar.gz";
#[cfg(target_os = "macos")]
const EXE: &str = "";
#[cfg(target_os = "linux")]
const FOLDER: &str = "txl10.8b.linux64";
#[cfg(target_os = "linux")]
const URL: &str = "http://www.txl.ca/download/13483-txl10.8b.linux64.tar.gz";
#[cfg(target_os = "linux")]
const EXE: &str = "";
#[cfg(target_os = "windows")]
const FOLDER: &str = "Txl108bwin64";
#[cfg(target_os = "windows")]
const URL: &str = "http://txl.ca/download/11888-Txl108bwin64.zip";
#[cfg(target_os = "windows")]
const EXE: &str = ".exe";

#[cfg(all())]
/// .
///
/// # Errors
///
/// This function will return an error if the txl command is not found.
pub fn txl(args: Vec<String>) -> Result<String, Error> {
    let mut my_args = args.clone();
    let cmd = format!("{FOLDER}/bin/txl{EXE}");
    if Path::new(&cmd).exists() {
        my_args.push("-i".to_string());
        my_args.push(format!("{}/lib/Rust", FOLDER));
    }
    println!("{FOLDER}");
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
            reqwest::blocking::get(URL)
        {
            if let Ok(bytes) = resp.bytes() {
                if URL.ends_with(".tar.gz") {
                    let tar = GzDecoder::new(&bytes[..]);
                    let mut archive = Archive::new(tar);
                    archive.unpack(".")?;
                } else {
                    let reader = std::io::Cursor::new(bytes);
                    let mut zip = zip::ZipArchive::new(reader)?;
                    zip.extract(".").ok();
                }
                if let Ok(path) = env::var("PATH") {
                    env::set_var(
                        "PATH",
                        format!("{:?}/{}/bin:{path}", env::current_dir(), FOLDER),
                    );
                }
                if let Ok(resp) =
                    reqwest::blocking::get("http://www.txl.ca/examples/Grammars/Rust/Rust.tar.gz")
                {
                    if let Ok(bytes) = resp.bytes() {
                        let tar = GzDecoder::new(&bytes[..]);
                        let mut archive = Archive::new(tar);
                        archive.unpack(format!("{}/lib", FOLDER))?;
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
