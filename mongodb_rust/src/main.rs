use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::time::error;
use mongodb::{Client, options:: ClientOptions, Collection};

type Result<T> = std::result::Result<T, error::Error>;

mod user_db;
mod user_handler;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub name: String,
}

#[tokio::main]
async fn main() -> Result<()> {

    Ok(())
}