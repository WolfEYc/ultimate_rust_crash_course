mod generators;
mod parser;

use clap::Parser;
use parser::{Cli, Command};

fn main() {
       
    let cli = Cli::parse();

    let mut img = cli.in_file_to_img();

    img = match cli.command {
        Command::Blur { amt } => img.blur(amt),
        Command::Brighten { amt } => img.brighten(amt),
        Command::Crop { x, y, width, height } => img.crop(x, y, width, height),
        Command::Rotate { degs } => match degs {
            90 => { img.rotate90() },
            180 => { img.rotate180() },
            270 => { img.rotate270() }
            _ => img
        },
        Command::Invert => { 
            img.invert();
            img
        },
        Command::Grayscale => img.grayscale(),
        Command::Generate => generators::generate(),
        Command::Fractal => generators::fractal()
    };

    cli.img_to_out_file(img);
}

