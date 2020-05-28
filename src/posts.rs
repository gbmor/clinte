use std::io;

use crate::db;
use crate::error;
use crate::user;

#[cfg(not(test))]
use crate::ed;

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
pub fn create() -> error::Result<()> {
    println!();
    println!("Title of the new post: ");

    #[cfg(test)]
    let title = String::from("TEST_TITLE");

    #[cfg(not(test))]
    let mut title = String::new();

    #[cfg(not(test))]
    io::stdin().read_line(&mut title)?;

    let title = str_to_utf8(title.trim());
    let title = if title.len() > 30 {
        &title[..30]
    } else {
        &title
    };

    println!();

    #[cfg(not(test))]
    let body_raw = str_to_utf8(&ed::call(""));

    #[cfg(not(test))]
    let body = if body_raw.len() > 500 {
        &body_raw[..500]
    } else {
        &body_raw
    };

    #[cfg(test)]
    let body = String::from("TEST_BODY");

    let trimmed_body = body.trim();
    let user = &*user::NAME;

    let mut all = db::Posts::get_all(db::PATH);
    let new = db::Post {
        author: user.into(),
        title: title.to_string(),
        body: trimmed_body.to_string(),
    };

    all.append(new);
    all.write();

    println!();
    Ok(())
}

// Shows the most recent posts.
pub fn display() -> error::Result<()> {
    let all = db::Posts::get_all(db::PATH);

    let mut postvec = Vec::new();
    all.posts().iter().enumerate().for_each(|(id, post)| {
        let newpost = format!(
            "{}. {} -> by {}\n{}\n\n",
            id + 1,
            post.title.trim(),
            post.author,
            post.body.trim()
        );
        postvec.push(newpost);
    });

    for (i, e) in postvec.iter().enumerate() {
        if (postvec.len() > 14 && i >= postvec.len() - 15) || postvec.len() < 15 {
            print!("{}", e);
        }
    }

    Ok(())
}

// First handler to update posts.
pub fn update_handler(id: usize) -> error::Result<()> {
    let mut id_num_in = if id == 0 {
        println!();
        println!("ID number of your post to edit?");
        let mut id_num_in = String::new();
        io::stdin().read_line(&mut id_num_in)?;
        id_num_in.trim().parse()?
    } else {
        id
    };

    id_num_in -= 1;

    #[cfg(not(test))]
    let user = &*user::NAME;

    let mut all = db::Posts::get_all(db::PATH);
    let post = all.get(id_num_in);

    #[cfg(test)]
    let user = &post.author;

    if *user != post.author {
        println!();
        println!("Users don't match. Can't update post!");
        println!();
        std::process::exit(1);
    }

    #[cfg(test)]
    let new_title = String::from("TEST_TITLE");

    #[cfg(not(test))]
    let mut new_title = String::new();

    println!("Updating post {}", id_num_in);
    println!();
    println!("Current Title: {}", post.title);
    println!();
    println!("Enter new title:");

    #[cfg(not(test))]
    io::stdin().read_line(&mut new_title)?;

    #[cfg(test)]
    let body_raw = String::from("TEST_BODY");

    #[cfg(not(test))]
    let body_raw = str_to_utf8(&ed::call(&post.body));

    let body = if body_raw.len() > 500 {
        &body_raw[..500]
    } else {
        &body_raw
    };

    let trimmed_body = body.trim();

    all.replace(
        id_num_in,
        db::Post {
            author: user.into(),
            title: new_title,
            body: trimmed_body.to_string(),
        },
    );

    all.write();

    println!();
    Ok(())
}

// First handler to remove a post
pub fn delete_handler(id: usize) -> error::Result<()> {
    let mut id_num_in = if id == 0 {
        println!();
        println!("ID of the post to delete?");
        let mut id_num_in = String::new();
        io::stdin().read_line(&mut id_num_in)?;
        println!();
        id_num_in.trim().parse()?
    } else {
        id
    };

    id_num_in -= 1;

    let mut all = db::Posts::get_all(db::PATH);
    let post = all.get(id_num_in);

    if *user::NAME != post.author {
        println!();
        println!("Users don't match. Can't delete post!");
        println!();
        std::process::exit(1);
    }

    all.delete(id_num_in);
    all.write();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;

    #[test]
    fn test_str_to_utf8() {
        let lhs = "foobar";
        let rhs = str_to_utf8(lhs);
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn display_doesnt_explode() {
        assert!(display().is_ok());
    }

    #[test]
    fn test_update_handler() {
        fs::copy(db::PATH, "clinte_bak.json").unwrap();
        update_handler(1).unwrap();
        let all = db::Posts::get_all(db::PATH);
        let post = all.get(0);
        assert_eq!(post.title, "TEST_TITLE");
        assert_eq!(post.body, "TEST_BODY");
        fs::rename("clinte_bak.json", db::PATH).unwrap();
    }

    #[test]
    fn test_create_delete() {
        fs::copy(db::PATH, "clinte_bak.json").unwrap();
        create().unwrap();
        let all = db::Posts::get_all(db::PATH);
        let post = all.get(1);

        assert_eq!(post.title, "TEST_TITLE");
        assert_eq!(post.body, "TEST_BODY");

        delete_handler(2).unwrap();

        fs::rename("clinte_bak.json", db::PATH).unwrap();
    }
}
