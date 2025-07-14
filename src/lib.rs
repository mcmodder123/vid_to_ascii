// Written by Juan Manuel Rodriguez (mcmodder123)

pub mod video {
    pub use hsl::HSL;
    use std::error::Error;
    use std::path::PathBuf;
    use std::thread::sleep;
    use std::time::Duration;
    use video_rs::{self, Decoder, Frame, Location};

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

        pub fn convert_to_ascii(frame: Frame) -> String {
            // converts video frames to ASCII
            let shape = frame.shape();
            let height = shape[0];
            let width = shape[1];
            let mut ascii_frame = String::new();

            for y in 0..height {
                for x in 0..width {
                    let r = frame[[y, x, 0]];
                    let g = frame[[y, x, 1]];
                    let b = frame[[y, x, 2]];

                    let hsl = HSL::from_rgb(&[r, g, b]);
                    ascii_frame.push(map_lightness_to_char(hsl.l));
                }
                ascii_frame.push('\n');
            }
            ascii_frame
        }
    }
    pub fn play_video(video: &Video) -> Result<(), Box<dyn Error>> {
        // plays the newly created ASCII video
        println!("Attempting to play video... {:?}", video);
        video_rs::init()?;
        let source = Location::File(PathBuf::from(&video.filename));
        let mut decoder = Decoder::new(&source)?;

        let frame_duration = Duration::from_secs_f64(1.0 / video.fps as f64);

        for frame_result in decoder.decode_iter() {
            if let Ok((_, frame)) = frame_result {
                let ascii_frame = Video::convert_to_ascii(frame);
                print!("\x1B[2J\x1B[1;1H{}", ascii_frame);
                sleep(frame_duration);
            } else if let Err(e) = frame_result {
                return Err(Box::new(e));
            } else {
                break;
            }
        }
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
} // lib.rs
