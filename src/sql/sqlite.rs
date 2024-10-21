use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use rusqlite::Connection;
use crate::config::LANG;
use crate::sql::table::{matrix_events, matrix_messages};

lazy_static!(
    pub static ref DATABASE: Database = Database::new();
);

pub struct Database {
    connection: Arc<Mutex<Connection>>,
}

impl Database {
    fn new() -> Self {
        // 建立连接
        let connection = Connection::open("matrix-onebot.db3").expect(
            &LANG.read().unwrap().error_database_connection_failed
        );

        // 初始化表
        let create_failed_msg = &LANG.read().unwrap().error_database_table_init_failed;
        connection.execute(matrix_events::TABLE_CREATE_SQL, [])
            .expect(&create_failed_msg.replace("{table}", matrix_events::TABLE_NAME));
        connection.execute(matrix_messages::TABLE_CREATE_SQL, [])
            .expect(&create_failed_msg.replace("{table}", matrix_messages::TABLE_NAME));

        Self {
            connection: Arc::new(Mutex::new(connection)),
        }
    }
    pub fn get_matrix_events_table(&self) -> matrix_events::Table { matrix_events::Table::new(self.connection.clone()) }
    pub fn get_matrix_messages_table(&self) -> matrix_messages::Table { matrix_messages::Table::new(self.connection.clone()) }
}

