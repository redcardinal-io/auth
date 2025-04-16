package migrations

import (
	"context"
	"database/sql"
	"fmt"
	"os"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upCreateAuditLogEntries, downCreateAuditLogEntries)
}

func upCreateAuditLogEntries(ctx context.Context, tx *sql.Tx) error {
	schemaName := os.Getenv("RCAUTH_SCHEMA_NAME")
	_, err := tx.ExecContext(ctx, fmt.Sprintf(`
		create table if not exists %s.audit_log_entries (
			id uuid not null,
			payload json null,
			created_at timestamptz null,
			constraint audit_log_entries_pkey primary key (id)
		);
	`, schemaName),
	)
	return err
}

func downCreateAuditLogEntries(ctx context.Context, tx *sql.Tx) error {
	_, err := tx.ExecContext(ctx, fmt.Sprintf(`drop table if exists %s.audit_log_entries;`, os.Getenv("RCAUTH_SCHEMA_NAME")))
	return err
}
