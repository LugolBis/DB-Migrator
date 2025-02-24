use std::process::Command;
use std::path::Path;
use std::fs;

#[derive(Debug)]
pub struct PostgreSQL {
    host : String,
    port : String,
    username : String,
    password : String,
    database : String
}

impl PostgreSQL {
    pub fn new(host:&str,port:&str,username:&str,password:&str,database:&str) -> Self {
        Self {
            host: host.to_string(), port: port.to_string(), username: username.to_string(),
            password: password.to_string(), database: database.to_string()
        }
    }

    pub fn execute_query(&self,query:&str) -> Result<String, String> {
        //! This method take in input only one PostgreSQL query.<br>
        //! To run more queries please use ```PostgreSQL.execute_script()```
        let output = Command::new("psql")
            .arg("-h").arg(&self.host)
            .arg("-p").arg(&self.port)
            .arg("-U").arg(&self.username)
            .arg("-d").arg(&self.database)
            .arg("-c").arg(query)
            .arg("--csv").env("PGPASSWORD", &self.password) 
            .output()
            .expect("Échec de l'exécution de la commande psql");
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let result = format!("{}",stdout);
            return Ok(result);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let result = format!("{}",stderr);
            return Err(result);
        }
    }

    pub fn execute_script(&self,script_path:&str) -> Result<String, String> {
        //! The path of the script need to be the reel path (not the relative path).
        let output = Command::new("psql")
            .arg("-h").arg(&self.host)
            .arg("-p").arg(&self.port)
            .arg("-U").arg(&self.username)
            .arg("-d").arg(&self.database)
            .arg("-f").arg(script_path)
            .arg("--csv").env("PGPASSWORD", &self.password) 
            .output()
            .expect("Échec de l'exécution de la commande psql");
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let result = format!("{}",stdout);
            return Ok(result);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let result = format!("{}",stderr);
            return Err(result);
        }
    }

    pub fn export_from_sql(&self,script_path:&str,function_name:&str,save_path:&str) -> Result<String, String> {
        //! This method allows you to export the result of the SQL function called ```function_name```
        //! and define in the PostgreSQL script ```script_path``` to the file specified in ```save_path```.
        //! You should use it to export the meta data/tables of your PostgreSQL database.
        match &self.execute_script(script_path) {
            Ok(res) => {
                println!("\nExport meta data - Successfully created the function {}\n",function_name);
                let query = format!(r"\copy (select {}()) to '{}'",function_name,save_path);
                match self.execute_query(query.as_str()) {
                    Ok(res) => { return Ok(res); },
                    Err(error) => { return Err(error.to_owned()); }
                }
            },
            Err(error) => { return Err(error.to_owned()); }
        }
    }
}

#[derive(Debug)]
pub struct Neo4j {
    uri : String,
    username : String,
    password : String,
    database : String,
    import_directory : String
}

impl Neo4j {
    pub fn new(uri:&str,username:&str,password:&str,database : &str,import_directory : &str) -> Self {
        Self { 
            uri: uri.to_string(), username: username.to_string(), password: password.to_string(),
            database: database.to_string(), import_directory: import_directory.to_string()
        }
    }

    pub fn execute_query(&self,query:&str) -> Result<String, String> {
        //! This method take in input only one Cypher query.<br>
        //! To run more queries please use ```Neo4j.execute_script()```
        let output = Command::new("cypher-shell")
            .arg("-a").arg(&self.uri)
            .arg("-u").arg(&self.username)
            .arg("-p").arg(&self.password)
            .arg("-d").arg(&self.database)
            .arg(format!("{}",query))
            .output()
            .expect("Échec de l'exécution de la commande psql");
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let result = format!("{}",stdout);
            return Ok(result);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let result = format!("{}",stderr);
            return Err(result);
        }
    }

    pub fn execute_script(&self,script_path:&str) -> Result<String, String> {
        //! The path of the script need to be the reel path (not the relative path).
        let output = Command::new("cypher-shell")
            .arg("-a").arg(&self.uri)
            .arg("-u").arg(&self.username)
            .arg("-p").arg(&self.password)
            .arg("-d").arg(&self.database)
            .arg("-f").arg(format!("{}",script_path))
            .output()
            .expect("Error when try to execute the cypher-shell command");
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let result = format!("{}",stdout);
            return Ok(result);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let result = format!("{}",stderr);
            return Err(result);
        }
    }

    pub fn convert_PostgreSQL_type(postgreSQL_type:&str) -> Result<String, String> {
        //! Convert PostgreSQL Type into Neo4j type.<br>
        //! CAUTION : TIMESTAMP -> DATETIME ; MONEY -> FLOAT
        let target_type = postgreSQL_type.to_uppercase();
        match target_type.as_str() {
            "INTEGER" | "BIGINT" | "SERIAL" => Ok("INT".to_string()),
            "REAL" | "DOUBLE" | "PRECISION" => Ok("FLOAT".to_string()),
            "NUMERIC" | "DECIMAL" => Ok("FLOAT".to_string()),
            "VARCHAR" | "TEXT" | "CHAR" | "CHARACTER VARYING" => Ok("STRING".to_string()),
            "BOOLEAN" => Ok("BOOLEAN".to_string()),
            "DATE" => Ok("DATE".to_string()),
            "TIME" => Ok("LOCALTIME".to_string()),
            "TIMESTAMP" => Ok("DATETIME".to_string()),
            "INTERVAL" => Ok("DURATION".to_string()),
            "JSON" | "JSONB" => Ok("MAP".to_string()),
            "UUID" => Ok("STRING".to_string()),
            "ARRAY" => Ok("LIST".to_string()),
            "BYTEA" => Ok("BYTES".to_string()),
            "ENUM" => Ok("STRING".to_string()),
            "POINT" | "POLYGON" => Ok("STRING".to_string()),
            "CIDR" | "INET" => Ok("STRING".to_string()),
            "MONEY" => Ok("FLOAT".to_string()),
            "XML" => Ok("STRING".to_string()),
            _ => Err(format!("ERROR : Can't convert '{}' into Neo4j type.",target_type))
        }
    }

    pub fn load_with_admin(&self) -> Result<String, String> {
        todo!();
        let mut command = Command::new("../bin/neo4j-admin");
        command.args(["database","import","full",&self.database]);

        let path = Path::new(&self.import_directory);
        let mut nodes: Vec<String> = Vec::new();
        let mut relationships: Vec<String> = Vec::new();
        match fs::read_dir(path) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry_) = entry {
                        let file = entry_.file_name().into_string().unwrap_or_default();
                        match (&file.find("_REF_"), &file.ends_with(".csv")) {
                            (Some(_), true) => { relationships.push(file); },
                            (None, true) => { nodes.push(file); }
                            (_, _) => {}
                        }
                    }
                }
                for node in nodes {
                    command.arg(format!("--nodes={}",node));
                }
                for relationship in relationships {
                    command.arg(format!("--relationships={}",relationship));
                }
                command.arg("--delimiter=';'"); command.arg("--array-delimiter=','");
                command.arg("--overwrite-destination");

                println!("Command Executed :\n{:?}\n",command.get_program());
                let output = command.output();
                match output {
                    Ok(output) => {
                        if output.status.success() {
                            let stdout = String::from_utf8_lossy(&output.stdout);
                            let result = format!("{}",stdout);
                            return Ok(result);
                        } else {
                            let stderr = String::from_utf8_lossy(&output.stderr);
                            let result = format!("{}",stderr);
                            return Err(result);
                        }
                    },
                    Err(error) => { 
                        return Err(format!("ERROR when try to execute the command :\n{:?}\n{}",command,error))
                    }
                }
            },
            Err(error) => { 
                return Err(format!("ERROR when try to list the files in the import directory :\n{}",error))
            }
        }
    }
}