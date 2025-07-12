// Written by Juan Manuel Rodriguez (mcmodder123)

use video_rs::{self, Decoder, Locator};
use std::thread::sleep;
use hsl::HSL;

pub mod video {
    // maps HSL lightness value to an ASCII character
    fn map_lightness_to_char(lightness: u8) -> char {
        if lightness >= 87.5 {
            return '$';
        } else if lightness >= 75 {
            return '#';
        } else if lightness >= 62.5 {
            return '@';
        } else if lightness >= 50 {
            return '%';
        } else if lightness >= 37.5 {
            return '*';
        } else if lightness >= 25 {
            return '~';
        } else if lightness >= 12.5 {
            return ',';
        } else {
            return '`';
        }
    }

    pub struct Video {
        filename: String,
        fps: u32
        // frames: Vec<Vec<char>>,
    }

    impl Video {
        pub fn new(path: String, fps: u32) -> Video {
            Video {
                filename: path,
                fps: fps,
            }
        }
        pub fn extract_frames(&self) -> Vec<Vec<HSL>> {
            // extract individual frames from a video
            video_rs::init()?;
            let source = Locator::Path(std::path::PathBuf::from(self.filename));
            let mut decoder = Decoder::new(&source)?;

            let mut frames: Vec<Vec<HSL>> = Vec::new();

            for frame_result in decoder.decode_iter() {
                if let Ok((_timestamp, frame)) = frame_result {
                    let width = frame.width();
                    let hieght = frame.height();
                    let mut frame_pixels: Vec<HSL> = Vec::new();

                    for y in 0..height {
                        for x in 0..width {
                            let pixel_index = ( y * width + x) as usize * 3;
                            let r = frame.data()[pixel_index];
                            let g = frame.data()[pixel_index + 1];
                            let b = frame.data()[pixel_index + 2];

                            let hsl = HSL::from_rgb(&[r, g, b]);
                            frame_pixels.push(hsl);
                        }
                    }
                    frames.push(frame_pixels);
                } else {
                    break; // video end
                }
            }
            frames
        }

        pub fn convert_to_ascii(frames: Vec<Vec<HSL>>) -> Vec<Vec<char>> {
            // converts video frames to ASCII
            let mut frames_buf = Vec::new()
            for i in frames {
                for j in frames[i] {
                    let mut frame_buf = Vec::new();
                    frame_buf.push(map_lightness_to_char(j.l));
                }
            }
        }
    }
    pub fn play_video(video: Video) {
        // plays the newly created ASCII video
        ascii_frames = convert_to_ascii(video.extract_frames());
        for i in ascii_frames {
            print!("{}", ascii_frames[i]);
        }
        sleep(1/video.fps);
    }
}

pub mod args {
    fn print_help(args:) {
        println!("Usage: {} <filename> <fps>", args[0])
    }

    pub fn parse_args(args: Vec<String>) -> video::Video {
        if args.len() < 3 {
            print_help(args);
            panic!("Not enough arguments!");
        }
        let mut video = video::Video::new(String::from(args[1]), args[2]);
        video
    }
}
// lib.rs
