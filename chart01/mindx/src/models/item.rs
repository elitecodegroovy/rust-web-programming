use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewItem {
    pub name: String,
    pub description: String,
}
