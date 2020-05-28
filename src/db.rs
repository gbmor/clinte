use fd_lock::FdLock;
use serde::{Deserialize, Serialize};

use std::fs;
use std::fs::File;

use crate::conf;
use crate::error;

#[cfg(test)]
pub const PATH: &str = "clinte.json";

#[cfg(not(test))]
pub const PATH: &str = "/usr/local/clinte/clinte.json";

#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    pub title: String,
    pub author: String,
    pub body: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Posts {
    pub posts: Vec<Post>,
}

#[derive(Debug)]
pub struct Conn {
    pub conn: FdLock<std::fs::File>,
}

impl Conn {
    pub fn init(path: &str) -> Self {
        if *conf::DEBUG {
            log::info!("Opening clinte.json");
        }

        let file = error::helper(File::open(path), "Couldn't open clinte.json");

        Self {
            conn: FdLock::new(file),
        }
    }
}

impl Posts {
    pub fn get_all(path: &str) -> Self {
        if *conf::DEBUG {
            log::info!("Retrieving posts...");
        }

        let mut db = Conn::init(path);
        let _guard = error::helper(db.conn.try_lock(), "Couldn't acquire lock on clinte.json");
        let strdata = error::helper(fs::read_to_string(PATH), "Couldn't read clinte.json");
        let out: Self = error::helper(serde_json::from_str(&strdata), "Couldn't parse clinte.json");

        out
    }

    pub fn replace(&mut self, n: usize, post: Post) {
        self.posts[n] = post;
    }

    pub fn get(&self, n: usize) -> &Post {
        &self.posts[n]
    }

    pub fn append(&mut self, post: Post) {
        self.posts.push(post);
    }

    pub fn delete(&mut self, n: usize) {
        self.posts.remove(n);
    }

    pub fn write(&self) {
        let strdata = error::helper(
            serde_json::to_string_pretty(&self),
            "Couldn't serialize posts",
        );

        let mut db_fd = Conn::init(PATH);
        let _guard = error::helper(
            db_fd.conn.try_lock(),
            "Couldn't acquire lock on clinte.json",
        );
        error::helper(
            fs::write(PATH, &strdata),
            "Couldn't write data to clinte.json",
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let mut conn = Conn::init(PATH);
        conn.conn.try_lock().unwrap();
    }

    #[test]
    fn retrieve_posts_and_examine() {
        let all = Posts::get_all(PATH);
        assert_eq!(all.posts.len(), 1);

        let post = all.get(0);
        assert_eq!(post.title, "Welcome to CLI NoTEs!");
        assert_eq!(post.author, "clinte!");
        assert_eq!(post.body, "Welcome to clinte! For usage, run 'clinte -h'");
    }
}
