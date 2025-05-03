package cmd

import (
	"database/sql"
	"fmt"
	"os"
	"strings"

	_ "github.com/jackc/pgx/v5/stdlib"
	"github.com/pressly/goose/v3"
	"github.com/spf13/cobra"
	"go.uber.org/zap"

	"github.com/redcardinal-io/auth/domain/pkg/config"
	"github.com/redcardinal-io/auth/domain/pkg/logger"
	_ "github.com/redcardinal-io/auth/migrations"
)

var (
	dbString     string
	migrationDir = "migrations"
	lg           *logger.Logger
)

// init configures the logger, migration command flags, and registers migration subcommands with the root CLI command.
func init() {
	lg, _ = logger.NewLogger(&config.LoggerConfig{
		Level: config.INFO,
		Mode:  "dev",
	})

	migrateCmd.Flags().StringVarP(
		&dbString,
		"db-string",
		"d",
		os.Getenv("RCAUTH_POSTGRES_URL"),
		"PostgreSQL database connection string (or set RCAUTH_POSTGRES_URL env var)",
	)

	// Add up and down subcommands
	migrateCmd.AddCommand(upCmd)
	migrateCmd.AddCommand(downCmd)

	rootCmd.AddCommand(migrateCmd)
}

var migrateCmd = &cobra.Command{
	Use:   "migrate",
	Short: "Database migration commands",
	Long:  "Run database migrations for PostgreSQL",
}

var upCmd = &cobra.Command{
	Use:   "up",
	Short: "Run migrations up",
	Long:  "Apply all pending migrations",
	RunE: func(cmd *cobra.Command, args []string) error {
		return runPostgresMigrations("up")
	},
}

var downCmd = &cobra.Command{
	Use:   "down",
	Short: "Reset all migrations",
	Long:  "Revert all migrations",
	RunE: func(cmd *cobra.Command, args []string) error {
		fmt.Println("WARNING: This will reset ALL migrations. All data will be lost.")
		fmt.Print("Are you sure you want to continue? (yes/no): ")
		var response string
		fmt.Scanln(&response)

		if strings.ToLower(response) != "yes" {
			fmt.Println("Operation cancelled.")
			return nil
		}
		return runPostgresMigrations("down")
	},
}

// runPostgresMigrations executes PostgreSQL database migrations in the specified direction ("up" to apply all pending migrations or "down" to reset all migrations).
// 
// Validates required environment variables and connection parameters, sets the migration version table based on the schema, establishes a database connection, and runs the migrations using the goose library.
// 
// Returns an error if required configuration is missing, the database connection fails, the migration direction is invalid, or if migration execution encounters an error.
func runPostgresMigrations(direction string) error {
	if dbString == "" {
		return fmt.Errorf("PostgreSQL database connection string is required")
	}

	if os.Getenv("RCAUTH_SCHEMA_NAME") == "" {
		return fmt.Errorf("RCAUTH_SCHEMA_NAME environment variable is required")
	}

	goose.SetTableName(fmt.Sprintf("%s.goose_db_version", os.Getenv("RCAUTH_SCHEMA_NAME")))

	lg.Info("Connecting to PostgreSQL database...")
	db, err := sql.Open("pgx", dbString)
	if err != nil {
		lg.Error("failed to connect to PostgreSQL database", zap.Error(err))
		return fmt.Errorf("failed to connect to PostgreSQL database: %w", err)
	}
	defer db.Close()

	if err := db.Ping(); err != nil {
		lg.Error("failed to ping PostgreSQL database", zap.Error(err))
		return fmt.Errorf("failed to ping PostgreSQL database: %w", err)
	}

	if err := goose.SetDialect("postgres"); err != nil {
		lg.Error("failed to set PostgreSQL dialect", zap.Error(err))
		return fmt.Errorf("failed to set PostgreSQL dialect: %w", err)
	}

	var migrationErr error
	if direction == "up" {
		lg.Info("Running migrations up...")
		migrationErr = goose.Up(db, migrationDir)
	} else if direction == "down" {
		lg.Info("Resetting all migrations...")
		migrationErr = goose.Reset(db, migrationDir)
	} else {
		return fmt.Errorf("invalid migration direction: %s", direction)
	}

	if migrationErr != nil {
		lg.Error("failed to run PostgreSQL migrations",
			zap.String("direction", direction),
			zap.Error(migrationErr))
		return fmt.Errorf("failed to run PostgreSQL migrations (%s): %w", direction, migrationErr)
	}

	lg.Info("PostgreSQL migrations completed successfully", zap.String("direction", direction))
	return nil
}
