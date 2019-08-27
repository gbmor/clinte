use rusqlite;
use std::sync::mpsc;

#[derive(Debug)]
pub struct Post {
    id: u32,
    title: String,
    author: String,
    body: String,
}

pub struct Conn {
    db: rusqlite::Connection,
    tx: mpsc::Sender<Cmd>,
}

#[derive(Debug)]
pub enum Cmd {
    Create,
    Update,
    Disconnect,
    NOOP,
}

impl Conn {
    fn init() -> rusqlite::Connection {
        let conn = rusqlite::Connection::open_with_flags(
            "/tmp/db.sql",
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

        conn
    }

    pub fn new(tx: mpsc::Sender<Cmd>) -> Self {
        Conn {
            db: Conn::init(),
            tx,
        }
    }
}

impl Cmd {
    pub fn new(txt: &str) -> Self {
        match txt {
            "create" => Cmd::Create,
            "update" => Cmd::Update,
            "disconnect" => Cmd::Disconnect,
            _ => Cmd::NOOP,
        }
    }
}

impl Post {
    pub fn id(&self) -> String {
        format!("{}", self.id)
    }
    pub fn title(&self) -> String {
        self.title.clone()
    }
    pub fn author(&self) -> String {
        self.author.clone()
    }
    pub fn body(&self) -> String {
        self.body.clone()
    }
}
