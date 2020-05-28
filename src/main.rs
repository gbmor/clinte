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

fn main() {
    let arg_matches = &*conf::ARGS;
    let start = time::Instant::now();
    logging::checked_init();

    log::info!("clinte starting up!");
    println!("clinte v{}", clap::crate_version!());
    println!("a community notices system");
    println!();

    if *conf::DEBUG {
        log::info!("Startup completed in {:?}ms", start.elapsed().as_millis());
    }

    if arg_matches.subcommand_matches("post").is_some() {
        log::info!("New post...");
        error::helper(posts::create(), "Error creating new post");
    } else if let Some(updmatch) = arg_matches.subcommand_matches("update") {
        let id: usize = if let Some(val) = updmatch.value_of("id") {
            error::helper(val.parse(), "Couldn't parse ID")
        } else {
            0
        };

        log::info!("Updating post ...");

        error::helper(
            posts::update_handler(id),
            format!("Error updating post {}", id).as_ref(),
        );
    } else if let Some(delmatch) = arg_matches.subcommand_matches("delete") {
        let id: usize = if let Some(val) = delmatch.value_of("id") {
            error::helper(val.parse(), "Couldn't parse ID")
        } else {
            0
        };

        log::info!("Deleting post");

        error::helper(
            posts::delete_handler(id),
            format!("Error deleting post {}", id).as_ref(),
        );
    }

    error::helper(posts::display(), "Error displaying posts");
}
