use crate::movie::Movie;
use sqlx::{migrate::MigrateDatabase, Connection, Result, Row, SqliteConnection};
use std::fmt::Debug;

// TODO: Adapt DB to the extra columns added to the Movie TABLE
pub struct MovieDB {
    db_url: String,
    executor: SqliteConnection,
}

impl MovieDB {
    pub async fn new(db_url: String) -> Result<Self> {
        let database = if !sqlx::Sqlite::database_exists(&db_url).await? {
            // creating the database if it does not exist
            sqlx::Sqlite::create_database(&db_url).await?;

            let executor = SqliteConnection::connect(&db_url).await?;
            let mut database = Self { db_url, executor };

            sqlx::query(
                "CREATE TABLE IF NOT EXISTS Movie (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        title TEXT NOT NULL,
                        -- watch_date TEXT,
                        -- comment TEXT,
                        -- rating REAL
                    );",
            )
            .execute(&mut database.executor)
            .await?;

            database
        } else {
            let executor = SqliteConnection::connect(&db_url).await?;
            Self { db_url, executor }
        };

        Ok(database)
    }

    pub async fn add_movie(&mut self, title: &str) -> Result<()> {
        // DONE! IMPROVEMENT: In CLI, if movie exists, display it and ask if you still want to add it
        //              in case of similar movie titles

        let exists = sqlx::query("SELECT * FROM Movie WHERE title = ?")
            .bind(title)
            .fetch_optional(&mut self.executor)
            .await?;

        if exists.is_none() {
            // insert if movie does not exist
            sqlx::query("INSERT INTO Movie (title) VALUES (?)")
                .bind(title)
                .execute(&mut self.executor)
                .await?;
            println!("\"{}\" has been added!", title);
        } else {
            // confirm insertion if movie with same title exists
            println!(
                "\"{}\" already exists, do you still want to insert it? (y/[N])",
                title
            );
            let input: String = text_io::read!("{}\n");
            if input.to_lowercase() == "y" || input.to_lowercase() == "yes" {
                sqlx::query("INSERT INTO Movie (title) VALUES (?)")
                    .bind(title)
                    .execute(&mut self.executor)
                    .await?;
            }
        }
        Ok(())
    }

    pub async fn remove_movie(&mut self, title: &str, force: bool) -> Result<()> {
        let movie_count = sqlx::query("SELECT COUNT(*) AS num FROM Movie WHERE title = ?")
            .bind(title)
            .fetch_one(&mut self.executor)
            .await?
            .get::<i32, _>("num");

        if movie_count > 1 && !force {
            println!("There are multiple movies with the same title. To delete them all, try:");
            println!("`movie-db remove --force \"{}\"`\n", title);
            println!("Movies with title \"{}\":", title);
            let _movies_print = sqlx::query("SELECT title FROM Movie WHERE title = ?")
                .bind(title)
                .fetch_all(&mut self.executor)
                .await?
                .iter()
                .enumerate()
                // TODO: have some function (or method from Movie struct) that prints out details about a movie
                //       it makes no sense to just print the same title multiple times
                .for_each(|(i, movie)| println!("{} - {}", i + 1, movie.get::<String, _>("title")));
        } else if movie_count == 0 {
            println!("There is no \"{}\", thus nothing was deleted", title);
        } else {
            // base case when there is one movie with the given title
            sqlx::query("DELETE FROM Movie WHERE title = ?")
                .bind(title)
                .execute(&mut self.executor)
                .await?;
            println!("{} was removed", title);
        }
        Ok(())
    }

    pub async fn remove_all(&mut self) -> Result<()> {
        // TODO: In CLI, have a confirmation of deletion
        //       Make a backup of movies.sqlite before deleting
        println!("This will delete ALL stored movies\nAre you sure you want to delete everything? (y/[N])");

        let input: String = text_io::read!("{}\n");
        if input.to_lowercase() == "y" || input.to_lowercase() == "yes" {
            sqlx::query("DELETE FROM Movie")
                .execute(&mut self.executor)
                .await?;

            println!("As for those movies... They NEVER EXISTED.")
        }
        Ok(())
    }

    pub async fn display_movie(&mut self, title: &str, debug: bool) -> Result<()> {
        let movies =
            sqlx::query_as::<_, Movie>("SELECT id, title FROM Movie WHERE LOWER(title) LIKE ?")
                .bind(format!("%{}%", title.to_lowercase()))
                .fetch_all(&mut self.executor)
                .await?;

        if debug {
            for movie in movies {
                println!("{:?}", movie)
            }
        } else {
            // TODO: have some function (or method from Movie struct) that prints out details about a movie
            for movie in movies {
                println!("{}", movie)
            }
        }
        Ok(())
    }

    pub async fn display_all(&mut self, debug: bool) -> Result<()> {
        let movies = sqlx::query_as::<_, Movie>("SELECT id, title FROM Movie")
            .fetch_all(&mut self.executor)
            .await?;

        if debug {
            let _print_count = self.count_all().await?;
            for movie in movies {
                println!("{:?}", movie)
            }
        } else {
            let _print_count = self.count_all().await?;
            // TODO: have some function (or method from Movie struct) that prints out details about a movie
            for movie in movies {
                println!("{}", movie)
            }
        }
        Ok(())
    }

    pub async fn count_all(&mut self) -> Result<i32> {
        let count = sqlx::query_scalar::<_, i32>("SELECT COUNT(id) FROM Movie")
            .fetch_one(&mut self.executor)
            .await?;

        if count == 1 {
            println!("You have 1 movie stored");
        } else {
            println!("You have {} movies stored", count);
        }
        Ok(count)
    }
}

impl Debug for MovieDB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MovieDB")
            .field("db_url", &self.db_url)
            .finish()
    }
}
