-- Add down migration script here
drop function if exists set_updated_at();
drop function if exists trigger_updated_at(regclass);
drop collation if exists case_insensitive;
