// Example

// Le Chargement massif avec des données locales pose des problèmes de configurations/sécuritées
// difficles à automatiser

// Création d'une contrainte d'unicité pour l'id (à exécuter une fois)
CREATE CONSTRAINT unique_editeur_id IF NOT EXISTS 
FOR (e:Editeur) 
REQUIRE e.id IS UNIQUE;

CREATE CONSTRAINT nonull_editeur_id IF NOT EXISTS 
FOR (e:Editeur) 
REQUIRE e.id IS not null;

CREATE CONSTRAINT type_LABEL_property if not exists
FOR (m:Movie)
REQUIRE valueType(m.year) = "INTEGER";

CALL apoc.trigger.add(
  'type_LABEL_property',
  "MATCH (m:Movie) 
   WHERE m.year IS NOT NULL AND NOT valueType(m.year) = 'LONG'
   CALL apoc.util.validate(true, 'ERREUR : La propriété year doit être un entier.', [])
   RETURN m",
  {phase: 'before'}
);

CALL apoc.periodic.iterate(
  'CALL apoc.load.json("file:///big_data.json") YIELD value RETURN value',
  'CREATE (n:Node) SET n = value',
  {batchSize: 10000, parallel: true}
);

/*
LOAD CSV FROM 'https://cdn.jsdelivr.net/gh/LugolBis/DB-Migrator@master/Data/editeurs.csv' AS line
WITH line
WHERE size(line) >= 3 AND line[0] <> ''
MERGE (e:Editeur {id: toInteger(line[0])})
SET e.nom = line[1], e.adresse = line[2]
RETURN count(e);
*/