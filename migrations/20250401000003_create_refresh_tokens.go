package migrations

import (
	"context"
	"database/sql"
	"fmt"
	"os"

	"github.com/pressly/goose/v3"
)

// init registers the migration functions for creating and dropping the refresh_tokens table with the goose migration tool.
func init() {
	goose.AddMigrationContext(upCreateRefreshTokens, downCreateRefreshTokens)
}

// upCreateRefreshTokens creates the refresh_tokens table and related indexes in the specified schema if they do not exist. 
// 
// The schema name is determined by the RCAUTH_SCHEMA_NAME environment variable. The table includes columns for a UUID primary key, token, user ID (with a foreign key constraint referencing the users table and cascade delete), revoked status, and timestamps. Indexes are created on the user_id and token columns, and a trigger or function is set up to manage the updated_at timestamp.
// 
// Returns an error if the SQL execution fails.
func upCreateRefreshTokens(ctx context.Context, tx *sql.Tx) error {
	schemaName := os.Getenv("RCAUTH_SCHEMA_NAME")
	_, err := tx.ExecContext(ctx, fmt.Sprintf(`
		create table if not exists %s.refresh_tokens (
			id uuid primary key default uuid_generate_v4(),
			"token" varchar(255) not null,
			user_id varchar(255) not null references %s.users(id) on delete cascade,
			revoked bool not null default false,
			created_at timestamptz not null default now(),
			updated_at timestamptz not null default now()
		);
		create index if not exists refresh_tokens_user_id_idx on %s.refresh_tokens using btree (user_id);
		create index if not exists refresh_tokens_token_idx on %s.refresh_tokens using btree (token);
		select %s.goose_manage_updated_at('%s.refresh_tokens');
	`,
		schemaName,
		schemaName,
		schemaName,
		schemaName,
		schemaName,
		schemaName,
	))

	return err
}

// downCreateRefreshTokens drops the refresh_tokens table from the schema specified by the RCAUTH_SCHEMA_NAME environment variable.
func downCreateRefreshTokens(ctx context.Context, tx *sql.Tx) error {
	_, err := tx.ExecContext(ctx, fmt.Sprintf(`drop table if exists %v.refresh_tokens;`, os.Getenv("RCAUTH_SCHEMA_NAME")))
	return err
}
