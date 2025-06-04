use std::net::{self, SocketAddr};
use chrono::{DateTime, Utc}; // For the timestamp field in postgres
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow; // map rows to structs


#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkStatus{
    pub network_id: String,
    pub network_address: SocketAddr,
}

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct PostgresStatus {
    pub id: i32, // Assuming SERIAL primary key
    pub event_time: DateTime<Utc>,
    pub agent_name: String,
    pub status_ok: bool,
    pub object_data: Option<JsonValue>,
}

pub fn json_network_return(network_status: &NetworkStatus) -> Result<JsonValue, serde_json::Error> {

    serde_json::to_value(network_status)
}

pub async fn insert_status_entry(pool: &sqlx::pgPool, status_entry: &PostgresStatus) -> Result<(), sqlx::Error> {
    let inserted_record = sqlx::query!(
        r#"
        INSERT INTO status_log_table (event_time, agent_name, status_ok, object_data)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
        status_entry.event_time,
        status_entry.agent_name,
        status_entry.status_ok,
        status_entry.object_data // This will be serialized to JSON/JSONB by sqlx
    )
    .fetch_one(pool) // Fetches the single row returned by RETURNING id
    .await?;

    Ok(inserted_record.id)
}