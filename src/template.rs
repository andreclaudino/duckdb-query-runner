use std::{sync::Arc, collections::HashMap, fs::read_to_string};


//TODO: Load from S3 or Git
pub async fn load_template(template_source_path: &str, template_name: &str) -> anyhow::Result<tera::Tera> {
    let content = read_to_string(template_source_path)?;

    let mut environment = tera::Tera::default();
    environment.add_raw_template(template_name, &content)?;

    Ok(environment)
}

pub fn render_one(environment: Arc<tera::Tera>, template_name: &str, parameters: &HashMap<String, serde_yaml::Value>) -> anyhow::Result<String> {
    let context = tera::Context::from_serialize(&parameters)?;
    let rendered = environment.render(template_name, &context)?;

    log::info!("Query for parameters = {parameters:?} rendered");

    Ok(rendered)
}