//  I wanna make up my mind
//  But I don't know myself
//  No I don't know myself
//  I get a little bit Genghis Khan
//  I don't want you to get it on
//  With nobody else but me

extern crate argparse;
extern crate globwalk;
extern crate regex;
extern crate zip;

use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let pwd: std::path::PathBuf = std::env::current_dir().unwrap();

    let mut verbos: bool = false;
    let mut overwr: bool = false;
    let mut uniqqq: bool = false;

    let mut runame: String = String::from("Pony_TxT");

    let mut arkdir: std::path::PathBuf = pwd
        .join("..")
        .join("pony")
        .join("fimfarchive-*")
        .join("epub");

    let mut copdir: std::path::PathBuf = pwd.join("..").join("pony").join("corp");

    // Argument Parsing block
    {
        use argparse::{ArgumentParser, Store, StoreTrue};
        let mut ap: ArgumentParser = ArgumentParser::new();
        ap.set_description("Commit crimes against human kind, just like God intended.");
        ap.refer(&mut verbos).add_option(&["-v", "--verbose"], StoreTrue, "Be verbose and slower.");
        ap.refer(&mut runame).add_option(&["-n", "--runame"], Store, "Run Name.");
        ap.refer(&mut overwr).add_option(&["--overwrite"], StoreTrue, "Overwrite existing corpus{es}; Useful when working with limited disk space.");
        ap.refer(&mut arkdir).add_option(&["-a", "--arkdir"], Store, "Directory directory where the EPUBs are stored, directory hierarchy does not matter.");
        ap.refer(&mut copdir).add_option(&["-o", "--output"], Store, "Directory where to store the resoulting file.");
        ap.refer(&mut uniqqq).add_option(&["-u", "--uniq"], StoreTrue, "NOT IMPLEMENBTED YET; Filter adjacent matching lines from corpus.");
        ap.parse_args_or_exit();
    }


    let mut corppath: std::path::PathBuf = copdir.join(runame.clone());
    
    let _ = std::fs::create_dir_all(&copdir);

    if overwr {
        for old in globwalk::GlobWalkerBuilder::from_patterns(copdir.as_path(), &["*.{txt,lst,npz}"]).follow_links(true).build().unwrap() {
            std::fs::remove_file(old.unwrap().path())?;
        }
    } else {
        corppath = copdir.join(runame.clone()).join([runame.clone(), String::from(".txt")].concat());
    }

    let mut corpfile = std::fs::OpenOptions::new().write(true).append(true).create(true).open(corppath).unwrap();

    let relbrk = regex::Regex::new("(\r)|(<p>)|(</p>)|(<br/>)").unwrap();   // match things that should be line breaks
    let remxml = regex::Regex::new("<[^>]*>").unwrap();                     // match XML tags
    let remtrl = regex::Regex::new("([ \t]*$)|([ ]*$)").unwrap();           // match continuous spaces/tabs

    for epub in globwalk::GlobWalkerBuilder::from_patterns(std::fs::canonicalize(arkdir.as_path()).unwrap().as_path(), &["*.epub"]).build().unwrap() {
        if verbos { println!("procession [{}]", epub.as_ref().unwrap().path().file_name().unwrap().to_str().unwrap()); }

        let f = std::fs::OpenOptions::new().read(true).open(epub.unwrap().path()).unwrap();
        let mut inpub = zip::ZipArchive::new(f).unwrap();
        let mut text = String::new();
        let mut _buff = String::new();
        
        for i in 0..inpub.len() {
            let mut inzip = inpub.by_index(i).unwrap();
            if ( inzip.name().starts_with("chapter-") | inzip.name().starts_with("Chapter") ) & inzip.name().ends_with(".html") {
                inzip.read_to_string(&mut text)?;
                corpfile.write_all(b"<|startoftext|>")?;
                for line in text.lines() {
                    if checkfor(line) { continue }
                    
                    let line = &relbrk.replace_all(line, "\n");
                    let line = &remxml.replace_all(line, " ");
                    let line = &remtrl.replace_all(line, " ");

                    if line.trim().is_empty() { continue }

                    corpfile.write_all(line.as_bytes())?;
                }
                corpfile.write_all(b"<|endoftext|>\n")?;
            } else {
                if verbos { println!("File: {} Excluded", inzip.name()); }
            }
        }
    }

    // Never more
    corpfile.sync_all()?;
    
    return Ok(());
}

fn checkfor(line: &str) -> bool {
    if regex::Regex::new("<title>").unwrap().is_match(line) { return true }
    if regex::Regex::new("<h1>Author's Note</h1>").unwrap().is_match(line) { return true }
    if regex::Regex::new("Author's Note").unwrap().is_match(line) { return true }
    return false;
}
