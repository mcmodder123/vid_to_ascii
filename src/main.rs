// Video to ASCII converter
// Written by Juan Manuel Rodriguez (mcmodder123)

use std::{env, error::Error};
use vid_to_ascii::{args, video};

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect();
    let video = args::parse_args(args);
    if let Err(e) = video::play_video(&video) {
        eprintln!("Failed to play video:");
        return Err(e);
    }
    Ok(())
} // main.rs
