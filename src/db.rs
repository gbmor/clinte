use log::info;
use rusqlite;
use std::time;

const DB_PATH: &str = "/tmp/clinte.db";

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

#[derive(Debug)]
pub enum Cmd {
    Create(Post),
    Update(Post),
    Disconnect,
    NOOP,
}

impl Conn {
    fn init() -> rusqlite::Connection {
        let start = time::Instant::now();
        info!("Connecting to database");
        let conn = rusqlite::Connection::open_with_flags(
            DB_PATH,
            rusqlite::OpenFlags::SQLITE_OPEN_FULL_MUTEX
                | rusqlite::OpenFlags::SQLITE_OPEN_CREATE
                | rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE,
        )
        .expect("Could not connect to DB");

        conn.execute(
            "CREATE TABLE IF NOT EXISTS posts (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            title TEXT NOT NULL,
            author TEXT NOT NULL,
            body TEXT NOT NULL
        )",
            rusqlite::NO_PARAMS,
        )
        .expect("Could not initialize DB");

        info!(
            "Database connection established in {}ms",
            start.elapsed().as_millis()
        );

        conn
    }

    pub fn new() -> Self {
        Conn { conn: Conn::init() }
    }
}

impl Cmd {
    pub fn new(txt: &str, post: Post) -> Self {
        match txt {
            "create" => Cmd::Create(post),
            "update" => Cmd::Update(post),
            "disconnect" => Cmd::Disconnect,
            _ => Cmd::NOOP,
        }
    }
}

impl Post {
    pub fn new(title: &str, author: &str, body: &str) -> Self {
        let id = 0;
        let title = title.to_string();
        let author = author.to_string();
        let body = body.to_string();
        Post {
            id,
            title,
            author,
            body,
        }
    }
}
