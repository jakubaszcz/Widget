use std::sync::{Mutex, OnceLock};
use crate::types::appdata::appdata::Appdata;

pub static APPDATA: OnceLock<Appdata> = OnceLock::new();
pub static MANIFEST: OnceLock<Mutex<crate::types::manifest::manifest::Manifest>> = OnceLock::new();