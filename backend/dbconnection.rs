use postgres::{Client, NoTls};
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::error::Error;
use std::fs;
use log::{info, error};

#[derive(Debug, Deserialize)]
struct Credentials {
    endpoint: String,
    database_name: String,
    username: String,
    password: String,
}

impl Credentials {
    fn connection_string(&self) -> String {
        format!(
            "postgresql://{}:{}@{}/{}",
            &self.username, &self.password, &self.endpoint, &self.database_name
        )
    }
}

pub fn establish_connection() -> Result<Client, Box<dyn Error>> {
    info!("Reading database connection credentials");

    // Read database connection credentials from safetynet-prod/.credentials.yaml
    let credentials = read_credentials()?;

    info!("Credentials read successfully");

    // Establish database connection
    let client = Client::connect(credentials.connection_string().as_str(), NoTls)?;

    info!("Database connection established");

    Ok(client)
}

fn read_credentials() -> Result<Credentials, Box<dyn Error>> {
    info!("Reading.credentials.yaml file");

    // Read contents of safetynet-prod/.credentials.yaml file
    let contents = fs::read_to_string("safetynet-prod/.credentials.yaml")?;

    info!(".credentials.yaml file read successfully");

    // Parse YAML contents into Credentials struct
    let credentials: Credentials = serde_yaml::from_str(&contents)?;

    Ok(credentials)
}