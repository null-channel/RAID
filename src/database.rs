use crate::sysinfo::SystemInfo;
use rusqlite::{Connection, Result};
use std::path::Path;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        let db = Database { conn };
        db.init_tables()?;
        Ok(db)
    }

    fn init_tables(&self) -> Result<()> {
        // Drop the old table if it exists
        self.conn
            .execute("DROP TABLE IF EXISTS system_checks", [])?;

        self.conn.execute(
            "CREATE TABLE system_checks (
                id INTEGER PRIMARY KEY,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
                system_info_json TEXT NOT NULL,
                analysis TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn store_check(&self, system_info: &SystemInfo, analysis: &str) -> Result<()> {
        let system_info_json = serde_json::to_string(system_info)
            .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;

        self.conn.execute(
            "INSERT INTO system_checks (system_info_json, analysis) VALUES (?1, ?2)",
            [&system_info_json, analysis],
        )?;
        Ok(())
    }

    pub fn get_recent_checks(&self, limit: i64) -> Result<Vec<(i64, String, SystemInfo, String)>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, timestamp, system_info_json, analysis FROM system_checks ORDER BY timestamp DESC LIMIT ?"
        )?;
        let rows = stmt.query_map([limit], |row| {
            let system_info_json: String = row.get(2)?;
            let system_info: SystemInfo = serde_json::from_str(&system_info_json)
                .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;

            Ok((row.get(0)?, row.get(1)?, system_info, row.get(3)?))
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }
}
