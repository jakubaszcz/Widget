#[derive(serde::Deserialize, Debug, Clone)]
pub struct WidgetSize {
    pub x: i32,
    pub y: i32,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct CreateWidget {
    pub size: Option<WidgetSize>,
    pub widget: String,
}