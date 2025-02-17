// Delete all nodes and relationships
match (n)
detach delete n;

create (p:Person {name:"Lulu", age:20});