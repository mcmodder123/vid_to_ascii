// Video to ASCII converter
// Written by Juan Manuel Rodriguez (mcmodder123)

use std::env;
use vid_to_ascii::{args, video};

fn main() {
    let args = env::args().collect();
    let video = parse_args(args);
    play_video(video);
} // main.rs
