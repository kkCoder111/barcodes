use clap::Parser;
use cli::Cli;
use colorful::{Colorful, RGB};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub mod cli;
pub mod types;

pub const TYPE_SORTS: [&str; 4] = ["█", "▊", "▌", "▎"];

fn main() {
    let cli = Cli::parse();
    let colors = cli.command.colors();

    let size = termsize::get().unwrap();

    let length = colors.len() as u16;
    let chunk = size.cols / length;

    for _ in 0..length {
        for _ in 0..chunk {
            let color = *colors.choose(&mut thread_rng()).unwrap();
            print!(
                "{}",
                TYPE_SORTS
                    .choose(&mut thread_rng())
                    .unwrap()
                    .color(RGB::from(color))
            )
        }
    }
    println!("");
}
