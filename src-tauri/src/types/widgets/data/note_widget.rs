#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct NoteData {
    pub content: String,
}