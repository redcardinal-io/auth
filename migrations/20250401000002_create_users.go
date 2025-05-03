package migrations

import (
	"context"
	"database/sql"
	"fmt"
	"os"

	"github.com/pressly/goose/v3"
)

// init registers the upCreateUsers and downCreateUsers migration functions with the goose migration tool.
func init() {
	goose.AddMigrationContext(upCreateUsers, downCreateUsers)
}

// upCreateUsers creates the "users" table and related indexes in the schema specified by the RCAUTH_SCHEMA_NAME environment variable.
// The table includes columns for user identification, authentication, metadata, timestamps, and enforces uniqueness on email and phone.
// It also invokes a custom function to manage the updated_at timestamp and creates indexes on frequently queried fields.
// Returns an error if the table creation or index setup fails.
func upCreateUsers(ctx context.Context, tx *sql.Tx) error {
	schemaName := os.Getenv("RCAUTH_SCHEMA_NAME")
	_, err := tx.ExecContext(ctx, fmt.Sprintf(`
  create table if not exists %v.users (
     id uuid primary key default uuid_generate_v4(),
     aud varchar(255),
     role varchar(255),
     email varchar(255) not null unique,
     encrypted_password varchar(255),
     email_confirmed_at timestamptz not null default now(),
     invited_at timestamptz,
     confirmation_token varchar(255),
     confirmation_sent_at timestamptz,
     recovery_token varchar(255),
     recovery_sent_at timestamptz,
     email_change_token varchar(255),
     email_change varchar(255),
     email_change_sent_at timestamptz,
     last_sign_in_at timestamptz,
     raw_app_metadata jsonb,
     raw_user_metadata jsonb,
     is_super_admin boolean default false,
     phone varchar(15) unique,
     phone_confirmed_at timestamptz,
     phone_change varchar(15) default '',
     phone_change_token varchar(255) default '',
     phone_change_sent_at timestamptz,
     created_at timestamptz not null default now(),
     updated_at timestamptz not null default now()  
  );

  select %s.goose_manage_updated_at('%s.users');
  
  -- Create indexes for commonly queried fields
  create index if not exists users_role_idx on %s.users("role");
  create index if not exists users_email_idx on %s.users(email);
  create index if not exists users_phone_idx on %s.users(phone);
  create index if not exists users_aud_idx on %s.users(aud);
  create index if not exists users_last_sign_in_at_idx on %s.users(last_sign_in_at);
`,
		schemaName,
		schemaName,
		schemaName,
		schemaName,
		schemaName,
		schemaName,
		schemaName,
		schemaName,
	))
	return err
}

// downCreateUsers drops the "users" table from the schema specified by the RCAUTH_SCHEMA_NAME environment variable.
// Returns an error if the environment variable is not set or if the table drop operation fails.
func downCreateUsers(ctx context.Context, tx *sql.Tx) error {
	schemaName := os.Getenv("RCAUTH_SCHEMA_NAME")
	if schemaName == "" {
		return fmt.Errorf("RCAUTH_SCHEMA_NAME environment variable is not set")
	}

	query := fmt.Sprintf(`DROP TABLE IF EXISTS %s.users;`, schemaName)

	_, err := tx.ExecContext(ctx, query)
	if err != nil {
		return fmt.Errorf("failed to drop users table: %w", err)
	}

	return nil
}
