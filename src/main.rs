use clap::App;
mod config;
mod lib;

fn main() {
    let _matches = App::new("cale")
        .version("0.1")
        .subcommand(App::new("now").about("shows current activity"))
        .subcommand(App::new("today").about("shows today plan"))
        .get_matches();

    if let Some(_matches) = _matches.subcommand_matches("now") {
        lib::now(String::from(config::PATH)).unwrap();
    }

    if let Some(_matches) = _matches.subcommand_matches("today") {
        lib::today(String::from(config::PATH)).unwrap();
    }
}
