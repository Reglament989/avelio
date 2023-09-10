-- Add migration script here
create table if not exists playlist (
    id uuid primary key,
    title text not null,
    total_playback_display text not null,
    playlist_owner uuid not null,
    foreign key (playlist_owner) references users (id) on delete cascade
)