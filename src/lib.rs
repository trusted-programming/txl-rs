extern crate reqwest;
use std::{
    env,
    io::{Error, ErrorKind},
    path::Path,
    process::{Command, Stdio},
};

use flate2::read::GzDecoder;
use regex::Regex;
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
fn get_file_arg_ext(args: Vec<String>) -> (String, String) {
    let file_args: Vec<&String> = args
        .iter()
        .filter(|x| std::path::PathBuf::from(x).exists())
        .collect();
    let mut ext = "";
    if !file_args.is_empty() {
        ext = file_args[0];
    }
    if let Some(ex) = Path::new(ext).extension() {
        let ex_str = ex.to_string_lossy();
        let grammar = lang_to_grammar(&ex_str);
        (ex_str.to_string(), grammar.to_string())
    } else {
        ("".to_string(), "".to_string())
    }
}

/// .
///
/// # Errors
///
/// This function will return an error if the txl command is not found.
pub fn txl(args: Vec<String>) -> Result<String, Error> {
    use std::path::PathBuf;
    let mut my_args = args.clone();
    let cmd = format!("{FOLDER}/bin/txl{EXE}");
    if Path::new(&cmd).exists() {
        let (ext, grammar) = get_file_arg_ext(args.clone());
        if !grammar.is_empty() {
            let grammar_file = PathBuf::from(format!("{}/lib/{}/{}.txl", FOLDER, grammar, ext));
            if grammar_file.exists() {
                my_args.push("-i".to_string());
                my_args.push(format!("{}/lib/{}/", FOLDER, grammar));
            } else {
                let grammar_file =
                    PathBuf::from(format!("{}/lib/{}/Txl/{}.txl", FOLDER, grammar, ext));
                if grammar_file.exists() {
                    my_args.push("-i".to_string());
                    my_args.push(format!("{}/lib/{}/Txl/", FOLDER, grammar));
                }
            }
        }
    }
    if let Ok(command) = Command::new(cmd)
        .args(&my_args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        if let Ok(output) = command.wait_with_output() {
            match String::from_utf8(output.stdout).ok() {
                Some(s) => {
                    if let Ok(s0) = String::from_utf8(output.stderr) {
                        if let Ok(re) = Regex::new(".*: TXL0944E.* file '(.*).txl'") {
                            if re.is_match(&s0) {
                                let mut found = true;
                                re.captures_iter(&s0).for_each(|cap| {
                                    if let Err(e) = download(&cap[1]) {
                                        println!("{e}");
                                        found = false;
                                    }
                                });
                                if !found {
                                    Err(Error::new(ErrorKind::Other, s0))
                                } else {
                                    txl(my_args)
                                }
                            } else if s0.is_empty() {
                                Ok(s)
                            } else {
                                if let Ok(re) = Regex::new(".*: TXL(.*)E.*") {
                                    if re.is_match(&s0) {
                                        Err(Error::new(ErrorKind::Other, s0))
                                    } else {
                                        Ok(s)
                                    }
                                } else {
                                    Ok(s)
                                }
                            }
                        } else {
                            Ok(s)
                        }
                    } else {
                        Ok(s)
                    }
                }
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
        if let Ok(resp) = reqwest::blocking::get(URL) {
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
                download("rs")?;
                txl(my_args)
            } else {
                Err(Error::new(ErrorKind::Other, "Bytes conversion error"))
            }
        } else {
            Err(Error::new(ErrorKind::Other, "Command `txl` not found"))
        }
    }
}

fn lang_to_grammar(lang: &str) -> &str {
    let mut grammar = lang;
    match lang {
        "atl" => {
            grammar = "ATL";
        }
        "ada" => {
            grammar = "Ada";
        }
        "c" => {
            grammar = "C18";
        }
        "cpp" => {
            grammar = "Cpp";
        }
        "cs" => {
            grammar = "CSharp";
        }
        "delphi" => {
            grammar = "Delphi2006";
        }
        "e" => {
            grammar = "Eiffel";
        }
        "f77" => {
            grammar = "Fortran";
        }
        "html" => {
            grammar = "HTML";
        }
        "java" => {
            grammar = "Java8";
        }
        "js" => {
            grammar = "JavaScript";
        }
        "mod" => {
            grammar = "Modula3";
        }
        "php" => {
            grammar = "PHP";
        }
        "py" => {
            grammar = "Python";
        }
        "rb" => {
            grammar = "Ruby";
        }
        "grm" | "txl" => {
            grammar = "TXL";
        }
        "rs" => {
            grammar = "Rust";
        }
        "swift" | "SWIFT" => {
            grammar = "Swift";
        }
        "vb" => {
            grammar = "VisualBasic";
        }
        "xml" => {
            grammar = "XML";
        }
        "y" => {
            grammar = "Yacc";
        }
        _ => {
            grammar = "unsupported";
        }
    };
    grammar
}

fn download(lang: &str) -> Result<String, Error> {
    let grammar = lang_to_grammar(lang);
    let mut grammar_name;
    match lang {
        "ada" => {
            grammar_name = "Ada_grammar";
        }
        "php" => {
            grammar_name = "PHP345";
        }
        "y" => {
            grammar_name = "YAXX";
        }
        _ => {
            if lang_to_grammar(lang) == "unsupported" {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("{lang} is not supported"),
                ))
            } else {
                grammar_name = grammar;
            }
        }
    }
    if grammar_name.is_empty() {
        grammar_name = grammar;
    }
    if let Ok(resp) = reqwest::blocking::get(format!(
        "http://www.txl.ca/examples/Grammars/{grammar}/{grammar_name}.tar.gz"
    )) {
        if let Ok(bytes) = resp.bytes() {
            let tar = GzDecoder::new(&bytes[..]);
            let mut archive = Archive::new(tar);
            archive.unpack(format!("{}/lib", FOLDER))?;
        }
    }
    Ok("success".to_string())
}
