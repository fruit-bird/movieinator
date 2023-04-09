#!/bin/zsh

# ~~ CHANGE THIS LINE! ~~
PATH_TO_DATABASE="/Users/<username>/Documents"

# Setting the env variable
# modify the path to `movie.sqlite` to where you want to store it
MOVIE_DATABASE_URL="sqlite://$PATH_TO_DATABASE/movies.db"

# Add the export command to the ~/.zshrc file
echo -e "\n# Movienator Database Path (https://github.com/fruit-bird/movieinator)" >> ~/.zshrc
echo "export MOVIE_DATABASE_URL=\"$MOVIE_DATABASE_URL\"" >> ~/.zshrc

# Reload the ~/.zshrc file
source ~/.zshrc

# Verify that the environment variable is set
echo "The MOVIE_DATABASE_URL environment variable is set to: $MOVIE_DATABASE_URL"
