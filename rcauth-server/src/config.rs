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
    /// Returns a `Config` instance with default server settings.
    ///
    /// The default configuration binds both API and management servers to `0.0.0.0` with ports 8000 and 8001, respectively.
    /// Swagger and CORS are enabled by default, and all origins are allowed for CORS.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = Config::default();
    /// assert_eq!(config.api_server_port, 8000);
    /// assert!(config.enable_swagger);
    /// ```
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
    /// Returns a new `ConfigBuilder` for constructing a `Config` instance.
    ///
    /// Use the builder to set configuration options incrementally before validation and creation.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = Config::builder()
    ///     .api_server_host("127.0.0.1")
    ///     .api_server_port(8080)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    /// Returns the API server address as a `host:port` string.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = Config::default();
    /// let addr = config.api_addr();
    /// assert_eq!(addr, "127.0.0.1:8080");
    /// ```
    pub fn api_addr(&self) -> String {
        format!("{}:{}", self.api_server_host, self.api_server_port)
    }

    /// Returns the management server address as a `host:port` string.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = Config::default();
    /// let addr = config.management_addr();
    /// assert!(addr.contains(':'));
    /// ```
    pub fn management_addr(&self) -> String {
        format!(
            "{}:{}",
            self.management_server_host, self.management_server_port
        )
    }

    /// Validates the configuration for correctness.
    ///
    /// Checks that server hosts are valid IP addresses or "localhost", ports are non-zero, API and management servers do not share the same host and port, and if CORS is enabled, that allowed origins are specified.
    ///
    /// # Errors
    ///
    /// Returns an error if any validation fails, describing the specific issue.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = Config::default();
    /// assert!(config.validate().is_ok());
    /// ```
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
    /// Sets the API server host in the configuration builder.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = ConfigBuilder::default().api_server_host("127.0.0.1");
    /// ```
    pub fn api_server_host<T: Into<String>>(mut self, host: T) -> Self {
        self.api_server_host = Some(host.into());
        self
    }

    /// Sets the API server port for the configuration builder.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = ConfigBuilder::default().api_server_port(8080);
    /// ```
    pub fn api_server_port(mut self, port: u16) -> Self {
        self.api_server_port = Some(port);
        self
    }

    /// Sets the management server host for the configuration builder.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = ConfigBuilder::default().management_server_host("127.0.0.1");
    /// ```
    pub fn management_server_host<T: Into<String>>(mut self, host: T) -> Self {
        self.management_server_host = Some(host.into());
        self
    }

    /// Sets the management server port in the configuration builder.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = Config::builder().management_server_port(9090);
    /// ```
    pub fn management_server_port(mut self, port: u16) -> Self {
        self.management_server_port = Some(port);
        self
    }

    /// Sets whether Swagger documentation is enabled in the configuration builder.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = ConfigBuilder::default().enable_swagger(true);
    /// ```
    pub fn enable_swagger(mut self, enable: bool) -> Self {
        self.enable_swagger = Some(enable);
        self
    }

    /// Sets whether CORS is enabled in the configuration builder.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = ConfigBuilder::default().enable_cors(true);
    /// ```
    pub fn enable_cors(mut self, enable: bool) -> Self {
        self.enable_cors = Some(enable);
        self
    }

    /// Sets the list of allowed CORS origins for the configuration builder.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = ConfigBuilder::default()
    ///     .cors_allowed_origins(vec!["https://example.com", "https://another.com"]);
    /// ```
    pub fn cors_allowed_origins<T: Into<String>>(mut self, origins: Vec<T>) -> Self {
        self.cors_allowed_origins = Some(origins.into_iter().map(|o| o.into()).collect());
        self
    }

    /// Builds a `Config` instance from the provided builder values, applying defaults where necessary and validating the result.
    ///
    /// Returns a validated `Config` if all parameters are valid, or an error if validation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = Config::builder()
    ///     .api_server_host("127.0.0.1")
    ///     .api_server_port(8080)
    ///     .build()
    ///     .unwrap();
    /// assert_eq!(config.api_server_host, "127.0.0.1");
    /// ```
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
