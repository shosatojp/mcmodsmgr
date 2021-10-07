use clap::{crate_authors, crate_description, crate_name, crate_version};
pub use clap::{App, Arg, ArgMatches};

pub fn build_cli() -> ArgMatches {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            App::new("search")
                .about("search mods from curseforge.com")
                .arg(Arg::new("name").takes_value(true).required(true))
                .arg(
                    Arg::new("version")
                        .short('v')
                        .long("version")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("modloader")
                        .short('l')
                        .long("modloader")
                        .takes_value(true),
                ),
        )
        .subcommand(
            App::new("install")
                .about("download and place mod file")
                .arg(Arg::new("name").takes_value(true))
                .arg(
                    Arg::new("version")
                        .short('v')
                        .long("version")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("dir")
                        .about("directory to place mod file(s)")
                        .short('d')
                        .long("dir"),
                ),
        )
        .get_matches();

    app
}
