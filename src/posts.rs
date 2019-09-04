use std::error::Error;
use std::io;

use rusqlite;
use users;

use crate::db;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

// Executes the sql statement that inserts a new post
// Broken off for unit testing.
pub fn exec_new(stmt: &mut rusqlite::Statement, title: &str, body: &str) -> Result<()> {
    let user = users::get_current_username()
        .unwrap()
        .into_string()
        .unwrap();

    stmt.execute_named(&[(":title", &title), (":author", &user), (":body", &body)])?;
    Ok(())
}

// Make sure nobody encodes narsty characters
// into a message to negatively affect other
// users
fn str_to_utf8(str: &str) -> String {
    str.chars()
        .map(|c| {
            let mut buf = [0; 4];
            c.encode_utf8(&mut buf).to_string()
        })
        .collect::<String>()
}

// First handler for creating a new post.
pub fn create(db: &db::Conn) {
    let mut stmt = db
        .conn
        .prepare("INSERT INTO posts (title, author, body) VALUES (:title, :author, :body)")
        .unwrap();

    println!();
    println!("Title of the new post: ");
    let mut title = String::new();
    io::stdin().read_line(&mut title).unwrap();
    let title = str_to_utf8(title.trim());
    let title = if title.len() > 30 {
        &title[..30]
    } else {
        &title
    };

    println!();
    println!("Body of the new post: ");
    let mut body = String::new();
    io::stdin().read_line(&mut body).unwrap();
    let body = str_to_utf8(body.trim());
    let body = if body.len() > 500 {
        &body[..500]
    } else {
        &body
    };

    exec_new(&mut stmt, title, body).unwrap();

    println!();
}

// Shows the most recent posts.
pub fn display(db: &db::Conn) {
    let mut stmt = db.conn.prepare("SELECT * FROM posts").unwrap();
    let out = stmt
        .query_map(rusqlite::NO_PARAMS, |row| {
            let id: u32 = row.get(0)?;
            let title: String = row.get(1)?;
            let author: String = row.get(2)?;
            let body: String = row.get(3)?;
            Ok(db::Post {
                id,
                title,
                author,
                body,
            })
        })
        .unwrap();

    let mut postvec = Vec::new();
    out.for_each(|row| {
        if let Ok(post) = row {
            postvec.push(format!(
                "{}. {} -> by {}\n{}\n\n",
                post.id, post.title, post.author, post.body
            ));
        }
    });

    for (i, e) in postvec.iter().enumerate() {
        if (postvec.len() > 14 && i >= postvec.len() - 15) || postvec.len() < 15 {
            print!("{}", e);
        }
    }
}

// First handler to update posts.
pub fn update_handler(db: &db::Conn, id: u32) {
    let cur_user = users::get_current_username()
        .unwrap()
        .into_string()
        .unwrap();

    let id_num_in = if id == 0 {
        println!();
        println!("ID number of your post to edit?");
        let mut id_num_in = String::new();
        io::stdin().read_line(&mut id_num_in).unwrap();
        id_num_in.trim().parse().unwrap()
    } else {
        id
    };

    let mut get_stmt = db
        .conn
        .prepare("SELECT * FROM posts WHERE id = :id")
        .unwrap();

    let row = get_stmt
        .query_row_named(&[(":id", &id_num_in)], |row| {
            let title: String = row.get(1).unwrap();
            let author = row.get(2).unwrap();
            let body = row.get(3).unwrap();
            Ok(vec![title, author, body])
        })
        .unwrap();

    if cur_user != row[1] {
        println!();
        println!("Username mismatch - can't update_handler post!");
        return;
    }

    let mut new_title = String::new();
    let mut new_body = String::new();

    println!("Updating post {}", id_num_in);
    println!();
    println!("Title: {}\n\nBody: {}", row[0], row[2]);
    println!();
    println!("Enter new title:");
    io::stdin().read_line(&mut new_title).unwrap();
    println!();
    println!("Enter new body:");
    io::stdin().read_line(&mut new_body).unwrap();
    println!();

    update(&new_title, &new_body, id_num_in, &db).unwrap();
}

// Allows editing of posts - called by main::update
pub fn update(new_title: &str, new_body: &str, id_num_in: u32, db: &db::Conn) -> Result<()> {
    let new_title = new_title.trim();
    let new_body = new_body.trim();

    let title_stmt = format!("UPDATE posts SET title = :title WHERE id = {}", id_num_in);
    let mut stmt = db.conn.prepare(&title_stmt)?;
    stmt.execute_named(&[(":title", &new_title)])?;
    let body_stmt = format!("UPDATE posts SET body = :body WHERE id = {}", id_num_in);
    let mut stmt = db.conn.prepare(&body_stmt)?;

    stmt.execute_named(&[(":body", &new_body)])?;

    Ok(())
}

// Helper to just run a sql statement.
pub fn exec_stmt_no_params(stmt: &mut rusqlite::Statement) -> Result<()> {
    stmt.execute(rusqlite::NO_PARAMS)?;

    Ok(())
}

// First handler to remove a post
pub fn delete_handler(db: &db::Conn, id: u32) {
    let cur_user = users::get_current_username()
        .unwrap()
        .into_string()
        .unwrap();

    let id_num_in: u32 = if id == 0 {
        println!();
        println!("ID of the post to delete?");
        let mut id_num_in = String::new();
        io::stdin().read_line(&mut id_num_in).unwrap();
        id_num_in.trim().parse().unwrap()
    } else {
        id
    };

    let del_stmt = format!("DELETE FROM posts WHERE id = {}", id_num_in);
    let get_stmt = format!("SELECT * FROM posts WHERE id = {}", id_num_in);

    let mut get_stmt = db.conn.prepare(&get_stmt).unwrap();
    let mut del_stmt = db.conn.prepare(&del_stmt).unwrap();

    let user_in_post: String = get_stmt
        .query_row(rusqlite::NO_PARAMS, |row| row.get(2))
        .unwrap();

    if cur_user != user_in_post {
        println!();
        println!("Users don't match. Can't delete!");
        println!();
        return;
    }

    exec_stmt_no_params(&mut del_stmt).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn post_new() {
        let db = db::Conn::init("/tmp/clinte.db");
        let db = db::Conn { conn: db };
        let mut stmt = db
            .conn
            .prepare("INSERT INTO posts (title, author, body) VALUES (:title, :author, :body)")
            .unwrap();

        let title = String::from("TEST TITLE");

        exec_new(&mut stmt, &title, "TEST BODY").unwrap();
        update("NEW TITLE", "TEST BODY", 1, &db).unwrap();

        let mut stmt = db
            .conn
            .prepare("SELECT * FROM posts WHERE title = :title")
            .unwrap();

        let title = String::from("NEW TITLE");
        let out: String = stmt
            .query_row_named(&[(":title", &title)], |row| row.get::<usize, String>(1))
            .unwrap();

        assert_eq!("NEW TITLE", &out);
    }
}
