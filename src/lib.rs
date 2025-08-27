/* Written by Juan Manuel Rodriguez (mcmodder123). */

pub mod video {
    use hsl::HSL;
    use std::error::Error;
    use std::path::PathBuf;
    use std::thread::sleep;
    use std::time::Duration;
    use video_rs::{self, Decoder, Frame, Location};

    /// Maps an HSL lightness value to an ASCII character.
    ///
    /// # Arguments
    /// * `l: f64` - HSL lightness value.
    ///
    /// # Examples
    /// ```
    /// use vid_to_ascii::map_lightness_to_char;
    /// use hsl::HSL;
    ///
    /// let yellow = [255, 255, 0];
    /// let yellow_hsl = HSL::from_rgb(&yellow);
    ///
    /// assert_eq!(map_lightness_to_char(yellow_hsl.l /* 0.5_f64 */), '%');
    /// ```
    pub fn map_lightness_to_char(l: f64) -> char {
        if l >= 0.875 {
            '$'
        } else if l >= 0.750 {
            '#'
        } else if l >= 0.625 {
            '@'
        } else if l >= 0.500 {
            '%'
        } else if l >= 0.375 {
            '*'
        } else if l >= 0.250 {
            '~'
        } else if l >= 0.125 {
            ','
        } else {
            '`'
        }
    }

    /// Represents a video with its filename and FPS.
    ///
    /// This struct holds basic information about a video file.
    ///
    /// # Examples
    ///
    /// ```
    /// use vid_to_ascii::video::*;
    ///
    /// let video = Video::new("my_awesome_video.mp4", 60);
    /// let _ = play_video(&video);
    /// ```
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Video {
        pub filename: String,
        pub fps: u32,
    }

    impl Video {
        pub fn new(path: String, fps: u32) -> Video {
            Video {
                filename: path,
                fps,
            }
        }

        /// Converts a video_rs::Frame to a String.
        ///
        fn convert_to_ascii(frame: Frame) -> String {
            let shape = frame.shape();
            let height = shape[0];
            let width = shape[1];
            let mut ascii_frame = String::new();

            // Iterate through each pixel and map it to a character.
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

    /// Converts a Video to ASCII and plays it in real time.
    ///
    /// # Arguments
    /// * `video` - An instance of the Video struct.
    ///
    /// # Examples
    ///
    /// ```
    /// use vid_to_ascii::video::*;
    ///
    /// let video = Video::new("my_awesome_video.mp4", 60);
    /// let _ = play_video(&video);
    /// ```
    pub fn play_video(video: &Video) -> Result<(), Box<dyn Error>> {
        video_rs::init()?;
        let source = Location::File(PathBuf::from(&video.filename));

        let mut decoder = Decoder::new(&source)?;

        let frame_duration = Duration::from_secs_f64(1.0 / video.fps as f64);

        for frame_result in decoder.decode_iter() {
            if let Ok((_, frame)) = frame_result {
                let ascii_frame = Video::convert_to_ascii(frame);
                print!("\x1B[2J\x1B[1;1H{ascii_frame}");
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
    // args module.
    pub use super::video::Video;

    /// Output help message.
    fn print_help(args: Vec<String>) {
        println!("Usage: {} <filename> <fps>", args[0])
    }

    /// Parse arguments to create a Video instance.
    pub fn parse_args(args: Vec<String>) -> Video {
        if args.len() < 3 {
            print_help(args);
            panic!("Not enough arguments!");
        }
        let fps = args[2].parse().expect("The FPS argument must be a number.");

        Video::new(args[1].clone(), fps)
    }
}

#[cfg(test)]
mod tests {
    // tests
    use super::*;

    #[test]
    fn test_map_lightness_boundaries() {
        assert_eq!(video::map_lightness_to_char(0.875), '$');
        assert_eq!(video::map_lightness_to_char(0.8749), '#');
        assert_eq!(video::map_lightness_to_char(0.750), '#');
        assert_eq!(video::map_lightness_to_char(0.749), '@');
        assert_eq!(video::map_lightness_to_char(0.0), '`');
        assert_eq!(video::map_lightness_to_char(1.0), '$');
    }

    #[test]
    fn test_parse_args_valid() {
        let args = vec![
            String::from("./vid_to_ascii"),
            String::from("test.mp4"),
            String::from("30"),
        ];
        let video = args::parse_args(args);
        assert_eq!(video.filename, String::from("test.mp4"));
        assert_eq!(video.fps, 30);
    }

    #[test]
    #[should_panic(expected = "Not enough arguments!")]
    fn test_parse_args_not_enough() {
        let args = vec![String::from("./vid_to_ascii"), String::from("test.mp4")];
        args::parse_args(args);
    }

    #[test]
    fn test_play_video_non_existent_file() {
        let video_instance = video::Video::new(String::from("non_existent_file.mp4"), 30);
        let result = video::play_video(&video_instance);
        assert!(result.is_err());
    }

    #[test]
    fn test_video_new() {
        let target = video::Video {
            filename: String::from("test.mp4"),
            fps: 30,
        };
        let result = video::Video::new(String::from("test.mp4"), 30);
        assert_eq!(target, result);
    }
}
// lib.rs
