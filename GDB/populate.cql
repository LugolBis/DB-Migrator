// Example

// Le Chargement massif avec des données locales pose des problèmes de configurations/sécuritées
// difficles à automatiser

/*
load CSV from 'file:///editeurs.csv' as row
merge (a:Editeur {id: toInteger(row[0]), nom: row[1], adresse: row[2]})
return a;
/*

/*
LOAD CSV WITH HEADERS FROM 'file:///bands-with-headers.csv' AS line
MERGE (n:$(line.Label) {name: line.Name})
RETURN n AS bandNodes
*/