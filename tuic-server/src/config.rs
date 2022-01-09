use getopts::{Fail, Options};
use std::num::ParseIntError;
use thiserror::Error;

pub struct ConfigBuilder<'cfg> {
    opts: Options,
    program: Option<&'cfg str>,
}

impl<'cfg> ConfigBuilder<'cfg> {
    pub fn new() -> Self {
        let mut opts = Options::new();

        opts.reqopt(
            "p",
            "port",
            "Set the listening port(Required)",
            "SERVER_PORT",
        );
        opts.reqopt(
            "t",
            "token",
            "Set the TUIC token for the authentication(Required)",
            "TOKEN",
        );

        opts.optflag("v", "version", "Print the version");
        opts.optflag("h", "help", "Print this help menu");

        Self {
            opts,
            program: None,
        }
    }

    pub fn get_usage(&self) -> String {
        self.opts.usage(&format!(
            "Usage: {} [options]",
            self.program.unwrap_or("tuic-server")
        ))
    }

    pub fn parse(&mut self, args: &'cfg [String]) -> Result<Config, ConfigError> {
        self.program = Some(&args[0]);

        let matches = self
            .opts
            .parse(&args[1..])
            .map_err(|err| ConfigError::Parse(err, self.get_usage()))?;

        if !matches.free.is_empty() {
            return Err(ConfigError::UnexpectedArgument(
                matches.free.join(", "),
                self.get_usage(),
            ));
        }

        if matches.opt_present("v") {
            return Err(ConfigError::Version(env!("CARGO_PKG_VERSION")));
        }

        if matches.opt_present("h") {
            return Err(ConfigError::Help(self.get_usage()));
        }

        let port = matches
            .opt_str("p")
            .unwrap()
            .parse()
            .map_err(|err| ConfigError::ParsePort(err, self.get_usage()))?;

        let token = {
            let token = matches.opt_str("t").unwrap();
            seahash::hash(&token.into_bytes())
        };

        Ok(Config { token, port })
    }
}

pub struct Config {
    pub port: u16,
    pub token: u64,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("{0}\n\n{1}")]
    Parse(Fail, String),
    #[error("Unexpected urgument: {0}\n\n{1}")]
    UnexpectedArgument(String, String),
    #[error("Failed to parse the port: {0}\n\n{1}")]
    ParsePort(ParseIntError, String),
    #[error("{0}")]
    Version(&'static str),
    #[error("{0}")]
    Help(String),
}
