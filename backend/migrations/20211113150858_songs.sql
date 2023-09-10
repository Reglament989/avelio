-- Add migration script here

CREATE EXTENSION IF NOT EXISTS "pg_trgm"; 
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

create table if not exists songs (
    id uuid primary key,
    title text not null,
    artist_name text not null,
    description text,
    header_image_thumbnail_url text not null,
    header_image_url text not null,
    genius_id text,
    recording_location text,
    release_date_for_display text not null,
    song_art_image_thumbnail_url text not null,
    album_cover_art_url text not null,
    album_name text not null,
    upload_date date not null default CURRENT_DATE
)