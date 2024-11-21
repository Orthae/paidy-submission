CREATE TABLE items
(
    id               UUID PRIMARY KEY,
    table_id         BIGINT NOT NULL,
    name             VARCHAR NOT NULL,
    preparation_time TIMESTAMPTZ NOT NULL
);

CREATE INDEX items_table_idx on items (table_id);