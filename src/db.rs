use log::info;
use rand;
use rusqlite;
use std::sync::mpsc;
use std::time;

const DB_PATH: &str = "/tmp/clinte.db";

#[derive(Debug)]
pub struct Post {
    id: u32,
    title: String,
    author: String,
    body: String,
}

#[derive(Debug)]
pub struct Conn {
    db: rusqlite::Connection,
    rx: mpsc::Receiver<Cmd>,
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
            id INTEGER PRIMARY KEY NOT NULL,
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

    pub fn new(rx: mpsc::Receiver<Cmd>) -> Self {
        Conn {
            db: Conn::init(),
            rx,
        }
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
        let id = rand::random::<u32>();
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
