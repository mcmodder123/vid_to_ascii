// Written by Juan Manuel Rodriguez (mcmodder123)

use video_rs::{self, Decoder, Locator};
use hsl::HSL;

pub mod video {
    // maps HSL lightness value to an ASCII character
    pub fn map_lightness_to_char(lightness: u8) -> char {
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
    }
    impl Video {
        pub fn extract_frames(&self) -> Vec<Vec<HSL>> {
            video_rs::init()?;
            let source = Locator::Path(std::path::PathBuf::from(self.filename));
            let mut decoder = Decoder::new(&source)?;

            let mut all_pixels: Vec<Vec<HSL>> = Vec::new();

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
                    all_pixels.push(frame_pixels);
                } else {
                    break; // video end
                }
            }
        }
        pub fn new(path: String) -> Video {
            Video {
                filename: filename,
            }
        }
    }
}
// lib.rs
