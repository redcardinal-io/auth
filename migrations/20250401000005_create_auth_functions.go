package migrations

import (
	"context"
	"database/sql"
	"fmt"
	"os"

	"github.com/pressly/goose/v3"
)

// init registers the migration functions for creating and dropping authentication SQL functions with goose.
func init() {
	goose.AddMigrationContext(upCreateAuthFunctions, downCreateAuthFunctions)
}

// upCreateAuthFunctions creates or replaces SQL functions in the specified schema to extract the JWT subject and role claims from the current session context. The functions return default values if the claims are missing or empty. Returns an error if the SQL execution fails.
func upCreateAuthFunctions(ctx context.Context, tx *sql.Tx) error {
	schemaName := os.Getenv("RCAUTH_SCHEMA_NAME")
	_, err := tx.ExecContext(ctx, fmt.Sprintf(`
		create or replace function %s.uid() returns uuid as $$
		select coalesce(nullif(current_setting('request.jwt.claim.sub', true), '')::uuid, '00000000-0000-0000-0000-000000000000');
		$$ language sql stable;

		create or replace function %s.role() returns text as $$
    select coalesce(nullif(current_setting('request.jwt.claim.role', true), '')::text, '');
		$$ language sql stable;
	`, schemaName,
		schemaName))
	return err
}

// downCreateAuthFunctions drops the uid() and role() SQL functions from the specified schema if they exist.
func downCreateAuthFunctions(ctx context.Context, tx *sql.Tx) error {
	schemaName := os.Getenv("RCAUTH_SCHEMA_NAME")
	_, err := tx.ExecContext(ctx, fmt.Sprintf(`
		drop function if exists %s.uid();
		drop function if exists %s.role();
	`, schemaName,
		schemaName))
	return err
}
