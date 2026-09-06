// DISABLED!!!
use serde::toml;

#[derive(Debug,Deserialize)]
pub struct Config {
    pub color: String,
    pub size: SizeOpts,
}

#[derive(Debug,Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SizeOpts {
    Big,
    Mid,
    Smol,
}

impl Config {
    fn read_from_conf(&mut self, path: &str) -> anyhow::Result<Config> {
        let conf = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&conf)?;
        Ok(config)
    }
}
