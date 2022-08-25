#[derive(Default, Builder, Debug, Clone)]
pub struct Config {
    pub home: String,

    pub server: String,

    pub deamon: bool,
}

impl ConfigBuilder {}
