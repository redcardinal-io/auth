package conf

import (
	"bytes"
	"net/url"
	"os"
	"path/filepath"
	"regexp"
	"strings"
	"time"

	"github.com/gobwas/glob"
	"github.com/joho/godotenv"
	"github.com/kelseyhightower/envconfig"
)

const defaultMinPasswordLength int = 6
const defaultChallengeExpiryDuration float64 = 300
const defaultFactorExpiryDuration time.Duration = 300 * time.Second
const defaultFlowStateExpiryDuration time.Duration = 300 * time.Second

// See: https://www.postgresql.org/docs/7.0/syntax525.htm
var postgresNamesRegexp = regexp.MustCompile(`^[a-zA-Z_][a-zA-Z0-9_]{0,62}$`)

// See: https://github.com/standard-webhooks/standard-webhooks/blob/main/spec/standard-webhooks.md
// We use 4 * Math.ceil(n/3) to obtain unpadded length in base 64
// So this 4 * Math.ceil(24/3) = 32 and 4 * Math.ceil(64/3) = 88 for symmetric secrets
// Since Ed25519 key is 32 bytes so we have 4 * Math.ceil(32/3) = 44
var symmetricSecretFormat = regexp.MustCompile(`^v1,whsec_[A-Za-z0-9+/=]{32,88}`)
var asymmetricSecretFormat = regexp.MustCompile(`^v1a,whpk_[A-Za-z0-9+/=]{44,}:whsk_[A-Za-z0-9+/=]{44,}$`)

// Time is used to represent timestamps in the configuration, as envconfig has
// trouble parsing empty strings, due to time.Time.UnmarshalText().
type Time struct {
	time.Time
}

func (t *Time) UnmarshalText(text []byte) error {
	trimed := bytes.TrimSpace(text)

	if len(trimed) < 1 {
		t.Time = time.Time{}
	} else {
		if err := t.Time.UnmarshalText(trimed); err != nil {
			return err
		}
	}

	return nil
}

type APIConfiguration struct {
	Host               string
	Port               string `envconfig:"PORT" default:"8081"`
	Endpoint           string
	RequestIDHeader    string        `envconfig:"REQUEST_ID_HEADER"`
	ExternalURL        string        `json:"external_url" envconfig:"API_EXTERNAL_URL" required:"true"`
	MaxRequestDuration time.Duration `json:"max_request_duration" split_words:"true" default:"10s"`
}

func (a *APIConfiguration) Validate() error {
	_, err := url.ParseRequestURI(a.ExternalURL)
	if err != nil {
		return err
	}

	return nil
}

type DBConfiguration struct {
	Driver    string `json:"driver" required:"true"`
	URL       string `json:"url" envconfig:"DATABASE_URL" required:"true"`
	Namespace string `json:"namespace" envconfig:"DB_NAMESPACE" default:"auth"`
	// MaxPoolSize defaults to 0 (unlimited).
	MaxPoolSize       int           `json:"max_pool_size" split_words:"true"`
	MaxIdlePoolSize   int           `json:"max_idle_pool_size" split_words:"true"`
	ConnMaxLifetime   time.Duration `json:"conn_max_lifetime,omitempty" split_words:"true"`
	ConnMaxIdleTime   time.Duration `json:"conn_max_idle_time,omitempty" split_words:"true"`
	HealthCheckPeriod time.Duration `json:"health_check_period" split_words:"true"`
	MigrationsPath    string        `json:"migrations_path" split_words:"true" default:"./migrations"`
	CleanupEnabled    bool          `json:"cleanup_enabled" split_words:"true" default:"false"`
}

func (c *DBConfiguration) Validate() error {
	return nil
}

type GlobalConfiguration struct {
	API           APIConfiguration
	DB            DBConfiguration
	Logging       LoggingConfig `envconfig:"LOG"`
	OperatorToken string        `split_words:"true" required:"false"`

	RateLimitHeader         string  `split_words:"true"`
	RateLimitEmailSent      Rate    `split_words:"true" default:"30"`
	RateLimitSmsSent        Rate    `split_words:"true" default:"30"`
	RateLimitVerify         float64 `split_words:"true" default:"30"`
	RateLimitTokenRefresh   float64 `split_words:"true" default:"150"`
	RateLimitSso            float64 `split_words:"true" default:"30"`
	RateLimitAnonymousUsers float64 `split_words:"true" default:"30"`
	RateLimitOtp            float64 `split_words:"true" default:"30"`

	SiteURL         string   `json:"site_url" split_words:"true" required:"true"`
	URIAllowList    []string `json:"uri_allow_list" split_words:"true"`
	URIAllowListMap map[string]glob.Glob
	DisableSignup   bool `json:"disable_signup" split_words:"true"`
}

func loadEnvironment(filename string) error {
	var err error
	if filename != "" {
		err = godotenv.Overload(filename)
	} else {
		err = godotenv.Load()
		if os.IsNotExist(err) {
			return nil
		}
	}
	return err
}

func LoadGlobal(filename string) (*GlobalConfiguration, error) {
	if err := loadEnvironment(filename); err != nil {
		return nil, err
	}

	config := new(GlobalConfiguration)
	if err := loadGlobal(config); err != nil {
		return nil, err
	}
	return config, nil
}

func loadGlobal(config *GlobalConfiguration) error {
	if err := envconfig.Process("rcauth", config); err != nil {
		return err
	}

	if err := config.ApplyDefaults(); err != nil {
		return err
	}
	if err := config.Validate(); err != nil {
		return err
	}

	return nil
}

func (config *GlobalConfiguration) ApplyDefaults() error {

	if config.URIAllowList == nil {
		config.URIAllowList = []string{}
	}

	if config.URIAllowList != nil {
		config.URIAllowListMap = make(map[string]glob.Glob)
		for _, uri := range config.URIAllowList {
			g := glob.MustCompile(uri, '.', '/')
			config.URIAllowListMap[uri] = g
		}
	}

	return populateGlobal(config)
}

func (c *GlobalConfiguration) Validate() error {
	validatables := []interface {
		Validate() error
	}{
		&c.API,
		&c.DB,
	}

	for _, validatable := range validatables {
		if err := validatable.Validate(); err != nil {
			return err
		}
	}

	return nil
}

func populateGlobal(_ *GlobalConfiguration) error {
	return nil
}

// LoadFile calls godotenv.Load() when the given filename is empty ignoring any
// errors loading, otherwise it calls godotenv.Overload(filename).
//
// godotenv.Load: preserves env, ".env" path is optional
// godotenv.Overload: overrides env, "filename" path must exist
func LoadFile(filename string) error {
	var err error
	if filename != "" {
		err = godotenv.Overload(filename)
	} else {
		err = godotenv.Load()
		// handle if .env file does not exist, this is OK
		if os.IsNotExist(err) {
			return nil
		}
	}
	return err
}

// LoadDirectory does nothing when configDir is empty, otherwise it will attempt
// to load a list of configuration files located in configDir by using ReadDir
// to obtain a sorted list of files containing a .env suffix.
//
// When the list is empty it will do nothing, otherwise it passes the file list
// to godotenv.Overload to pull them into the current environment.
func LoadDirectory(configDir string) error {
	if configDir == "" {
		return nil
	}

	// Returns entries sorted by filename
	ents, err := os.ReadDir(configDir)
	if err != nil {
		// We mimic the behavior of LoadGlobal here, if an explicit path is
		// provided we return an error.
		return err
	}

	var paths []string
	for _, ent := range ents {
		if ent.IsDir() {
			continue // ignore directories
		}

		// We only read files ending in .env
		name := ent.Name()
		if !strings.HasSuffix(name, ".env") {
			continue
		}

		// ent.Name() does not include the watch dir.
		paths = append(paths, filepath.Join(configDir, name))
	}

	// If at least one path was found we load the configuration files in the
	// directory. We don't call override without config files because it will
	// override the env vars previously set with a ".env", if one exists.
	return loadDirectoryPaths(paths...)
}

func loadDirectoryPaths(p ...string) error {
	// If at least one path was found we load the configuration files in the
	// directory. We don't call override without config files because it will
	// override the env vars previously set with a ".env", if one exists.
	if len(p) > 0 {
		if err := godotenv.Overload(p...); err != nil {
			return err
		}
	}
	return nil
}

// LoadGlobalFromEnv will return a new *GlobalConfiguration value from the
// currently configured environment.
func LoadGlobalFromEnv() (*GlobalConfiguration, error) {
	config := new(GlobalConfiguration)
	if err := loadGlobal(config); err != nil {
		return nil, err
	}
	return config, nil
}
