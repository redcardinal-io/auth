package config

import (
	"fmt"
	"log"
	"strings"

	"github.com/spf13/viper"
)

type Config struct {
	Server ServerConfig
	Logger LoggerConfig
}

type ServerConfig struct {
	Host string
	Port string
}

type LogLevel string

const (
	INFO     LogLevel = "info"
	WARN     LogLevel = "warn"
	CRITICAL LogLevel = "critical"
	ERROR    LogLevel = "error"
)

type LoggerConfig struct {
	Level   LogLevel
	LogFile string
	Mode    string
}

func initializeViper() error {
	viper.SetEnvPrefix("rcauth")
	viper.SetConfigName("config")
	viper.SetConfigType("env")
	viper.AddConfigPath(".")

	// Allow viper to use environment variables
	viper.AutomaticEnv()

	// Configure viper to handle nested keys
	viper.SetEnvKeyReplacer(strings.NewReplacer(".", "_"))

	// Tell viper about the structure of our config
	viper.BindEnv("server.host", "RCAUTH_SERVER_HOST")
	viper.BindEnv("server.port", "RCAUTH_SERVER_PORT")
	viper.BindEnv("logger.level", "RCAUTH_LOGGER_LEVEL")
	viper.BindEnv("logger.logfile", "RCAUTH_LOGGER_LOGFILE")
	viper.BindEnv("logger.mode", "RCAUTH_LOGGER_MODE")

	if err := viper.ReadInConfig(); err != nil {
		log.Printf("Error reading config file: %s", err)
		log.Println("Using environment variables")
		return nil
	}

	return nil
}

func setDefaults() {
	viper.SetDefault("server.host", "localhost")
	viper.SetDefault("server.port", "8000")
	viper.SetDefault("logger.level", string(INFO))
	viper.SetDefault("logger.mode", "dev")
}

func validateConfig() error {
	if viper.GetString("server.host") == "" {
		return fmt.Errorf("server host is required")
	}
	if viper.GetString("server.port") == "" {
		return fmt.Errorf("server port must be greater than 0")
	}
	if viper.GetString("logger.level") == "" {
		return fmt.Errorf("logger level is required")
	}
	if viper.GetString("logger.mode") == "" {
		return fmt.Errorf("logger mode is required")
	}
	return nil
}

func LoadConfig() (*Config, error) {
	if err := initializeViper(); err != nil {
		return nil, err
	}

	setDefaults()

	if err := validateConfig(); err != nil {
		return nil, err
	}

	var config Config
	if err := viper.Unmarshal(&config); err != nil {
		return nil, fmt.Errorf("unable to decode into struct: %v", err)
	}

	return &config, nil
}
