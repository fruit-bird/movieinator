mod cli;
mod movie;
mod movie_db;

use crate::cli::MovieCLI;
use crate::movie_db::MovieDB;
use clap::Parser;
use std::env;

#[tokio::main]
async fn main() -> sqlx::Result<()> {
    let db_url = match env::var("MOVIE_DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            eprintln!("You must set a `MOVIE_DATABASE_URL` environment variable.\nSee https://github.com/fruit-bird/movieinator#setup for instructions");
            return Ok(());
        }
    };

    // creating/loading + connecting to the database
    let mut database = MovieDB::new(db_url).await?;

    // parsing command line arguments + running commands on the database
    let cli = MovieCLI::parse();
    cli.run(&mut database).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const ADD_MOVIE: [&str; 3] = ["movienator_test", "add", "Cloud Atlas"];
    const REMOVE_MOVIE: [&str; 3] = ["movienator_test", "remove", "Cloud Atlas"];
    const LIST_ALL: [&str; 2] = ["movienator_test", "list"];

    #[tokio::test]
    async fn add_movie() -> anyhow::Result<()> {
        let _load_dotenv_vars = dotenv::dotenv().ok();
        let db_url = env::var("TEST_DATABASE_URL")?;

        let mut database = MovieDB::new(db_url).await?;

        let cli = MovieCLI::parse_from(ADD_MOVIE);
        cli.run(&mut database).await?;

        Ok(())
    }

    #[tokio::test]
    async fn remove_movie() -> anyhow::Result<()> {
        let _load_dotenv_vars = dotenv::dotenv().ok();
        let db_url = env::var("TEST_DATABASE_URL")?;

        let mut database = MovieDB::new(db_url).await?;

        let cli = MovieCLI::parse_from(REMOVE_MOVIE);
        cli.run(&mut database).await?;

        Ok(())
    }

    #[tokio::test]
    async fn list_movies() -> anyhow::Result<()> {
        let _load_dotenv_vars = dotenv::dotenv().ok();
        let db_url = env::var("TEST_DATABASE_URL")?;

        let mut database = MovieDB::new(db_url).await?;

        let cli = MovieCLI::parse_from(LIST_ALL);
        cli.run(&mut database).await?;

        Ok(())
    }
}
