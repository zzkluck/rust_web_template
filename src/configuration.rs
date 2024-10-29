#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub application_port: u16,
    pub comfy_address: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;
    settings.try_deserialize::<Settings>()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_configuration_should_success() {
        let configuration = get_configuration();
        assert!(configuration.is_ok())
    }
}