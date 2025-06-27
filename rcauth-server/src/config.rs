use serde::Deserialize;
use std::net::IpAddr;
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "RCAUTH_API_SERVER_HOST")]
    pub api_server_host: String,
    #[serde(rename = "RCAUTH_API_SERVER_PORT")]
    pub api_server_port: u16,
    #[serde(rename = "RCAUTH_MANAGEMENT_SERVER_HOST")]
    pub management_server_host: String,
    #[serde(rename = "RCAUTH_MANAGEMENT_SERVER_PORT")]
    pub management_server_port: u16,
    #[serde(rename = "RCAUTH_ENABLE_SWAGGER")]
    pub enable_swagger: bool,
    #[serde(rename = "RCAUTH_ENABLE_CORS")]
    pub enable_cors: bool,
    #[serde(rename = "RCAUTH_CORS_ALLOWED_ORIGINS")]
    pub cors_allowed_origins: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_server_host: "0.0.0.0".to_string(),
            api_server_port: 8000,
            management_server_host: "0.0.0.0".to_string(),
            management_server_port: 8001,
            enable_swagger: true,
            enable_cors: true,
            cors_allowed_origins: vec!["*".to_string()],
        }
    }
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    pub fn api_addr(&self) -> String {
        format!("{}:{}", self.api_server_host, self.api_server_port)
    }

    pub fn management_addr(&self) -> String {
        format!(
            "{}:{}",
            self.management_server_host, self.management_server_port
        )
    }

    pub fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Validate API server host
        if IpAddr::from_str(&self.api_server_host).is_err() && self.api_server_host != "localhost" {
            return Err(format!("Invalid API server host: {}", self.api_server_host).into());
        }

        // Validate management server host
        if IpAddr::from_str(&self.management_server_host).is_err()
            && self.management_server_host != "localhost"
        {
            return Err(format!(
                "Invalid management server host: {}",
                self.management_server_host
            )
            .into());
        }

        // Validate port numbers
        if self.api_server_port == 0 {
            return Err("API server port cannot be 0".into());
        }

        if self.management_server_port == 0 {
            return Err("Management server port cannot be 0".into());
        }

        // Validate that API and management servers don't use the same port if on the same host
        if self.api_server_host == self.management_server_host
            && self.api_server_port == self.management_server_port
        {
            return Err(format!(
                "API and management servers cannot share the same host:port combination ({}:{})",
                self.api_server_host, self.api_server_port
            )
            .into());
        }

        // If CORS is enabled, validate that we have allowed origins
        if self.enable_cors && self.cors_allowed_origins.is_empty() {
            return Err("CORS is enabled but no allowed origins are specified".into());
        }

        Ok(())
    }
}

#[derive(Default)]
pub struct ConfigBuilder {
    api_server_host: Option<String>,
    api_server_port: Option<u16>,
    management_server_host: Option<String>,
    management_server_port: Option<u16>,

    enable_swagger: Option<bool>,
    enable_cors: Option<bool>,
    cors_allowed_origins: Option<Vec<String>>,
}

impl ConfigBuilder {
    pub fn api_server_host<T: Into<String>>(mut self, host: T) -> Self {
        self.api_server_host = Some(host.into());
        self
    }

    pub fn api_server_port(mut self, port: u16) -> Self {
        self.api_server_port = Some(port);
        self
    }

    pub fn management_server_host<T: Into<String>>(mut self, host: T) -> Self {
        self.management_server_host = Some(host.into());
        self
    }

    pub fn management_server_port(mut self, port: u16) -> Self {
        self.management_server_port = Some(port);
        self
    }

    pub fn enable_swagger(mut self, enable: bool) -> Self {
        self.enable_swagger = Some(enable);
        self
    }

    pub fn enable_cors(mut self, enable: bool) -> Self {
        self.enable_cors = Some(enable);
        self
    }

    pub fn cors_allowed_origins<T: Into<String>>(mut self, origins: Vec<T>) -> Self {
        self.cors_allowed_origins = Some(origins.into_iter().map(|o| o.into()).collect());
        self
    }

    pub fn build(self) -> Result<Config, Box<dyn std::error::Error>> {
        let default_config = Config::default();

        let config = Config {
            api_server_host: self
                .api_server_host
                .unwrap_or(default_config.api_server_host),
            api_server_port: self
                .api_server_port
                .unwrap_or(default_config.api_server_port),
            management_server_host: self
                .management_server_host
                .unwrap_or(default_config.management_server_host),
            management_server_port: self
                .management_server_port
                .unwrap_or(default_config.management_server_port),
            enable_swagger: self.enable_swagger.unwrap_or(default_config.enable_swagger),
            enable_cors: self.enable_cors.unwrap_or(default_config.enable_cors),
            cors_allowed_origins: self
                .cors_allowed_origins
                .unwrap_or(default_config.cors_allowed_origins),
        };

        // Validate the configuration
        config.validate()?;

        Ok(config)
    }
}
