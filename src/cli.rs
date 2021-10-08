use clap::{crate_authors, crate_description, crate_name, crate_version};
pub use clap::{App, AppSettings, Arg, ArgMatches};

pub fn build_cli() -> App<'static, 'static> {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("mcversion")
                .short("v")
                .long("mcversion")
                .global(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("modloader")
                .short("l")
                .long("modloader")
                .help("mod loader")
                .takes_value(true)
                .global(true)
                .possible_value("forge")
                .possible_value("fabric"),
        )
        .arg(
            Arg::with_name("full")
                .short("F")
                .long("full")
                .takes_value(false)
                .global(true)
                .help("show all files instead of listing only latest files for each version"),
        )
        .arg(
            Arg::with_name("lockfile")
                .short("L")
                .long("lockfile")
                .global(true)
                .takes_value(true),
        )
        .subcommand(
            App::new("search")
                .about("search mods from curseforge.com")
                .arg(Arg::with_name("name").takes_value(true).required(true)),
        )
        .subcommand(
            App::new("install")
                .about("download and place mod file")
                .arg(Arg::with_name("name").takes_value(true).required(true))
                .arg(
                    Arg::with_name("fileid")
                        .short("f")
                        .long("fileid")
                        .validator(|e| {
                            e.parse::<usize>()
                                .and(Ok(()))
                                .or(Err("parse int error".to_string()))
                        })
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("filename")
                        .short("n")
                        .long("filename")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("dir")
                        .help("directory to place mod file(s)")
                        .short("d")
                        .long("dir")
                        .takes_value(true),
                ),
        )
        .subcommand(
            App::new("describe")
                .about("describe a mod and list available files")
                .arg(Arg::with_name("name").takes_value(true).required(true)),
        );

    app
}
