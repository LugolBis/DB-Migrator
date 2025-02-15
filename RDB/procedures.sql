-- This procedure is used for the users who want to return the book that they borrow
create or replace procedure retour(exemplaire_id in INT) as $$
begin
    update exemplaires set statut="disponible" where exemplaire_id=exemplaire_id;
end;
$$ LANGUAGE PLPGSQL;