#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
	  create user redcardinal_admin login createrole createdb replication bypassrls;

    -- redcardinal super admin
    create user redcardinal_auth_admin noinherit createrole login noreplication password 'root';
    create schema if not exists $DB_NAMESPACE authorization redcardinal_auth_admin;
    grant create on database postgres to redcardinal_auth_admin;
    alter user redcardinal_auth_admin set search_path = '$DB_NAMESPACE';
EOSQL
