use msg_parser::Outlook;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use anyhow::Result;

// path to run on every .msg file when not provided with path/tp/file.msg argument
pub const DEFAULT_PATH: &str = "."; 

// in OS folder create_temp_dir creates directories to store the attachments in. 
// This constant defines the number of hex-characters 
const LEN_TEMP_DIR_HEX: usize = 12; 

pub struct Message {
    parser: Outlook,
    origin: PathBuf,
    dest: PathBuf,
}

impl Message {
    pub fn new(path: &PathBuf) -> Result<Self, anyhow::Error> {
        let origin = path.clone();
        let parser = Outlook::from_path(path)?;
        let dest = crate::create_temp_dir()?;

        Ok(Message {
            parser,
            origin,
            dest,
        })
    }

    pub fn extract_attachments(&self) -> anyhow::Result<()> {
        for attachment in &self.parser.attachments {
            if attachment.file_name.len() > 3 {
                // decode from hexadecimal to bytes
                let buf = hex::decode(&attachment.payload).unwrap();

                let filename = self.dest.join(&attachment.display_name);
                std::fs::write(&filename, buf)?;
            }
        }
        Ok(())
    }

    pub fn move_msg_to_dest(&self) -> io::Result<()> {
        fs::copy(
            &self.origin,
            self.dest.join(self.origin.file_name().unwrap()),
        )?;
        if !cfg!(debug_assertions) {
            fs::remove_file(&self.origin)?;
        }
        Ok(())
    }

    pub fn os_open_target(&self) -> io::Result<()> {
        open::that(&self.dest)
    }
}

pub fn is_msg_file(path: &Path) -> bool {
    path.is_file() || path.extension() == Some(OsStr::new("msg"))
}

pub fn read_dir() -> Result<Vec<PathBuf>, io::Error> {
    let mut entries = fs::read_dir(DEFAULT_PATH)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    entries = entries
        .into_iter()
        .filter(|path| path.is_file() && path.extension() == Some(OsStr::new("msg")))
        .collect::<Vec<PathBuf>>();

    Ok(entries)
}

fn create_temp_dir() -> Result<PathBuf, io::Error> {
    // create folder inside a random hex folder inside OS' temp-dir
    let mut out_path = env::temp_dir();
    out_path.push(get_rnd_hex(LEN_TEMP_DIR_HEX));
    match create_dir(&out_path) {
        Ok(_) => Ok(out_path),
        Err(e) => Err(e),
    }
}

fn create_dir(path: &PathBuf) -> io::Result<()> {
    if fs::read_dir(path).is_err() {
        fs::create_dir_all(path)?
    }
    Ok(())
}

fn get_rnd_hex(len: usize) -> String {
    let mut generated_string = String::new();
    for _ in 0..len {
        generated_string.push(rand::random::<char>());
    }
    let mut hex = hex::encode(generated_string);
    hex.truncate(len.into());
    hex
}
