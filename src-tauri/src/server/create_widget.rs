#[derive(Debug, serde::Deserialize)]
pub struct RegisterWidget {
    pub id: String,
    pub title: String,
    pub width: u32,
    pub height: u32,
}