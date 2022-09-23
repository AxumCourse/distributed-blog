use serde::Deserialize;

#[derive(Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}
