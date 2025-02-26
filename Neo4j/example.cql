// Some test with cypher and apoc.
// This script is never used in the final project.

CREATE (charlie:Person:Actor {name: 'Charlie Sheen'}),
       (martin:Person:Actor {name: 'Martin Sheen'}),
       (michael:Person:Actor {name: 'Michael Douglas'}),
       (oliver:Person:Director {name: 'Oliver Stone'}),
       (rob:Person:Director {name: 'Rob Reiner'}),
       (wallStreet:Movie {title: 'Wall Street'}),
       (charlie)-[:ACTED_IN {role: 'Bud Fox'}]->(wallStreet),
       (martin)-[:ACTED_IN {role: 'Carl Fox'}]->(wallStreet),
       (michael)-[:ACTED_IN {role: 'Gordon Gekko'}]->(wallStreet),
       (oliver)-[:DIRECTED]->(wallStreet),
       (thePresident:Movie {title: 'The American President'}),
       (martin)-[:ACTED_IN {role: 'A.J. MacInerney'}]->(thePresident),
       (michael)-[:ACTED_IN {role: 'President Andrew Shepherd'}]->(thePresident),
       (rob)-[:DIRECTED]->(thePresident);

match (a1:Actor {name: 'Charlie Sheen'})
match (b1:Actor {name: 'Martin Sheen'})
with [a1, b1] AS FK
call apoc.util.validate(
    any(node in FK where node is null),
    'Issue with the foreign keys : %s', 
    [FK]
)
create (lulu: Person {name: 'Lulu'})
FOREACH (
node in FK |
create (lulu)-[r:REF_]->(node)
SET r.type = 'REF_' + apoc.node.labels(node)[0]
)
return lulu;

match (n) return n;

match path = (l:Person{name: 'Lulu'})-[]->(n) return path;