package migrations

import (
	"context"
	"database/sql"
	"fmt"
	"os"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upCreateAuthFunctions, downCreateAuthFunctions)
}

func upCreateAuthFunctions(ctx context.Context, tx *sql.Tx) error {
	schemaName := os.Getenv("RCAUTH_SCHEMA_NAME")
	_, err := tx.ExecContext(ctx, fmt.Sprintf(`
		-- Gets the User ID from the request cookie
		create or replace function %s.uid() returns uuid as $$
		select nullif(current_setting('request.jwt.claim.sub', true), '')::uuid;
		$$ language sql stable;

		-- Gets the User role from the request cookie
		create or replace function %s.role() returns text as $$
		select nullif(current_setting('request.jwt.claim.role', true), '')::text;
		$$ language sql stable;
	`, schemaName,
		schemaName))
	return err
}

func downCreateAuthFunctions(ctx context.Context, tx *sql.Tx) error {
	schemaName := os.Getenv("RCAUTH_SCHEMA_NAME")
	_, err := tx.Exec(fmt.Sprintf(`
		drop function if exists %s.uid();
		drop function if exists %s.role();
	`, schemaName,
		schemaName))
	return err
}
