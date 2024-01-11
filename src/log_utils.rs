pub fn initialize_log() {
    pretty_env_logger::env_logger::builder()
    .format_timestamp_secs()
    .format_level(true)
    .format_indent(Some(4))
    .format_module_path(true)
    .init();
}