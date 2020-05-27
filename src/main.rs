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

    let db = db::Conn::new();

    if *conf::DEBUG {
        log::info!("Startup completed in {:?}ms", start.elapsed().as_millis());
    }

    if arg_matches.subcommand_matches("post").is_some() {
        log::info!("New post...");
        if let Err(e) = posts::create(&db) {
            log::error!("Error creating new post");
            if *conf::DEBUG {
                log::error!("--> {}", e);
            }
            std::process::exit(1);
        }
    } else if arg_matches.subcommand_matches("update").is_some() {
        let id: u32 = if let Some(val) = arg_matches.subcommand_matches("update_handler") {
            match val.value_of("id").unwrap_or_else(|| "0").parse() {
                Ok(n) => n,
                Err(e) => {
                    log::error!("Couldn't parse ID");
                    if *conf::DEBUG {
                        log::error!("--> {}", e);
                    }
                    std::process::exit(1);
                }
            }
        } else {
            0
        };
        log::info!("Updating post ...");
        if let Err(e) = posts::update_handler(&db, id) {
            log::error!("Error updating post {}", id);
            if *conf::DEBUG {
                log::error!("--> {}", e);
            }
            std::process::exit(1);
        }
    } else if arg_matches.subcommand_matches("delete").is_some() {
        let id: u32 = if let Some(val) = arg_matches.subcommand_matches("update_handler") {
            match val.value_of("id").unwrap_or_else(|| "0").parse() {
                Ok(n) => n,
                Err(_) => {
                    log::error!("Couldn't parse ID");
                    std::process::exit(1);
                }
            }
        } else {
            0
        };
        log::info!("Deleting post");
        if let Err(e) = posts::delete_handler(&db, id) {
            log::error!("Error deleting post {}", id);
            if *conf::DEBUG {
                log::error!("--> {}", e);
            }
            std::process::exit(1);
        }
    }

    if let Err(e) = posts::display(&db) {
        log::error!("Error displaying posts");
        if *conf::DEBUG {
            log::error!("--> {}", e);
        }
        std::process::exit(1);
    }
}
