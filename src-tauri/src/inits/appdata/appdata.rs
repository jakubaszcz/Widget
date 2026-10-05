use directories::ProjectDirs;
use crate::types::appdata::appdata::Appdata;

const QUALIFIER: &str = "com";
const ORGANISATION: &str = "widget";
const APPLICATION: &str = "widget";

pub fn init() -> Appdata {
    let directories = ProjectDirs::from(
        QUALIFIER,
        ORGANISATION,
        APPLICATION,
    ).unwrap();

    let data = directories.data_dir().to_path_buf();
    let cache = directories.cache_dir().to_path_buf();

    println!("data: {:?}", &data);

    Appdata { data, cache }
}