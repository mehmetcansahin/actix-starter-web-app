use serde::{Deserialize, Serialize};

pub const SECRET_KEY: &'static str = "SECRET_KEY";

pub const EXP: usize = 360000000000;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}
