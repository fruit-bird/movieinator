# Movienator

A CLI to store movie titles

Uses an `sqlite` database as means of storage

## Usage
```
Usage: movienator <COMMAND>

Commands:
  add     Add a movie entry
  list    Print movie info
  remove  Remove a movie
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Setup
The only prerequisite is having a `MOVIE_DATABASE_URL` environment variable. The [`setup.sh`](setup.sh) script will do that, though it may need some altering

Replace `~/.zshrc` with `~/.bash_profile` everywhere in the file if you don't have `zsh`

Execute with
```bash
$ zsh setup.sh # or bash
$ echo $MOVIE_DATABASE_URL
  sqlite:////Users/<usename>/Documents/movies.db
```

## Example
```bash
$ movienator add "Cloud Atlas"
  "Cloud Atlas" has been added!
```

## What is This?
Jusr trying to pick up [`sqlx`](https://crates.io/crates/sqlx)

And for how simple this is, it could be generalized to store more than movies. It really could store anything with a few modifications
