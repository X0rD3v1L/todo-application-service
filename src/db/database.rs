use std::env;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

#[derive(Clone)]
pub struct Database {
    pub client: Surreal<Client>,
    pub name_space: String,
    pub db_name: String,
}

impl Database {
    // Initialize a new database connection
    pub async fn init() -> Result<Self, Error> {
        dotenv::dotenv().ok();
        // Retrieve database credentials from environment variables
        let username = env::var("DB_USERNAME").expect("DB_USERNAME not set in environment");
        let password = env::var("DB_PASSWORD").expect("DB_PASSWORD not set in environment");

        // Connect to the Surreal database engine via WebSocket
        let client = Surreal::new::<Ws>("127.0.0.1:8000").await?;

        // Sign in to the Surreal database with root credentials
        client
            .signin(Root {
                username: &username,
                password: &password,
            })
            .await?;
        client.use_ns("surreal").use_db("todoapp").await.unwrap();

        // Returns a new instance of the Database struct
        Ok(Database {
            client,
            name_space: String::from("surreal"),
            db_name: String::from("todoapp"),
        })
    }
}
