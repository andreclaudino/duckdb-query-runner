use std::{collections::HashMap, fs::File, io::BufReader};


//TODO: Load from S3 or Git
pub fn load_parameters_list(parameters_source_path: &str) -> anyhow::Result<Vec<HashMap<String, serde_yaml::Value>>> {
    let input_file = File::open(parameters_source_path)?;
    let reader = BufReader::new(input_file);
    let u: Vec<HashMap<String, serde_yaml::Value>> = serde_yaml::from_reader(reader)?;
    Ok(u)
}