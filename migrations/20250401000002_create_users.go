package migrations

import (
	"context"
	"database/sql"
	"fmt"
	"os"

	"github.com/pressly/goose/v3"
)

// init registers the migration functions for creating and dropping the users table with the goose migration tool.
func init() {
	goose.AddMigrationContext(upCreateUsers, downCreateUsers)
}

// upCreateUsers creates the "users" table within the schema specified by the RCAUTH_SCHEMA_NAME environment variable, including all necessary columns, constraints, and indexes for user management.
func upCreateUsers(ctx context.Context, tx *sql.Tx) error {
	schemaName := os.Getenv("RCAUTH_SCHEMA_NAME")
	_, err := tx.ExecContext(ctx, fmt.Sprintf(`
  create table if not exists %v.users (
	  id uuid not null unique default uuid_generate_v4(),
	  aud varchar(255) null,
	  "role" varchar(255) null,
	  email varchar(255) null unique,
	  encrypted_password varchar(255) null,
	  email_confirmed_at timestamptz null,
	  invited_at timestamptz null,
	  confirmation_token varchar(255) null,
	  confirmation_sent_at timestamptz null,
	  recovery_token varchar(255) null,
	  recovery_sent_at timestamptz null,
	  email_change_token varchar(255) null,
	  email_change varchar(255) null,
	  email_change_sent_at timestamptz null,
	  last_sign_in_at timestamptz null,
	  raw_app_meta_data jsonb null,
	  raw_user_meta_data jsonb null,
	  is_super_admin bool null,
	  phone varchar(15) null unique default null,
	  phone_confirmed_at timestamptz null default null,
	  phone_change varchar(15) null default '',
	  phone_change_token varchar(255) null default '',
	  phone_change_sent_at timestamptz null default null,
	  created_at timestamptz null default now(),
	  updated_at timestamptz null default now(),
	  constraint users_pkey primary key (id)
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

// downCreateUsers drops the "users" table from the schema specified by the RCAUTH_SCHEMA_NAME environment variable if it exists.
func downCreateUsers(ctx context.Context, tx *sql.Tx) error {
	schemaName := os.Getenv("RCAUTH_SCHEMA_NAME")
	_, err := tx.ExecContext(ctx, fmt.Sprintf(`
  drop table if exists %s.users;
  `, schemaName))
	return err
}
