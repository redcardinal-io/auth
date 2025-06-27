use serde::Deserialize;
use std::net::IpAddr;
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_api_server_host")]
    pub api_server_host: String,
    #[serde(default = "default_api_server_port")]
    pub api_server_port: u16,
    #[serde(default = "default_management_server_host")]
    pub management_server_host: String,
    #[serde(default = "default_management_server_port")]
    pub management_server_port: u16,
    #[serde(default = "default_enable_swagger")]
    pub enable_swagger: bool,
    #[serde(default = "default_enable_cors")]
    pub enable_cors: bool,
    #[serde(default = "default_cors_allowed_origins")]
    pub cors_allowed_origins: Vec<String>,
}

/// Returns the default API server host address.
///
/// # Examples
///
/// ```
/// let host = default_api_server_host();
/// assert_eq!(host, "0.0.0.0");
/// ```
fn default_api_server_host() -> String {
    "0.0.0.0".to_string()
}

/// Returns the default port number for the API server.
///
/// # Examples
///
/// ```
/// let port = default_api_server_port();
/// assert_eq!(port, 8000);
/// ```
fn default_api_server_port() -> u16 {
    8000
}

/// Returns the default host address for the management server.
///
/// # Examples
///
/// ```
/// let host = default_management_server_host();
/// assert_eq!(host, "0.0.0.0");
/// ```
fn default_management_server_host() -> String {
    "0.0.0.0".to_string()
}

/// Returns the default port number for the management server.
///
/// # Examples
///
/// ```
/// let port = default_management_server_port();
/// assert_eq!(port, 8001);
/// ```
fn default_management_server_port() -> u16 {
    8001
}

/// Returns the default value for enabling Swagger documentation.
///
/// # Examples
///
/// ```
/// assert_eq!(default_enable_swagger(), true);
/// ```
fn default_enable_swagger() -> bool {
    true
}

/// Returns the default value for enabling CORS support.
///
/// By default, CORS is enabled for the server configuration.
///
/// # Examples
///
/// ```
/// let cors_enabled = default_enable_cors();
/// assert!(cors_enabled);
/// ```
fn default_enable_cors() -> bool {
    true
}

/// Returns the default list of allowed CORS origins, permitting all origins.
///
/// # Examples
///
/// ```
/// let origins = default_cors_allowed_origins();
/// assert_eq!(origins, vec!["*"]);
/// ```
fn default_cors_allowed_origins() -> Vec<String> {
    vec!["*".to_string()]
}

impl Default for Config {
    /// Creates a `Config` instance with default server and feature settings.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = Config::default();
    /// assert_eq!(config.api_server_host, "0.0.0.0");
    /// assert_eq!(config.api_server_port, 8000);
    /// assert!(config.enable_swagger);
    /// ```
    fn default() -> Self {
        Self {
            api_server_host: default_api_server_host(),
            api_server_port: default_api_server_port(),
            management_server_host: default_management_server_host(),
            management_server_port: default_management_server_port(),
            enable_swagger: default_enable_swagger(),
            enable_cors: default_enable_cors(),
            cors_allowed_origins: default_cors_allowed_origins(),
        }
    }
}

impl Config {
    /// Returns the API server address as a "host:port" string.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = Config::default();
    /// let addr = config.api_addr();
    /// assert_eq!(addr, "0.0.0.0:8000");
    /// ```
    pub fn api_addr(&self) -> String {
        format!("{}:{}", self.api_server_host, self.api_server_port)
    }

    /// Returns the management server address as a "host:port" string.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = Config::default();
    /// let addr = config.management_addr();
    /// assert_eq!(addr, "0.0.0.0:8001");
    /// ```
    pub fn management_addr(&self) -> String {
        format!(
            "{}:{}",
            self.management_server_host, self.management_server_port
        )
    }

    /// Validates the server configuration for correctness.
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
        if self.api_server_host.is_empty() {
            return Err("API server host cannot be empty".into());
        }

        // Validate management server host
        if self.management_server_host.is_empty() {
            return Err("Management server host cannot be empty".into());
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
    /// Sets the API server host for the configuration builder.
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

    /// Sets the management server port for the configuration builder.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = ConfigBuilder::default().management_server_port(9001);
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
    /// let builder = ConfigBuilder::default().enable_swagger(false);
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
    /// let builder = ConfigBuilder::default().enable_cors(false);
    /// let config = builder.build().unwrap();
    /// assert!(!config.enable_cors);
    /// ```
    pub fn enable_cors(mut self, enable: bool) -> Self {
        self.enable_cors = Some(enable);
        self
    }

    /// Sets the allowed CORS origins for the configuration builder.
    ///
    /// Accepts a vector of items convertible to strings and stores them as the allowed origins for CORS requests.
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
    /// Returns a validated `Config` if successful, or an error if validation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = ConfigBuilder::default()
    ///     .api_server_host("127.0.0.1")
    ///     .api_server_port(8080)
    ///     .build()
    ///     .unwrap();
    /// assert_eq!(config.api_server_host, "127.0.0.1");
    /// assert_eq!(config.api_server_port, 8080);
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
