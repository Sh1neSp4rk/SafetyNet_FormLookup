mod dbconnection;
mod query;

use dbconnection::establish_connection;
use query::run_queries;
use std::error::Error;
use log::{info, error};

fn main() -> Result<(), Box<dyn Error>> {
    info!("Starting application");

    // Establish database connection
    let mut client = establish_connection()?;

    info!("Database connection established");

    // Run queries and save results
    run_queries(&mut client)?;

    info!("Queries executed successfully!");

    Ok(())
}