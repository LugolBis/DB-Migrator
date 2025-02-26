use std::process::Command;
use serde_json::Value;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::env;
use std::fs;

#[derive(Debug)]
pub struct Neo4j {
    uri : String,
    username : String,
    password : String,
    database : String,
    import_folder : String
}

impl Neo4j {
    pub fn new(uri:&str,username:&str,password:&str,database : &str,import_folder : &str) -> Self {
        Self { 
            uri: uri.to_string(), username: username.to_string(), password: password.to_string(),
            database: database.to_string(), import_folder: import_folder.to_string()
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
            .expect("Échec de l'exécution de la commande cypher-shell");
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let result = format!("\nResult of the cypher query :\n{}",stdout);
            return Ok(result);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let result = format!("\nError when try to execute the cypher query : {}\n{}",query,stderr);
            return Err(result);
        }
    }

    pub fn execute_script(&self,script_path:&str) -> Result<String, String> {
        //! The path of the Cypher script need to be the reel path (not the relative path).
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

    pub fn convert_postgresql_type(postgresql_type:&str) -> Result<String, String> {
        //! Convert PostgreSQL Type into Neo4j type.<br>
        //! CAUTION : TIMESTAMP -> DATETIME ; MONEY -> FLOAT
        let target_type = postgresql_type.to_uppercase();
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
        //! This method perform the 'neo4j-admin import' from the ```&self.import_folder```<br><br>
        //! **WARNING** : This method construct the command 'neo4j-admin import' by detecting <br>
        //! the **CSV** files in the folder, you need to assert that there isn't other CSV files <br>
        //! than these you need for the import. Moreover assert that the CSV files who are contain the <br>
        //! *relationships* have '_REF_' in their name.
        if let Err(error) = env::set_current_dir(Path::new(&self.import_folder)) {
            return Err(format!("{}",error));
        }

        let mut command:Command;
        if cfg!(target_os = "windows") { command = Command::new("bin\neo4j-admin.bat"); }
        else { command = Command::new("../bin/neo4j-admin"); }
        command.args(["database","import","full",&self.database]);

        let path = Path::new(&self.import_folder);
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
                command.args(["--delimiter=;", "--array-delimiter=,", "--overwrite-destination", "--verbose"]);

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

    pub fn create_csv_headers(&self,meta_data_path:&str) -> Result<String, String> {
        //! Generate **CSV** files who contains the **HEADERS** needed to generate and organise the
        //! data to be imported to Neo4j.
        if let Err(error) = clean_directory(&self.import_folder) {
            return Err(error);
        }

        let content = fs::read_to_string(meta_data_path)
            .map_err(|error| format!("{}",error))?;

        let json_object:Value = serde_json::from_str(&content)
            .map_err(|error| format!("{}",error))?;

        match json_object {
            Value::Array(vector) => {
                for table in vector {
                    let label = String::from(table["table_name"].as_str()
                        .ok_or_else(|| 
                        format!("Error when try to get the 'table_name' field in {}",table))?).to_uppercase();
                    let mut headers = String::from(":ID;");
                    let mut foreign_keys: Vec<String> = Vec::new();

                    let columns = table["columns"].as_array()
                        .ok_or_else(|| 
                        format!("Error when try to get the 'columns' field in {}",table))?;
                    
                    for column in columns {
                        let column_name = String::from(column["column_name"].as_str()
                            .ok_or_else(|| format!("Error when try to get the 'column_name' field in {}",column))?);
                        let function_name = format!("{}_{}",label.to_lowercase(),column_name);

                        if let Some(value) = column["primary_key"].as_bool() {
                            if value == true {
                                match &self.execute_query(&format!("create constraint unique_{} if not exists for (n:{}) 
                                require n.{} is unique;", function_name,label,column_name)) {
                                    Ok(result) => println!("{}",result),
                                    Err(error) => { return Err(format!("{}",error)); }
                                }
                            }
                        }
                        if let Some(value) = column["is_nullable"].as_str() {
                            if value == "NO" {
                                match &self.execute_query(&format!("create constraint nonull_{} if not exists for (n:{}) 
                                require n.{} is not null;", function_name,label,column_name)) {
                                    Ok(result) => println!("{}",result),
                                    Err(error) => { return Err(format!("{}",error)); }
                                }
                            }
                        }
                        match &column["foreign_key"] {
                            Value::Null => {
                                let pg_data_type = column["data_type"].as_str()
                                    .ok_or_else(|| format!("Error when try to get the 'data_type' field in {}",column))?;
                                let data_type = Neo4j::convert_postgresql_type(pg_data_type)
                                    .map_err(|error| format!("{}",error))?;
                                headers.push_str(&format!("{}:{};",column_name,data_type));
                            },
                            Value::Array(vector) => {
                                let key = String::from(vector[0]["referenced_table"].as_str()
                                    .ok_or_else(|| format!("Error when try to get the 'referenced_table' field in {}",vector[0]))?)
                                    .to_uppercase();
                                if !foreign_keys.contains(&key) {
                                    foreign_keys.push(key.clone());
                                }
                            },
                            _ => { return Err(format!("Error when try to match the 'foreign_keys' field in {}", column)); }
                        }
                    }
                    
                    headers.push_str(":LABEL");
                    let file_path = format!("{}{}.csv",self.import_folder,label);
                    let mut file = OpenOptions::new().write(true).create(true).open(&file_path)
                        .map_err(|error| format!("{}",error))?;
                    match file.write_all(&headers.as_bytes()) {
                        Ok(_) => println!("\nSuccessfully write the headers in {}\n",file_path),
                        Err(error) => { return Err(format!("{}",error)); }
                    }

                    const HEADERS_FK:&str = ":START_ID;:END_ID;:TYPE";
                    for fk in foreign_keys {
                        let file_path = format!("{}{}_REF_{}.csv",self.import_folder,label,fk);
                        let mut file = OpenOptions::new().write(true).create(true).open(&file_path)
                            .map_err(|error| format!("{}",error))?;
                        match file.write_all(HEADERS_FK.as_bytes()) {
                            Ok(_) => println!("\nSuccessfully write the fk headers in {}\n",file_path),
                            Err(error) => { return Err(format!("{}",error)); }
                        }
                    }
                }
                return Ok("Successfully create and write the Headers for the Neo4j import.".to_string());
            },
            _ => { return Err(format!("Expected a Value::Object(Map<_,_>) but found :\n{}",json_object)) }
        }
    }

    pub fn extract_nodes(&self,tables_path:&str) -> Result<String, String> {
        //! Read the JSON file that contains all the lines of the PostgreSQL database and save them <br>
        //! in the CSV files in the the import folder. <br><br>
        //! **WARNING** this method need to be used after ```&self.extract_csv_headers(...)```
        let content = fs::read_to_string(tables_path)
            .map_err(|error| format!("{}",error))?;

        let json_object:Value = serde_json::from_str(&content)
            .map_err(|error| format!("{}",error))?;

        let map = json_object.as_object()
            .ok_or_else(|| format!("The following json object is not a map :\n{}",json_object))?;

        for (table, value) in map {
            let label = table.clone().to_uppercase();
            let lines = value.as_array()
                .ok_or_else(|| format!("The following json object is not a map :\n{}",json_object))?;

            let headers = fs::read_to_string(format!("{}{}.csv",
                self.import_folder,label))
                .map_err(|error| format!("{}",error))?;
            let headers = headers.split(";").map(|c| c.split(":")
                .collect::<Vec<&str>>()[0]).collect::<Vec<&str>>();
            let headers = headers.iter().skip(1).take(headers.len() - 2)
                .cloned().collect::<Vec<&str>>();

            let mut counter : u64 = 0u64;
            let mut content = String::new();
            for line in lines {
                let mut new_line = format!("\n{}{};",label,counter);
                for property in &headers {
                    let line_content = format!("{}",line[format!("{}",property)])
                        .replace(";","").replace(",","");
                    new_line.push_str(&line_content); new_line.push_str(";");
                }
                new_line.push_str(&label);
                content.push_str(&new_line);
                counter += 1
            }

            let file_path = format!("{}{}.csv",self.import_folder,label);
            let mut file = OpenOptions::new().write(true).append(true).create(true).open(&file_path)
                .map_err(|error| format!("{}",error))?;
            match file.write_all(&content.as_bytes()) {
                Ok(_) => println!("\nSuccessfully write the nodes in {}\n",file_path),
                Err(error) => { return Err(format!("{}",error)); }
            }
        }
        Ok("\nSuccessfully extract the nodes and store them in the CSV files !".to_string())
    }

    pub fn extract_edges(&self,foreign_key_path:&str) -> Result<String, String> {
        //! Read the JSON file that contains all the couple of foreign keys of the PostgreSQL database <br>
        //! and save them in the CSV files in the the import folder. <br><br>
        //! **WARNING** this method need to be used after ```&self.extract_csv_headers(...)```
        let content = fs::read_to_string(foreign_key_path)
            .map_err(|error| format!("{}",error))?;

        let json_object:Value = serde_json::from_str(&content)
            .map_err(|error| format!("{}",error))?;

        let map = json_object.as_object()
            .ok_or_else(|| format!("Error when try to parse into a Map this object :\n{}",json_object))?;

        for (file_name, lines) in map {
            let labels = file_name.split("_REF_").collect::<Vec<&str>>();
            let mut content = String::new();
            if let Some(vector) = lines.as_array() {
                for couple in vector {
                    if let Some(duo) = couple.as_array() {
                        content.push_str(&format!("\n{}{};{}{};{}",
                            labels[0],duo[0],labels[1],duo[1],file_name
                        ));
                    }
                }
            }

            let file_path = format!("{}{}.csv",self.import_folder,file_name);
            let mut file = OpenOptions::new().write(true).append(true).create(true).open(&file_path)
                .map_err(|error| format!("{}",error))?;
            match file.write_all(&content.as_bytes()) {
                Ok(_) => println!("\nSuccessfully write the edges in {}\n",file_path),
                Err(error) => { return Err(format!("{}",error)); }
            }
        }
        Ok("Successfully extract the edges and store them in the CSV files !".to_string())
    }
}

fn clean_directory(folder_path:&str) -> Result<String, String> {
    //! Delete all the CSV files in the folder in input.
    let entries = fs::read_dir(folder_path)
        .map_err(|error| format!("{}",error))?;

    for entry in entries {
        let entry = entry.map_err(|error| format!("{}",error))?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "csv" {
                    fs::remove_file(&path).map_err(|error|format!("{}",error))?;
                }
            }
        }
    }
    Ok("Successfully clean the directory !".to_string())
}