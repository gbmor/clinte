use clap::{Arg, ArgMatches};

lazy_static! {
    pub static ref ARGS: ArgMatches<'static> = get_config();
    pub static ref DEBUG: bool = ARGS.is_present("verbose");
}

fn get_config() -> clap::ArgMatches<'static> {
    clap::App::new("clinte")
        .version(clap::crate_version!())
        .author("Ben Morrison <ben@gbmor.dev>")
        .about("Command-line community notices system")
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Verbose logging"),
        )
        .subcommand(clap::SubCommand::with_name("post").about("Post a new notice"))
        .subcommand(clap::SubCommand::with_name("update").about("Update a notice you've posted"))
        .subcommand(clap::SubCommand::with_name("delete").about("Delete a notice you've posted"))
        .get_matches()
}
