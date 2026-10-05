use std::fs;
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};
use crate::global::global::{APPDATA, MANIFEST};
use crate::types::manifest::manifest::Manifest;

pub fn write_widget_manifest<T>(path: &Path, data: &T) -> Result<(), String>
where
    T: serde::Serialize + serde::de::DeserializeOwned + Clone,
{
    let bytes = serde_json::to_vec_pretty(data).map_err(|e| e.to_string())?;
    let mut file = tempfile::NamedTempFile::new_in(path.parent().ok_or("Missing parent directory")?)
        .map_err(|e| e.to_string())?;
    file.write_all(&bytes).map_err(|e| e.to_string())?;
    file.as_file().sync_all().map_err(|e| e.to_string())?;
    file.persist(path).map_err(|e| e.to_string())?;
    Ok(())
}

fn load<T>(path: &PathBuf, data: &T) -> Result<T, String>
where
    T: serde::Serialize + serde::de::DeserializeOwned + Clone,
{
    match fs::read(path) {
        Ok(contents) => serde_json::from_slice::<T>(&contents)
            .map_err(|err| err.to_string()),

        Err(err) if err.kind() == ErrorKind::NotFound => {
            write_widget_manifest(path, data)?;
            Ok(data.clone())
        }

        Err(err) => Err(err.to_string()),
    }
}

pub fn init<T>(id: String, data: &T) -> Result<T, String>
where
    T: serde::Serialize + serde::de::DeserializeOwned + Clone,
{
    let cache = &APPDATA.get().unwrap().cache;

    fs::create_dir_all(cache)
        .map_err(|err| err.to_string())?;

    let path = cache.join(format!("{id}.json"));
    let loaded = load(&path, data)?;

    let mut manifest = MANIFEST.get().unwrap().lock()
        .map_err(|err| err.to_string())?;

    if let Some(widget) = manifest.widgets.iter_mut()
        .find(|widget| widget.id == id)
    {
        widget.data = path;
    }

    Ok(loaded)
}