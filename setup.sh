#!/bin/zsh

# Setting the env variable
# modify the path to `movie.sqlite` to where you want to store it
MOVIE_DATABASE_URL='sqlite:///Users/<username>/Documents/movies.db'

# Add the export command to the ~/.zshrc file
echo "# Movienator database path (<repo link here>)" >> ~/.zshrc
echo "export MOVIE_DATABASE_URL=\"$MOVIE_DATABASE_URL\"" >> ~/.zshrc

# Reload the ~/.zshrc file
source ~/.zshrc

# Verify that the environment variable is set
echo "The MOVIE_DATABASE_URL environment variable is set to: $MOVIE_DATABASE_URL"
