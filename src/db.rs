use std::time;

use crate::conf;
use crate::error;

const DB_PATH: &str = "/usr/local/clinte/clinte.db";

#[derive(Debug)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub body: String,
}

#[derive(Debug)]
pub struct Conn {
    pub conn: rusqlite::Connection,
}

impl Conn {
    pub fn init(path: &str) -> rusqlite::Connection {
        let start = time::Instant::now();

        if *conf::DEBUG {
            log::info!("Connecting to database");
        }

        let conn = error::helper(
            rusqlite::Connection::open_with_flags(
                path,
                rusqlite::OpenFlags::SQLITE_OPEN_FULL_MUTEX
                    | rusqlite::OpenFlags::SQLITE_OPEN_CREATE
                    | rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE,
            ),
            "Could not connect to DB",
        );

        error::helper(
            conn.execute(
                "CREATE TABLE IF NOT EXISTS posts (
            id INTEGER PRIMARY KEY NOT NULL,
            title TEXT NOT NULL,
            author TEXT NOT NULL,
            body TEXT NOT NULL
        )",
                rusqlite::NO_PARAMS,
            ),
            "Could not initialize DB",
        );

        if *conf::DEBUG {
            log::info!(
                "Database connection established in {}ms",
                start.elapsed().as_millis()
            );
        }

        conn
    }

    pub fn new() -> Self {
        Conn {
            conn: Conn::init(DB_PATH),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let conn = Conn::init(":memory:");
        let mut stmt = conn.prepare("SELECT * FROM POSTS").unwrap();

        stmt.query_map(rusqlite::NO_PARAMS, |_| Ok(())).unwrap();
    }
}
