mod cli;
mod movie;
mod movie_db;

use crate::cli::MovieCLI;
use crate::movie_db::MovieDB;
use clap::Parser;
use std::env;

#[tokio::main]
async fn main() -> sqlx::Result<()> {
    // DATABASE_URL environment variable is set with the desired location
    // let _env_file_contents = dotenv::dotenv().ok();
    let db_url = env::var("MOVIE_DATABASE_URL")
        .expect("You must set a `MOVIE_DATABASE_URL` environment variable. See <readme setup section link> for instructions");

    // creating/loading + connecting to the database
    let mut database = MovieDB::new(db_url).await?;

    // parsing command line arguments + running commands on the database
    let cli = MovieCLI::parse();
    cli.run(&mut database).await?;

    Ok(())
}
