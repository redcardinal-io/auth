package migrations

import (
	"context"
	"database/sql"
	"fmt"
	"os"

	"github.com/pressly/goose/v3"
)

// init registers the initial migration functions with goose for database schema setup and teardown.
func init() {
	goose.AddMigrationContext(upInitial, downInitial)
}

// upInitial applies the initial database migration for the configured schema.
//
// This migration creates two PostgreSQL functions for managing automatic updates to the `updated_at` column via triggers, ensures the `uuid-ossp` extension is available for UUID generation, and creates a case-insensitive collation named `case_insensitive` using ICU. The schema name is determined by the `RCAUTH_SCHEMA_NAME` environment variable.
//
// Returns an error if any SQL command fails during execution.
func upInitial(ctx context.Context, tx *sql.Tx) error {
	schemaName := os.Getenv("RCAUTH_SCHEMA_NAME")
	_, err := tx.ExecContext(ctx, fmt.Sprintf(`
		create or replace function %s.goose_manage_updated_at(_tbl regclass) returns void as $$
		begin
    		execute format('create trigger set_updated_at before update on %%s
                    		for each row execute procedure %s.goose_set_updated_at()', _tbl, _tbl, '%s');
		end;
		$$ language plpgsql;

		create or replace function %s.goose_set_updated_at() returns trigger as $$
		begin
    		if (
        		new is distinct from old and
        		new.updated_at is not distinct from old.updated_at
    		) then
        		new.updated_at := current_timestamp;
    		end if;
    		return new;
		end;
		$$ language plpgsql;

		create extension if not exists "uuid-ossp";
		create collation if not exists case_insensitive (provider = icu, locale = 'und-u-ks-level2');
	`, schemaName,
		schemaName,
		schemaName,
		schemaName))

	return err
}

// downInitial reverses the initial migration by dropping the trigger functions, case-insensitive collation, and uuid-ossp extension from the schema specified by the RCAUTH_SCHEMA_NAME environment variable. Returns any error encountered during execution.
func downInitial(ctx context.Context, tx *sql.Tx) error {
	schemaName := os.Getenv("RCAUTH_SCHEMA_NAME")
	_, err := tx.ExecContext(ctx, fmt.Sprintf(`
		drop function if exists %s.goose_manage_updated_at(_tbl regclass);
		drop function if exists %s.goose_set_updated_at();
		drop collation if exists case_insensitive;
		drop extension if exists "uuid-ossp";
	`, schemaName,
		schemaName))

	return err
}
