use clap::Args;
use sqlx::FromRow;
use std::fmt::Display;

#[derive(Debug, Args, FromRow)]
pub struct Movie {
    pub id: i64,
    pub title: String,
    // just store the date as string for now
    // pub watch_date: Option<String>,
    // pub comment: Option<String>,
    // pub rating: Option<f32>,
}

impl Display for Movie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "* {}", self.title)
    }
}
