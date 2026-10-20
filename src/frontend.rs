use std::error::Error;
use std::io::Write;
use std::sync::mpsc::Receiver;
use std::thread::sleep;
use std::time::Instant;

use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Canvas, Texture, TextureCreator, WindowCanvas};
use sdl2::video::{Window, WindowContext};
use sdl2::{EventPump, Sdl};

use crate::args::Args;
use crate::gbc::{Debugger, FRAME_TIME, GameBoy, HEIGHT, SCALE, WIDTH};

enum Mode {
    Running,
    Paused,
}

pub struct Frontend {
    sdl: Sdl,
    event_pump: EventPump,
    canvas: Canvas<Window>,
    texture: Texture,
    dbg: Option<Debugger>,
    mode: Mode,
    commands: Receiver<String>,
    last_cmd: String,
}

impl Frontend {
    pub fn new(args: Args, sdl: Sdl) -> Result<Self, Box<dyn Error>> {
        let mut event_pump = sdl.event_pump()?;

        let (canvas, texture) = Self::init_texture(&sdl)?;
        let mode = Mode::Running;
        let dbg = Debugger::new(&args);
        let commands = Self::init_debug_thread();
        let last_cmd = String::new();
        Ok(Self { sdl, event_pump, canvas, texture, dbg, mode, commands, last_cmd })
    }

    fn init_texture(sdl: &Sdl) -> Result<(Canvas<Window>, Texture), Box<dyn Error>> {
        let window = sdl
            .video()?
            .window("RetoEmu", WIDTH as u32 * SCALE, HEIGHT as u32 * SCALE)
            .position_centered()
            .always_on_top()
            .build()?;
        let mut canvas = window.into_canvas().present_vsync().build()?;
        canvas.set_logical_size(WIDTH as u32, HEIGHT as u32)?;
        canvas.set_integer_scale(true)?;
        let texture_creator = canvas.texture_creator();
        let mut texture =
            texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, WIDTH as u32, HEIGHT as u32)?;
        Ok((canvas, texture))
    }

    fn init_debug_thread() -> Receiver<String> {
        let (tx, commands) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            for line in std::io::stdin().lines().map_while(Result::ok) {
                if tx.send(line).is_err() {
                    break;
                }
            }
        });
        commands
    }

    pub fn run(&mut self, game_boy: &mut GameBoy) -> Result<(), Box<dyn Error>> {
        let mut next_frame = Instant::now();
        'running: loop {
            next_frame += FRAME_TIME;

            for event in self.event_pump.poll_iter() {
                if let Event::Quit { .. } | Event::KeyDown { scancode: Some(Scancode::Escape), .. } = event {
                    break 'running;
                }
            }

            while let Ok(mut cmd) = self.commands.try_recv() {
                if cmd.is_empty() {
                    cmd = self.last_cmd.clone();
                }
                self.handle_command(&cmd, game_boy);
            }

            if let Mode::Running = self.mode
                && let Some(reason) = game_boy.run_frame(self.dbg.as_mut())
            {
                println!("{reason}");
                self.pause();
            }

            self.draw(game_boy)?;

            let now = Instant::now();
            match next_frame.checked_duration_since(now) {
                Some(remaining) => sleep(remaining),
                None if now - next_frame > FRAME_TIME * 5 => next_frame = now,
                None => {}
            }
        }
        Ok(())
    }

    fn draw(&mut self, game_boy: &mut GameBoy) -> Result<(), Box<dyn Error>> {
        if game_boy.take_redraw() {
            self.texture.update(None, game_boy.pixels(), WIDTH * 3)?;
            self.canvas.copy(&self.texture, None, None)?;
            self.canvas.present();
        }
        Ok(())
    }

    fn handle_command(&mut self, cmd: &str, game_boy: &mut GameBoy) {
        let Some(dbg) = self.dbg.as_mut() else { return };
        let mut it = cmd.split_whitespace();
        self.last_cmd = cmd.to_string();
        match it.next() {
            Some("c") => {
                dbg.resume();
                self.mode = Mode::Running;
                return;
            }
            Some("s") => {
                let n = Self::parse_int(it.next());
                dbg.step(n);
                self.mode = Mode::Running;
                return;
            }
            Some("r") | Some("reg") => {
                game_boy.print_registers();
            }
            Some("b") => {
                let addr = Self::parse_addr(it.next());
                dbg.breakpoints.insert(addr);
            }
            Some("d") => {
                let addr = Self::parse_addr(it.next());
                dbg.breakpoints.remove(&addr);
            }
            Some("bl") => {
                println!("Breakpoints:");
                for b in dbg.breakpoints.iter() {
                    println!("{:04x}", b);
                }
            }
            Some(other) => {
                println!("unknown command: {other}");
            }
            None => (),
        }
        self.pause();
    }

    fn parse_addr(s: Option<&str>) -> u16 { s.and_then(|s| u16::from_str_radix(s, 16).ok()).unwrap_or(1_u16) }
    fn parse_int(s: Option<&str>) -> u32 { s.and_then(|s| s.parse().ok()).unwrap_or(1) }

    fn pause(&mut self) {
        self.mode = Mode::Paused;
        print!("(gbdb) ");
        let _ = std::io::stdout().flush();
    }
}
