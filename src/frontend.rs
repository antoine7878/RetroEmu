use std::error::Error;
use std::thread::sleep;
use std::time::Instant;

use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

use crate::gbc::{FRAME_TIME, GameBoy, HEIGHT, SCALE, WIDTH};

// const KEYMAP: [(Scancode, u8); 16] = [
//     (Scancode::Num1, 0x1),
//     (Scancode::Num2, 0x2),
//     (Scancode::Num3, 0x3),
//     (Scancode::Num4, 0xC),
//     (Scancode::Q, 0x4),
//     (Scancode::W, 0x5),
//     (Scancode::E, 0x6),
//     (Scancode::R, 0xD),
//     (Scancode::A, 0x7),
//     (Scancode::S, 0x8),
//     (Scancode::D, 0x9),
//     (Scancode::F, 0xE),
//     (Scancode::Z, 0xA),
//     (Scancode::X, 0x0),
//     (Scancode::C, 0xB),
//     (Scancode::V, 0xF),
// ];

pub fn run(game_boy: &mut GameBoy) -> Result<(), Box<dyn Error>> {
    let sdl = sdl2::init()?;
    let mut event_pump = sdl.event_pump()?;

    let window =
        sdl.video()?.window("RetoEmu", WIDTH as u32 * SCALE, HEIGHT as u32 * SCALE).position_centered().build()?;
    let mut canvas = window.into_canvas().present_vsync().build()?;
    canvas.set_logical_size(WIDTH as u32, HEIGHT as u32)?;
    canvas.set_integer_scale(true)?;
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, WIDTH as u32, HEIGHT as u32)?;

    let mut next_frame = Instant::now();
    'running: loop {
        next_frame += FRAME_TIME;

        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } | Event::KeyDown { scancode: Some(Scancode::Escape), .. } = event {
                break 'running;
            }
        }

        game_boy.run_frame();

        draw(game_boy, &mut texture, &mut canvas)?;

        let now = Instant::now();
        match next_frame.checked_duration_since(now) {
            Some(remaining) => sleep(remaining),
            None if now - next_frame > FRAME_TIME * 5 => next_frame = now,
            None => {}
        }
    }
    Ok(())
}

fn draw(game_boy: &mut GameBoy, texture: &mut Texture, canvas: &mut Canvas<Window>) -> Result<(), Box<dyn Error>> {
    if game_boy.take_redraw() {
        texture.update(None, game_boy.pixels(), WIDTH * 3)?;
        canvas.copy(texture, None, None)?;
        canvas.present();
    }
    Ok(())
}
