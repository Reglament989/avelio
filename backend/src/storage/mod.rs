use minio_rs::minio::{Client, Credentials};

use crate::settings::Settings;

#[derive(Clone)]
pub struct S3Storage {
    pub client: Client,
}

impl S3Storage {
    pub fn new(config: Settings) -> Self {
        let mut c = Client::new(&config.s3.endpoint).unwrap();
        c.set_credentials(Credentials::new(
            &config.s3.access_key,
            &config.s3.secret_key,
        ));
        S3Storage { client: c }
    }
}
