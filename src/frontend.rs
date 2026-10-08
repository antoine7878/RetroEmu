use std::error::Error;
use std::thread::sleep;
use std::time::{Duration, Instant};

use sdl2::audio::{AudioQueue, AudioSpecDesired};
use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::pixels::PixelFormatEnum;

use crate::gbc::{Chip8, FRAME_RATE, HEIGHT, SAMPLE_RATE, WIDTH};

const SCALE: u32 = 12;
const FRAME_TIME: Duration = Duration::from_nanos(1_000_000_000 / FRAME_RATE as u64);
const MAX_QUEUED_SAMPLES: u32 = SAMPLE_RATE / 10;

const ON_COLOR: [u8; 3] = [0xFF, 0xFF, 0xFF];
const OFF_COLOR: [u8; 3] = [0x00, 0x00, 0x00];

const KEYMAP: [(Scancode, u8); 16] = [
    (Scancode::Num1, 0x1),
    (Scancode::Num2, 0x2),
    (Scancode::Num3, 0x3),
    (Scancode::Num4, 0xC),
    (Scancode::Q, 0x4),
    (Scancode::W, 0x5),
    (Scancode::E, 0x6),
    (Scancode::R, 0xD),
    (Scancode::A, 0x7),
    (Scancode::S, 0x8),
    (Scancode::D, 0x9),
    (Scancode::F, 0xE),
    (Scancode::Z, 0xA),
    (Scancode::X, 0x0),
    (Scancode::C, 0xB),
    (Scancode::V, 0xF),
];

pub fn run(chip: &mut Chip8) -> Result<(), Box<dyn Error>> {
    let sdl = sdl2::init()?;
    let mut event_pump = sdl.event_pump()?;

    let window = sdl
        .video()?
        .window("CHIP-8", WIDTH as u32 * SCALE, HEIGHT as u32 * SCALE)
        .position_centered()
        .build()?;
    let mut canvas = window.into_canvas().present_vsync().build()?;
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, WIDTH as u32, HEIGHT as u32)?;

    let spec = AudioSpecDesired {
        freq: Some(SAMPLE_RATE as i32),
        channels: Some(1),
        samples: None,
    };
    let audio: AudioQueue<f32> = sdl.audio()?.open_queue(None, &spec)?;
    audio.resume();
    let mut samples = Vec::new();

    let mut next_frame = Instant::now();
    'running: loop {
        next_frame += FRAME_TIME;

        for event in event_pump.poll_iter() {
            if let Event::Quit { .. }
            | Event::KeyDown {
                scancode: Some(Scancode::Escape),
                ..
            } = event
            {
                break 'running;
            }
        }
        let keyboard = event_pump.keyboard_state();
        for (scancode, key) in KEYMAP {
            chip.set_key(key, keyboard.is_scancode_pressed(scancode));
        }

        chip.run_frame()?;

        samples.clear();
        chip.drain_audio(&mut samples);
        let queued = audio.size() / size_of::<f32>() as u32;
        if queued < MAX_QUEUED_SAMPLES {
            audio.queue_audio(&samples)?;
        }

        if chip.take_redraw() {
            texture.with_lock(None, |buffer, pitch| {
                for (y, row) in chip.pixels().chunks(WIDTH).enumerate() {
                    for (x, &lit) in row.iter().enumerate() {
                        let offset = y * pitch + x * 3;
                        let color = if lit != 0 { ON_COLOR } else { OFF_COLOR };
                        buffer[offset..offset + 3].copy_from_slice(&color);
                    }
                }
            })?;
            canvas.copy(&texture, None, None)?;
            canvas.present();
        }

        let now = Instant::now();
        match next_frame.checked_duration_since(now) {
            Some(remaining) => sleep(remaining),
            None if now - next_frame > FRAME_TIME * 5 => next_frame = now,
            None => {}
        }
    }
    Ok(())
}
