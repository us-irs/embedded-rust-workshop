use std::{
    fs::File,
    path::{Path, PathBuf},
    time::SystemTime,
};

use anyhow::Context as _;

pub fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

pub fn config_file_init() -> anyhow::Result<File> {
    let config_file_path = Path::new("config.toml");
    if config_file_path.exists() {
        return File::open(config_file_path).with_context(|| "opening existing config.toml");
    }
    let manifest_dir = env!("CARGO_MANIFEST_DIR");

    let mut template = PathBuf::from(manifest_dir);
    let mut search_path = PathBuf::from(manifest_dir);
    template.push("config.toml.template");
    search_path.push("config.toml");
    if !search_path.exists() {
        log::info!("did not find config.toml, creating from config.toml.template");
        std::fs::copy(template, &search_path)?;
    }
    File::open(&search_path)
        .with_context(|| format!("opening newly created config.toml at {:?}", search_path))
}

pub mod toml {
    #[derive(Debug, Clone, serde::Deserialize)]
    pub struct Config {
        pub serial_port: String,
    }
}
