mod lib;
use std::process::Command;
use std::env;
use serde_json::Value;
use std::fs;

fn test_shell(shell_command:&str) {
    let output = Command::new(shell_command)
        .output()
        .expect("Error when try to run the command.");
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("OUTPUT :\n{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error :\n{}", stderr);
    }
}

fn test_json(path: String) -> Result<Value, String> {
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Error when try to read the file : {}", e))?;
    
    let json_object = serde_json::from_str(&content)
        .map_err(|e| format!("Error when try to parse JSON: {}", e))?;
    
    println!("Contenu parsé : {:?}", json_object);
    
    Ok(json_object)
}

fn main() {
    use lib::PostgreSQL;
    use lib::Neo4j;
    test_shell("ls");
}