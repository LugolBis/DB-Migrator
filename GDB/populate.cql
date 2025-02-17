// Example

// Le Chargement massif avec des données locales pose des problèmes de configurations/sécuritées
// difficles à automatiser

/*
load CSV from 'file:///editeurs.csv' as row
merge (a:Editeur {id: toInteger(row[0]), nom: row[1], adresse: row[2]})
return a;
/*

/*
load CSV from 'https://cdn.jsdelivr.net/gh/LugolBis/DB-Migrator@master/GDB/editeurs.csv' as row
merge (a:Editeur {id: toInteger(row[0]), nom: row[1], adresse: row[2]})
return a;
*/

// Création d'une contrainte d'unicité pour l'id (à exécuter une fois)
CREATE CONSTRAINT unique_editeur_id IF NOT EXISTS 
FOR (e:Editeur) 
REQUIRE e.id IS UNIQUE;

// Requête d'import avec gestion des headers et typage explicite
LOAD CSV WITH HEADERS FROM 'https://cdn.jsdelivr.net/gh/LugolBis/DB-Migrator@master/GDB/editeurs.csv' AS row 
FIELDTERMINATOR ';'
MERGE (e:Editeur {id: toInteger(row.editeur_id)})
ON CREATE SET 
    e.nom = row.nom,
    e.adresse = row.adresse
ON MATCH SET 
    e.nom = row.nom,
    e.adresse = row.adresse
RETURN count(e) AS nodesTraites;

// https://cdn.jsdelivr.net/gh/LugolBis/DB-Migrator@master/GDB/editeurs.csv
// https://github.com/LugolBis/DB-Migrator/blob/master/GDB/editeurs.csv

/*
LOAD CSV WITH HEADERS FROM 'file:///bands-with-headers.csv' AS line
MERGE (n:$(line.Label) {name: line.Name})
RETURN n AS bandNodes
*/