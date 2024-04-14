use mysql::prelude::*;
use mysql::*;
use std::io;
// use std::vec::Vec;
// use std::option::Option;

#[derive(Debug, PartialEq, Eq)]
struct User {
    username: String,
    password: String,
}

fn main() {
    let url = "mysql://root:toor@localhost:3306/rust_tut";
    let pool: Pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let _res = conn.query_drop(
        "CREATE TABLE users (
            username varchar(20) primary key not null,
            password varchar(30) not null
        )",
    );

    let mut choice = String::new();
    println!(
        "Enter your choice\n r: Register\n l: Login"
    );
    io::stdin().read_line(&mut choice).expect("enter valid choice");
    let choice = choice.trim();

    if choice == "r" {
        signup(conn);
    }
    else if choice == "l" {
        signin(conn);
    }

}

fn signin(mut conn: PooledConn) {
    let mut username = String::new();
    println!("Enter username: ");
    io::stdin()
        .read_line(&mut username)
        .expect("enter valid username");
    let username: &str = username.trim();

    let user: Option<(String, String)> = conn
        .query_first(format!(
            "SELECT username, password FROM users WHERE username=\"{username}\""
        ))
        .expect("msg");

    // let mut u: (String, String) = (String::new(), String::new());
    let u: (String, String) = match user {
        Some(user) => user,
        None => (String::new(), String::new()),
    };

    let mut password = String::new();
    println!("Enter password: ");
    io::stdin()
        .read_line(&mut password)
        .expect("enter valid password");
    let password: &str = password.trim();

    if password == u.1 {
        println!("Login successful");
    } else {
        println!("Login failed");
    }
}

fn signup(mut conn: PooledConn) {
    let mut username = String::new();
    println!("Enter username: ");
    io::stdin()
        .read_line(&mut username)
        .expect("enter valid username");
    let username = username.trim();

    let user: Option<String> = conn
        .query_first(format!(
            "SELECT username FROM users WHERE username=\"{username}\""
        ))
        .expect("msg");

    if user == None {
        let mut password = String::new();
        println!("Enter password: ");
        io::stdin()
            .read_line(&mut password)
            .expect("enter valid password");

        conn.exec_drop(
            r"INSERT INTO users VALUES (?, ?)",
            (username.trim(), password.trim()),
        )
        .expect("error adding user");
    } else {
        println!("username taken!");
    }
}
