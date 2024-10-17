use std::sync::{Arc, Mutex};
use rusqlite::Connection;

pub mod matrix_events;
pub mod matrix_messages;

pub trait TableAssociatedType {
    type Model;
}

trait TablePrivateCommonOpera: TableAssociatedType {
    fn get_table_name() -> &'static str;
    fn get_connection(&self) -> Arc<Mutex<Connection>>;
    fn tool_handle_query_row_result(
        data: rusqlite::Result<Option<Self::Model>>
    ) -> rusqlite::Result<Option<Self::Model>> {
        if let Err(e) = data {
            match e {
                rusqlite::Error::QueryReturnedNoRows => Ok(None),
                _ => { Err(e) }
            }
        } else { data }
    }
}

pub trait TableCommonOpera: TablePrivateCommonOpera {
    fn insert_or_update(&self, model: Self::Model) -> rusqlite::Result<()>;
    fn query(&self, event_id: &str) -> rusqlite::Result<Option<Self::Model>>;
    fn delete(&self, event_id: &str) -> rusqlite::Result<Option<Self::Model>> {
        let event =
            if let Some(e) = self.query(event_id)? { e }
            else {  return Ok(None); };

        let coon_lock = self.get_connection();
        let conn = coon_lock.lock().unwrap();
        let query = format!(
            "DELETE FROM {} WHERE event_id = ?1", Self::get_table_name()
        );
        conn.execute(
            &query,
            [event_id]
        )?;

        Ok(Some(event))
    }
}