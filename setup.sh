#!/bin/zsh
# !! THIS SETUP SCRIPT IS AUTOMATICALLY RAN WHEN BUILDING THE PROJECT !!

# exit immediately if anything fails
set -e

PATH_TO_DATABASE=~/Documents # ~ CHANGE THIS LINE TO THE DESIRED PATH ~

# ------------------ SETTING THE ENVIRONMENT VARIABLE ------------------
# Adding the export command to the ~/.zshrc file if it does not exist
if [ -z ${MOVIE_DATABASE_URL} ]; then
    MOVIE_DB_URL="sqlite:///${PATH_TO_DATABASE}/movies.db"

    # exporting it to the shell
    # these 2 lines keep getting executed no matter what
    echo "\n# Movienator Database Path (https://github.com/fruit-bird/movieinator)" >> ~/.zshrc
    echo "export MOVIE_DATABASE_URL=\"${MOVIE_DB_URL}\"" >> ~/.zshrc
fi
# ------------------ SETTING THE ENVIRONMENT VARIABLE ------------------


# ------------------ SETTING UP TERMINAL AUTOCOMPLETION ------------------
GLOBAL_COMPLETIONS_PATH=${HOME}/.custom-zsh-completions/_movienator.sh

if [[ ! -e ${GLOBAL_COMPLETIONS_PATH} ]]; then
    FULL_FILE_PATH=$(realpath "./completions/_movienator.sh")
    mkdir -p ~/.custom-zsh-completions
    cp "${FULL_FILE_PATH}" ~/.custom-zsh-completions

    echo "source ~/.custom-zsh-completions/_movienator.sh" >> ~/.zshrc
fi
# ------------------ SETTING UP TERMINAL AUTOCOMPLETION ------------------


# Reloading the ~/.zshrc file
source ~/.zshrc
echo "The MOVIE_DATABASE_URL environment variable is set to:\t\"${MOVIE_DATABASE_URL}\"" 
echo "The terminal tab completions file is stored in:\t\t\"${GLOBAL_COMPLETIONS_PATH}\""
