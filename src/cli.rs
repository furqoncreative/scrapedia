use clap::{arg, ArgAction, Command};

pub fn cli() -> Command {
    Command::new("scrapedia")
        .about("Scrape Product from Tokopedia")
        .version("v0.0.1")
        .arg(
            arg!(-q --query <VALUE>)
                .default_value("Samsung")
                .action(ArgAction::Set)
        )
}