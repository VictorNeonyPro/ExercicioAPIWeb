use std::fmt::Display;
use serde::{Deserialize, Serialize};
use crate::models::date::Date;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    name: String,
    cpf: i32,
    date: Date
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}) (created {})", self.name, self.cpf, self.date)
    }
}
