use postgres::{Client, NoTls};
use std::error::Error;
use std::fs::File;
use log::{info, error};
use serde_json;
use std::io::{BufWriter, Write};

pub fn run_queries(client: &mut Client) -> Result<(), Box<dyn Error>> {
    info!("Running queries");

    // Run queries and save results
    let tables = vec![
        "ecomp_forms", 
        "ecomp_site_hierarchy", 
        "ecomp_templates", 
        "ecomp_user_details", 
        "ecomp_user_training_details", 
        "users"
    ];

    for table in tables {
        let query = format!("SELECT * FROM {}", table);
        let rows = client.query(&query, &[])?;

        info!("Query executed successfully for table {}", table);

        // Write query results to a file
        let filename = match table {
            "ecomp_forms" => "formdata",
            "ecomp_site_hierarchy" => "sites",
            "ecomp_templates" => "templates",
            "ecomp_user_details" => "userdetails",
            "ecomp_user_training_details" => "trainingdetails",
            "users" => "users",
            _ => panic!("Invalid table name"),
        };

        save_results(rows, filename)?;
    }

    info!("Queries executed successfully!");

    Ok(())
}

fn save_results(rows: Vec<postgres::Row>, filename: &str) -> Result<(), Box<dyn Error>> {
    info!("Saving query results to file {}", filename);

    let file = BufWriter::new(File::create(format!("safetynet-prod/{}.json", filename))?);

    // Convert query results to JSON and write to file
    let mut results = Vec::new();
    for row in rows {
        let row_json = serde_json::to_string(&row)?;
        results.push(row_json);
    }

    file.write_all(results.join("\n").as_bytes())?;

    info!("Query results saved to file {}", filename);

    Ok(())
}