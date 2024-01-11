use std::env;
use std::path::PathBuf;

use msg_pic_extract::*;

fn main() -> anyhow::Result<()> {
    let mut paths = vec![];

    // working on given file
    if let Some(arg) = env::args().nth(1) {
        paths.push(PathBuf::from(arg));
    // all files in directory
    } else {
        paths = crate::read_dir()?;
    }

    for path in &paths {
        if !crate::is_msg_file(path) {
            println!(
                "Error: {} ist keine .msg Datei. Wird nicht bearbeitet.",
                path.display()
            );
            continue;
        }
        let m = Message::new(dbg!(path))?;
        m.extract_attachments()?;
        m.move_msg_to_dest()?;
        m.os_open_target()?
    }
    Ok(())
}
