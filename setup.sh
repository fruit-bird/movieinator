#!/bin/zsh
# !! THIS SETUP SCRIPT MUST BE RAN IN THE PROJECT ROOT !!

# exit immediately if anything fails
set -e

PATH_TO_DATABASE=~/Documents # ~ CHANGE THIS LINE TO THE DESIRED PATH ~
FULL_PATH_TO_DATABASE="$(realpath "${PATH_TO_DATABASE}")"

# --- Setting the environment variable ---
# Adding the export command to the ~/.zshrc file if it does not exist
if ! env | grep -q "^MOVIE_DATABASE_URL="; then
    # if `MOVIE_DATABASE_URL` does not exist
    MOVIE_DATABASE_URL="sqlite://${PATH_TO_DATABASE}/movies.db"

    # exporting it to the shell
    echo -e "\n# Movienator Database Path (https://github.com/fruit-bird/movieinator)" >> ~/.zshrc
    echo "export MOVIE_DATABASE_URL=\"${MOVIE_DATABASE_URL}\"" >> ~/.zshrc

    # printing to verify
    echo "The MOVIE_DATABASE_URL environment variable is set to: ${MOVIE_DATABASE_URL}" 
fi

# --- Setting up terminal autocompletion ---
FULL_FILE_PATH=$(realpath "./completions/_movienator")
mkdir -p ~/.custom-zsh-completions

if [ ! -f "~/.custom-zsh-completions/_movienator" ]; then
    # this branch needs sudo to execute. trap catches errors if sudo is not enabled
    cp "${FULL_FILE_PATH}" ~/.custom-zsh-completions
    echo "source ~/.custom-zsh-completions/_movienator" >> ~/.zshrc
fi

# Reload the ~/.zshrc file
source ~/.zshrc
