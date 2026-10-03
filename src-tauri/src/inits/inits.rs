use std::sync::Mutex;
use crate::global::global::{APPDATA, MANIFEST};
use crate::inits;
use crate::inits::appdata::appdata;

pub fn inits() {
    APPDATA.get_or_init(appdata::init);
    MANIFEST.set(Mutex::new(inits::manifest::manifest::init().expect("Unable to load application manifest"))).unwrap();
}