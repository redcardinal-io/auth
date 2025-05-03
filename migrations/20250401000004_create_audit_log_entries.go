package migrations

import (
	"context"
	"database/sql"
	"fmt"
	"os"

	"github.com/pressly/goose/v3"
)

// init registers the migration functions for creating and dropping the audit_log_entries table with the goose migration tool.
func init() {
	goose.AddMigrationContext(upCreateAuditLogEntries, downCreateAuditLogEntries)
}

// upCreateAuditLogEntries creates the audit_log_entries table in the schema specified by the RCAUTH_SCHEMA_NAME environment variable. The table includes columns for a UUID primary key, a nullable JSON payload, and a timestamp with time zone. Returns an error if the table creation fails.
func upCreateAuditLogEntries(ctx context.Context, tx *sql.Tx) error {
	schemaName := os.Getenv("RCAUTH_SCHEMA_NAME")
	_, err := tx.ExecContext(ctx, fmt.Sprintf(`
		create table if not exists %s.audit_log_entries (
			id uuid primary key not null default uuid_generate_v4(),
			payload json null,
			created_at timestamptz not null default now()
		);
	`, schemaName),
	)
	return err
}

// downCreateAuditLogEntries drops the audit_log_entries table from the schema specified by the RCAUTH_SCHEMA_NAME environment variable.
func downCreateAuditLogEntries(ctx context.Context, tx *sql.Tx) error {
	_, err := tx.ExecContext(ctx, fmt.Sprintf(`drop table if exists %s.audit_log_entries;`, os.Getenv("RCAUTH_SCHEMA_NAME")))
	return err
}
