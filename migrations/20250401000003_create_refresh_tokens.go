package migrations

import (
	"context"
	"database/sql"
	"fmt"
	"os"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upCreateRefreshTokens, downCreateRefreshTokens)
}

func upCreateRefreshTokens(ctx context.Context, tx *sql.Tx) error {
	schemaName := os.Getenv("RCAUTH_SCHEMA_NAME")
	_, err := tx.ExecContext(ctx, fmt.Sprintf(`
		create table if not exists %s.refresh_tokens (
			id uuid not null unique default uuid_generate_v4(),
			"token" varchar(255) null,
			user_id varchar(255) null,
			revoked bool null,
			created_at timestamptz null,
			updated_at timestamptz null,
			constraint refresh_tokens_pkey primary key (id)
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
	))

	return err
}

func downCreateRefreshTokens(ctx context.Context, tx *sql.Tx) error {
	_, err := tx.ExecContext(ctx, fmt.Sprintf(`drop table if exists %v.refresh_tokens;`, os.Getenv("RCAUTH_SCHEMA_NAME")))
	return err
}
