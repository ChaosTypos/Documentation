use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {

    //Audio input
    let (_stream, handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handle).unwrap();
    let audio = File::open("music.mp3").unwrap();
    let _reader = BufReader::new(audio);
    let _source = Decoder::new(_reader).unwrap();
    sink.append(_source);
    
    //minifb window
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut playing = true;
    let mut window = Window::new(
        "MP@3",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 0; // write something more funny here!
        }

        if window.is_key_down(Key::Space) {
            if playing {
                sink.pause();
            } else {
                sink.play();
            }
            playing = !playing;
        }


        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }

}