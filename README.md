# DuckDB Query Runner

A jinja SQL data transformer based on [DuckDB](https://duckdb.org/).

## Usage

```
Usage: duckdb-query-runner [OPTIONS] --query-template-path <QUERY_TEMPLATE_PATH> --parameters-source-path <PARAMETERS_SOURCE_PATH>

Options:
      --query-template-path <QUERY_TEMPLATE_PATH>
          The path for the Jina.SQL Query template file [env: QUERY_TEMPLATE_PATH=]
      --parameters-source-path <PARAMETERS_SOURCE_PATH>
          The path for a YAML file containing an Array of the parameters for the YAML template [env: PARAMETERS_SOURCE_PATH=]
      --max-concurrent-queries <MAX_CONCURRENT_QUERIES>
          The number of queries (elements of parameters array file) to be executed at the same time [env: MAX_CONCURRENT_QUERIES=] [default: 1]
      --aws-access-key-id <AWS_ACCESS_KEY_ID>
          The AWS access key id to load data from S3 [env: AWS_ACCESS_KEY_ID=]
      --aws-secret-access-key <AWS_SECRET_ACCESS_KEY>
          The AWS secret access key to load data from S3 [env: AWS_SECRET_ACCESS_KEY=]
      --aws-region <AWS_REGION>
          The AWS region to load data from S3 [env: AWS_REGION=]
  -h, --help
          Print help
  -V, --version
          Print version
```


## Changelog

### v1.0.0

Add support for [H3 extension](https://github.com/isaacbrodsky/h3-duckdb).

### v1.1.0

Now `--parameters-source-path` can point to a file containing a single JSON object instead of a list of objects.

