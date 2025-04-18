use crate::histogram::save_histogram;
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::{CODEC_TYPE_NULL, DecoderOptions};
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use std::{io::Write, fs::File};

const FILE_NAME: &str = "data/keyboard.wav";

pub fn calculate_rms(samples: &[f32]) -> f32 {
    let sum_of_squares: f32 = samples.iter().map(|&x| x.powi(2)).sum();
    let mean_square = sum_of_squares / samples.len() as f32;
    mean_square.sqrt()
}

pub fn audio() {
    let src = File::open(FILE_NAME).unwrap();
    let mss = MediaSourceStream::new(Box::new(src), Default::default());

    let mut hint = Hint::new();
    hint.with_extension("wav");

    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();

    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &fmt_opts, &meta_opts)
        .unwrap();

    let mut format = probed.format;

    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .unwrap();

    let dec_opts: DecoderOptions = Default::default();

    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &dec_opts)
        .unwrap();

    let track_id = track.id;

    let mut statistics: [u128; 10] = [0; 10];

    while let Ok(packet) = format.next_packet() {
        if packet.track_id() != track_id {
            continue;
        }

        let decoded = decoder.decode(&packet).unwrap();

        let spec = decoded.spec();
        let duration = decoded.capacity();

        let mut buf = SampleBuffer::<f32>::new(duration as u64, *spec);
        buf.copy_interleaved_ref(decoded);

        let rms = calculate_rms(buf.samples());

        for char in rms.to_string().chars() {
            if char != '0' && char != '.' {
                statistics[char.to_digit(10).unwrap() as usize] += 1;
                break;
            }
        }
    }

    let sum = statistics.iter().sum::<u128>();
    save_histogram(
        10,
        statistics.map(|x| x as f64 / sum as f64).to_vec(),
        "rms.png",
    )
    .unwrap();
}
