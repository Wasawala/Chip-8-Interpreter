use sdl3::{
    Sdl, VideoSubsystem, event::Event, keyboard::Keycode, pixels::Color, render::Canvas,
    video::Window,
};
use std::time::Duration;

use crate::chip8::Interpreter;

pub struct WindowSpecs {
    name: String,
    width: u32,
    height: u32,
    centered: bool,
    fullscreen: bool,
}

impl WindowSpecs {
    pub fn new(name: String, width: u32, height: u32, centered: bool, fullscreen: bool) -> Self {
        Self {
            name,
            width,
            height,
            centered,
            fullscreen,
        }
    }
}
pub struct App {
    sdl_context: Sdl,
    video: VideoSubsystem,
    canvas: Canvas<Window>,
    interpreter: Interpreter,
}

impl App {
    pub fn new(specs: WindowSpecs, rom_path: &str) -> Self {
        let context = sdl3::init().unwrap();
        let video = context.video().unwrap();
        let canvas = Self::create_window(&video, specs);
        Self {
            sdl_context: context,
            video,
            canvas,
            interpreter: Interpreter::new(rom_path),
        }
    }

    fn create_window(video: &VideoSubsystem, specs: WindowSpecs) -> Canvas<Window> {
        let mut window_builder = video.window(&specs.name, specs.width, specs.height);
        if specs.centered {
            window_builder.position_centered();
        }
        if specs.fullscreen {
            window_builder.fullscreen();
        }

        let window = window_builder.build().unwrap();
        window.into_canvas()
    }

    pub fn run(&mut self) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();
        self.canvas.present();

        self.interpreter.dump_memory();

        let mut event_pump = self.sdl_context.event_pump().unwrap();
        'running: loop {
            self.canvas.clear();

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,

                    _ => {}
                }
            }

            self.interpreter
                .next_instruction(event_pump.keyboard_state());

            self.interpreter.draw(&mut self.canvas);
            self.canvas.present();
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
