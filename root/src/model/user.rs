use serde::{Deserialize, Serialize};
use deadpool_postgres::{Client, PoolError};

#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
}
