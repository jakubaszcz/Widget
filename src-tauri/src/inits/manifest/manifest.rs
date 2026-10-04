use std::fmt::format;
use std::fs;
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};
use crate::global::global::{APPDATA, MANIFEST};
use crate::types::manifest::manifest::{Manifest, ManifestWidget};

pub fn write(path: &Path, manifest: &Manifest) -> Result<(), String> {
    println!("Writing manifest {}", path.display());
    let bytes = serde_json::to_vec_pretty(manifest).map_err(|e| e.to_string())?;
    let mut file = tempfile::NamedTempFile::new_in(path.parent().ok_or("Missing parent directory")?)
        .map_err(|e| e.to_string())?;
    file.write_all(&bytes).map_err(|e| e.to_string())?;
    file.as_file().sync_all().map_err(|e| e.to_string())?;
    file.persist(path).map_err(|e| e.to_string())?;
    Ok(())
}
fn load(path: &PathBuf) -> Result<Manifest, String> {
    match fs::read(path) {
        Ok(contents) => serde_json::from_slice::<Manifest>(&contents)
            .map_err(|err| err.to_string()),
        Err(err) if err.kind() == ErrorKind::NotFound => {
            let manifest = Manifest::default();
            write(path, &manifest)?;
            Ok(manifest)
        }
        Err(err) => Err(err.to_string()),
    }
}
pub fn init() -> Result<Manifest, String> {
    let data = &APPDATA.get().unwrap().data;

    fs::create_dir_all(data)
        .map_err(|err| format!("{err}"))?;

    load(&data.join("manifest.json"))
}

pub fn add_widget(widget: ManifestWidget) -> Result<(), String> {
    let mut current = MANIFEST
        .get()
        .ok_or("Manifest not initialized")?
        .lock()
        .map_err(|e| e.to_string())?;

    let mut next = current.clone();
    next.widgets.push(widget);

    let appdata = APPDATA.get().ok_or("Appdata not initialized")?;
    write(&appdata.data.join("manifest.json"), &next)?;

    *current = next;

    Ok(())
}