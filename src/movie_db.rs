use crate::movie::Movie;
use sqlx::{migrate::MigrateDatabase, Connection, Result, Row, SqliteConnection};

pub struct MovieDB {
    #[allow(dead_code)]
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
                        watch_date TEXT,
                        rating INTEGER,
                        thoughts TEXT
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

    pub async fn add_movie(
        &mut self,
        title: &str,
        watch_date: Option<&str>,
        thoughts: Option<&str>,
        rating: Option<u8>,
    ) -> Result<()> {
        let movie_exists = sqlx::query("SELECT id FROM Movie WHERE title = ?")
            .bind(title)
            .fetch_optional(&mut self.executor)
            .await?;

        if movie_exists.is_some() {
            // confirm insertion if movie with same title exists
            eprintln!("\"{title}\" already exists, do you still want to insert it? (y/[n])");

            let input: String = text_io::read!("{}\n");
            if input.to_lowercase() != "y" && input.to_lowercase() != "yes" {
                return Ok(());
            }
        }

        if let Some(r) = rating {
            if !(0..=5).contains(&r) {
                eprintln!("Rating must be between 0 and 5");
                return Ok(());
            }
        }

        sqlx::query("INSERT INTO Movie (title, watch_date, thoughts, rating) VALUES (?, ?, ?, ?)")
            .bind(title)
            .bind(watch_date)
            .bind(thoughts)
            .bind(rating)
            .execute(&mut self.executor)
            .await?;
        println!("\"{}\" has been added!", title);

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
            println!("`movienator remove --force \"{}\"`\n", title);
            println!("Movies with title \"{}\":", title);
            let _print_movies = sqlx::query_as::<_, Movie>("SELECT * FROM Movie WHERE title = ?")
                .bind(title)
                .fetch_all(&mut self.executor)
                .await?
                .iter()
                .for_each(|movie| println!("{}", movie));
        } else if movie_count == 0 {
            println!("There is no \"{}\". Nothing was deleted", title);
        } else {
            // base case when there is one movie with the given title
            sqlx::query("DELETE FROM Movie WHERE title = ?")
                .bind(title)
                .execute(&mut self.executor)
                .await?;
            println!("\"{}\" has been removed", title);
        }
        Ok(())
    }

    pub async fn remove_all(&mut self) -> Result<()> {
        // TODO: In CLI, have a confirmation of deletion
        //       Make a backup of movies.sqlite before deleting??
        println!(
            "This will delete ALL movies!!\nAre you SURE you want to delete everything? (y/[N])"
        );

        let input: String = text_io::read!("{}\n");
        if input.to_lowercase() == "y" || input.to_lowercase() == "yes" {
            sqlx::query("DELETE FROM Movie")
                .execute(&mut self.executor)
                .await?;
            println!("As for those movies... They NEVER EXISTED.")
        }
        Ok(())
    }

    pub async fn display_movies(
        &mut self,
        title: &str,
        sort: Option<&str>,
        debug: bool,
    ) -> Result<()> {
        let movies = if let Some(col) = sort {
            let order = if col == "title" { "ASC" } else { "DESC" };
            let sort_query = format!(
                "SELECT * FROM Movie WHERE LOWER(title) LIKE ? ORDER BY LOWER({}) {}",
                col, order
            );
            
            sqlx::query_as::<_, Movie>(&sort_query)
                .fetch_all(&mut self.executor)
                .await?
        } else {
            sqlx::query_as::<_, Movie>("SELECT * FROM Movie WHERE LOWER(title) LIKE ?")
                .bind(format!("%{}%", title.to_lowercase()))
                .fetch_all(&mut self.executor)
                .await?
        };

        pager::Pager::new().setup();
        match debug {
            true => movies.iter().for_each(|m| println!("{:?}", m)),
            false => movies.iter().for_each(|m| println!("{}", m)),
        }
        Ok(())
    }

    pub async fn display_all(&mut self, sort: Option<&str>, debug: bool) -> Result<()> {
        let movies = if let Some(col) = sort {
            // Sorting for different values of col
            //      - title: ASC
            //      - watch_date | rating: DESC
            let order = if col == "title" { "ASC" } else { "DESC" };
            let sort_query = format!("SELECT * FROM Movie ORDER BY LOWER({}) {}", col, order);

            sqlx::query_as::<_, Movie>(&sort_query)
                .fetch_all(&mut self.executor)
                .await?
        } else {
            sqlx::query_as::<_, Movie>("SELECT * FROM Movie")
                .fetch_all(&mut self.executor)
                .await?
        };

        let _print_count = self.count_all().await?;
        pager::Pager::new().setup();
        match debug {
            true => movies.iter().for_each(|m| println!("{:?}", m)),
            false => movies.iter().for_each(|m| println!("{}", m)),
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
