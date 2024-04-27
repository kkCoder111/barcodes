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
    let colors_length = 2;

    let size = termsize::get().unwrap();
    let length = colors.len() as u16;
    let chunk = size.cols / length;

    let mut buffer = String::from("");

    for _ in 0..length {
        for _ in 0..chunk {
            let color = *colors.choose(&mut thread_rng()).unwrap();
            buffer.push_str(
                format!(
                    "{}",
                    TYPE_SORTS
                        .choose(&mut thread_rng())
                        .unwrap()
                        .color(RGB::from(color))
                )
                .as_str(),
            )
        }
    }

    let line = buffer.clone();
    for _ in 0..colors_length - 1 {
        buffer.push_str(line.as_str());
    }

    println!("{buffer}");
}
