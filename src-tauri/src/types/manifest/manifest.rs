use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]

pub struct Vector2 {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Manifest {
    pub widgets: Vec<ManifestWidget>
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]

pub struct ManifestWidget {
    pub id: String,
    pub widget_type: String,
    pub size: Vector2,
    pub position: Vector2,
    pub data: PathBuf
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            widgets: Vec::new(),
        }
    }
}