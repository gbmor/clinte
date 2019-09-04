use crate::db;
use rusqlite;
use std::error::Error;
use users;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn new(stmt: &mut rusqlite::Statement, title: &str, body: &str) -> Result<()> {
    let user = users::get_current_username()
        .unwrap()
        .into_string()
        .unwrap();

    stmt.execute_named(&[(":title", &title), (":author", &user), (":body", &body)])?;
    Ok(())
}

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

pub fn exec_stmt_no_params(stmt: &mut rusqlite::Statement) -> Result<()> {
    stmt.execute(rusqlite::NO_PARAMS)?;

    Ok(())
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

        new(&mut stmt, &title, "TEST BODY").unwrap();
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