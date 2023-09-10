-- Add migration script here
create table if not exists users (
    id uuid primary key,
    login text not null,
    username text not null,
    password_hash text not null,
    blacklist_tokens text[] not null
)

CREATE INDEX users_search_idx 
   ON users USING gin (login gin_trgm_ops, username gin_trgm_ops);