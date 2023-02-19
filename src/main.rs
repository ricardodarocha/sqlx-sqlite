use sqlx::{FromRow, Connection};
use serde::{Serialize, Deserialize};

//Model todo! Move to models.rs
#[derive(Serialize, Deserialize, Debug, FromRow)]
struct User { 
    id: i64,
    name: String,
    age: i64,
 }

//database todo! Move to database.rs

use sqlx::SqliteConnection;

//routes todo! Move to routes.rs
#[macro_use]
extern crate dotenv_codegen;

async fn get_all() -> Vec<User> {
    let mut connection: SqliteConnection = SqliteConnection::connect(dotenv!("DATABASE_URL")).await.unwrap();
    let result = sqlx::query_as!(User, "SELECT * FROM users")
    .fetch_all(&mut connection).await;
    result.unwrap()
}

#[async_std::main]
async fn main()  {
    for user in get_all().await {
        print!("{:?}",user)
    }

}