-- This file was automatically created by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.




-- Sets up a trigger for the given table to automatically set a column called
-- `updated_at` whenever the row is modified (unless `updated_at` was included
-- in the modified columns)
--
-- # Example
--
-- ```sql
-- CREATE TABLE users (id SERIAL PRIMARY KEY, updated_at TIMESTAMP NOT NULL DEFAULT NOW());
--
-- SELECT diesel_manage_updated_at('users');
-- ```
CREATE OR REPLACE FUNCTION diesel_manage_updated_at(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION diesel_set_updated_at() RETURNS trigger AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION next_uuid(OUT result uuid) AS $$
DECLARE
    now_micros bigint;
    second_rand bigint;
    hex_value text;
    shard_id int:=1;
    version int:=7;
BEGIN
    -- Can use clock_timestamp() / statement_timestamp() / transaction_timestamp() / current_timestamp
    select (extract(epoch from current_timestamp)*1000000)::BIGINT INTO now_micros;
    select ((random() * 10^18)::BIGINT) INTO second_rand;
    -- Uncomment below line to ignore sharding.
    shard_id := now_micros%1000;
    -- select ((random() * 10^6)::INT) INTO shard_id;
    -- [milliseconds(6 bytes) + microseconds(12 bits) + shard(4 bits) + random(8 bytes)]
    -- hex_value := LPAD(TO_HEX(now_micros/1000), 12, '0')||LPAD(TO_HEX(now_micros%1000), 3, '0')||LPAD(TO_HEX(shard_id), 1, '0')||LPAD(TO_HEX(second_rand), 16, '0');

    -- UUID v7: [milliseconds(6 bytes) + version(4 bits) + microseconds/shard(12 bits)+ var(2 bits) + random(62 bits)]
    select (((random() * 10^18)::BIGINT) & x'3FFFFFFFFFFFFFFF'::BIGINT) |x'8000000000000000'::BIGINT INTO second_rand;
    hex_value := LPAD(TO_HEX(now_micros/1000), 12, '0')||LPAD(TO_HEX(version), 1, '0')||LPAD(TO_HEX(shard_id), 3, '0')||LPAD(TO_HEX(second_rand), 16, '0');
    
    result := CAST(hex_value AS UUID);
    -- TEST PERFOMANCE
    -- EXPLAIN ANALYZE
    -- SELECT next_uuid() FROM generate_series(1,100000);
END;
$$ LANGUAGE PLPGSQL;

CREATE OR REPLACE FUNCTION min_uuid(uuid, uuid)
    RETURNS uuid AS $$
    BEGIN
        -- if they're both null, return null
        IF $2 IS NULL AND $1 IS NULL THEN
            RETURN NULL ;
        END IF;

        -- if just 1 is null, return the other
        IF $2 IS NULL THEN
            RETURN $1;
        END IF ;
        IF $1 IS NULL THEN
            RETURN $2;
          END IF;

        -- neither are null, return the smaller one
        IF $1 > $2 THEN
            RETURN $2;
        END IF;

        RETURN $1;
    END;
    $$ LANGUAGE plpgsql;

create aggregate min(uuid) (
      sfunc = min_uuid,
      stype = uuid,
      combinefunc = min_uuid,
      parallel = safe,
      sortop = operator (<)
    );
	
	