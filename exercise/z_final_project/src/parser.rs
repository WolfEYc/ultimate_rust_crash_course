
use clap::{Args, Parser, Subcommand};
use image::DynamicImage;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {

    #[arg(short, long, value_name = "FILE")]
    input_file: String,

    #[arg(short, long, value_name = "FILE")]
    output_file: String,

    #[command(subcommand)]
    pub command: Command
}


#[derive(Subcommand)]
pub enum Command {
    Blur {
        amt: f32
    },
    Brighten {
        amt: i32
    },
    Crop {
        x: u32, y: u32, width: u32, height: u32
    },
    Rotate {
        degs: u32
    },
    Invert,
    Grayscale,
    Generate,
    Fractal
}

impl Cli {
    pub fn in_file_to_img(&self) -> DynamicImage {
        return image::open(self.input_file.as_str()).expect("Failed to open INFILE.");
    }
    pub fn img_to_out_file(&self, img: DynamicImage) {
        img.save(self.output_file.as_str()).expect("Failed writing OUTFILE.");
    }
}
