use std::process::Command;

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
                println!("\nExport data from PostgreSQL - Successfully created the function {}\n",function_name);
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