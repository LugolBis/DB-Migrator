use std::process::Command;

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

fn test_neo4j() {
    use std::sync::Arc;

    use neo4j::address::Address;
    use neo4j::driver::auth::AuthToken;
    use neo4j::driver::{ConnectionConfig, Driver, DriverConfig, RoutingControl};
    use neo4j::retry::ExponentialBackoff;
    use neo4j::{value_map, ValueReceive};

    let host = "localhost";
    let port = 7687;
    let user = "userA";
    let password = "password";
    let database = "usertest";

    let database = Arc::new(String::from(database));
    let address = Address::from((host, port));
    let auth_token = AuthToken::new_basic_auth(user, password);
    let driver = Driver::new(
        // tell the driver where to connect to
        ConnectionConfig::new(address),
        // configure how the driver works locally (e.g., authentication)
        DriverConfig::new().with_auth(Arc::new(auth_token)),
    );

    // Driver::execute_query() is the easiest way to run a query.
    // It will be sufficient for most use-cases and allows the driver to apply some optimizations.
    // So it's recommended to use it whenever possible.
    // For more control, see sessions and transactions.
    let result = driver.execute_query("RETURN 199 AS x")
        .with_database(database)
        .run();

    match result {
        Ok(res) => {
            let values = res.into_values()
            .map(|values| values
                .map(|value| value.try_into_int().unwrap())
                .collect::<Vec<_>>())
            .collect::<Vec<_>>();
            println!("{:?}",values)
        },
        Err(error) => println!("{}",error)
    }
}

fn main() {
    test_shell("ls");
    test_neo4j();
}