use std::io;

use crate::db;
use crate::ed;
use crate::error;
use crate::user;

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

    let mut title = String::new();
    io::stdin().read_line(&mut title)?;

    let title = str_to_utf8(title.trim());
    let title = if title.len() > 30 {
        &title[..30]
    } else {
        &title
    };

    println!();

    let body_raw = str_to_utf8(&ed::call(""));
    let body = if body_raw.len() > 500 {
        &body_raw[..500]
    } else {
        &body_raw
    };
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
    all.posts.iter().enumerate().for_each(|(id, post)| {
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

    let user = &*user::NAME;
    let mut all = db::Posts::get_all(db::PATH);
    let post = all.get(id_num_in);

    if *user != post.author {
        println!();
        println!("Users don't match. Can't update post!");
        println!();
        std::process::exit(1);
    }

    let mut new_title = String::new();

    println!("Updating post {}", id_num_in);
    println!();
    println!("Current Title: {}", post.title);
    println!();
    println!("Enter new title:");
    io::stdin().read_line(&mut new_title)?;

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
