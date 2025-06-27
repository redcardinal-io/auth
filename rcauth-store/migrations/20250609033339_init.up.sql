create table if not exists tenants (
    id uuid primary key default uuid_generate_v1mc(),
    name text not null,
    slug text not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);
select trigger_updated_at('tenants');
create unique index if not exists tenants_slug_idx on tenants (lower(slug));

create table if not exists organizations (
    id uuid primary key default uuid_generate_v1mc(),
    tenant_id uuid not null references tenants(id) on delete cascade,
    name text not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);
select trigger_updated_at('organizations');
create index if not exists organizations_tenant_id_idx on organizations (tenant_id);

create table if not exists users (
    id uuid primary key default uuid_generate_v1mc(),
    tenant_id uuid not null references tenants(id) on delete cascade,
    organization_id uuid null references organizations(id) on delete cascade,
    email text not null,
    encrypted_password text not null,
    role varchar(255) not null,
    email_confirmed_at timestamptz,
    confirmation_token varchar(255),
    confirmation_sent_at timestamptz,
    recovery_token varchar(255),
    recovery_sent_at timestamptz,
    last_sign_in_at timestamptz,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);
select trigger_updated_at('users');

create unique index if not exists users_tenant_id_email_idx on users (tenant_id, lower(email));
create index if not exists users_organization_id_idx on users (organization_id);

create table if not exists refresh_tokens (
    id bigserial primary key,
    tenant_id uuid not null references tenants(id) on delete cascade,
    session_id uuid not null,
    user_id uuid not null references users(id) on delete cascade,
    token varchar(255) not null unique,
    revoked boolean default false,
    parent varchar(255),
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);
select trigger_updated_at('refresh_tokens');
create index if not exists refresh_tokens_tenant_id_idx on refresh_tokens (tenant_id);
create index if not exists refresh_tokens_session_id_idx on refresh_tokens (session_id);
