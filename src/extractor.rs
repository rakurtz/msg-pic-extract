use msg_parser::Outlook;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use anyhow::Result;

pub struct Extractor {
    pub temp_dir: PathBuf,
    message_paths: Vec<PathBuf>,
}

impl Extractor {
    pub fn new() -> Self {
        Extractor {
            temp_dir: self::create_temp_dir()
                .expect("Error: Couldn't create temporary folder. Exiting."),
            message_paths: vec![],
        }
    }

    pub fn open_temp(&self) -> io::Result<()> {
        open::that(&self.temp_dir)
    }

    pub fn run(&mut self) -> Result<(), anyhow::Error> {
        self.read_dir()?;
        for path in &self.message_paths {
            if !is_msg_file(path) {
                continue;
            }
            let message = Message::new(path)?;
            message.extract_attachments()?;
            message.move_msg_to_dest()?;
        }
        Ok(())
    }

    fn read_dir(&mut self) -> Result<(), io::Error> {
        let entries = fs::read_dir(&self.temp_dir)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;
        self.message_paths = entries
            .into_iter()
            .filter(|path| path.is_file() && path.extension() == Some(OsStr::new("msg")))
            .collect::<Vec<PathBuf>>();

        Ok(())
    }

    pub fn clean_up(&self) -> Result<(), io::Error> {
        fs::remove_dir_all(&self.temp_dir)
    }
}

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
        let dest = parent.join(truncated_unique_mail_identifier(
            &parser.sender.name,
            &parser.subject,
        ));
        create_dir(&dest)?;

        Ok(Message { parser, file, dest })
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
        fs::copy(&self.file, self.dest.join(self.file.file_name().unwrap()))?;
        fs::remove_file(&self.file)?;
        Ok(())
    }
}

// helper functions

fn is_msg_file(path: &Path) -> bool {
    path.is_file() || path.extension() == Some(OsStr::new("msg"))
}

fn create_temp_dir() -> Result<PathBuf, io::Error> {
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
