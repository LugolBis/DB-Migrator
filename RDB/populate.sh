#!/bin/bash

# That bash script fin the .csv files and use them to pouplate the Database

function populate {
    DIR=$(pwd)
    declare -A FILES
    liste=$(ls *.csv)

    for f in $liste; do
        path="$DIR/$f"
        FILES["${f%.csv}"]="$path"
    done

    psql -d bibliotheque -c "\copy auteurs from '${FILES["auteurs"]}' with (format csv, header true, delimiter ';');"
    psql -d bibliotheque -c "\copy categories from '${FILES["categories"]}' with (format csv, header true, delimiter ';');"
    psql -d bibliotheque -c "\copy editeurs from '${FILES["editeurs"]}' with (format csv, header true, delimiter ';');"
    psql -d bibliotheque -c "\copy livres from '${FILES["livres"]}' with (format csv, header true, delimiter ';');"
    psql -d bibliotheque -c "\copy exemplaires from '${FILES["exemplaires"]}' with (format csv, header true, delimiter ';');"
    psql -d bibliotheque -c "\copy utilisateurs from '${FILES["utilisateurs"]}' with (format csv, header true, delimiter ';');"
    psql -d bibliotheque -c "\copy emprunts from '${FILES["emprunts"]}' with (format csv, header true, delimiter ';');"
}

populate