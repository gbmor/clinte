use clap;
use log::info;
use std::io;
use std::process;
use std::time;
use users;

mod db;
mod logging;

fn main() {
    let arg_matches = clap::App::new("clinte")
        .version(clap::crate_version!())
        .author("Ben Morrison (gbmor)")
        .about("Command-line community notices system")
        //.subcommand(clap::SubCommand::with_name("list").about("Display notices"))
        .subcommand(clap::SubCommand::with_name("post").about("Post a new notice"))
        /*.subcommand(
            clap::SubCommand::with_name("update")
                .about("Update a notice you've posted")
                .arg(clap::Arg::with_name("id").help("Numeric ID of the post")),
        )
        .subcommand(
            clap::SubCommand::with_name("delete")
                .about("Delete a notice you've posted")
                .arg(clap::Arg::with_name("id").help("Numeric ID of the post")),
        )*/
        .get_matches();

    let start = time::Instant::now();
    logging::init();
    info!("clinte starting up!");
    println!("clinte-0.1-dev");
    println!("a community notices system");
    println!();

    let db = db::Conn::new();

    info!("Startup completed in {:?}ms", start.elapsed().as_millis());

    if let Some(_) = arg_matches.subcommand_matches("post") {
        info!("New post...");
        post(&db);
        list_matches(&db);
        process::exit(0);
    }

    list_matches(&db);
}

fn list_matches(db: &db::Conn) {
    let mut stmt = db.conn.prepare("SELECT * FROM posts").unwrap();
    let out = stmt
        .query_map(rusqlite::NO_PARAMS, |row| {
            let id = row.get(0)?;
            let title = row.get(1)?;
            let author = row.get(2)?;
            let body = row.get(3)?;
            Ok(db::Post {
                id,
                title,
                author,
                body,
            })
        })
        .unwrap();

    out.for_each(|row| {
        if let Ok(post) = row {
            println!(
                "{}. {} -> by {}\n{}",
                post.id, post.title, post.author, post.body
            );
            println!();
        }
    });
}

fn post(db: &db::Conn) {
    let mut stmt = db
        .conn
        .prepare("INSERT INTO posts (title, author, body) VALUES (:title, :author, :body)")
        .unwrap();

    println!();
    println!("Title of the new post: ");
    let mut title = String::new();
    io::stdin().read_line(&mut title).unwrap();
    let title = title.trim();
    let title = if title.len() > 30 {
        &title[..30]
    } else {
        &title
    };

    println!();
    println!("Body of the new post: ");
    let mut body = String::new();
    io::stdin().read_line(&mut body).unwrap();
    let body = body.trim();
    let body = if body.len() > 500 {
        &body[..500]
    } else {
        &body
    };

    let user = users::get_current_username()
        .unwrap()
        .into_string()
        .unwrap();

    stmt.execute_named(&[(":title", &title), (":author", &user), (":body", &body)]);
}
