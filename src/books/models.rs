use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct CreatedBookData {
    pub name: String,
    pub publisher: String,
}

#[derive(Deserialize, Clone)]
pub struct UpdateBookData {
    pub name: String,
    pub publisher: String,
}
