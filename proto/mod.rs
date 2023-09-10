pub mod auth {
    include!(concat!(env!("OUT_DIR"), "/avelio.auth.rs"));
}

pub mod general {
    include!(concat!(env!("OUT_DIR"), "/avelio.general.rs"));
}
pub mod upload {
    include!(concat!(env!("OUT_DIR"), "/avelio.upload.rs"));
}

pub mod track {
    include!(concat!(env!("OUT_DIR"), "/avelio.track.rs"));
}

pub mod profile {
    include!(concat!(env!("OUT_DIR"), "/avelio.profile.rs"));
}

pub mod playlists {
    include!(concat!(env!("OUT_DIR"), "/avelio.playlists.rs"));
}
