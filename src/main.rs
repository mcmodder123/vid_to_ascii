/*
vid_to_ascii -- Allows you to view videos as ASCII text in your terminal.
Copyright (C) 2025 Juan Manuel Rodriguez

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>. */
/* Written by Juan Manuel Rodriguez (mcmodder123). */

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
