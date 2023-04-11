use clap::Args;
use sqlx::FromRow;
use std::fmt::Display;

#[derive(Debug, Args, FromRow)]
pub struct Movie {
    pub id: i64,
    pub title: String,
    // just store the date as a string for now
    // possibly convert to chrono::NaiveDate
    // if wanting to allow sorting by date when listing
    pub watch_date: Option<String>,
    pub thoughts: Option<String>,
    pub rating: Option<u8>,
}

impl Display for Movie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title_width = 40;
        let default_width = 10;

        let title = match self.title.len() > title_width - 3 {
            true => format!(
                "{}...", // .unwrap_or() to avoid panic if something odd occurs
                self.title.get(..title_width - 4).unwrap_or("[DISPLAY ERR]")
            ),
            false => self.title.to_string(),
        };

        let watch_date = self
            .watch_date
            .as_ref()
            .map_or("_".repeat(default_width - 2), |wd| wd.to_string());

        let rating = self.rating.map_or("_".repeat(3), |r| format!("{}/5", r));

        let thoughts = self
            .thoughts
            .as_ref()
            .map_or("(no thoughts)", |s| s.as_str());

        write!(
            f,
            "* | {:4$}| {:^date_width$} |{:^rating_width$}| {:^}",
            title,
            watch_date,
            rating,
            thoughts,
            title_width, // 4th positional argument
            date_width = default_width + 2,
            rating_width = default_width - 1,
        )
    }
}
