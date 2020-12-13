extern crate rpassword;
use std::io;
use rustforce::{Client, Error};
use rustforce::response::{QueryResponse, ErrorResponse};
use serde::Deserialize;
use std::env;
use std::collections::HashMap;
// use tokio::stream::{Stream, StreamExt};
// use tokio::sync::{mpsc, Mutex};
// use tokio_util::codec::{Framed, LinesCodec, LinesCodecError};
// use mini_redis::{client, Result};
// use tokio::net::TcpListener;
extern crate tokio;
use tokio::prelude::*;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Account {
    #[serde(rename = "attributes")]
    attributes: Attribute,
    id: String,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Attribute {
    url: String,
    #[serde(rename = "type")]
    sobject_type: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut username = String::new();
    println!("Username: ");
    match io::stdin().read_line(&mut username) {
        Ok(_) => {
            println!("got username: {}", username);
        },
        Err(e) => println!("Oops! Something went wrong: {}", e)
    }

    let password = rpassword::prompt_password_stdout("Password: ").unwrap();
    println!("Your password is {}", password);
    
    let client_id = env::var("SFDC_CLIENT_ID").unwrap();
    let client_secret = env::var("SFDC_CLIENT_SECRET").unwrap();
    // let username = env::var("SFDC_USERNAME").unwrap();
    // let password = env::var("SFDC_PASSWORD").unwrap();

    let mut client = Client::new(client_id, client_secret);
    client.login_with_credential(username, password).await?;
    let mut params = HashMap::new();
    params.insert("Name", "hello rust");
    let res = client.create("Account", params).await?;
    println!("{:?}", res);
    Ok(())
}