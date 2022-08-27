use genius_rs::song::Song;
use owo_colors::{
    colors::{css::Orange, Black, BrightGreen, BrightYellow, Cyan, Magenta, Yellow},
    OwoColorize,
};
use rand::Rng;

pub fn print_colorized(song: &Song) {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..5) {
        0 => {
            println!(
                "\n{}{}{}",
                song.primary_artist.name.fg::<Black>().bg::<Magenta>(),
                " - ".fg::<Black>().bg::<Magenta>(),
                song.title.fg::<Black>().bg::<Magenta>()
            );
            println!("{}\n", song.url.fg::<Magenta>());
        }
        1 => {
            println!(
                "\n{}{}{}",
                song.primary_artist.name.fg::<Black>().bg::<Cyan>(),
                " - ".fg::<Black>().bg::<Cyan>(),
                song.title.fg::<Black>().bg::<Cyan>()
            );
            println!("{}\n", song.url.fg::<Cyan>());
        }
        2 => {
            println!(
                "\n{}{}{}",
                song.primary_artist.name.fg::<Black>().bg::<Orange>(),
                " - ".fg::<Black>().bg::<Orange>(),
                song.title.fg::<Black>().bg::<Orange>()
            );
            println!("{}\n", song.url.fg::<Orange>());
        }
        3 => {
            println!(
                "\n{}{}{}",
                song.primary_artist.name.fg::<Black>().bg::<BrightGreen>(),
                " - ".fg::<Black>().bg::<BrightGreen>(),
                song.title.fg::<Black>().bg::<BrightGreen>()
            );
            println!("{}\n", song.url.fg::<BrightGreen>());
        }
        4 => {
            println!(
                "\n{}{}{}",
                song.primary_artist.name.fg::<Black>().bg::<Yellow>(),
                " - ".fg::<Black>().bg::<Yellow>(),
                song.title.fg::<Black>().bg::<Yellow>()
            );
            println!("{}\n", song.url.fg::<Yellow>());
        }

        5 => {
            println!(
                "\n{}{}{}",
                song.primary_artist.name.fg::<Black>().bg::<BrightYellow>(),
                " - ".fg::<Black>().bg::<BrightYellow>(),
                song.title.fg::<Black>().bg::<BrightYellow>()
            );
            println!("{}\n", song.url.fg::<BrightYellow>());
        }
        _ => {
            println!(
                "\n{}{}{}",
                song.primary_artist.name.fg::<Black>().bg::<Magenta>(),
                " - ".fg::<Black>().bg::<Magenta>(),
                song.title.fg::<Black>().bg::<Magenta>()
            );
            println!("{}\n", song.url.fg::<Magenta>());
        }
    }
}
