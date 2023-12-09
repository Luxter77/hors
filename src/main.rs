//  I wanna make up my mind
//  But I don't know myself
//  No I don't know myself
//  I get a little bit Genghis Khan
//  I don't want you to get it on
//  With nobody else but me

extern crate alphanumeric_sort;
extern crate argparse;
extern crate globwalk;
extern crate regex;
extern crate zip;

use std::io::prelude::*;
use alphanumeric_sort::sort_str_slice;
use std::fs::create_dir_all;

fn main() -> std::io::Result<()> {
    let pwd: std::path::PathBuf = std::env::current_dir().unwrap();

    let mut verbos: bool = false;
    let mut overwr: bool = false;
    let mut uniqqq: bool = false;

    let mut prefix: String = String::from("<|startoftext|>");
    let mut suffix: String = String::from("<|endoftext|>\n");

    let mut runame: String = String::from("Pony_TxT");

    let corppath: std::path::PathBuf;

    let mut arkdir: std::path::PathBuf = pwd.join("..").join("pony").join("archive");
    let mut copdir: std::path::PathBuf = pwd.join("..").join("pony").join("corp");
    
    {   // Argument Parsing block
        use argparse::{ArgumentParser, Store, StoreTrue};
        let mut ap: ArgumentParser = ArgumentParser::new();
        ap.set_description("Commit crimes against human kind, just like God intended.");
        ap.refer(&mut arkdir).add_option(&["-a", "--arkdir"], Store, "Directory directory where the EPUBs are stored, directory hierarchy do not matter.");
        ap.refer(&mut runame).add_option(&["-n", "--runame"], Store, "Name of this run.");
        ap.refer(&mut copdir).add_option(&["-o", "--output"], Store, "Directory where to store the resoulting file.");
        ap.refer(&mut verbos).add_option(&["-v", "--verbose"], StoreTrue, "Be verbose and slower.");
        ap.refer(&mut uniqqq).add_option(&["-u", "--uniq"], StoreTrue, "Filter adjacent matching lines from corpus.");
        ap.refer(&mut overwr).add_option(&["--overwrite"], StoreTrue, "Overwrite existing corpus{es}, Useful when working with limited disk space.");
        ap.refer(&mut prefix).add_option(&["--prefix"], Store, "Prefix to put before each chapter");
        ap.refer(&mut suffix).add_option(&["--suffix"], Store, "suffix to put after  each chapter"); // the second space is for sthetic purposes
        ap.parse_args_or_exit();
    }

    copdir = copdir.join(runame.clone());

    create_dir_all(copdir.as_os_str()).expect("Can't create output file or directory!");

    let _ = std::fs::create_dir_all(&copdir).expect("can't create output directory");

    if overwr {
        for old in globwalk::GlobWalkerBuilder::from_patterns(copdir.as_path(), &[[&runame.clone(), ".txt"].concat()]).follow_links(true).build().expect("I cant remove the files already there") {
            std::fs::remove_file(old.unwrap().path())?;
        }
        corppath = copdir.join([runame.clone(), String::from(".txt")].concat());
    } else {
        corppath = copdir.join([
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("no tachionic time allowed, sorry").as_secs().to_string(),
            String::from("-"),
            runame.clone(),
            String::from(".txt"),
        ].concat());
    }

    if verbos {
        println!("creating output file {}", corppath.to_str().expect("invalid characters on file name"));
    }

    let mut corpfile = std::fs::OpenOptions::new().create(true).write(true).append(true).open(corppath).expect("can't create output file");

    let relbrk = regex::Regex::new("(\r)|(<p>)|(</p>)|(<br/>)").unwrap();   // match things that should be line breaks
    let remxml = regex::Regex::new("<[^>]*>").unwrap();                     // match XML tags
    let remtrl = regex::Regex::new("([ \t]*$)|([ ]*$)").unwrap();           // match continuous spaces/tabs

    if verbos { println!("walking down {}", arkdir.to_str().expect("invalid path")); }
    

    for epub in globwalk::GlobWalkerBuilder::from_patterns(std::fs::canonicalize(arkdir.as_path()).expect("can't open arkdir").as_path(), &["*.epub"]).build().unwrap() {
        println!("now processing: {}", epub.as_ref().unwrap().path().file_name().unwrap().to_str().unwrap());

        let f = std::fs::OpenOptions::new().read(true).open(epub.unwrap().path()).expect("can't open file!");

        let mut inpub = zip::ZipArchive::new(f).unwrap();

        let mut text = String::new();
        let mut oldline = String::new();

        let mut infiles: Vec<String> = Vec::new();

        for i in 0..inpub.len() {
            let inzip = inpub.by_index(i).expect("can't open one of the archives");
            if ( inzip.name().starts_with("chapter-") | inzip.name().starts_with("Chapter") ) & inzip.name().ends_with(".html") {
                if verbos {
                    println!("included: {}", inzip.name())
                }
                infiles.push(inzip.name().to_owned());
            } else {
                if verbos {
                    println!("excluded: {} ", inzip.name());
                }
            }
        }

        sort_str_slice(&mut infiles);

        for inname in infiles {
            let mut inzip = inpub.by_name(&inname).expect("can't open one of the archives");
            if verbos { println!("processiong {}", inname); }
            inzip.read_to_string(&mut text)?;
            corpfile.write_all(prefix.as_bytes())?;
            for line in text.lines() {
                if checkfor(line) { continue }

                let line = &relbrk.replace_all(line, "\n");
                let line = &remxml.replace_all(line, " ");
                let line = &remtrl.replace_all(line, " ");

                if line.trim().is_empty() { continue }

                if uniqqq {
                    if line.as_ref() == oldline {
                        continue
                    } else {
                        oldline = line.as_ref().to_owned();
                    }
                }
                
                corpfile.write_all(line.as_bytes())?;
            }
            corpfile.write_all(suffix.as_bytes())?;
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
