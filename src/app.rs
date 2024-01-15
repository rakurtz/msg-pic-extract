use std::{path::PathBuf, fs, io};
use crate::extractor;

pub struct ExtractorApp {
    temp_path: PathBuf,
}

impl Default for ExtractorApp {
    fn default() -> Self {
        Self {
            temp_path: crate::extractor::create_temp_dir().unwrap(),
        }
    }
}

impl ExtractorApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for ExtractorApp {
    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Outlook Message Extractor");
            ui.add_space(12.0);
            ui.label("Extrahiert alle Anhänge aus .msg-Dateien und legt diese in einem Order ab.");
            ui.add_space(18.0);
            ui.label("Anleitung:");
            ui.label("1. Temporären Ordner öffnen");
            ui.label("2. .msg-Datei aus Outlook hineinziehen (drag and drop)");
            ui.label("3. Umwandeln klicken");
            ui.add_space(30.0);

            ui.horizontal(|ui| {
                if ui.button("Temporären Ordner öffnen").clicked() {
                    let _ = open::that(&self.temp_path);
                }
                ui.add_space(12.0);
                if ui.button("Umwandeln!").clicked() {
                    let _ = self.extract();
                }

            });
          
            ui.add_space(30.0);
            if ui.button("Auräumen und Beenden").clicked() {
                // todo: Afräumen und dann scnließen
                if let Err(e) = self.clean_up() {
                    eprintln!("{}", e);
                }
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }

        });
    }
}


impl ExtractorApp {
    fn extract(&self) -> anyhow::Result<()> {
        let paths = extractor::read_dir(&self.temp_path).unwrap();
        for path in &paths {
            if !extractor::is_msg_file(path) {
                eprintln!(
                    "Error: {} ist keine .msg Datei. Wird nicht bearbeitet.",
                    path.display()
                );
                continue;
            }
            let m = extractor::Message::new(dbg!(path))?;
            m.extract_attachments()?;
            m.move_msg_to_dest()?;
        }
        Ok(())
    }

    fn clean_up(&self) -> io::Result<()> {
        fs::remove_dir_all(&self.temp_path)
    }
}