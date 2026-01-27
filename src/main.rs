use std::{env, process::exit};

use clap::{Arg, Command};
use colored_json::ToColoredJson;
use genius_rust::Genius;
use owo_colors::{
    colors::{css::Orange, Black, BrightGreen, BrightYellow, Cyan, Magenta, Yellow},
    OwoColorize,
};
use rand::Rng;
use serde::Deserialize;

// API response structure from https://genius-mcp.xvzf.workers.dev/api/song/{id}/lyrics
// All fields are part of the API response and need to be present for deserialization
// Some fields aren't directly used in the code but are required for proper JSON parsing
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct LyricsApiResponse {
    #[serde(default)]
    id: u32,
    title: String,
    #[serde(default)]
    artist_names: String,  // Alternative to primary_artist.name
    url: String,
    lyrics: String,
    primary_artist: ApiPrimaryArtist,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ApiPrimaryArtist {
    #[serde(default)]
    id: u32,
    name: String,
    url: String,
}

fn print_song_info(artist_name: &str, title: &str, url: &str) {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..6) {
        0 => {
            println!(
                "\n{}{}{}",
                artist_name.fg::<Black>().bg::<Magenta>(),
                " - ".fg::<Black>().bg::<Magenta>(),
                title.fg::<Black>().bg::<Magenta>()
            );
            println!("{}\n", url.fg::<Magenta>());
        }
        1 => {
            println!(
                "\n{}{}{}",
                artist_name.fg::<Black>().bg::<Cyan>(),
                " - ".fg::<Black>().bg::<Cyan>(),
                title.fg::<Black>().bg::<Cyan>()
            );
            println!("{}\n", url.fg::<Cyan>());
        }
        2 => {
            println!(
                "\n{}{}{}",
                artist_name.fg::<Black>().bg::<Orange>(),
                " - ".fg::<Black>().bg::<Orange>(),
                title.fg::<Black>().bg::<Orange>()
            );
            println!("{}\n", url.fg::<Orange>());
        }
        3 => {
            println!(
                "\n{}{}{}",
                artist_name.fg::<Black>().bg::<BrightGreen>(),
                " - ".fg::<Black>().bg::<BrightGreen>(),
                title.fg::<Black>().bg::<BrightGreen>()
            );
            println!("{}\n", url.fg::<BrightGreen>());
        }
        4 => {
            println!(
                "\n{}{}{}",
                artist_name.fg::<Black>().bg::<Yellow>(),
                " - ".fg::<Black>().bg::<Yellow>(),
                title.fg::<Black>().bg::<Yellow>()
            );
            println!("{}\n", url.fg::<Yellow>());
        }
        5 => {
            println!(
                "\n{}{}{}",
                artist_name.fg::<Black>().bg::<BrightYellow>(),
                " - ".fg::<Black>().bg::<BrightYellow>(),
                title.fg::<Black>().bg::<BrightYellow>()
            );
            println!("{}\n", url.fg::<BrightYellow>());
        }
        _ => {
            println!(
                "\n{}{}{}",
                artist_name.fg::<Black>().bg::<Magenta>(),
                " - ".fg::<Black>().bg::<Magenta>(),
                title.fg::<Black>().bg::<Magenta>()
            );
            println!("{}\n", url.fg::<Magenta>());
        }
    }
}

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
            
            // Fetch lyrics from the new API
            let url = format!("https://genius-mcp.xvzf.workers.dev/api/song/{}/lyrics", id);
            let client = reqwest::Client::new();
            
            match client.get(&url).send().await {
                Ok(response) => {
                    match response.json::<LyricsApiResponse>().await {
                        Ok(lyrics_data) => {
                            // Print song info using our helper function
                            print_song_info(
                                &lyrics_data.primary_artist.name,
                                &lyrics_data.title,
                                &lyrics_data.url
                            );
                            
                            // Print the lyrics
                            println!("{}", lyrics_data.lyrics);
                        }
                        Err(e) => {
                            eprintln!("Error parsing API response: {}", e);
                            eprintln!("Please check that the song ID exists.");
                            exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error fetching lyrics from API: {}", e);
                    eprintln!("Please check your internet connection and try again.");
                    exit(1);
                }
            }
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
