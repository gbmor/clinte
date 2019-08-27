use log::info;
use std::sync::mpsc;

mod db;
mod logging;

fn main() {
    logging::init();
    info!("clinte starting up!");
    println!("clinte-0.1-dev");
    println!("a community notices system");

    let (_tx, rx) = mpsc::channel::<db::Cmd>();
    let db = db::Conn::new(rx);

    println!("{:?}", db);
}
