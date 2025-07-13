// Written by Juan Manuel Rodriguez (mcmodder123)

pub mod video {
    pub use hsl::HSL;
    use std::error::Error;
    use std::thread::sleep;
    use std::time::Duration;
    use video_rs::{self, Decoder, Location};

    // maps HSL lightness value to an ASCII character
    fn map_lightness_to_char(lightness: f64) -> char {
        if lightness >= 0.875 {
            return '$';
        } else if lightness >= 0.750 {
            return '#';
        } else if lightness >= 0.625 {
            return '@';
        } else if lightness >= 0.500 {
            return '%';
        } else if lightness >= 0.375 {
            return '*';
        } else if lightness >= 0.250 {
            return '~';
        } else if lightness >= 0.125 {
            return ',';
        } else {
            return '`';
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Video {
        filename: String,
        pub fps: u32,
    }

    impl Video {
        pub fn new(path: String, fps: u32) -> Video {
            Video {
                filename: path,
                fps,
            }
        }
        pub fn extract_frames(&self) -> Result<Vec<Vec<HSL>>, Box<dyn Error>> {
            // extract individual frames from a video
            video_rs::init()?;
            println!("Extracting frames from video...");
            let source = Location::File(std::path::PathBuf::from(&self.filename));
            let mut decoder = Decoder::new(&source)?;

            let mut frames: Vec<Vec<HSL>> = Vec::new();

            for frame_result in decoder.decode_raw_iter() {
                if let Ok(current_frame) = frame_result {
                    let width = current_frame.width();
                    let height = current_frame.height();
                    let mut frame_pixels: Vec<HSL> = Vec::new();

                    for y in 0..height {
                        for x in 0..width {
                            let pixel_index = (y * width + x) as usize * 3;
                            println!("Pixel index: {}", pixel_index);
                            let r = current_frame.data(0)[pixel_index];
                            let g = current_frame.data(0)[pixel_index + 1];
                            let b = current_frame.data(0)[pixel_index + 2];

                            let hsl = HSL::from_rgb(&[r, g, b]);
                            frame_pixels.push(hsl);
                        }
                    }
                    frames.push(frame_pixels);
                } else {
                    break; // video has ended
                }
            }
            Ok(frames)
        }

        pub fn convert_to_ascii(frames: Vec<Vec<HSL>>) -> Vec<String> {
            // converts video frames to ASCII
            println!("Converting frames to ASCII...");
            let mut ascii_frames = Vec::new();

            for frame in frames {
                let mut ascii_frame = String::new();
                for pixel in frame {
                    ascii_frame.push(map_lightness_to_char(pixel.l));
                }
                ascii_frames.push(ascii_frame);
            }
            ascii_frames
        }
    }
    pub fn play_video(video: &Video) -> Result<(), Box<dyn Error>> {
        // plays the newly created ASCII video
        println!("Attempting to play video... {:?}", video);
        let extracted_frames = video.extract_frames()?;
        let ascii_frames = Video::convert_to_ascii(extracted_frames);
        let frame_duration = Duration::from_secs_f64(1.0 / video.fps as f64);

        for frame_str in ascii_frames {
            print!("{}", frame_str);
        }
        sleep(frame_duration);
        Ok(())
    }
}

pub mod args {
    pub use super::video::Video;

    fn print_help(args: Vec<String>) {
        println!("Usage: {} <filename> <fps>", args[0])
    }

    pub fn parse_args(args: Vec<String>) -> Video {
        if args.len() < 3 {
            print_help(args);
            panic!("Not enough arguments!");
        }
        let fps: u32 = args[2].parse().expect("The FPS argument must be a number");

        let video = Video::new(args[1].clone(), fps);
        video
    }
}
// lib.rs
