-- Add up migration script here
create extension if not exists "uuid-ossp";

create or replace function set_updated_at()
    returns trigger as
$$
begin
    NEW.updated_at = now();
    return NEW;
end;
$$ language plpgsql;

create or replace function trigger_updated_at(tablename regclass)
    returns void as
$$
begin
    execute format('create trigger set_updated_at
        before update
        on %s
        for each row
        when (old is distinct from new)
    execute function set_updated_at();', tablename);
end;
$$ language plpgsql;

create collation if not exists case_insensitive (provider = icu, locale = 'und-u-ks-level2', deterministic = false);
