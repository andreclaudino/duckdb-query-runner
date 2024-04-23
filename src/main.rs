use crate::core::run_for_parameters_list;
use std::sync::Arc;

use crate::constants::TEMPLATE_NAME;
use crate::duckdb_client::DuckDBClient;
use crate::log_utils::initialize_log;
use crate::persistence::load_parameters_list;
use crate::template::load_template;

mod command_line;
mod constants;
mod persistence;
mod template;
mod core;
mod duckdb_client;
mod log_utils;


#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    initialize_log();

    let arguments = command_line::CommandLine::fetch();

    let duckdb_client =
        Arc::new(DuckDBClient::new(arguments.aws_region, arguments.aws_access_key_id, arguments.aws_secret_access_key, &arguments.database_path, &arguments.max_memory)?);
    let template_environment = Arc::new(load_template(&arguments.query_template_path, TEMPLATE_NAME).await?);
    let parameters_list = load_parameters_list(&arguments.parameters_source_path)?;

    run_for_parameters_list(duckdb_client, template_environment, parameters_list, arguments.max_concurrent_queries).await?;

    Ok(())
}
