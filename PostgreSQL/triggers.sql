create or replace function verifier_disponibilite() 
returns trigger as $$
declare
    DISPONIBILITE varchar(20);
begin
    select statut into DISPONIBILITE
    from exemplaires
    where exemplaire_id = new.exemplaire_id;

    if DISPONIBILITE = 'disponible' then
        update exemplaires 
        set statut = 'emprunté' 
        where exemplaire_id = new.exemplaire_id;
    else
        RAISE EXCEPTION 'The book % is already borrowed.', new.exemplaire_id;
    end if;

    return new;
end;
$$ LANGUAGE plpgsql;

create trigger disponible 
before insert on emprunts 
for each row
execute function verifier_disponibilite();