use std::sync::{Arc, Mutex};
use rusqlite::{params, Connection};
use crate::sql::table::{TableAssociatedType, TableCommonOpera, TablePrivateCommonOpera};

// model
pub struct Event {
    pub event_id: String,
    pub ty: String,
    pub timestamp: i64,
}

// table
pub const TABLE_NAME: &'static str = "matrix_events";
pub const TABLE_CREATE_SQL: &'static str =
    "CREATE TABLE IF NOT EXISTS matrix_events (
        event_id    TEXT PRIMARY KEY,
        type        TEXT,
        timestamp   BIGINT NOT NULL
        )";

#[derive(Clone)]
pub struct Table {
    connection: Arc<Mutex<Connection>>
}

impl TableAssociatedType for Table { type Model = Event; }

// todo: 实现 #[derive(TablePrivateCommonOpera)] 宏以避免重复代码
impl TablePrivateCommonOpera for Table {
    fn get_table_name() -> &'static str {
        TABLE_NAME
    }
    fn get_connection(&self) -> Arc<Mutex<Connection>> { Arc::clone(&self.connection) }
}

impl TableCommonOpera for Table {
    fn insert_or_update(&self, model: Self::Model) -> rusqlite::Result<()> {
        let conn = self.connection.lock().unwrap();
        let query = format!(
            "INSERT OR REPLACE INTO {} (event_id, type, timestamp) VALUES (?1, ?2, ?3)", TABLE_NAME
        );
        conn.execute(
            &query,
            params![model.event_id, model.ty, model.timestamp]
        )?;
        Ok(())
    }

    fn query(&self, event_id: &str) -> rusqlite::Result<Option<Self::Model>> {
        // // todo: 疑似可以优化为一次只 SELECT 一项
        // let mut stmt = self.connection.prepare(&format!("SELECT data FROM {}", EVENT_TABLE_NAME))?;
        // let rows = stmt.query_map([], |row| {
        //     Ok((
        //         row.get::<_, String>(0)?,
        //         row.get::<_, Vec<u8>>(1)?
        //     ))
        // })?;
        // rows.filter_map(|row| row.ok())
        //     .find_map(|data| {
        //         if data.0.eq(event_id) {
        //             Some(Ok(data.1))
        //         } else { None }
        //     })
        //     .unwrap_or_else(|| Err(rusqlite::Error::QueryReturnedNoRows))
        let conn = self.connection.lock().unwrap();
        let query = format!(
            "SELECT type, timestamp FROM {} WHERE event_id = ?1", TABLE_NAME
        );
        let data = conn.query_row(
            &query,
            [event_id],
            |row| Ok(Some(Event {
                event_id: event_id.to_owned(),
                ty: row.get(0)?,
                timestamp: row.get(1)?,
            }))
        );
        Self::tool_handle_query_row_result(data)
    }
}

impl Table {
    pub fn new(connection: Arc<Mutex<Connection>>) -> Self {
        Self { connection }
    }
}