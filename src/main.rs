use std::time;

#[macro_use]
extern crate lazy_static;

use clap;

mod db;
mod ed;
mod error;
mod logging;
mod posts;
mod user;

fn main() -> error::Result<()> {
    let arg_matches = clap::App::new("clinte")
        .version(clap::crate_version!())
        .author("Ben Morrison <ben@gbmor.dev>")
        .about("Command-line community notices system")
        .subcommand(clap::SubCommand::with_name("post").about("Post a new notice"))
        .subcommand(clap::SubCommand::with_name("update").about("Update a notice you've posted"))
        .subcommand(clap::SubCommand::with_name("delete").about("Delete a notice you've posted"))
        .get_matches();

    let start = time::Instant::now();
    let file = format!("/tmp/clinte_{}.log", *user::NAME);
    logging::init(&file)?;
    log::info!("clinte starting up!");
    println!("clinte v{}", clap::crate_version!());
    println!("a community notices system");
    println!();

    let db = db::Conn::new();

    log::info!("Startup completed in {:?}ms", start.elapsed().as_millis());

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
