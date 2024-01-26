use clap::Parser;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
pub struct CommandLine {
    #[clap(long, env)]
    /// The path for the Jina.SQL Query template file
    pub query_template_path: String,

    #[clap(long, env)]
    /// The path for a YAML file containing an Array of the parameters for the YAML template
    pub parameters_source_path: Option<String>,
    
    #[clap(long, env, default_value="1")]
    /// The number of queries (elements of parameters array file) to be executed at the same time
    pub max_concurrent_queries: usize,

    #[clap(long, env)]
    /// The AWS access key id to load data from S3
    pub aws_access_key_id: Option<String>,

    #[clap(long, env)]
    /// The AWS secret access key to load data from S3
    pub aws_secret_access_key: Option<String>,

    #[clap(long, env)]
    /// The AWS region to load data from S3
    pub aws_region: Option<String>,
}

impl CommandLine {
    pub fn fetch() -> Self {
        CommandLine::parse()
    }
}