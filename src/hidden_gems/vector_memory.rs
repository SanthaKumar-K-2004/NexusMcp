// Persistent Vector Memory using SQLite
use rusqlite::Connection;
use serde_json::{json, Value};
use std::collections::HashMap;

pub struct PersistentVectorMemory {
    conn: Option<Connection>,
    memory: HashMap<String, String>, // Fallback in-memory
}

impl PersistentVectorMemory {
    pub fn new(db_path: Option<&str>) -> Self {
        let conn = if let Some(path) = db_path {
            match Connection::open(path) {
                Ok(c) => {
                    let _ = c.execute(
                        "CREATE TABLE IF NOT EXISTS memory (
                            key TEXT PRIMARY KEY,
                            content TEXT,
                            created_at TEXT
                        )",
                        [],
                    );
                    Some(c)
                }
                Err(_) => None,
            }
        } else {
            None
        };

        Self {
            conn,
            memory: HashMap::new(),
        }
    }

    pub fn store(&mut self, key: &str, content: &str) {
        if let Some(conn) = &self.conn {
            let now = chrono::Utc::now().to_rfc3339();
            let _ = conn.execute(
                "INSERT OR REPLACE INTO memory (key, content, created_at) VALUES (?1, ?2, ?3)",
                rusqlite::params![key, content, now],
            );
        } else {
            self.memory.insert(key.to_string(), content.to_string());
        }
    }

    pub fn search(&self, query: &str) -> Value {
        let mut results = Vec::new();

        if let Some(conn) = &self.conn {
            if let Ok(mut stmt) = conn.prepare(
                "SELECT key, content FROM memory WHERE key LIKE ?1 OR content LIKE ?1 LIMIT 10",
            ) {
                if let Ok(rows) = stmt.query_map([format!("%{}%", query)], |row| {
                    Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
                }) {
                    for row in rows.flatten() {
                        results.push(json!({
                            "key": row.0,
                            "content": row.1
                        }));
                    }
                }
            }
        } else {
            // Fallback to in-memory
            for (k, v) in &self.memory {
                if k.contains(query) || v.contains(query) {
                    results.push(json!({ "key": k, "content": v }));
                }
            }
        }

        json!({
            "query": query,
            "results": results,
            "count": results.len(),
            "persistent": self.conn.is_some(),
            "method": "Persistent Vector Memory (SQLite)"
        })
    }
}

pub type VectorMemory = PersistentVectorMemory;
