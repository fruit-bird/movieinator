use crate::movie_db::MovieDB;
use clap::{Parser, Subcommand};
use sqlx::Result;

#[derive(Debug, Parser)]
#[clap(author, version)]
pub struct MovieCLI {
    #[clap(subcommand)]
    command: MovieOptions,
    // TODO: Clarify that you need to set a DATABASE_URL environment variable
}

impl MovieCLI {
    pub async fn run(&self, database: &mut MovieDB) -> Result<()> {
        self.command.parse(database).await?;
        Ok(())
    }
}

#[derive(Debug, Subcommand)]
enum MovieOptions {
    /// Insert a movie
    Add {
        /// Title of the movie
        title: String,
        /// Date when movie was seen (Format: YYYY-MM-DD)
        #[clap(short = 'd')]
        watch_date: Option<String>,
        /// 0 to 5 rating
        #[clap(short)]
        rating: Option<u8>,
        /// Thoughts about the movie
        #[clap(short)]
        thoughts: Option<String>,
    },
    /// Print movie information
    List {
        /// Matches movies with given pattern
        title: Option<String>,
        // TODO: have something that paginates through the entries instead of limiting number to display
        // /// Number of movies to display
        // #[clap(long, short, default_value = "1000")]
        // limit: u32,
        /// Print number of stored movies
        #[clap(long, short, conflicts_with = "title")]
        count: bool,
        /// Print all info about movies
        #[clap(long, short)]
        debug: bool,
    },
    /// Remove a movie
    Remove {
        /// Title of the movie to remove
        #[clap(conflicts_with = "all")]
        title: Option<String>,
        /// Remove *ALL* movies
        #[clap(short, long, conflicts_with = "title")]
        all: bool,
        /// Force removal when multiple movies share a title
        #[clap(short, long, conflicts_with = "all")]
        // IMPROVEMENT: kinda scuffed ngl, should be able to choose which movie to remove
        //              instead of removing everything or nothing
        force: bool,
    },
}

impl MovieOptions {
    async fn parse(&self, database: &mut MovieDB) -> Result<()> {
        match self {
            MovieOptions::Add {
                title,
                watch_date,
                thoughts,
                rating,
            } => {
                database
                    .add_movie(&title, watch_date.as_deref(), thoughts.as_deref(), *rating)
                    .await?
            }
            MovieOptions::List {
                title,
                count,
                debug,
            } => match title {
                Some(ref t) => database.display_movies(t, *debug).await?,
                None => match count {
                    true => _ = database.count_all().await?,
                    false => database.display_all(*debug).await?,
                },
            },
            MovieOptions::Remove { title, all, force } => match all {
                true => database.remove_all().await?,
                false => {
                    if let Some(ref t) = title {
                        database.remove_movie(t, *force).await?;
                    }
                }
            },
        }
        Ok(())
    }
}
