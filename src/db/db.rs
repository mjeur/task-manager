use postgres::{Client, NoTls};
use std::env;

pub struct Database {
    client: Client,
}

impl Database {
    pub fn new(db_url: &str) -> Self {
        //let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let client = Client::connect(db_url, NoTls).expect("Failed to connect to database");
        Database { client }
    }

    pub fn get_client(&self) -> &Client {
        &self.client
    }
}