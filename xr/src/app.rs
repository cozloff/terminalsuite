use crate::{
    config::{AppConfig, Mode},
    openxr::probe,
};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::from_env();

    match config.mode {
        Mode::Probe => {
            let report = probe::run(&config)?;
            report.print();
        }
    }

    Ok(())
}
