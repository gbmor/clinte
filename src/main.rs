use std::time;

#[macro_use]
extern crate lazy_static;

mod conf;
mod db;
mod ed;
mod error;
mod logging;
mod posts;
mod user;

fn main() -> error::Result<()> {
    let arg_matches = &*conf::ARGS;
    let start = time::Instant::now();
    let logfile = format!("/tmp/clinte_{}.log", *user::NAME);
    logging::init(&logfile)?;

    log::info!("clinte starting up!");
    println!("clinte v{}", clap::crate_version!());
    println!("a community notices system");
    println!();

    let db = db::Conn::new();

    if *conf::DEBUG {
        log::info!("Startup completed in {:?}ms", start.elapsed().as_millis());
    }

    if arg_matches.subcommand_matches("post").is_some() {
        log::info!("New post...");
        posts::create(&db)?;
    } else if arg_matches.subcommand_matches("update").is_some() {
        let id: u32 = if let Some(val) = arg_matches.subcommand_matches("update_handler") {
            val.value_of("id").unwrap().parse()?
        } else {
            0
        };
        log::info!("Updating post ...");
        posts::update_handler(&db, id)?;
    } else if arg_matches.subcommand_matches("delete").is_some() {
        let id: u32 = if let Some(val) = arg_matches.subcommand_matches("update_handler") {
            val.value_of("id").unwrap_or_else(|| "0").parse()?
        } else {
            0
        };
        log::info!("Deleting post");
        posts::delete_handler(&db, id)?;
    }

    posts::display(&db)?;

    Ok(())
}
