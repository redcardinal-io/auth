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

// upCreateAuthFunctions creates or replaces the SQL functions uid() and role() in the schema specified by the RCAUTH_SCHEMA_NAME environment variable. 
// The uid() function returns a UUID from the current session's JWT subject claim, defaulting to a zero UUID if missing. 
// The role() function returns the JWT role claim as text, defaulting to an empty string if missing.
// Returns any error encountered during execution.
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

// downCreateAuthFunctions drops the uid() and role() SQL functions from the schema specified by the RCAUTH_SCHEMA_NAME environment variable.
func downCreateAuthFunctions(ctx context.Context, tx *sql.Tx) error {
	schemaName := os.Getenv("RCAUTH_SCHEMA_NAME")
	_, err := tx.ExecContext(ctx, fmt.Sprintf(`
		drop function if exists %s.uid();
		drop function if exists %s.role();
	`, schemaName,
		schemaName))
	return err
}
