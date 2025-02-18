// Example

// Le Chargement massif avec des données locales pose des problèmes de configurations/sécuritées
// difficles à automatiser

// Création d'une contrainte d'unicité pour l'id (à exécuter une fois)
CREATE CONSTRAINT unique_editeur_id IF NOT EXISTS 
FOR (e:Editeur) 
REQUIRE e.id IS UNIQUE and not nullable;


/*
LOAD CSV FROM 'https://cdn.jsdelivr.net/gh/LugolBis/DB-Migrator@master/Data/editeurs.csv' AS line
WITH line
WHERE size(line) >= 3 AND line[0] <> ''
MERGE (e:Editeur {id: toInteger(line[0])})
SET e.nom = line[1], e.adresse = line[2]
RETURN count(e);
*/