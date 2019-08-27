use clap;
use log::info;
use std::sync::mpsc;
use std::time;

mod db;
mod logging;

fn main() {
    let arg_matches = clap::App::new("clinte")
        .version(clap::crate_version!())
        .author("Ben Morrison (gbmor)")
        .about("Command-line community notices system")
        .subcommand(clap::SubCommand::with_name("list").about("Display notices"))
        .subcommand(clap::SubCommand::with_name("post").about("Post a new notice"))
        .subcommand(
            clap::SubCommand::with_name("update")
                .about("Update a notice you've posted")
                .arg(clap::Arg::with_name("id").help("Numeric ID of the post")),
        )
        .subcommand(
            clap::SubCommand::with_name("delete")
                .about("Delete a notice you've posted")
                .arg(clap::Arg::with_name("id").help("Numeric ID of the post")),
        )
        .get_matches();

    let start = time::Instant::now();
    logging::init();
    info!("clinte starting up!");
    println!("clinte-0.1-dev");
    println!("a community notices system");

    let (_tx, rx) = mpsc::channel::<db::Cmd>();
    let db = db::Conn::new(rx);

    info!("Startup completed in {:?}ms", start.elapsed().as_millis());
}
