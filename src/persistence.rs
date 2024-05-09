use std::{collections::HashMap, fs::File, io::BufReader};

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(untagged)]
enum Parameter {
    List(Vec<HashMap<String, serde_yaml::Value>>),
    Object(HashMap<String, serde_yaml::Value>)
}

//TODO: Load from S3 or Git
pub fn load_parameters_list(maybe_parameters_source_path: &Option<String>) -> anyhow::Result<Vec<HashMap<String, serde_yaml::Value>>> {
    if let Some(parameters_source_path) = maybe_parameters_source_path {
        let input_file = File::open(parameters_source_path)?;
        let reader = BufReader::new(input_file);
        let parameter: Parameter = serde_yaml::from_reader(reader)?;

        let parameter_list =
            match parameter {
                Parameter::List(elements) => elements,
                Parameter::Object(object) => vec![object]
            };

        Ok(parameter_list)

    } else {
        Ok(vec![HashMap::new()])
    }
    
}