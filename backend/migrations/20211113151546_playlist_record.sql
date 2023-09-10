-- Add migration script here
create table if not exists playlist_record (
    song_id uuid,
    playlist_id uuid,
    foreign key (playlist_id) references playlist (id) on delete restrict,
    foreign key (song_id) references songs (id) on delete restrict,
    primary key (song_id, playlist_id)
)