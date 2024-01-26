use std::{sync::Arc, collections::HashMap};

use futures::StreamExt;

use crate::{duckdb_client::DuckDBClient, template::render_one, constants::TEMPLATE_NAME};

pub async fn run_for_parameters_list(athena_client: Arc<DuckDBClient>, environment: Arc<tera::Tera>, parameters_list: Vec<HashMap<String, serde_yaml::Value>>, max_concurrent_queries: usize) -> anyhow::Result<()> {

    let mut stream = tokio_stream::iter(parameters_list).map(|parameters| async {
        let result = run_for_parameters(athena_client.clone(), environment.clone(), parameters).await?;
        anyhow::Ok(result)
    })
    .buffer_unordered(max_concurrent_queries);

    while let Some(result) = stream.next().await {
        log::info!("Query for parameters: {parameters:?} has been finished", parameters=result);
    }

    Ok(())
}

async fn run_for_parameters(duckdb_client: Arc<DuckDBClient>, environment: Arc<tera::Tera>, parameters: HashMap<String, serde_yaml::Value>) -> anyhow::Result<HashMap<String, serde_yaml::Value>> {
    let query = render_one(environment, TEMPLATE_NAME, &parameters)?;
    duckdb_client.run_query(&query).await?;
    Ok(parameters)
}
