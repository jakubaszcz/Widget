use std::thread::sleep;
use crate::global::global::{APPDATA, MANIFEST};
use std::time::{Duration, SystemTime};
use crate::inits;

pub fn auto_save() {
    std::thread::spawn(move || {
        let mut previous = MANIFEST.get().unwrap().lock().unwrap().clone();
        let data = &APPDATA.get().unwrap().data;

        loop {
            sleep(Duration::new(5, 0));
            let new = MANIFEST.get().unwrap().lock().unwrap().clone();
            if new != previous {
                println!("Data changed");
                match inits::manifest::manifest::write(&data.join("manifest.json"), &new) {
                    Ok(()) => previous = new,
                    Err(error) => eprintln!("Erreur de sauvegarde : {error}"),
                }
            } else {
                println!("No changes data wont move");
            }
        }
    });
}