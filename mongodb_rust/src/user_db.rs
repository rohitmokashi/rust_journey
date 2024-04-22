use crate::{error::Error, Result, User};
use mongodb::{
    options::{ClientOptions},
    Client, Collection,
};

pub struct DB {
    pub client: Client,
}

impl DB {
    pub async fn init() -> Result<Self> {
        let mut client_options = ClientOptions::parse("mongodb://127.0.0.1:27017").await.unwrap();
        client_options.app_name = Some("baatein".to_string());

        Ok(Self {
            client: Client::with_options(client_options).unwrap(),
        })
    }

    pub async fn create_user() {}
}
