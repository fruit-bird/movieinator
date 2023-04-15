mod cli;
mod movie;
mod movie_db;

use crate::cli::MovieCLI;
use crate::movie_db::MovieDB;
use clap::Parser;
use std::{env, process};

#[tokio::main]
async fn main() -> sqlx::Result<()> {
    let db_url = match env::var("MOVIE_DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            eprintln!("You must set a `MOVIE_DATABASE_URL` environment variable.\nSee https://github.com/fruit-bird/movieinator#setup for instructions");
            process::exit(1);
        }
    };

    // creating/loading + connecting to the database
    let mut database = MovieDB::new(db_url).await?;

    // parsing command line arguments + running commands on the database
    let cli = MovieCLI::parse();
    cli.run(&mut database).await?;

    Ok(())
}
