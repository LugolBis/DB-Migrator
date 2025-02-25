mod postgresql;
mod neo4j;

fn main() {
    demo()
}

fn demo() {
    use postgresql::PostgreSQL;
    use neo4j::Neo4j;
    
    // PostgreSQL part

    let db_postgresql = PostgreSQL::new("127.0.0.1", "5432", "lugolbis", 
    "LUGOL2656", "bibliotheque");

    let script_meta_data = "/home/lugolbis/Bureau/UVSQ/L3_Info/S6/DB-Migrator/PostgreSQL/meta_data.sql";
    let function_meta_data = "export_tables_metadata";
    let save_meta_data = "/home/lugolbis/Bureau/UVSQ/L3_Info/S6/DB-Migrator/Neo4j/postgresql_meta_data.json";

    let script_tables = "/home/lugolbis/Bureau/UVSQ/L3_Info/S6/DB-Migrator/PostgreSQL/export_tables.sql";
    let function_tables = "export_tables_to_json";
    let save_tables = "/home/lugolbis/Bureau/UVSQ/L3_Info/S6/DB-Migrator/Neo4j/postgresql_tables.json";

    let script_fk = "/home/lugolbis/Bureau/UVSQ/L3_Info/S6/DB-Migrator/PostgreSQL/export_fk.sql";
    let function_fk = "export_fk_relationships";
    let save_fk = "/home/lugolbis/Bureau/UVSQ/L3_Info/S6/DB-Migrator/Neo4j/postgresql_fk.json";

    /* 
    match db_postgresql.export_from_sql(script_meta_data,function_meta_data,save_meta_data) {
        Ok(_) => println!("Successfuly export meta data !"),
        Err(result) => println!("ERROR when try to export meta data :\n{}",result),
    }

    match db_postgresql.export_from_sql(script_tables,function_tables,save_tables) {
        Ok(_) => println!("Successfuly export tables !"),
        Err(result) => println!("ERROR when try to export tables :\n{}",result),
    }

    match db_postgresql.export_from_sql(script_fk,function_fk,save_fk) {
        Ok(_) => println!("Successfuly export foreign keys !"),
        Err(result) => println!("ERROR when try to export foreign keys :\n{}",result),
    }*/

    // Neo4j

    let db_neo4j = Neo4j::new("neo4j://localhost:7687", "userA", "password", 
    "userTest", "/home/lugolbis/.config/Neo4j Desktop/Application/relate-data/dbmss/dbms-e7b73ebf-0aff-4b89-bdff-e92c6868df84/import/");

    let script_delete = "/home/lugolbis/Bureau/UVSQ/L3_Info/S6/DB-Migrator/Neo4j/delete.cql";

    /*
    match db_neo4j.execute_script(script_delete) {
        Ok(result) => println!("{}", result),
        Err(result) => println!("{}",result)
    }

    match db_neo4j.create_csv_headers(save_meta_data) {
        Ok(result) => println!("{}", result),
        Err(result) => println!("{}",result)
    } 

    match db_neo4j.extract_nodes(save_tables) {
        Ok(result) => println!("{}", result),
        Err(result) => println!("{}",result)
    }

    match db_neo4j.extract_edges(save_fk) {
        Ok(result) => println!("{}", result),
        Err(result) => println!("{}",result)
    }*/

    match db_neo4j.load_with_admin() {
        Ok(result) => println!("{}", result),
        Err(result) => println!("{}",result)
    }
}