use msg_parser::Outlook;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use anyhow::Result;


pub struct Message {
    parser: Outlook,
    file: PathBuf,
    dest: PathBuf,
}

impl Message {
    pub fn new(path: &PathBuf) -> Result<Self, anyhow::Error> {
        let file = path.clone();
        let parent = file.parent().unwrap();
        let parser = Outlook::from_path(path)?;
        let dest = parent.join(truncated_unique_mail_identifier(&parser.sender.name, &parser.subject));
        create_dir(&dest)?;

        Ok(Message {
            parser,
            file,
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
            &self.file,
            self.dest.join(self.file.file_name().unwrap()),
        )?;
        fs::remove_file(&self.file)?;
        Ok(())
    }

    pub fn os_open_target(&self) -> io::Result<()> {
        open::that(&self.dest)
    }
}

pub fn is_msg_file(path: &Path) -> bool {
    path.is_file() || path.extension() == Some(OsStr::new("msg"))
}

pub fn read_dir(path: &Path) -> Result<Vec<PathBuf>, io::Error> {
    let mut entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    entries = entries
        .into_iter()
        .filter(|path| path.is_file() && path.extension() == Some(OsStr::new("msg")))
        .collect::<Vec<PathBuf>>();

    Ok(entries)
}

pub fn create_temp_dir() -> Result<PathBuf, io::Error> {
    // create folder inside a random hex folder inside OS' temp-dir
    let mut out_path = env::temp_dir();
    let mut folder_name = get_rnd_hex(5);
    folder_name.push_str("_msg_extractor");
    out_path.push(folder_name);
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

fn truncated_unique_mail_identifier(name: &str, subject: &str) -> PathBuf {
    let prefix = get_rnd_hex(5);

    let name = match name.len() {
        0 => "Unbekannt",
        1..=15 => name,
        _ => &name[0..15],
    };    
    let subject = match subject.len() {
        0 => "mail_ohne_Betreff",
        1..=25 => subject,
        _ => &subject[0..25],
    };
    
    let identifer = format!("{}_{}_{}", prefix, name, subject);
    PathBuf::from(identifer)
}

fn get_rnd_hex(len: usize) -> String {
    let mut generated_string = String::new();
    for _ in 0..len {
        generated_string.push(rand::random::<char>());
    }
    let mut hex = hex::encode(generated_string);
    hex.truncate(len);
    hex
}
