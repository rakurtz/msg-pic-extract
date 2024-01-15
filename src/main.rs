use std::{sync::Mutex, rc::Rc};
use msg_extractor::extractor::Extractor;


pub fn main() {
    let slint_app = SlintApp::new().unwrap();
    let extractor = Rc::new(Mutex::new(Extractor::new()));
    let extractor1 = extractor.clone();
    let extractor2 = extractor.clone();

    slint_app.on_button_tmp_pressed( move || {
        extractor.lock().unwrap().open_temp().expect("Error: Couldn't open temporary folder.");
    });

    
    slint_app.on_button_extract_pressed( move || {
        if let Err(e) = extractor1.lock().unwrap().run() {
            eprintln!("Error: Extraction didn't work: {:?}", e)
        }
    });

    slint_app.on_button_exit_pressed( move || {
        match extractor2.lock().unwrap().clean_up() {
            Err(e) => eprintln!("Error: Couldn't clean up temporary folder {:?}", e),
            Ok(_) => {
                println!("Good bye!");
                exit();
            }
        }
    }); 

    slint_app.run().unwrap();
 
}

fn exit() {
    println!("Button exit clicked");
    std::process::exit(0);
}


slint::slint! {

    import { Button, VerticalBox } from "std-widgets.slint";
    export component SlintApp inherits Window {
        callback button-tmp-pressed <=> btn_tmp.clicked;
        callback button-extract-pressed <=> btn_extract.clicked;
        callback button-exit-pressed <=> btn_exit.clicked;
        
        VerticalBox {
            alignment: start;
            Text {
                text: "Message Extractor";
                font-size: 22px;
                horizontal-alignment: left;
            }
            Text {text: " ";}
            Text {
                text: "Anleitung:";
            }
            Text {text: "1. Temporären Ordner öffnen";}
            Text {text: "2. .msg-Dateien per drag & drop hineinziehen";}
            Text {text: "3. \"Umwandeln!\" klicken";}
            Text {text: " ";}

            HorizontalLayout { 
                btn_tmp := Button { text: "temporären Ordner öffnen"; }
                btn_extract := Button { text: "Umwandeln!"; } 
            }
            btn_exit := Button { text: "Berenigen und Beenden"; }
        }
    }
}