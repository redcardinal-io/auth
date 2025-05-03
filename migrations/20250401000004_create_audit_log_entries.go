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

// upCreateAuditLogEntries creates the audit_log_entries table in the schema specified by the RCAUTH_SCHEMA_NAME environment variable. The table includes an id as the primary key, a JSON payload, and a created_at timestamp.
// Returns an error if the table creation fails.
func upCreateAuditLogEntries(ctx context.Context, tx *sql.Tx) error {
	schemaName := os.Getenv("RCAUTH_SCHEMA_NAME")
	_, err := tx.ExecContext(ctx, fmt.Sprintf(`
		create table if not exists %s.audit_log_entries (
			id uuid not null,
			payload json null,
			created_at timestamptz not null default now(),
			constraint audit_log_entries_pkey primary key (id)
		);
	`, schemaName),
	)
	return err
}

// downCreateAuditLogEntries drops the audit_log_entries table from the schema specified by the RCAUTH_SCHEMA_NAME environment variable. Returns an error if the operation fails.
func downCreateAuditLogEntries(ctx context.Context, tx *sql.Tx) error {
	_, err := tx.ExecContext(ctx, fmt.Sprintf(`drop table if exists %s.audit_log_entries;`, os.Getenv("RCAUTH_SCHEMA_NAME")))
	return err
}
