use rusqlite::{Connection, Result as SqlResult};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub proxy: Option<String>,
    pub stealth_level: String,
    pub created_at: String,
}

pub struct ProfileManager {
    conn: Connection,
}

impl ProfileManager {
    pub fn new(db_path: &str) -> SqlResult<Self> {
        let conn = Connection::open(db_path)?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS profiles (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                proxy TEXT,
                stealth_level TEXT NOT NULL,
                created_at TEXT NOT NULL
            )",
            [],
        )?;

        Ok(Self { conn })
    }

    pub fn create_profile(&self, name: &str, proxy: Option<&str>, stealth_level: &str) -> SqlResult<Profile> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        self.conn.execute(
            "INSERT INTO profiles (id, name, proxy, stealth_level, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![id, name, proxy, stealth_level, now],
        )?;

        Ok(Profile {
            id,
            name: name.to_string(),
            proxy: proxy.map(|s| s.to_string()),
            stealth_level: stealth_level.to_string(),
            created_at: now,
        })
    }

    pub fn get_profile(&self, id: &str) -> SqlResult<Option<Profile>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, proxy, stealth_level, created_at FROM profiles WHERE id = ?1"
        )?;

        let profile = stmt.query_row([id], |row| {
            Ok(Profile {
                id: row.get(0)?,
                name: row.get(1)?,
                proxy: row.get(2)?,
                stealth_level: row.get(3)?,
                created_at: row.get(4)?,
            })
        });

        match profile {
            Ok(p) => Ok(Some(p)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn list_profiles(&self) -> SqlResult<Vec<Profile>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, proxy, stealth_level, created_at FROM profiles ORDER BY created_at DESC"
        )?;

        let profiles = stmt.query_map([], |row| {
            Ok(Profile {
                id: row.get(0)?,
                name: row.get(1)?,
                proxy: row.get(2)?,
                stealth_level: row.get(3)?,
                created_at: row.get(4)?,
            })
        })?;

        profiles.collect()
    }
}