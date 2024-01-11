use clap::Parser;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
pub struct CommandLine {
    #[clap(long, env)]
    pub query_template_path: String,

    #[clap(long, env)]
    pub parameters_source_path: String,

    #[clap(long, env, default_value="1")]
    pub max_concurrent_queries: usize,

    #[clap(long, env)]
    pub aws_access_key_id: Option<String>,

    #[clap(long, env)]
    pub aws_secret_access_key: Option<String>,

    #[clap(long, env)]
    pub aws_region: Option<String>,
}

impl CommandLine {
    pub fn fetch() -> Self {
        CommandLine::parse()
    }
}