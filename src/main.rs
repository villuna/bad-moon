#![feature(thread_sleep_until)]
use std::{
    fs::File, io::BufReader, path::PathBuf, str::FromStr, time::{Duration, Instant}
};

use color_eyre::eyre::Result;
use ordered_float::NotNan;
use rodio::{OutputStream, Source};

const VIDEO_WIDTH: usize = 480;
const VIDEO_HEIGHT: usize = 360;

const CHARS: &str = "🌕🌖🌗🌘🌑🌒🌓🌔";
const BLOCKS: &[[f32; 4]] = &[
    [1.0, 1.0, 1.0, 1.0],
    [1.0, 1.0, 1.0, 0.0],
    [1.0, 1.0, 0.0, 0.0],
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
    [0.0, 0.0, 1.0, 1.0],
    [0.0, 1.0, 1.0, 1.0],
];

// Returns the pythagorean distance between two 4-dimensional vectors
// I'm using it as a general metric for how similar two lines of pixels are
fn pythag_distance(a: &[f32], b: &[f32]) -> f32 {
    a.iter()
        .zip(b.iter())
        .map(|(a, b)| (a - b) * (a - b))
        .sum::<f32>()
        .sqrt()
}

fn val_to_char(line: &[f32]) -> char {
    let closest = BLOCKS
        .iter()
        .enumerate()
        .min_by_key(|&(_, block)| NotNan::new(pythag_distance(&line, block)).unwrap())
        .map(|(i, _)| i)
        .unwrap();

    CHARS.chars().nth(closest).unwrap()
}

fn main() -> Result<()> {
    video_rs::init().expect("Couldn't initialise video_rs");
    tracing_subscriber::fmt::init();
    color_eyre::install()?;

    let source = PathBuf::from_str("resources/bad apple.mp4")?;
    let mut decoder = video_rs::Decoder::new(source)?;

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open("resources/bad apple.mp3").unwrap());
    let source = rodio::Decoder::new(file).unwrap();
    stream_handle.play_raw(source.convert_samples())?;

    let mut next_frame = Instant::now();
    let frame_time = Duration::from_secs_f32(1.0 / 30.0);
    for frame in decoder.decode_iter() {
        std::thread::sleep_until(next_frame);
        next_frame += frame_time;

        let Ok((_, frame)) = frame else {
            break;
        };

        for y in 0..VIDEO_HEIGHT / 8 {
            for x in 0..VIDEO_WIDTH / 8 {
                let mut val = frame
                    .slice(ndarray::s![y * 8 + 4, (x * 8)..((x + 1) * 8), 0])
                    .iter()
                    .map(|val| *val as f32 / 255.0)
                    .collect::<Vec<_>>();

                for i in [7, 5, 3, 1] {
                    val.remove(i);
                }

                print!("{}", val_to_char(&val));
            }
            println!();
        }

        println!();
    }

    Ok(())
}
