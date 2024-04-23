use duckdb::Connection;

pub struct DuckDBClient {
    connection: duckdb::Connection
}

impl DuckDBClient {
    pub fn new(maybe_aws_region: Option<String>, maybe_s3_access_key_id: Option<String>, maybe_s3_secret_access_key: Option<String>, maybe_database_path: &Option<String>, maybe_max_memory: &Option<String>) -> anyhow::Result<DuckDBClient> {
        let config =
            duckdb::Config::default()
                .allow_unsigned_extensions()?;
        
        let config = 
            if let Some(max_memory) = maybe_max_memory {
                config.max_memory(max_memory)?
            } else {
                config
            };

        let connection = 
            if let Some(database_path) = maybe_database_path {
                Connection::open_with_flags(database_path, config)?
            } else {
                Connection::open_in_memory_with_flags(config)?
            };
        
        match (maybe_aws_region, maybe_s3_access_key_id, maybe_s3_secret_access_key) {
            (Some(aws_region), Some(s3_access_key_id), Some(s3_secret_access_key)) => setup_aws_modules(&connection, &aws_region, &s3_access_key_id, &s3_secret_access_key)?,
            _ => log::info!("Skiping AWS modules initialization because credentials were not provided")
        }        
        
        let client = DuckDBClient { connection };        
        Ok(client)
    }

    pub async fn run_query(&self, query: &str) -> anyhow::Result<()> {       
        log::info!("Running {query}");
        self.connection.execute_batch(query)?;
        Ok(())
    }

}


fn setup_aws_modules(connection: &Connection, aws_region: &str, s3_access_key_id: &str, s3_secret_access_key: &str) -> anyhow::Result<()> {
    log::info!("Seting up AWS modules");
    let command = format!(
        "INSTALL httpfs;
        LOAD httpfs;

        SET s3_region = '{aws_region}';
        SET s3_access_key_id = '{s3_access_key_id}';
        SET s3_secret_access_key = '{s3_secret_access_key}';

        install '/application/libs/h3ext.duckdb_extension';
        load 'h3ext';
    ");

    connection.execute_batch(&command)?;
    log::info!("AWS module setup complete");

    Ok(())
}