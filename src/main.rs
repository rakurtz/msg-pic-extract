#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use msg_extractor::extractor::Extractor;
use std::path::Path;
use std::{fs, rc::Rc, sync::Mutex};

pub fn main() {
    let slint_app = SlintApp::new().unwrap();
    let extractor = Rc::new(Mutex::new(Extractor::new()));
    let tmp_folder = extractor.lock().unwrap().temp_dir.clone();
    write_licence_file(&tmp_folder);
    let extractor1 = extractor.clone();
    let extractor2 = extractor.clone();

    slint_app.on_button_tmp_pressed(move || {
        extractor
            .lock()
            .unwrap()
            .open_temp()
            .expect("Error: Couldn't open temporary folder.");
    });

    slint_app.on_button_extract_pressed(move || {
        if let Err(e) = extractor1.lock().unwrap().run() {
            eprintln!("Error: Extraction didn't work: {:?}", e)
        }
    });

    slint_app.on_button_exit_pressed(move || match extractor2.lock().unwrap().clean_up() {
        Err(e) => eprintln!("Error: Couldn't clean up temporary folder {:?}", e),
        Ok(_) => {
            println!("Good bye!");
            std::process::exit(0);
        }
    });

    slint_app.on_show_info(move || {
        if let Err(e) = open::that(tmp_folder.join("Lizenz.html")) {
            eprintln!("Error: Couldn't open licence file in Browser. {:?}", e);
        }
    });

    slint_app.run().unwrap();
}

fn write_licence_file(tmp_folder: &Path) {
    if let Err(e) = fs::write(
        tmp_folder.join("Lizenz.html"),
        msg_extractor::licence::LICENCE,
    ) {
        eprintln!(
            "Error: Couldn't write licence file to temporary folder. {:?}",
            e
        );
    }
}

slint::slint! {

    import { Button, VerticalBox, ScrollView } from "std-widgets.slint";

    component InfoButton inherits Rectangle {
        in-out property text <=> txt.text;
        callback clicked <=> touch.clicked;
        border-radius: root.height / 3;
        border-width: 1px;
        border-color: root.background.darker(25%);
        background: touch.pressed ? #6b8282 : touch.has-hover ? #6c616c :  #456;
        height: txt.preferred-height * 1.33;
        min-width: txt.preferred-width + 20px;
    txt := Text {
        x: (parent.width - self.width)/2 + (touch.pressed ? 2px : 0);
        y: (parent.height - self.height)/2 + (touch.pressed ? 1px : 0);
        color: touch.pressed ? #fff : #eee;
    }
        touch := TouchArea { }
    }


    export component SlintApp inherits Window {
        callback button-tmp-pressed <=> btn_tmp.clicked;
        callback button-extract-pressed <=> btn_extract.clicked;
        callback button-exit-pressed <=> btn_exit.clicked;
        callback show-info <=> info.clicked;

        VerticalBox {
            alignment: start;
            HorizontalLayout {
                spacing: 10px;
                Text {
                    text: "MSG-Extractor";
                    font-size: 22px;
                    horizontal-alignment: left;
                }
                info := InfoButton {
                    text: "Info";
                    preferred-height: 10px;
                    //text-size: 11px;
                }
            }

            Text {text: " ";}
            Text {
                text: "Anleitung:";
            }
            Text {text: "1. Temporären Ordner öffnen";}
            Text {text: "2. .msg-Dateien per drag & drop hineinziehen";}
            Text {text: "3. \"Extrahieren!\" klicken";}
            Text {text: " ";}

            HorizontalLayout {
                btn_tmp := Button { text: "Temporären Ordner öffnen"; }
                btn_extract := Button { text: "Extrahieren!"; }
            }
            btn_exit := Button { text: "Bereinigen und Beenden"; }
        }
    }

}
