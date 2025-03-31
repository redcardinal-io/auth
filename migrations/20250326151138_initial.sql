-- +goose up
create table if not exists auth.users (
	instance_id uuid null,
	id uuid not null unique,
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
	created_at timestamptz null,
	updated_at timestamptz null,
	constraint users_pkey primary key (id)
);
create index if not exists users_instance_id_email_idx on auth.users using btree (instance_id, email);
create index if not exists users_instance_id_idx on auth.users using btree (instance_id);
comment on table auth.users is 'Auth: Stores user login data within a secure schema.';
-- auth.refresh_tokens definition
create table if not exists auth.refresh_tokens (
	instance_id uuid null,
	id bigserial not null,
	"token" varchar(255) null,
	user_id varchar(255) null,
	revoked bool null,
	created_at timestamptz null,
	updated_at timestamptz null,
	constraint refresh_tokens_pkey primary key (id)
);
create index if not exists refresh_tokens_instance_id_idx on auth.refresh_tokens using btree (instance_id);
create index if not exists refresh_tokens_instance_id_user_id_idx on auth.refresh_tokens using btree (instance_id, user_id);
create index if not exists refresh_tokens_token_idx on auth.refresh_tokens using btree (token);
comment on table auth.refresh_tokens is 'Auth: Store of tokens used to refresh JWT tokens once they expire.';
-- auth.instances definition
create table if not exists auth.instances (
	id uuid not null,
	uuid uuid null,
	raw_base_config text null,
	created_at timestamptz null,
	updated_at timestamptz null,
	constraint instances_pkey primary key (id)
);
comment on table auth.instances is 'Auth: Manages users across multiple sites.';
-- auth.audit_log_entries definition
create table if not exists auth.audit_log_entries (
	instance_id uuid null,
	id uuid not null,
	payload json null,
	created_at timestamptz null,
	constraint audit_log_entries_pkey primary key (id)
);
create index if not exists audit_logs_instance_id_idx on auth.audit_log_entries using btree (instance_id);
comment on table auth.audit_log_entries is 'Auth: Audit trail for user actions.';
-- auth.schema_migrations definition
create table if not exists auth.schema_migrations (
	"version" varchar(255) not null,
	constraint schema_migrations_pkey primary key ("version")
);
comment on table auth.schema_migrations is 'Auth: Manages updates to the auth system.';
		
-- Gets the User ID from the request cookie
create or replace function auth.uid() returns uuid as $$ select nullif(current_setting('request.jwt.claim.sub', true), '')::uuid; $$ language sql stable;

-- Gets the User role from the request cookie
create or replace function auth.role() returns text as $$ select nullif(current_setting('request.jwt.claim.role', true), '')::text; $$ language sql stable;

-- +goose down
drop table if exists auth.users;
drop table if exists auth.refresh_tokens;
drop table if exists auth.instances;
drop table if exists auth.audit_log_entries;
drop table if exists auth.schema_migrations;
drop function if exists auth.uid();
drop function if exists auth.role();
