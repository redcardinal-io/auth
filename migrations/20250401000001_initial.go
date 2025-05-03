package migrations

import (
	"context"
	"database/sql"
	"fmt"
	"os"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upInitial, downInitial)
}

func upInitial(ctx context.Context, tx *sql.Tx) error {
	schemaName := os.Getenv("RCAUTH_SCHEMA_NAME")
	_, err := tx.ExecContext(ctx, fmt.Sprintf(`
		create or replace function %s.goose_manage_updated_at(_tbl regclass) returns void as $$
		begin
    		execute format('create trigger set_updated_at before update on %s
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
		"%s",
		schemaName,
		schemaName,
		schemaName))

	return err
}

func downInitial(ctx context.Context, tx *sql.Tx) error {
	schemaName := os.Getenv("RCAUTH_SCHEMA_NAME")
	_, err := tx.ExecContext(ctx, `
		do $$
		declare
			trigger_rec record;
		begin
			for trigger_rec in 
				select tgname, relname 
				from pg_trigger t 
				join pg_proc p on t.tgfoid = p.oid 
				join pg_class c on t.tgrelid = c.oid
				where p.proname = 'goose_set_updated_at'
			loop
				execute format('drop trigger if exists %i on %i', trigger_rec.tgname, trigger_rec.relname);
			end loop;
		end $$;`+fmt.Sprintf(
		`drop function if exists %s.goose_manage_updated_at(_tbl regclass);
		drop function if exists %s.goose_set_updated_at();
		drop collation if exists case_insensitive;
		drop extension if exists "uuid-ossp";
	`, schemaName,
		schemaName))

	return err
}
