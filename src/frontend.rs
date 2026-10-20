use std::error::Error;
use std::process::Command;
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
use crate::debugger::{DebugAction, Debugger};
use crate::gbc::{FRAME_TIME, GameBoy, HEIGHT, SCALE, WIDTH};

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
    terminal: Option<String>,
}

impl Frontend {
    pub fn new(args: Args, sdl: Sdl) -> Result<Self, Box<dyn Error>> {
        let mut event_pump = sdl.event_pump()?;

        let (canvas, texture) = Self::init_texture(&sdl)?;
        let mode = Mode::Running;
        let dbg = Debugger::new(&args);
        let commands = Self::init_debug_thread();
        let last_cmd = String::new();
        let terminal = std::env::var("__CFBundleIdentifier").ok();
        Ok(Self { sdl, event_pump, canvas, texture, dbg, mode, commands, last_cmd, terminal })
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
                if let Some(dbg) = self.dbg.as_mut() {
                    dbg.log(reason.to_string());
                }
                self.focus_terminal();
                self.pause(game_boy);
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
        self.last_cmd = cmd.to_string();
        match dbg.handle_command(cmd, game_boy) {
            DebugAction::Continue => {
                self.mode = Mode::Running;
                self.canvas.window_mut().raise();
            }
            DebugAction::Step => self.mode = Mode::Running,
            DebugAction::Stay => self.pause(game_boy),
        }
    }

    fn focus_terminal(&self) {
        if let Some(id) = &self.terminal {
            let _ = Command::new("open").args(["-b", id]).status();
        }
    }

    fn pause(&mut self, game_boy: &GameBoy) {
        self.mode = Mode::Paused;
        if let Some(dbg) = &self.dbg {
            dbg.draw(game_boy);
        }
    }
}
