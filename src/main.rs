use std::{env, process::exit};

use clap::{Arg, Command};
use colored_json::ToColoredJson;
use genius_cli::colorizer::print_colorized;
use genius_rust::Genius;

fn cli() -> Command<'static> {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    Command::new("genius")
        .version(VERSION)
        .author("Tsiry Sandratraina <tsiry.sndr@aol.com>")
        .about(
            r#"
    ______           _               ________    ____
   / ____/__  ____  (_)_  _______   / ____/ /   /  _/
  / / __/ _ \/ __ \/ / / / / ___/  / /   / /    / /  
 / /_/ /  __/ / / / / /_/ (__  )  / /___/ /____/ /   
 \____/\___/_/ /_/_/\__,_/____/   \____/_____/___/   

 Genius CLI helps you search for lyrics or song informations from Genius.com.
                                                     
"#,
        )
        .subcommand_required(true)
        .subcommand(
            Command::new("search").about("Search for a song").arg(
                Arg::with_name("query")
                    .help("The query to search for, e.g. 'Niu Raza Any E'")
                    .required(true)
                    .index(1),
            ),
        )
        .subcommand(
            Command::new("lyrics")
                .about("Get the lyrics of a song")
                .arg(
                    Arg::with_name("id")
                        .help("The id of the song, e.g. '8333752'")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            Command::new("song").about("Get song informations").arg(
                Arg::with_name("id")
                    .help("The id of the song, e.g. '8333752'")
                    .required(true)
                    .index(1),
            ),
        )
}

#[tokio::main]
async fn main() {
    if env::var("GENIUS_TOKEN").unwrap().is_empty() {
        println!("Please set the GENIUS_TOKEN environment variable");
        exit(1);
    }

    let genius = Genius::new(env::var("GENIUS_TOKEN").unwrap());

    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("search", sub_matches)) => {
            let query = sub_matches.get_one::<String>("query").unwrap();
            let response = genius.search(query).await.unwrap();
            println!(
                "{}",
                serde_json::to_string(&response)
                    .unwrap()
                    .to_colored_json_auto()
                    .unwrap()
            );
        }
        Some(("lyrics", sub_matches)) => {
            let id = sub_matches
                .get_one::<String>("id")
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let result = genius.get_song(id, "plain").await;
            let song = result.unwrap();

            print_colorized(&song);

            let lyrics = genius.get_lyrics(id).await.unwrap();
            println!("{}", lyrics.join("\n"));
        }
        Some(("song", sub_matches)) => {
            let id = sub_matches
                .get_one::<String>("id")
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let song = genius.get_song(id, "plain").await.unwrap();
            println!(
                "{}",
                serde_json::to_string(&song)
                    .unwrap()
                    .to_colored_json_auto()
                    .unwrap()
            );
        }
        _ => unreachable!(),
    }
}
