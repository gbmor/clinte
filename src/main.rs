use std::time;

#[macro_use]
extern crate lazy_static;

use clap;
use log::info;

mod db;
mod ed;
mod error;
mod logging;
mod posts;
mod user;

fn main() {
    let arg_matches = clap::App::new("clinte")
        .version(clap::crate_version!())
        .author("Ben Morrison (gbmor)")
        .about("Command-line community notices system")
        .subcommand(clap::SubCommand::with_name("post").about("Post a new notice"))
        .subcommand(clap::SubCommand::with_name("update").about("Update a notice you've posted"))
        .subcommand(clap::SubCommand::with_name("delete").about("Delete a notice you've posted"))
        .get_matches();

    let start = time::Instant::now();
    logging::init();
    info!("clinte starting up!");
    println!("clinte v{}", clap::crate_version!());
    println!("a community notices system");
    println!();

    let db = db::Conn::new();

    info!("Startup completed in {:?}ms", start.elapsed().as_millis());

    if arg_matches.subcommand_matches("post").is_some() {
        info!("New post...");
        posts::create(&db);
    } else if arg_matches.subcommand_matches("update").is_some() {
        let id: u32 = if let Some(val) = arg_matches.subcommand_matches("update_handler") {
            val.value_of("id").unwrap().parse().unwrap()
        } else {
            0
        };
        info!("Updating post ...");
        posts::update_handler(&db, id);
    } else if arg_matches.subcommand_matches("delete").is_some() {
        let id: u32 = if let Some(val) = arg_matches.subcommand_matches("update_handler") {
            val.value_of("id").unwrap().parse().unwrap()
        } else {
            0
        };
        info!("Deleting post");
        posts::delete_handler(&db, id);
    }

    posts::display(&db);
}
