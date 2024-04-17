use mongodb::{
    bson::{doc, Document}, options::{ClientOptions, ResolverConfig}, Client, Database
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    sender: User,
    reciever: User,
    message: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mongodb_uri = "mongodb://localhost:27017/";
    let db_name = "rust_tut";

    let runtime = tokio::runtime::Runtime::new().unwrap();
    let db = runtime.block_on(make_conn(mongodb_uri, db_name));

    // runtime.block_on(add_user(
    //     db,
    //     String::from("Pratiksha Patil"),
    //     String::from("pratupatil"),
    // ));

    // let u1 = runtime.block_on(find_user(db.clone(), "rohitmokashi".to_string()));
    // let u2 = runtime.block_on(find_user(db.clone(), "pgaikwad".to_string()));
    // let msg = String::from("kasa challay production");

    // runtime.block_on(add_msg(db, u1, u2, msg));

    runtime.block_on(find_user(db, "rohitmokashi".to_string()));

    Ok(())
}

async fn make_conn(mongodb_uri: &str, db_name: &str) -> Database {
    let options =
        ClientOptions::parse_with_resolver_config(mongodb_uri, ResolverConfig::cloudflare())
            .await
            .unwrap();
    let client = Client::with_options(options).unwrap();

    client.database(db_name)
}

async fn add_user(db: Database, name: String, username: String) {
    let coll = db.collection::<User>("users");
    
    let u = coll.find_one(doc! {"username": username.clone()}, None).await.unwrap();
    let u1: User = User { username, name };
    // println!("{:?}", u);
    
    let user_exists = match u {
        Some(_user) => true,
        None => false
    };
    
    match user_exists {
        true => {
            println!("User already exists");
        },
        false => {
            coll.insert_one(u1, None).await.expect("msg");
            println!("User added successfully");
        }
    }
}

async fn find_user(db: Database, username: String) {
    let coll = db.collection::<Document>("users");
    let a = coll.find_one(doc! {"username": username.clone()}, None).await.unwrap();
    
    match a {
        Some(d) => println!("{:?}", d),
        None => println!("User does not exist"),
    }
}

async fn add_msg(db: Database, sender: User, reciever: User, message: String) {
    let coll = db.collection::<Message>("messages");

    let m: Message = Message {
        sender,
        reciever,
        message,
    };
    coll.insert_one(m, None).await.expect("msg");
}
